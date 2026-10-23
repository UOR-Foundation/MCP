//! The completion layer, run against the generated lifecycle root: the values a
//! provider offers for an argument of a prompt or of a resource template, filtered by
//! the value typed so far, the bound of 100 values, the arguments already resolved and
//! the refusal of a reference or an argument the catalog does not declare.

mod common;

use common::{assert_declared, complete_request, run_register, send};
use serde_json::{json, Value};

fn complete(params: &Value) -> (u32, Value) {
    let (response, body) = send(&complete_request(params));
    (response.status, body)
}

fn prompt_ref(name: &str) -> Value {
    json!({"type": "ref/prompt", "name": name})
}

fn assert_refused(params: &Value) {
    let (status, body) = complete(params);
    assert_eq!(status, 400, "{params}: {body}");
    assert_eq!(body["error"]["code"], -32602, "{params}: {body}");
    assert!(body.get("result").is_none());
    assert_eq!(body["id"], 7);
}

// ---- CO-01: values for a prompt and for a template ----

#[test]
fn values_begin_with_the_typed_value_in_the_providers_order_co_01() {
    assert_eq!(run_register("CO-01"), 1);
    let (status, body) = complete(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg1", "value": "par"}
    }));
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        body["result"]["completion"],
        json!({"values": ["paris", "park", "party"], "total": 3, "hasMore": false})
    );
}

#[test]
fn an_empty_value_matches_every_candidate_co_01() {
    let (_, body) = complete(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg1", "value": ""}
    }));
    assert_eq!(body["result"]["completion"]["total"], 7);
}

#[test]
fn a_value_that_matches_nothing_gives_an_empty_list_co_01() {
    for value in ["zzz", "PAR", "paris!"] {
        let (status, body) = complete(&json!({
            "ref": prompt_ref("test_prompt_with_arguments"),
            "argument": {"name": "arg1", "value": value}
        }));
        assert_eq!(status, 200, "{body}");
        assert_eq!(
            body["result"]["completion"],
            json!({"values": [], "total": 0, "hasMore": false}),
            "{value}"
        );
    }
}

#[test]
fn a_resource_template_argument_is_completed_co_01() {
    let (status, body) = complete(&json!({
        "ref": {"type": "ref/resource", "uri": "test://template/{id}/data"},
        "argument": {"name": "id", "value": "12"}
    }));
    assert_eq!(status, 200, "{body}");
    assert_eq!(
        body["result"]["completion"],
        json!({"values": ["123", "124", "125"], "total": 3, "hasMore": false})
    );
}

#[test]
fn an_argument_without_candidates_gives_an_empty_list_co_01() {
    let (status, body) = complete(&json!({
        "ref": prompt_ref("test_prompt_with_embedded_resource"),
        "argument": {"name": "resourceUri", "value": "nothing://"}
    }));
    assert_eq!(status, 200, "{body}");
    assert_eq!(body["result"]["completion"]["values"], json!([]));
}

// ---- CO-02: the bound ----

#[test]
fn at_most_one_hundred_values_are_returned_co_02() {
    let (status, body) = complete(&json!({
        "ref": prompt_ref("test_prompt_with_embedded_resource"),
        "argument": {"name": "resourceUri", "value": "test://item/"}
    }));
    assert_eq!(status, 200, "{body}");
    let completion = &body["result"]["completion"];
    assert_eq!(completion["values"].as_array().expect("values").len(), 100);
    assert_eq!(completion["values"][0], "test://item/0");
    assert_eq!(completion["values"][99], "test://item/99");
    assert_eq!(completion["total"], 150);
    assert_eq!(completion["hasMore"], true);
}

#[test]
fn a_page_of_exactly_the_bound_has_no_more_co_02() {
    let (_, body) = complete(&json!({
        "ref": prompt_ref("test_prompt_with_embedded_resource"),
        "argument": {"name": "resourceUri", "value": "test://item/14"}
    }));
    let completion = &body["result"]["completion"];
    assert_eq!(completion["values"].as_array().expect("values").len(), 11);
    assert_eq!(completion["total"], 11);
    assert_eq!(completion["hasMore"], false);
}

// ---- CO-03: the context ----

#[test]
fn the_resolved_arguments_reach_the_provider_co_03() {
    let without = complete(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg2", "value": ""}
    }))
    .1;
    assert_eq!(
        without["result"]["completion"]["values"],
        json!(["world", "west"])
    );
    let with = complete(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg2", "value": ""},
        "context": {"arguments": {"arg1": "paris"}}
    }))
    .1;
    assert_eq!(
        with["result"]["completion"]["values"],
        json!(["france", "french"])
    );
    let other = complete(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg2", "value": "w"},
        "context": {"arguments": {"arg1": "london"}}
    }))
    .1;
    assert_eq!(
        other["result"]["completion"]["values"],
        json!(["world", "west"])
    );
}

// ---- CO-04: the refusals ----

#[test]
fn a_reference_to_an_unlisted_prompt_is_refused_co_04() {
    assert_refused(&json!({
        "ref": prompt_ref("no_such_prompt"),
        "argument": {"name": "arg1", "value": ""}
    }));
}

#[test]
fn an_argument_the_prompt_does_not_declare_is_refused_co_04() {
    assert_refused(&json!({
        "ref": prompt_ref("test_prompt_with_arguments"),
        "argument": {"name": "arg9", "value": ""}
    }));
    assert_refused(&json!({
        "ref": prompt_ref("test_simple_prompt"),
        "argument": {"name": "arg1", "value": ""}
    }));
}

#[test]
fn a_reference_to_an_unlisted_template_is_refused_co_04() {
    assert_refused(&json!({
        "ref": {"type": "ref/resource", "uri": "test://nothing/{id}"},
        "argument": {"name": "id", "value": ""}
    }));
    // A uri that matches the template is not the template.
    assert_refused(&json!({
        "ref": {"type": "ref/resource", "uri": "test://template/1/data"},
        "argument": {"name": "id", "value": ""}
    }));
}

#[test]
fn a_variable_the_template_does_not_declare_is_refused_co_04() {
    assert_refused(&json!({
        "ref": {"type": "ref/resource", "uri": "test://template/{id}/data"},
        "argument": {"name": "other", "value": ""}
    }));
    assert_refused(&json!({
        "ref": {"type": "ref/resource", "uri": "test://template/{id}/data"},
        "argument": {"name": "i", "value": ""}
    }));
}

#[test]
fn a_request_without_a_reference_or_an_argument_is_invalid_params_co_04() {
    assert_refused(&json!({"argument": {"name": "arg1", "value": ""}}));
    assert_refused(&json!({"ref": prompt_ref("test_simple_prompt")}));
    assert_refused(&json!({
        "ref": {"type": "ref/other", "name": "x"},
        "argument": {"name": "a", "value": ""}
    }));
}

// ---- CO-05: the theorems ----

#[test]
fn the_completion_layer_states_its_theorems_co_05() {
    assert_declared(
        &["CompletionLaws"],
        &[
            "a_page_holds_at_most_the_bound",
            "the_bound_is_one_hundred",
            "a_page_states_its_total",
            "a_page_states_whether_more_remain",
            "a_candidate_must_begin_with_the_value",
            "an_unknown_prompt_reference_is_refused",
            "an_undeclared_argument_is_refused",
            "an_unknown_template_reference_is_refused",
        ],
    );
}
