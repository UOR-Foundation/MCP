//! The prompts layer, run against the generated lifecycle root: the catalog and its
//! order, the messages a prompt builds from its arguments (text, embedded resource,
//! image), the refusal of an unknown prompt and of a bad argument list, the faults of
//! a catalog and the results the core refuses to write.
//!
//! The requests the pinned scenarios send are the vectors of `vectors/resources.json`;
//! the tests here add the requests the scenarios do not send.

mod common;

use common::{
    assert_declared, constructors_of, list_request, prompt_request, request_with, run_register,
    send, source,
};
use serde_json::{json, Value};

/// The names of the fixture catalog, in the order `Fixture.fixturePrompts` declares them.
const NAMES: [&str; 5] = [
    "test_simple_prompt",
    "test_prompt_with_arguments",
    "test_prompt_with_embedded_resource",
    "test_prompt_with_image",
    "test_prompt_bad_payload",
];

fn listed() -> Vec<Value> {
    let (response, body) = send(&list_request("prompts/list", None));
    assert_eq!(response.status, 200, "{body}");
    body["result"]["prompts"]
        .as_array()
        .expect("prompts")
        .clone()
}

fn assert_invalid_params(name: &str, arguments: Option<&Value>) {
    let (response, body) = send(&prompt_request(name, arguments));
    assert_eq!(response.status, 400, "{name} {arguments:?}: {body}");
    assert_eq!(
        body["error"]["code"], -32602,
        "{name} {arguments:?}: {body}"
    );
    assert!(body.get("result").is_none(), "{body}");
    assert_eq!(body["id"], 6);
}

// ---- PM-01: the catalog and its order ----

#[test]
fn prompts_list_answers_the_catalog_in_its_declared_order_pm_01() {
    assert_eq!(run_register("PM-01"), 1);
    let prompts = listed();
    let names: Vec<&str> = prompts
        .iter()
        .map(|p| p["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(names, NAMES);
    assert_eq!(prompts[0]["title"], "Simple Test Prompt");
    assert_eq!(
        prompts[0]["description"],
        "A simple prompt without arguments"
    );
    assert!(prompts[0].get("arguments").is_none(), "{}", prompts[0]);
    assert_eq!(
        prompts[1]["arguments"],
        json!([
            {"name": "arg1", "description": "First test argument", "required": true},
            {"name": "arg2", "description": "Second test argument", "required": true}
        ])
    );
    assert_eq!(
        prompts[2]["arguments"],
        json!([{"name": "resourceUri", "description": "URI of the resource to embed", "required": true}])
    );
    assert!(prompts[3].get("arguments").is_none());
    for prompt in &prompts {
        assert!(prompt["description"].is_string(), "{prompt}");
    }
}

#[test]
fn the_prompt_order_does_not_change_between_requests_pm_01() {
    let first = common::exchange_checked(&list_request("prompts/list", None)).expect("a frame");
    for _ in 0..3 {
        let again = common::exchange_checked(&list_request("prompts/list", None)).expect("a frame");
        assert_eq!(first.body, again.body);
    }
}

#[test]
fn a_cursor_on_prompts_list_is_refused_as_invalid_params_pm_01() {
    let (response, body) = send(&list_request("prompts/list", Some("page-2")));
    assert_eq!(response.status, 400);
    assert_eq!(body["error"]["code"], -32602);
    assert_eq!(body["id"], 1);
}

// ---- PM-02 to PM-05: the messages ----

#[test]
fn a_prompt_without_arguments_answers_its_messages_pm_02() {
    assert_eq!(run_register("PM-02"), 1);
    let (_, body) = send(&prompt_request("test_simple_prompt", None));
    assert_eq!(body["result"]["resultType"], "complete");
    assert!(
        body["result"].get("ttlMs").is_none(),
        "a prompt is not cached"
    );
}

#[test]
fn an_empty_arguments_object_is_the_same_as_none_pm_02() {
    let (response, body) = send(&prompt_request("test_simple_prompt", Some(&json!({}))));
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(
        body["result"]["messages"][0]["content"]["text"],
        "This is a simple prompt for testing."
    );
}

#[test]
fn every_supplied_argument_reaches_the_prompt_in_declaration_order_pm_03() {
    assert_eq!(run_register("PM-03"), 1);
    // The arguments arrive in the order the client wrote them; the prompt receives them
    // in the order it declares them.
    let (_, body) = send(&prompt_request(
        "test_prompt_with_arguments",
        Some(&json!({"arg2": "second", "arg1": "first"})),
    ));
    assert_eq!(
        body["result"]["messages"][0]["content"]["text"],
        "Prompt with arguments: arg1='first', arg2='second'"
    );
}

#[test]
fn argument_values_are_kept_exactly_pm_03() {
    let arg1 = "q\"uote \\ é \u{1F600}";
    let (response, body) = send(&prompt_request(
        "test_prompt_with_arguments",
        Some(&json!({"arg1": arg1, "arg2": ""})),
    ));
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(
        body["result"]["messages"][0]["content"]["text"],
        format!("Prompt with arguments: arg1='{arg1}', arg2=''")
    );
}

#[test]
fn an_embedded_resource_carries_the_uri_the_client_gave_pm_04() {
    assert_eq!(run_register("PM-04"), 1);
    let (_, body) = send(&prompt_request(
        "test_prompt_with_embedded_resource",
        Some(&json!({"resourceUri": "test://other"})),
    ));
    assert_eq!(
        body["result"]["messages"][0]["content"]["resource"]["uri"],
        "test://other"
    );
}

#[test]
fn an_image_message_carries_canonical_base64_pm_05() {
    assert_eq!(run_register("PM-05"), 1);
}

// ---- PM-06: the refusals ----

#[test]
fn an_unknown_prompt_is_invalid_params_pm_06() {
    assert_invalid_params("no_such_prompt", None);
    assert_invalid_params("TEST_SIMPLE_PROMPT", None);
    assert_invalid_params("test_simple_prompt2", Some(&json!({})));
}

#[test]
fn a_missing_required_argument_is_invalid_params_pm_06() {
    assert_invalid_params("test_prompt_with_arguments", None);
    assert_invalid_params("test_prompt_with_arguments", Some(&json!({})));
    assert_invalid_params("test_prompt_with_arguments", Some(&json!({"arg1": "only"})));
    assert_invalid_params("test_prompt_with_arguments", Some(&json!({"arg2": "only"})));
    assert_invalid_params("test_prompt_with_embedded_resource", Some(&json!({})));
}

#[test]
fn an_undeclared_argument_is_invalid_params_pm_06() {
    assert_invalid_params(
        "test_prompt_with_arguments",
        Some(&json!({"arg1": "a", "arg2": "b", "arg3": "c"})),
    );
    assert_invalid_params("test_simple_prompt", Some(&json!({"x": "y"})));
    assert_invalid_params("test_prompt_with_image", Some(&json!({"x": "y"})));
}

#[test]
fn an_argument_that_is_not_text_is_invalid_params_pm_06() {
    assert_invalid_params(
        "test_prompt_with_arguments",
        Some(&json!({"arg1": 1, "arg2": "b"})),
    );
    assert_invalid_params("test_prompt_with_arguments", Some(&json!(["a", "b"])));
}

#[test]
fn a_get_without_a_text_name_is_invalid_params_pm_06() {
    for params in [json!({}), json!({"name": 3})] {
        let (response, body) = send(&request_with("prompts/get", &params, Some("x"), "6"));
        assert_eq!(response.status, 400, "{params}: {body}");
        assert_eq!(body["error"]["code"], -32602);
    }
}

// ---- PM-07: the results the core refuses to write ----

#[test]
fn a_payload_that_is_not_canonical_base64_is_refused_pm_07() {
    let (response, body) = send(&prompt_request("test_prompt_bad_payload", None));
    assert_eq!(response.status, 500, "{body}");
    assert_eq!(body["error"]["code"], -32603);
    assert_eq!(body["error"]["message"], "Internal error");
    assert!(body["error"].get("data").is_none(), "{body}");
    assert!(body.get("result").is_none());
}

#[test]
fn the_refusal_of_a_prompt_payload_is_typed_and_stated_pm_07() {
    let refusals = constructors_of("Refusal", "ProviderRefusal");
    assert!(refusals.iter().any(|r| r == "badPromptOutput"));
    let laws = source("PromptLaws") + &source("Refusal") + &source("Status");
    assert!(laws.contains("ProviderRefusal.badPromptOutput"));
}

// ---- PM-08 and PM-09: the faults of a catalog, and the theorems ----

#[test]
fn every_fault_of_a_prompt_catalog_has_a_negative_theorem_pm_08() {
    let faults = constructors_of("Catalogs", "CatalogFault");
    let prompt_faults: Vec<&String> = faults
        .iter()
        .filter(|f| f.starts_with("prompt") || f.starts_with("argument"))
        .collect();
    assert_eq!(prompt_faults.len(), 4, "{faults:?}");
    let laws = source("PromptLaws");
    for fault in prompt_faults {
        assert!(
            laws.contains(&format!("CatalogFault.{fault}")),
            "no theorem in PromptLaws states CatalogFault.{fault}"
        );
    }
}

#[test]
fn the_prompts_layer_states_its_theorems_pm_09() {
    assert_declared(
        &["PromptLaws"],
        &[
            "the_catalog_lists_its_prompts_in_the_declared_order",
            "prompts_list_answers_with_the_catalog_in_its_declared_order",
            "an_unknown_prompt_is_unknown_prompt",
            "an_unknown_prompt_has_the_invalid_params_code",
            "a_missing_required_argument_is_invalid_arguments",
            "an_undeclared_argument_is_invalid_arguments",
            "binding_keeps_every_supplied_argument",
            "binding_follows_the_declaration_order",
            "a_prompt_payload_that_is_not_canonical_base64_is_refused",
        ],
    );
}
