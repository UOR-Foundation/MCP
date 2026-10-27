//! The tools layer, run against the generated lifecycle root: the catalog and its
//! order, the content a call returns, argument and output validation, the
//! parameter headers derived from the catalog, progress, the rate limit and the
//! sanitization of what the core writes.
//!
//! The requests the pinned scenarios send are the vectors of `vectors/tools.json`;
//! the tests here add the requests the scenarios do not send. Each is a falsifying
//! probe authored from the specification text the register cites, never a verdict.

mod common;

use common::{
    after_calls, call, header, post_owned, root, rpc, run_register, send, standard, suite_text,
    META,
};
use lifecycle_oracle::{Request, Response};
use serde_json::{json, Value};

/// The names of the fixture catalog, in the order `Fixture.fixtureTools` declares them.
const NAMES: [&str; 35] = [
    "echo",
    "execute_sql",
    "test_missing_capability",
    "test_simple_text",
    "test_image_content",
    "test_audio_content",
    "test_embedded_resource",
    "test_multiple_content_types",
    "test_error_handling",
    "test_tool_with_progress",
    "json_schema_2020_12_tool",
    "test_arguments",
    "test_nested_header",
    "test_structured_output",
    "test_bad_structured_output",
    "test_bad_payload",
    "test_unsafe_text",
    "test_backward_progress",
    "test_streaming_elicitation",
    "test_logging_tool",
    "test_input_required_result_elicitation",
    "test_input_required_result_sampling",
    "test_input_required_result_list_roots",
    "test_input_required_result_request_state",
    "test_input_required_result_multiple_inputs",
    "test_input_required_result_multi_round",
    "test_input_required_result_tampered_state",
    "test_input_required_result_capabilities",
    "greet",
    "slow_compute",
    "failing_job",
    "protocol_error_job",
    "confirm_delete",
    "multi_input",
    "test_tool_with_task",
];

fn list(params: &str) -> Request {
    post_owned(
        &standard("tools/list", None),
        &rpc("tools/list", params, "1"),
    )
}

fn listed() -> Vec<Value> {
    let (response, body) = send(&list(&format!(r#"{{"_meta":{META}}}"#)));
    assert_eq!(response.status, 200);
    body["result"]["tools"]
        .as_array()
        .expect("a tools array")
        .clone()
}

/// A tools/call of `name` with `arguments`, the standard headers and `extra` headers.
fn invoke(name: &str, arguments: &Value, extra: &[(&str, &str)]) -> Request {
    let mut headers = standard("tools/call", Some(name));
    for (n, v) in extra {
        headers.push(((*n).to_owned(), (*v).to_owned()));
    }
    post_owned(&headers, &call(name, &arguments.to_string(), "7"))
}

/// A tools/call of `name` whose `_meta` carries `token` as its progressToken, from a client
/// that accepts both a JSON body and an event stream, as the suite's client does.
fn invoke_with_token(name: &str, arguments: &Value, token: &Value) -> Request {
    let mut headers = standard("tools/call", Some(name));
    headers.push((
        "Accept".to_owned(),
        "application/json, text/event-stream".to_owned(),
    ));
    let body = format!(
        r#"{{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{{"name":"{name}","arguments":{arguments},"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{{}},"progressToken":{token}}}}}}}"#
    );
    post_owned(&headers, &body)
}

/// The progress notifications the answer delivered: the events of the stream before the
/// result (issue 11; before it they travelled beside the body in the frame).
fn notes(response: &Response) -> Vec<Value> {
    common::notes_of(response)
}

/// The text of the first content block of a result.
fn text_of(body: &Value) -> &str {
    body["result"]["content"][0]["text"]
        .as_str()
        .expect("a text block")
}

fn assert_tool_error(body: &Value, expected: &str) {
    assert_eq!(body["result"]["isError"], true, "{body}");
    assert_eq!(body["result"]["resultType"], "complete", "{body}");
    assert_eq!(text_of(body), expected, "{body}");
}

// ---- TL-01: the catalog and its order ----

#[test]
fn tools_list_answers_the_catalog_in_its_declared_order_tl_01() {
    // TS-01, and TS-12: the suite lists the tools twice more and compares the order
    assert_eq!(run_register("TL-01"), 2);
    let tools = listed();
    for _ in 0..2 {
        assert_eq!(listed(), tools, "consecutive tools/list answers differ");
    }
    let names: Vec<&str> = tools
        .iter()
        .map(|t| t["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(names, NAMES);
    for tool in &tools {
        assert!(tool["description"].is_string(), "{tool}");
        assert_eq!(tool["inputSchema"]["type"], "object", "{tool}");
        let structured = ["test_structured_output", "test_bad_structured_output"];
        assert_eq!(
            tool.get("outputSchema").is_some(),
            structured.contains(&tool["name"].as_str().expect("name")),
            "{tool}"
        );
    }
}

#[test]
fn the_order_does_not_change_between_requests_tl_01() {
    let first =
        common::exchange_checked(&list(&format!(r#"{{"_meta":{META}}}"#))).expect("a frame");
    for _ in 0..3 {
        let again =
            common::exchange_checked(&list(&format!(r#"{{"_meta":{META}}}"#))).expect("a frame");
        assert_eq!(first.body, again.body);
    }
}

#[test]
fn a_cursor_is_refused_as_invalid_params_tl_01() {
    let (response, body) = send(&list(&format!(r#"{{"cursor":"page-2","_meta":{META}}}"#)));
    assert_eq!(response.status, 400);
    assert_eq!(body["error"]["code"], -32602);
    assert_eq!(body["id"], 1);
}

// ---- TL-02: JSON Schema 2020-12 ----

#[test]
fn the_focal_schema_survives_listing_member_for_member_tl_02() {
    assert_eq!(run_register("TL-02"), 1);
}

#[test]
fn a_schema_that_names_a_dialect_names_2020_12_tl_02() {
    for tool in listed() {
        if let Some(dialect) = tool["inputSchema"].get("$schema") {
            assert_eq!(dialect, "https://json-schema.org/draft/2020-12/schema");
        }
    }
}

#[test]
fn the_focal_schema_is_enforced_by_its_conditionals_and_compositions_tl_02() {
    let tool = "json_schema_2020_12_tool";
    let accepted = [
        json!({"name": "a", "email": "e", "contactMethod": "email"}),
        json!({"contactMethod": "phone", "phone": "1"}),
        json!({"contactMethod": "phone", "phone": "1", "address": {"city": "c"}}),
    ];
    for arguments in accepted {
        let (response, body) = send(&invoke(tool, &arguments, &[]));
        assert_eq!(response.status, 200);
        assert!(
            body["result"].get("isError").is_none(),
            "{arguments}: {body}"
        );
    }
    let refused = [
        // allOf / anyOf: neither phone nor email; the composition names itself
        (
            json!({"name": "a"}),
            "Invalid arguments: the arguments object fails anyOf",
        ),
        // if / then: a phone contact method needs a phone
        (
            json!({"contactMethod": "phone", "email": "e"}),
            "Invalid arguments: the arguments object fails required",
        ),
        // if / else: otherwise an email (no contact method does not match the if)
        (
            json!({"contactMethod": "email", "phone": "1"}),
            "Invalid arguments: the arguments object fails required",
        ),
        (
            json!({"name": "a", "phone": "1"}),
            "Invalid arguments: the arguments object fails required",
        ),
        // additionalProperties
        (
            json!({"email": "e", "fax": "2"}),
            "Invalid arguments: the arguments object fails additionalProperties",
        ),
        // $ref into $defs
        (
            json!({"contactMethod": "phone", "phone": "1", "address": {"city": 3}}),
            "Invalid arguments: /address/city fails type",
        ),
        // enum
        (
            json!({"contactMethod": "post", "email": "e"}),
            "Invalid arguments: /contactMethod fails enum",
        ),
    ];
    for (arguments, text) in refused {
        let (response, body) = send(&invoke(tool, &arguments, &[]));
        assert_eq!(response.status, 200, "{arguments}");
        assert_tool_error(&body, text);
    }
}

// ---- TL-03 .. TL-08: the content a call returns ----

#[test]
fn a_text_tool_returns_its_text_and_no_error_member_tl_03() {
    assert_eq!(run_register("TL-03"), 1);
    // the arguments member is optional, and an empty object is the same call
    let omitted = rpc(
        "tools/call",
        &format!(r#"{{"name":"test_simple_text","_meta":{META}}}"#),
        "3",
    );
    let (_, a) = send(&post_owned(
        &standard("tools/call", Some("test_simple_text")),
        &omitted,
    ));
    let (_, b) = send(&invoke("test_simple_text", &json!({}), &[]));
    assert_eq!(a["result"], b["result"]);
    assert!(a["result"].get("structuredContent").is_none());
}

fn vendored_constant(name: &str) -> String {
    let text = suite_text("examples/servers/typescript/everything-server.ts");
    let at = text
        .find(&format!("const {name} ="))
        .unwrap_or_else(|| panic!("{name} is in the reference fixture"));
    let rest = &text[at..];
    let open = rest.find('\'').expect("a quoted value") + 1;
    let close = open + rest[open..].find('\'').expect("a closing quote");
    rest[open..close].to_owned()
}

#[test]
fn the_image_is_the_reference_fixtures_image_tl_04() {
    assert_eq!(run_register("TL-04"), 1);
    let (_, body) = send(&invoke("test_image_content", &json!({}), &[]));
    assert_eq!(
        body["result"]["content"][0]["data"],
        vendored_constant("TEST_IMAGE_BASE64")
    );
}

#[test]
fn the_audio_is_the_reference_fixtures_audio_tl_05() {
    assert_eq!(run_register("TL-05"), 1);
    let (_, body) = send(&invoke("test_audio_content", &json!({}), &[]));
    assert_eq!(
        body["result"]["content"][0]["data"],
        vendored_constant("TEST_AUDIO_BASE64")
    );
}

#[test]
fn the_embedded_resource_has_text_and_no_blob_tl_06() {
    assert_eq!(run_register("TL-06"), 1);
    let (_, body) = send(&invoke("test_embedded_resource", &json!({}), &[]));
    let resource = &body["result"]["content"][0]["resource"];
    assert!(resource.get("blob").is_none(), "{resource}");
    assert!(resource["text"].is_string());
}

#[test]
fn mixed_content_keeps_the_order_the_tool_produced_tl_07() {
    assert_eq!(run_register("TL-07"), 1);
    let (_, body) = send(&invoke("test_multiple_content_types", &json!({}), &[]));
    let types: Vec<&str> = body["result"]["content"]
        .as_array()
        .expect("content")
        .iter()
        .map(|b| b["type"].as_str().expect("type"))
        .collect();
    assert_eq!(types, ["text", "image", "resource"]);
}

#[test]
fn a_failing_tool_is_a_result_and_not_a_json_rpc_error_tl_08() {
    assert_eq!(run_register("TL-08"), 1);
    let (response, body) = send(&invoke("test_error_handling", &json!({}), &[]));
    assert_eq!(response.status, 200);
    assert!(body.get("error").is_none());
    assert_eq!(body["id"], 7);
}

// ---- TL-09: protocol errors ----

#[test]
fn an_unlisted_tool_is_invalid_params_with_400_and_the_request_id_tl_09() {
    for name in ["no_such_tool", "", "Echo", "echoo", "ECHO"] {
        let (response, body) = send(&invoke(name, &json!({}), &[]));
        assert_eq!(response.status, 400, "{name:?}");
        assert_eq!(body["error"]["code"], -32602, "{name:?}");
        assert_eq!(body["id"], 7, "{name:?}");
        assert!(body.get("result").is_none());
    }
}

#[test]
fn a_call_without_a_text_name_or_with_non_object_arguments_is_invalid_params_tl_09() {
    let headers = standard("tools/call", Some("echo"));
    let bodies = [
        format!(r#"{{"_meta":{META}}}"#),
        format!(r#"{{"name":7,"_meta":{META}}}"#),
        format!(r#"{{"name":null,"_meta":{META}}}"#),
        format!(r#"{{"name":"echo","arguments":[],"_meta":{META}}}"#),
        format!(r#"{{"name":"echo","arguments":"x","_meta":{META}}}"#),
        format!(r#"{{"name":"echo","arguments":7,"_meta":{META}}}"#),
    ];
    for params in bodies {
        let (response, body) = send(&post_owned(&headers, &rpc("tools/call", &params, "9")));
        assert_eq!(response.status, 400, "{params}");
        assert_eq!(body["error"]["code"], -32602, "{params}");
        assert_eq!(body["id"], 9, "{params}");
    }
}

// ---- TL-10: argument validation ----

#[test]
fn an_argument_that_fails_its_schema_is_a_tool_error_naming_the_instance_and_keyword_tl_10() {
    let cases = [
        (
            json!({}),
            "Invalid arguments: the arguments object fails required",
        ),
        (json!({"name": 1}), "Invalid arguments: /name fails type"),
        (
            json!({"name": "a", "label": ""}),
            "Invalid arguments: /label fails minLength",
        ),
        (
            json!({"name": "a", "count": 11}),
            "Invalid arguments: /count fails maximum",
        ),
        (
            json!({"name": "a", "count": -1}),
            "Invalid arguments: /count fails minimum",
        ),
        (
            json!({"name": "a", "count": 1.5}),
            "Invalid arguments: /count fails type",
        ),
        (
            json!({"name": "a", "mode": "medium"}),
            "Invalid arguments: /mode fails enum",
        ),
        (
            json!({"name": "a", "tags": ["a", "b", "c", "d"]}),
            "Invalid arguments: /tags fails maxItems",
        ),
        (
            json!({"name": "a", "tags": [1]}),
            "Invalid arguments: /tags/0 fails type",
        ),
        (
            json!({"name": "a", "nested": {"flag": "yes"}}),
            "Invalid arguments: /nested/flag fails type",
        ),
        (
            json!({"name": "a", "nested": {"other": true}}),
            "Invalid arguments: /nested fails additionalProperties",
        ),
        (
            json!({"name": "a", "extra": 1}),
            "Invalid arguments: the arguments object fails additionalProperties",
        ),
    ];
    for (arguments, text) in cases {
        let (response, body) = send(&invoke("test_arguments", &arguments, &[]));
        assert_eq!(response.status, 200, "{arguments}");
        assert_tool_error(&body, text);
        assert!(body.get("error").is_none());
    }
}

#[test]
fn a_member_name_with_a_slash_or_a_tilde_is_escaped_in_the_pointer_tl_10() {
    let (_, body) = send(&invoke(
        "test_arguments",
        &json!({"name": "a", "a/b~c": "x"}),
        &[],
    ));
    assert_tool_error(&body, "Invalid arguments: /a~1b~0c fails type");
}

#[test]
fn arguments_that_satisfy_the_schema_run_the_tool_tl_10() {
    let arguments = json!({
        "name": "a", "count": 10, "mode": "fast", "tags": ["x", "y", "z"], "nested": {"flag": true}
    });
    let (_, body) = send(&invoke("test_arguments", &arguments, &[]));
    assert!(body["result"].get("isError").is_none(), "{body}");
    assert_eq!(text_of(&body), "arguments accepted");
    // an integer may be written with a fraction that is zero
    let (_, body) = send(&invoke(
        "test_arguments",
        &json!({"name": "a", "count": 3.0}),
        &[],
    ));
    assert_eq!(text_of(&body), "arguments accepted");
}

#[test]
fn an_undeclared_argument_stops_the_tool_before_it_runs_tl_10() {
    // test_tool_with_progress would report three steps; it reports none because it did not run
    let (response, body) = send(&invoke_with_token(
        "test_tool_with_progress",
        &json!({"unexpected": true}),
        &json!("t"),
    ));
    assert_eq!(response.status, 200);
    assert_tool_error(
        &body,
        "Invalid arguments: the arguments object fails additionalProperties",
    );
    assert!(response.notifications.is_empty());
    // the tool that always fails reports its own text only when it runs
    let (_, body) = send(&invoke("test_error_handling", &json!({"x": 1}), &[]));
    assert!(text_of(&body).starts_with("Invalid arguments:"), "{body}");
}

#[test]
fn an_unknown_tool_is_refused_before_its_arguments_are_read_tl_10() {
    let (response, body) = send(&invoke("no_such_tool", &json!({"x": 1}), &[]));
    assert_eq!(response.status, 400);
    assert_eq!(body["error"]["code"], -32602);
}

// ---- TL-11: results the core refuses to write ----

#[test]
fn structured_content_that_conforms_is_returned_beside_its_text_tl_11() {
    let (response, body) = send(&invoke("test_structured_output", &json!({}), &[]));
    assert_eq!(response.status, 200);
    assert_eq!(
        body["result"]["structuredContent"],
        json!({"sum": 3, "label": "three"})
    );
}

#[test]
fn structured_content_that_violates_the_output_schema_is_an_internal_error_tl_11() {
    let (response, body) = send(&invoke("test_bad_structured_output", &json!({}), &[]));
    assert_eq!(response.status, 500);
    assert_eq!(body["error"]["code"], -32603);
    assert_eq!(body["error"]["message"], "Internal error");
    assert!(body["error"].get("data").is_none());
    assert!(!String::from_utf8_lossy(&response.body).contains("three"));
}

#[test]
fn a_payload_that_is_not_canonical_base64_is_an_internal_error_tl_11() {
    let (response, body) = send(&invoke("test_bad_payload", &json!({}), &[]));
    assert_eq!(response.status, 500);
    assert_eq!(body["error"]["code"], -32603);
    assert!(!String::from_utf8_lossy(&response.body).contains("not base64"));
}

#[test]
fn progress_that_does_not_increase_is_an_internal_error_and_sends_no_notification_tl_11() {
    let (response, body) = send(&invoke_with_token(
        "test_backward_progress",
        &json!({}),
        &json!("t"),
    ));
    assert_eq!(response.status, 500);
    assert_eq!(body["error"]["code"], -32603);
    assert!(response.notifications.is_empty());
}

// ---- TL-12: parameter headers come from the catalog ----

fn annotated_headers(schema: &Value, out: &mut Vec<String>) {
    match schema {
        Value::Object(members) => {
            if let Some(Value::String(name)) = members.get("x-mcp-header") {
                out.push(name.clone());
            }
            for value in members.values() {
                annotated_headers(value, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|i| annotated_headers(i, out)),
        _ => {}
    }
}

#[test]
fn the_headers_the_core_enforces_are_the_headers_the_catalog_annotates_tl_12() {
    let mut annotated = Vec::new();
    for tool in listed() {
        annotated_headers(&tool["inputSchema"], &mut annotated);
    }
    annotated.sort();
    assert_eq!(annotated, ["Dry-Run", "Limit", "Region", "Zone"]);
}

#[test]
fn a_mirrored_parameter_must_arrive_in_its_header_tl_12() {
    let arguments = json!({"region": "us", "query": "select 1", "limit": 5, "dry_run": true});
    let all = [
        ("Mcp-Param-Region", "us"),
        ("Mcp-Param-Limit", "5"),
        ("Mcp-Param-Dry-Run", "true"),
    ];
    let (response, body) = send(&invoke("execute_sql", &arguments, &all));
    assert_eq!(response.status, 200, "{body}");
    for omitted in 0..all.len() {
        let some: Vec<(&str, &str)> = all
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != omitted)
            .map(|(_, p)| *p)
            .collect();
        let (response, body) = send(&invoke("execute_sql", &arguments, &some));
        assert_eq!(response.status, 400, "without {:?}", all[omitted]);
        assert_eq!(body["error"]["code"], -32020);
    }
    // a value that differs from the body is a mismatch
    let wrong = [
        ("Mcp-Param-Region", "eu"),
        ("Mcp-Param-Limit", "5"),
        ("Mcp-Param-Dry-Run", "true"),
    ];
    let (response, body) = send(&invoke("execute_sql", &arguments, &wrong));
    assert_eq!(
        (response.status, body["error"]["code"].as_i64()),
        (400, Some(-32020))
    );
}

#[test]
fn a_parameter_that_is_not_annotated_needs_no_header_tl_12() {
    let arguments = json!({"region": "us", "query": "select 1"});
    let (response, _) = send(&invoke(
        "execute_sql",
        &arguments,
        &[("Mcp-Param-Region", "us"), ("Mcp-Param-Query", "anything")],
    ));
    assert_eq!(response.status, 200);
}

#[test]
fn a_nested_annotation_is_mirrored_from_its_path_tl_12() {
    let arguments = json!({"target": {"zone": "z1"}});
    let (response, _) = send(&invoke(
        "test_nested_header",
        &arguments,
        &[("Mcp-Param-Zone", "z1")],
    ));
    assert_eq!(response.status, 200);
    let (response, body) = send(&invoke("test_nested_header", &arguments, &[]));
    assert_eq!(
        (response.status, body["error"]["code"].as_i64()),
        (400, Some(-32020))
    );
    let (response, body) = send(&invoke(
        "test_nested_header",
        &arguments,
        &[("Mcp-Param-Zone", "z2")],
    ));
    assert_eq!(
        (response.status, body["error"]["code"].as_i64()),
        (400, Some(-32020))
    );
    // with no target, nothing is expected
    let (response, _) = send(&invoke("test_nested_header", &json!({}), &[]));
    assert_eq!(response.status, 200);
}

// ---- TL-13: progress ----

#[test]
fn a_token_gets_increasing_progress_notifications_that_echo_it_tl_13() {
    assert_eq!(run_register("TL-13"), 1);
    for token in [json!("a-token"), json!(7), json!(0), json!("")] {
        let (response, body) = send(&invoke_with_token(
            "test_tool_with_progress",
            &json!({}),
            &token,
        ));
        assert_eq!(response.status, 200);
        assert_eq!(body["id"], 8);
        let sent = notes(&response);
        assert_eq!(sent.len(), 3, "{token}");
        let mut previous = -1.0;
        for note in &sent {
            assert_eq!(note["params"]["progressToken"], token);
            let progress = note["params"]["progress"].as_f64().expect("a number");
            assert!(progress > previous, "{sent:?}");
            previous = progress;
        }
    }
}

#[test]
fn a_request_without_a_token_gets_no_notification_tl_13() {
    let (response, body) = send(&invoke("test_tool_with_progress", &json!({}), &[]));
    assert_eq!(response.status, 200);
    assert!(response.notifications.is_empty());
    assert_eq!(body["result"]["content"][0]["type"], "text");
}

#[test]
fn a_tool_that_reports_no_progress_sends_no_notification_even_with_a_token_tl_13() {
    let (response, _) = send(&invoke_with_token(
        "test_simple_text",
        &json!({}),
        &json!("t"),
    ));
    assert_eq!(response.status, 200);
    assert!(response.notifications.is_empty());
}

#[test]
fn the_notifications_and_the_result_are_one_event_stream_for_a_client_that_accepts_it_tl_13() {
    let (response, body) = send(&invoke_with_token(
        "test_tool_with_progress",
        &json!({}),
        &json!("t"),
    ));
    assert_eq!(
        header(&response, "content-type"),
        Some(&b"text/event-stream"[..])
    );
    assert_eq!(notes(&response).len(), 3);
    assert!(body["result"].is_object(), "the last event is the result");
}

#[test]
fn a_client_that_does_not_accept_a_stream_gets_the_result_alone_as_json_tl_13() {
    // the same call without an Accept header: progress is optional, so it is not sent
    let body = r#"{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"test_tool_with_progress","arguments":{},"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{},"progressToken":"t"}}}"#.to_owned();
    let (response, json) = send(&post_owned(
        &standard("tools/call", Some("test_tool_with_progress")),
        &body,
    ));
    assert_eq!(response.status, 200);
    assert_eq!(
        header(&response, "content-type"),
        Some(&b"application/json"[..])
    );
    assert!(notes(&response).is_empty());
    assert!(json["result"].is_object());
}

// ---- TL-14: the rate limit ----

const LIMIT: u32 = 100;

fn limited(name: &str, calls: u32) -> (Response, Value) {
    send(&after_calls(invoke(name, &json!({}), &[]), calls))
}

#[test]
fn a_count_below_the_limit_is_served_and_a_count_at_it_is_429_tl_14() {
    for calls in [0, 1, LIMIT - 1] {
        let (response, _) = limited("test_simple_text", calls);
        assert_eq!(response.status, 200, "count {calls}");
    }
    for calls in [LIMIT, LIMIT + 1, 10_000, u32::MAX] {
        let (response, body) = limited("test_simple_text", calls);
        assert_eq!(response.status, 429, "count {calls}");
        assert_eq!(body["error"]["code"], 429);
        assert_eq!(body["error"]["data"], json!({"limit": LIMIT}));
        assert_eq!(body["id"], 7);
        assert!(body.get("result").is_none());
    }
}

#[test]
fn only_tools_call_is_limited_tl_14() {
    let (response, _) = send(&after_calls(
        list(&format!(r#"{{"_meta":{META}}}"#)),
        u32::MAX,
    ));
    assert_eq!(response.status, 200);
    let discover = post_owned(
        &standard("server/discover", None),
        &rpc("server/discover", &format!(r#"{{"_meta":{META}}}"#), "1"),
    );
    let (response, _) = send(&after_calls(discover, u32::MAX));
    assert_eq!(response.status, 200);
}

#[test]
fn a_call_over_the_limit_runs_nothing_and_reports_nothing_tl_14() {
    let request = after_calls(
        invoke_with_token("test_tool_with_progress", &json!({}), &json!("t")),
        LIMIT,
    );
    let (response, body) = send(&request);
    assert_eq!(response.status, 429);
    assert!(response.notifications.is_empty());
    assert_eq!(
        body["error"]["message"],
        "Rate limit exceeded: too many tool calls"
    );
}

#[test]
fn the_limit_is_decided_after_the_request_is_read_tl_14() {
    // a request that is refused for another reason is refused for it, whatever the count
    let (response, body) = send(&after_calls(
        post_owned(&standard("tools/call", Some("echo")), "{"),
        u32::MAX,
    ));
    assert_eq!(
        (response.status, body["error"]["code"].as_i64()),
        (400, Some(-32700))
    );
}

// ---- TL-15: sanitization ----

#[test]
fn control_and_bidirectional_characters_are_replaced_in_a_tool_text_tl_15() {
    let (_, body) = send(&invoke("test_unsafe_text", &json!({}), &[]));
    assert_eq!(
        text_of(&body),
        "bell\u{fffd} escape\u{fffd}[31m rtl\u{fffd}"
    );
}

#[test]
fn text_that_comes_from_the_client_is_sanitized_too_tl_15() {
    // the progress token is the client's text, and each notification echoes it
    let controls = [
        ('\u{0}', true),
        ('\u{7}', true),
        ('\u{1b}', true),
        ('\u{7f}', true),
        ('\u{80}', true),
        ('\u{9f}', true),
        ('\u{202a}', true),
        ('\u{202e}', true),
        ('\u{2066}', true),
        ('\u{2069}', true),
        ('\t', false),
        ('\n', false),
        ('\r', false),
        ('\u{a0}', false),
        ('\u{2029}', false),
        ('\u{2065}', false),
        ('\u{202f}', false),
        ('\u{206a}', false),
        ('e', false),
        ('\u{1f600}', false),
    ];
    for (c, replaced) in controls {
        let token = format!("k{c}k");
        let want = if replaced {
            "k\u{fffd}k".to_owned()
        } else {
            token.clone()
        };
        let (response, _) = send(&invoke_with_token(
            "test_tool_with_progress",
            &json!({}),
            &json!(token),
        ));
        let sent = notes(&response);
        assert_eq!(sent.len(), 3, "U+{:04X}", c as u32);
        for note in sent {
            assert_eq!(
                note["params"]["progressToken"],
                json!(want),
                "U+{:04X}",
                c as u32
            );
        }
    }
}

#[test]
fn a_progress_message_is_sanitized_with_the_notification_tl_15() {
    // the fixture's own messages are clean; the notification path is the same sanitizer
    let (response, _) = send(&invoke_with_token(
        "test_tool_with_progress",
        &json!({}),
        &json!("t\u{7}"),
    ));
    let sent = notes(&response);
    assert_eq!(sent.len(), 3);
    for note in sent {
        assert_eq!(note["params"]["progressToken"], "t\u{fffd}");
    }
}

// ---- TL-16 and TL-17: refusals and theorems are stated ----

fn source(name: &str) -> String {
    std::fs::read_to_string(root().join("src").join(format!("{name}.lex.tex")))
        .unwrap_or_else(|e| panic!("src/{name}.lex.tex reads: {e}"))
}

fn constructors_of(module: &str, inductive: &str) -> Vec<String> {
    let text = source(module);
    let at = text
        .find(&format!("name: {inductive},"))
        .unwrap_or_else(|| panic!("{inductive} is declared in {module}"));
    let block = &text[at..];
    let end = block.find("\n}").expect("the declaration ends");
    block[..end]
        .split("{name: ")
        .skip(1)
        .map(|c| c.split(',').next().expect("a name").to_owned())
        .collect()
}

#[test]
fn every_admission_fault_has_a_negative_theorem_tl_16() {
    let faults = constructors_of("Admission", "AdmitFault");
    assert_eq!(faults.len(), 13, "{faults:?}");
    let laws = source("ToolLaws");
    for fault in faults {
        assert!(
            laws.contains(&format!("AdmitFault.{fault}")),
            "no theorem in ToolLaws states AdmitFault.{fault}"
        );
    }
}

#[test]
fn every_provider_refusal_of_the_tools_layer_is_stated_tl_16() {
    let refusals = constructors_of("Refusal", "ProviderRefusal");
    let laws = source("ToolLaws") + &source("Refusal") + &source("Status");
    for refusal in [
        "unknownTool",
        "invalidCursor",
        "badToolOutput",
        "rateLimited",
    ] {
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

#[test]
fn the_named_theorems_of_the_tools_layer_are_declared_tl_17() {
    let laws = source("ToolLaws");
    for name in [
        "the_catalog_lists_its_tools_in_the_declared_order",
        "tools_list_answers_with_the_catalog_in_its_declared_order",
        "an_unknown_tool_is_invalid_params",
        "an_unknown_tool_has_the_invalid_params_code",
        "an_invalid_argument_is_a_tool_result_with_is_error",
        "a_call_at_the_limit_is_rate_limited",
        "a_call_below_the_limit_reaches_the_tool",
        "the_limit_does_not_apply_to_listing",
    ] {
        assert!(
            laws.contains(&format!("name: {name},")),
            "ToolLaws declares no theorem {name}"
        );
    }
}

#[test]
fn the_vendored_requirements_list_the_tool_scenarios_as_required_tl_17() {
    let yaml = suite_text("requirements/2026-07-28.yaml");
    let server: Vec<&str> = yaml
        .lines()
        .skip_while(|l| *l != "server:")
        .skip(1)
        .take_while(|l| l.starts_with("  - "))
        .collect();
    for scenario in [
        "tools-list",
        "tools-call-simple-text",
        "tools-call-image",
        "tools-call-audio",
        "tools-call-embedded-resource",
        "tools-call-mixed-content",
        "tools-call-error",
        "tools-call-with-progress",
    ] {
        assert!(
            server.contains(&format!("  - {scenario}").as_str()),
            "{scenario} is a required server scenario"
        );
    }
    let at = yaml
        .find("  - scenario: json-schema-2020-12\n    leg: server\n    reason: pending")
        .expect("json-schema-2020-12 is recorded as pending");
    assert!(at > yaml.find("not_scored:").expect("a not_scored section"));
}

// ---- TL-19: the diagnostic tools of server-stateless ----

/// A tools/call of `name` whose `_meta` is `meta`, from a client that accepts both a JSON
/// body and an event stream.
fn invoke_meta(name: &str, meta: &str) -> Request {
    let mut headers = standard("tools/call", Some(name));
    headers.push((
        "Accept".to_owned(),
        "application/json, text/event-stream".to_owned(),
    ));
    let body = format!(
        r#"{{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{{"name":"{name}","arguments":{{}},"_meta":{meta}}}}}"#
    );
    post_owned(&headers, &body)
}

#[test]
fn the_diagnostic_tools_of_server_stateless_are_listed_and_answer_text_tl_19() {
    assert_eq!(run_register("TL-19"), 2, "the vectors TS-10 and TS-11");
    let tools = listed();
    let find = |name: &str| -> &Value {
        tools
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap_or_else(|| panic!("{name} is listed"))
    };
    assert_eq!(
        find("test_streaming_elicitation")["description"],
        "Diagnostic tool validating response progress streams"
    );
    assert_eq!(
        find("test_logging_tool")["description"],
        "Diagnostic logging validator tool"
    );
    for tool in ["test_streaming_elicitation", "test_logging_tool"] {
        assert_eq!(find(tool)["inputSchema"]["type"], "object");
    }

    // The metas the pinned scenario sends, and one that sets a log level.
    let elicitation = META.replace(
        r#""clientCapabilities":{}"#,
        r#""clientCapabilities":{"elicitation":{}}"#,
    );
    let logged = META.replace(
        r#""clientCapabilities":{}"#,
        r#""clientCapabilities":{},"io.modelcontextprotocol/logLevel":"debug""#,
    );
    for (tool, text) in [
        ("test_streaming_elicitation", "Streaming complete"),
        ("test_logging_tool", "Logging evaluated"),
    ] {
        for meta in [META, elicitation.as_str(), logged.as_str()] {
            let (response, body) = send(&invoke_meta(tool, meta));
            assert_eq!(response.status, 200, "{tool} {meta}: {body}");
            assert_eq!(body["result"]["resultType"], "complete", "{body}");
            assert_ne!(body["result"]["isError"], true, "{body}");
            assert_eq!(body["result"]["content"].as_array().map(Vec::len), Some(1));
            assert_eq!(text_of(&body), text, "{body}");
            // Nothing but the result: no server request and no log notification.
            assert!(
                notes(&response).is_empty(),
                "{tool}: {:?}",
                notes(&response)
            );
            let raw = String::from_utf8_lossy(&response.body).into_owned();
            assert!(!raw.contains("notifications/message"), "{raw}");
            assert!(!raw.contains("elicitation/create"), "{raw}");
        }
    }
}
