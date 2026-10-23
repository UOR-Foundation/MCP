//! The resources layer, run against the generated lifecycle root: the catalog of
//! direct resources and of resource templates, reads of both, the refusal of a uri
//! the catalog does not know (SEP-2164), the template rule, the faults of a catalog,
//! the results the core refuses to write, and the same on the second root under a
//! very large input.
//!
//! The requests the pinned scenarios send are the vectors of `vectors/resources.json`;
//! the tests here add the requests the scenarios do not send. Each is a falsifying
//! probe authored from the specification text the register cites, never a verdict.

mod common;

use common::{
    assert_declared, constructors_of, crowd_checked, list_request, read_request, request_with,
    run_register, send, source, standard,
};
use serde_json::{json, Value};

/// The uris of the fixture catalog, in the order `Fixture.fixtureResources` declares them.
const URIS: [&str; 4] = [
    "test://static-text",
    "test://static-binary",
    "test://bad-empty-contents",
    "test://bad-blob",
];

/// The 1x1 PNG the suite's reference server serves as its binary resource.
const TEST_IMAGE_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==";

fn listed(method: &str, member: &str) -> Vec<Value> {
    let (response, body) = send(&list_request(method, None));
    assert_eq!(response.status, 200, "{body}");
    body["result"][member]
        .as_array()
        .unwrap_or_else(|| panic!("a {member} array in {body}"))
        .clone()
}

fn assert_not_found(uri: &str) {
    let (response, body) = send(&read_request(uri));
    assert_eq!(response.status, 400, "{uri}: {body}");
    assert_eq!(body["error"]["code"], -32602, "{uri}: {body}");
    assert_eq!(body["error"]["data"]["uri"], uri, "{uri}: {body}");
    assert!(body.get("result").is_none(), "{uri}: {body}");
    assert_eq!(body["id"], 5);
}

// ---- RS-01: the catalog and its order ----

#[test]
fn resources_list_answers_the_catalog_in_its_declared_order_rs_01() {
    assert_eq!(run_register("RS-01"), 1);
    let resources = listed("resources/list", "resources");
    let uris: Vec<&str> = resources
        .iter()
        .map(|r| r["uri"].as_str().expect("a uri"))
        .collect();
    assert_eq!(uris, URIS);
    assert_eq!(resources[0]["name"], "static-text");
    assert_eq!(resources[0]["title"], "Static Text Resource");
    assert_eq!(
        resources[0]["description"],
        "A static text resource for testing"
    );
    assert_eq!(resources[0]["mimeType"], "text/plain");
    assert_eq!(resources[1]["name"], "static-binary");
    assert_eq!(resources[1]["title"], "Static Binary Resource");
    assert_eq!(resources[1]["mimeType"], "image/png");
    for resource in &resources {
        assert!(resource["name"].is_string(), "{resource}");
    }
}

#[test]
fn the_resource_order_does_not_change_between_requests_rs_01() {
    let first = common::exchange_checked(&list_request("resources/list", None)).expect("a frame");
    for _ in 0..3 {
        let again =
            common::exchange_checked(&list_request("resources/list", None)).expect("a frame");
        assert_eq!(first.body, again.body);
    }
}

#[test]
fn a_cursor_on_resources_list_is_refused_as_invalid_params_rs_01() {
    let (response, body) = send(&list_request("resources/list", Some("page-2")));
    assert_eq!(response.status, 400);
    assert_eq!(body["error"]["code"], -32602);
    assert_eq!(body["id"], 1);
}

// ---- RS-02 and RS-03: reading ----

#[test]
fn a_text_resource_is_read_as_text_rs_02() {
    assert_eq!(run_register("RS-02"), 1);
    let (_, body) = send(&read_request("test://static-text"));
    let contents = body["result"]["contents"].as_array().expect("contents");
    assert_eq!(contents.len(), 1);
    assert_eq!(body["result"]["resultType"], "complete");
}

#[test]
fn a_binary_resource_is_read_as_a_blob_rs_03() {
    assert_eq!(run_register("RS-03"), 1);
    let (_, body) = send(&read_request("test://static-binary"));
    let entry = &body["result"]["contents"][0];
    assert_eq!(entry["blob"], TEST_IMAGE_BASE64);
    assert!(entry.get("text").is_none(), "{entry}");
}

// ---- RS-04: the templates ----

#[test]
fn resources_templates_list_answers_the_catalog_in_its_declared_order_rs_04() {
    let templates = listed("resources/templates/list", "resourceTemplates");
    assert_eq!(templates.len(), 1, "{templates:?}");
    let template = &templates[0];
    assert_eq!(template["name"], "template");
    assert_eq!(template["title"], "Resource Template");
    assert_eq!(template["uriTemplate"], "test://template/{id}/data");
    assert_eq!(
        template["description"],
        "A resource template with parameter substitution"
    );
    assert_eq!(template["mimeType"], "application/json");
}

#[test]
fn a_cursor_on_templates_list_is_refused_as_invalid_params_rs_04() {
    let (response, body) = send(&list_request("resources/templates/list", Some("next")));
    assert_eq!(response.status, 400);
    assert_eq!(body["error"]["code"], -32602);
}

// ---- RS-05: reading through a template ----

#[test]
fn a_uri_that_matches_a_template_is_read_with_its_variable_bound_rs_05() {
    assert_eq!(run_register("RS-05"), 1);
    for id in ["abc", "0", "a-b_c.d~e", "é", "a b"] {
        let uri = format!("test://template/{id}/data");
        let (response, body) = send(&read_request(&uri));
        assert_eq!(response.status, 200, "{uri}: {body}");
        let entry = &body["result"]["contents"][0];
        assert_eq!(entry["uri"], uri);
        let text: Value = serde_json::from_str(entry["text"].as_str().expect("text"))
            .expect("the fixture writes JSON");
        assert_eq!(
            text,
            json!({"id": id, "templateTest": true, "data": format!("Data for ID: {id}")})
        );
    }
}

#[test]
fn a_bound_value_is_escaped_in_the_json_the_provider_writes_rs_05() {
    let uri = "test://template/a\"b\\c/data";
    let (response, body) = send(&read_request(uri));
    assert_eq!(response.status, 200, "{body}");
    let text: Value = serde_json::from_str(
        body["result"]["contents"][0]["text"]
            .as_str()
            .expect("text"),
    )
    .expect("the text is JSON");
    assert_eq!(text["id"], "a\"b\\c");
}

#[test]
fn a_uri_that_does_not_follow_the_template_is_not_found_rs_05() {
    for uri in [
        "test://template//data",
        "test://template/1/2/data",
        "test://template/1/data/",
        "test://template/1/dat",
        "test://template/1/",
        "test://template/data",
        "test://template/1",
        "test://template",
        "test://Template/1/data",
        "xtest://template/1/data",
        "test://template/1/data?x=1",
    ] {
        assert_not_found(uri);
    }
}

// ---- RS-06: SEP-2164 ----

#[test]
fn a_uri_the_catalog_does_not_know_is_not_found_rs_06() {
    assert_eq!(run_register("RS-06"), 1);
    for uri in [
        "test://",
        "test://static",
        "test://static-text/",
        "test://static-text2",
        "TEST://static-text",
        "test://Static-Text",
        "test://example-resource",
        "file:///etc/passwd",
        "test://nöt-there",
    ] {
        assert_not_found(uri);
    }
}

#[test]
fn the_not_found_error_names_the_uri_with_every_character_it_has_rs_06() {
    assert_not_found("test://quote\"back\\slash\ttab/\u{1F600}");
}

#[test]
fn an_unknown_resource_with_a_string_id_echoes_it_rs_06() {
    let request = common::post_owned(
        &standard("resources/read", Some("nowhere://x")),
        &format!(
            r#"{{"jsonrpc":"2.0","id":"abc","method":"resources/read","params":{{"uri":"nowhere://x","_meta":{}}}}}"#,
            common::META
        ),
    );
    let (response, body) = send(&request);
    assert_eq!(response.status, 400);
    assert_eq!(body["id"], "abc");
    assert_eq!(body["error"]["code"], -32602);
}

#[test]
fn a_read_without_a_text_uri_is_invalid_params_rs_06() {
    for params in [
        json!({}),
        json!({"uri": 7}),
        json!({"uri": null}),
        json!({"uri": ["a"]}),
    ] {
        let (response, body) = send(&request_with("resources/read", &params, Some("x"), "5"));
        assert_eq!(response.status, 400, "{params}: {body}");
        assert_eq!(body["error"]["code"], -32602, "{params}: {body}");
        assert_eq!(body["id"], 5);
    }
}

// ---- RS-07 and RS-10: the template rule and the layer are stated in theorems ----

#[test]
fn the_template_rule_is_stated_in_kernel_checked_theorems_rs_07() {
    assert_declared(
        &["Templates"],
        &[
            "a_uri_that_follows_the_template_is_matched",
            "a_variable_is_bound_to_the_text_between_its_literals",
            "a_final_variable_takes_the_rest_of_the_uri",
            "two_variables_are_bound_in_order",
            "a_uri_with_a_missing_literal_is_not_matched",
            "an_empty_variable_is_not_matched",
            "a_trailing_byte_is_not_matched",
            "a_short_tail_is_not_matched",
            "a_variable_does_not_cross_the_byte_that_follows_it",
        ],
    );
}

#[test]
fn the_resources_layer_states_its_theorems_rs_10() {
    assert_declared(
        &["ResourceLaws"],
        &[
            "the_catalog_lists_its_resources_in_the_declared_order",
            "resources_list_answers_with_the_catalog_in_its_declared_order",
            "the_catalog_lists_its_templates_in_the_declared_order",
            "resource_templates_list_answers_with_the_catalog_in_its_declared_order",
            "an_unknown_resource_is_resource_not_found",
            "an_unknown_resource_has_the_invalid_params_code",
            "an_unknown_resource_names_its_uri_in_the_error_data",
            "a_listed_resource_is_read",
            "an_empty_read_result_is_refused",
            "a_blob_that_is_not_canonical_base64_is_refused",
            "a_cursor_is_an_invalid_cursor",
        ],
    );
}

// ---- RS-08: the faults of a catalog ----

#[test]
fn every_fault_of_a_resource_or_template_catalog_has_a_negative_theorem_rs_08() {
    let faults = constructors_of("Catalogs", "CatalogFault");
    let resourceful: Vec<&String> = faults
        .iter()
        .filter(|f| !f.starts_with("prompt") && !f.starts_with("argument"))
        .collect();
    assert_eq!(resourceful.len(), 10, "{faults:?}");
    let laws = source("ResourceLaws");
    for fault in resourceful {
        assert!(
            laws.contains(&format!("CatalogFault.{fault}")),
            "no theorem in ResourceLaws states CatalogFault.{fault}"
        );
    }
}

#[test]
fn a_provider_refusal_of_the_resources_layer_is_typed_and_stated_rs_08() {
    let refusals = constructors_of("Refusal", "ProviderRefusal");
    let laws = source("ResourceLaws") + &source("Refusal") + &source("Status");
    for refusal in ["resourceNotFound", "badResourceOutput"] {
        assert!(
            refusals.iter().any(|r| r == refusal),
            "{refusal} is declared"
        );
        assert!(
            laws.contains(&format!("ProviderRefusal.{refusal}")),
            "no theorem states {refusal}"
        );
    }
}

// ---- RS-09: the results the core refuses to write ----

fn assert_bad_output(uri: &str) {
    let (response, body) = send(&read_request(uri));
    assert_eq!(response.status, 500, "{uri}: {body}");
    assert_eq!(body["error"]["code"], -32603, "{uri}: {body}");
    assert_eq!(body["error"]["message"], "Internal error");
    assert!(body["error"].get("data").is_none(), "{body}");
    assert!(body.get("result").is_none());
}

#[test]
fn an_empty_contents_array_is_refused_rs_09() {
    assert_bad_output("test://bad-empty-contents");
}

#[test]
fn a_blob_that_is_not_canonical_base64_is_refused_rs_09() {
    assert_bad_output("test://bad-blob");
}

// ---- RS-11: the capabilities ----

#[test]
fn discover_declares_the_capabilities_the_fixture_offers_rs_11() {
    let body = common::rpc(
        "server/discover",
        &format!(r#"{{"_meta":{}}}"#, common::META),
        "1",
    );
    let request = common::post_owned(&standard("server/discover", None), &body);
    let (_, json) = send(&request);
    let capabilities = &json["result"]["capabilities"];
    for name in ["tools", "resources", "prompts", "completions"] {
        assert!(capabilities[name].is_object(), "{name}: {capabilities}");
    }
    assert!(capabilities.get("logging").is_none(), "{capabilities}");
}

#[test]
fn the_dispatch_offers_each_method_only_to_the_capability_that_declares_it_rs_11() {
    assert_declared(
        &["Dispatch"],
        &[
            "unoffered_method_never_reaches_the_provider",
            "dispatch_gates_on_the_declared_capability",
            "a_provider_without_capabilities_answers_only_discovery",
        ],
    );
}

// ---- RS-12: the second root, over a catalog of many resources, templates and prompts ----

/// The count of each family the crowd root declares (`Crowd.crowdSize`).
const CROWD: usize = 60;

#[test]
fn a_catalog_of_many_resources_templates_and_prompts_is_listed_rs_12() {
    for (method, member, count) in [
        ("resources/list", "resources", CROWD + 1),
        ("resources/templates/list", "resourceTemplates", CROWD + 1),
        ("prompts/list", "prompts", CROWD),
    ] {
        let response = crowd_checked(&list_request(method, None));
        assert_eq!(response.status, 200, "{method}");
        let body = common::json_of(&response);
        assert_eq!(
            body["result"][member].as_array().expect("an array").len(),
            count,
            "{method}"
        );
    }
}

#[test]
fn a_read_of_a_very_long_uri_is_answered_rs_12() {
    let key = "k".repeat(256 * 1024);
    let uri = format!("crowd://t7/{key}");
    let response = crowd_checked(&read_request(&uri));
    assert_eq!(response.status, 200);
    let body = common::json_of(&response);
    assert_eq!(body["result"]["contents"][0]["uri"], uri);
}

#[test]
fn a_very_long_uri_that_matches_nothing_is_refused_rs_12() {
    let uri = format!("crowd://zzz/{}", "z".repeat(256 * 1024));
    let response = crowd_checked(&read_request(&uri));
    assert_eq!(response.status, 400);
    let body = common::json_of(&response);
    assert_eq!(body["error"]["data"]["uri"], uri);
}

#[test]
fn a_template_with_many_variables_is_read_rs_12() {
    let values: Vec<String> = (0..300).map(|i| format!("v{i}")).collect();
    let uri = format!("crowd://many/{}", values.join("/"));
    let response = crowd_checked(&read_request(&uri));
    assert_eq!(response.status, 200);
    let body = common::json_of(&response);
    assert_eq!(body["result"]["contents"][0]["uri"], uri);
    // One fewer value than the template has variables does not match.
    let short = format!("crowd://many/{}", values[..299].join("/"));
    let response = crowd_checked(&read_request(&short));
    assert_eq!(response.status, 400);
}

#[test]
fn a_resource_with_a_very_large_blob_is_read_rs_12() {
    let response = crowd_checked(&read_request("crowd://blob"));
    assert_eq!(response.status, 200);
    let body = common::json_of(&response);
    let blob = body["result"]["contents"][0]["blob"]
        .as_str()
        .expect("a blob");
    assert!(blob.len() >= 256 * 1024, "{}", blob.len());
    assert!(blob.bytes().all(|b| b == b'A'));
}
