//! Requests the scenarios do not send, run against the generated root: the JSON-RPC
//! faults that come before a request exists, the order the checks run in, what the
//! core decides about HTTP, and the headers beyond the pinned cases.
//!
//! Each test is authored from the specification text the register cites, and each is
//! a falsifying probe, not a verdict.

mod common;

use common::{call, outcome, post, post_owned, rpc, send, standard, META};
use lifecycle_oracle::Request;
use serde_json::{json, Value};

fn discover(id: &str) -> String {
    rpc("server/discover", &format!(r#"{{"_meta":{META}}}"#), id)
}

fn with_headers(method: &str, name: Option<&str>, body: &str) -> Request {
    post_owned(&standard(method, name), body)
}

// ---- LC-08: faults before a request exists, and the id every refusal echoes ----

#[test]
fn text_that_is_not_json_is_a_parse_error_with_no_id_member_lc_08() {
    for body in [
        "",
        "{",
        r#"{"jsonrpc":"2.0","id":1,"method":"#,
        "nope",
        "\u{feff}{}",
    ] {
        let (response, json) = send(&post(&[("MCP-Protocol-Version", "2026-07-28")], body));
        assert_eq!(response.status, 400, "{body:?}");
        assert_eq!(json["error"]["code"], -32700, "{body:?}");
        assert!(json.get("id").is_none(), "{body:?}");
        assert_eq!(json["jsonrpc"], "2.0");
    }
}

#[test]
fn a_batch_is_an_invalid_request_with_no_id_member_lc_08() {
    let body = format!("[{}]", discover("1"));
    let (response, json) = send(&with_headers("server/discover", None, &body));
    assert_eq!(
        (response.status, &json["error"]["code"]),
        (400, &json!(-32600))
    );
    assert!(json.get("id").is_none());
}

#[test]
fn a_notification_is_refused_because_the_core_defines_none_over_http_lc_08() {
    let body = r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{}}"#;
    let (response, json) = send(&with_headers("notifications/cancelled", None, body));
    assert_eq!(
        (response.status, &json["error"]["code"]),
        (400, &json!(-32600))
    );
    assert!(json.get("id").is_none());
}

#[test]
fn a_response_message_is_an_invalid_request_lc_08() {
    for body in [
        r#"{"jsonrpc":"2.0","id":1,"result":{}}"#,
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-1,"message":"x"}}"#,
    ] {
        let (status, code) = outcome(&with_headers("server/discover", None, body));
        assert_eq!((status, code), (400, Some(-32600)), "{body}");
    }
}

#[test]
fn a_request_that_is_malformed_as_json_rpc_is_an_invalid_request_lc_08() {
    let cases = [
        r#"{"jsonrpc":"1.0","id":1,"method":"server/discover"}"#,
        r#"{"id":1,"method":"server/discover"}"#,
        r#"{"jsonrpc":"2.0","id":1}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":7}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"server/discover","params":[]}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"server/discover","params":"x"}"#,
        r#"[]"#,
        r#"7"#,
        r#"null"#,
        r#""server/discover""#,
    ];
    for body in cases {
        let (status, code) = outcome(&with_headers("server/discover", None, body));
        assert_eq!((status, code), (400, Some(-32600)), "{body}");
    }
}

#[test]
fn an_id_that_is_not_a_string_or_a_whole_number_is_an_invalid_request_lc_08() {
    for id in ["null", "true", "1.5", "[]", "{}"] {
        let body = format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"server/discover"}}"#);
        let (response, json) = send(&with_headers("server/discover", None, &body));
        assert_eq!(response.status, 400, "id {id}");
        assert_eq!(json["error"]["code"], -32600, "id {id}");
        assert!(json.get("id").is_none(), "id {id}");
    }
}

#[test]
fn a_string_id_and_a_number_id_are_echoed_exactly_lc_08() {
    for (id, want) in [
        (r#""abc""#, json!("abc")),
        (r#""""#, json!("")),
        ("0", json!(0)),
        ("-7", json!(-7)),
        ("9007199254740991", json!(9007199254740991_i64)),
    ] {
        let (response, json) = send(&with_headers("server/discover", None, &discover(id)));
        assert_eq!(response.status, 200, "id {id}");
        assert_eq!(json["id"], want, "id {id}");
        // and on a refusal
        let body = rpc("no/such", &format!(r#"{{"_meta":{META}}}"#), id);
        let (response, json) = send(&with_headers("no/such", None, &body));
        assert_eq!(response.status, 404, "id {id}");
        assert_eq!(json["id"], want, "id {id}");
    }
}

#[test]
fn an_invalid_request_that_carries_an_id_echoes_it_lc_08() {
    let body = r#"{"jsonrpc":"1.0","id":42,"method":"server/discover"}"#;
    let (response, json) = send(&with_headers("server/discover", None, body));
    assert_eq!(response.status, 400);
    assert_eq!(json["error"]["code"], -32600);
    assert_eq!(json["id"], 42);
}

// ---- LC-09: HTTP is decided by the core ----

#[test]
fn only_post_is_served_and_anything_else_is_405_with_allow_post_lc_09() {
    for method in [
        "GET", "DELETE", "PUT", "PATCH", "HEAD", "OPTIONS", "post", "Post", "",
    ] {
        let request = Request {
            method: method.as_bytes().to_vec(),
            target: b"/mcp".to_vec(),
            fields: vec![],
            body: discover("1").into_bytes(),
            tool_calls: 0,
        };
        let response = common::exchange_checked(&request).expect("a response frame");
        assert_eq!(response.status, 405, "method {method:?}");
        assert_eq!(
            common::header(&response, "allow"),
            Some(&b"POST"[..]),
            "method {method:?}"
        );
    }
}

#[test]
fn the_status_follows_the_error_code_for_every_refusal_the_core_makes_lc_09() {
    let cases: Vec<(Request, u32, i64)> = vec![
        (with_headers("server/discover", None, "{"), 400, -32700),
        (
            with_headers("server/discover", None, r#"{"jsonrpc":"2.0","method":"x"}"#),
            400,
            -32600,
        ),
        (
            with_headers("server/discover", None, &rpc("server/discover", "{}", "1")),
            400,
            -32602,
        ),
        (
            with_headers(
                "no/such",
                None,
                &rpc("no/such", &format!(r#"{{"_meta":{META}}}"#), "1"),
            ),
            404,
            -32601,
        ),
        (
            post_owned(
                &standard("server/discover", None)
                    .into_iter()
                    .filter(|(n, _)| n != "Mcp-Method")
                    .collect::<Vec<_>>(),
                &discover("1"),
            ),
            400,
            -32020,
        ),
        (
            with_headers(
                "tools/call",
                Some("test_missing_capability"),
                &call("test_missing_capability", "{}", "1"),
            ),
            400,
            -32021,
        ),
    ];
    for (request, status, code) in cases {
        assert_eq!(outcome(&request), (status, Some(code)));
    }
}

#[test]
fn a_frame_the_core_cannot_read_is_a_server_error_with_a_fixed_body_lc_09() {
    let valid = common::standard("server/discover", None);
    let good = lifecycle_oracle::encode_request(&post_owned(&valid, &discover("1")));
    let mut frames: Vec<Vec<u8>> = vec![vec![], vec![0, 0, 0, 1], vec![255; 12]];
    // every proper prefix of a valid frame is truncated, and one more byte is trailing
    for cut in (0..good.len()).step_by(7) {
        frames.push(good[..cut].to_vec());
    }
    let mut trailing = good.clone();
    trailing.push(0);
    frames.push(trailing);
    for frame in frames {
        let response = common::frame_checked(&frame).expect("a response frame");
        assert_eq!(response.status, 500, "frame of {} bytes", frame.len());
        let body: Value = serde_json::from_slice(&response.body).expect("JSON body");
        assert_eq!(body["error"]["code"], -32603);
        assert!(body.get("id").is_none());
    }
}

// ---- LC-10: the order of the checks ----

fn code_of(headers: &[(String, String)], body: &str) -> i64 {
    outcome(&post_owned(headers, body))
        .1
        .expect("an error code")
}

#[test]
fn meta_is_checked_before_headers_version_route_and_params_lc_10() {
    // _meta lacks protocolVersion, the Mcp-Method header disagrees, the method is unknown.
    let body = rpc("no/such", r#"{"_meta":{}}"#, "1");
    let headers = standard("prompts/list", None);
    assert_eq!(code_of(&headers, &body), -32602);
}

#[test]
fn headers_are_checked_before_the_version_the_route_and_the_params_lc_10() {
    // The version is unsupported, the method is unknown, the params are invalid, and
    // the Mcp-Method header disagrees with the body.
    let meta = r#"{"io.modelcontextprotocol/protocolVersion":"v999.0.0","io.modelcontextprotocol/clientCapabilities":{}}"#;
    let body = rpc("no/such", &format!(r#"{{"_meta":{meta}}}"#), "1");
    let mut headers = standard("prompts/list", None);
    headers[1].1 = "v999.0.0".to_owned();
    assert_eq!(code_of(&headers, &body), -32020);
}

#[test]
fn the_version_is_checked_before_the_route_and_the_params_lc_10() {
    let meta = r#"{"io.modelcontextprotocol/protocolVersion":"v999.0.0","io.modelcontextprotocol/clientCapabilities":{}}"#;
    let body = rpc("no/such", &format!(r#"{{"_meta":{meta}}}"#), "1");
    let mut headers = standard("no/such", None);
    headers[1].1 = "v999.0.0".to_owned();
    assert_eq!(code_of(&headers, &body), -32022);
}

#[test]
fn the_route_is_checked_before_the_params_lc_10() {
    // prompts/get is a known method the fixture does not offer; its params are invalid.
    let body = rpc("prompts/get", &format!(r#"{{"_meta":{META}}}"#), "1");
    let headers = standard("prompts/get", Some("anything"));
    assert_eq!(outcome(&post_owned(&headers, &body)), (404, Some(-32601)));
}

#[test]
fn a_header_check_runs_even_for_a_method_the_server_does_not_offer_lc_10() {
    // resources/read is not offered, but a missing Mcp-Name is still a header mismatch.
    let body = rpc(
        "resources/read",
        &format!(r#"{{"uri":"file:///a","_meta":{META}}}"#),
        "1",
    );
    assert_eq!(
        outcome(&post_owned(&standard("resources/read", None), &body)),
        (400, Some(-32020))
    );
    assert_eq!(
        outcome(&post_owned(
            &standard("resources/read", Some("file:///a")),
            &body
        )),
        (404, Some(-32601))
    );
}

#[test]
fn params_are_checked_last_lc_10() {
    // tools/call with a conformant envelope and header but no name in the body.
    let body = rpc("tools/call", &format!(r#"{{"_meta":{META}}}"#), "1");
    assert_eq!(
        outcome(&post_owned(&standard("tools/call", Some("echo")), &body)),
        (400, Some(-32602))
    );
    // the same body without the Mcp-Name header is a header fault, which comes first
    assert_eq!(
        outcome(&post_owned(&standard("tools/call", None), &body)),
        (400, Some(-32020))
    );
    // a tool the server does not have
    let body = call("no_such_tool", "{}", "1");
    assert_eq!(
        outcome(&post_owned(
            &standard("tools/call", Some("no_such_tool")),
            &body
        )),
        (400, Some(-32602))
    );
    // arguments that are not an object
    let body = rpc(
        "tools/call",
        &format!(r#"{{"name":"echo","arguments":"x","_meta":{META}}}"#),
        "1",
    );
    assert_eq!(
        outcome(&post_owned(&standard("tools/call", Some("echo")), &body)),
        (400, Some(-32602))
    );
}

// ---- LC-11: what the core reads, and what it does not ----

#[test]
fn the_target_and_unrelated_headers_do_not_change_the_response_lc_11() {
    let base = with_headers(
        "tools/list",
        None,
        &rpc("tools/list", &format!(r#"{{"_meta":{META}}}"#), "5"),
    );
    let want = common::exchange_checked(&base).expect("a response");
    for target in ["/", "", "/mcp?x=1", "/a/b/c", "*", "/\u{0}"] {
        let mut request = base.clone();
        request.target = target.as_bytes().to_vec();
        assert_eq!(
            common::exchange_checked(&request).unwrap(),
            want,
            "{target:?}"
        );
    }
    let mut request = base.clone();
    request.fields.extend([
        // Origin and Host are read since issue 11 (SH-04); Referer is not
        (b"Referer".to_vec(), b"http://evil.example".to_vec()),
        (b"Accept".to_vec(), b"text/plain".to_vec()),
        (b"Content-Length".to_vec(), b"0".to_vec()),
        (b"Authorization".to_vec(), b"Bearer x".to_vec()),
        (b"X-Anything".to_vec(), b"\x01\xff".to_vec()),
    ]);
    assert_eq!(common::exchange_checked(&request).unwrap(), want);
    let mut request = base;
    request
        .fields
        .insert(0, (b"Mcp-Param-Unknown".to_vec(), b"x".to_vec()));
    assert_eq!(
        common::exchange_checked(&request).unwrap(),
        want,
        "an unrecognized Mcp-Param header is ignored"
    );
}

#[test]
fn the_same_request_gets_the_same_bytes_every_time_lc_11() {
    let request = with_headers("server/discover", None, &discover("9"));
    let first = common::exchange_checked(&request).expect("a response");
    for _ in 0..3 {
        assert_eq!(common::exchange_checked(&request).unwrap(), first);
    }
}

// ---- HH-01: the protocol version header ----

fn without(headers: Vec<(String, String)>, name: &str) -> Vec<(String, String)> {
    headers.into_iter().filter(|(n, _)| n != name).collect()
}

#[test]
fn a_missing_protocol_version_header_is_a_header_mismatch_hh_01() {
    let headers = without(standard("server/discover", None), "MCP-Protocol-Version");
    assert_eq!(
        outcome(&post_owned(&headers, &discover("1"))),
        (400, Some(-32020))
    );
}

#[test]
fn a_protocol_version_header_that_differs_in_any_way_is_a_header_mismatch_hh_01() {
    for value in ["2026-07-27", "2026-07-28x", "", "v999.0.0", "2026-07-2"] {
        let mut headers = standard("server/discover", None);
        headers[1].1 = value.to_owned();
        assert_eq!(
            outcome(&post_owned(&headers, &discover("1"))),
            (400, Some(-32020)),
            "{value:?}"
        );
    }
}

#[test]
fn a_repeated_protocol_version_header_is_a_header_mismatch_hh_01() {
    let mut headers = standard("server/discover", None);
    headers.push(("mcp-protocol-version".to_owned(), "2026-07-28".to_owned()));
    assert_eq!(
        outcome(&post_owned(&headers, &discover("1"))),
        (400, Some(-32020))
    );
}

#[test]
fn a_protocol_version_header_may_be_padded_with_whitespace_hh_01() {
    let mut headers = standard("server/discover", None);
    headers[1].1 = " \t2026-07-28 ".to_owned();
    assert_eq!(outcome(&post_owned(&headers, &discover("1"))).0, 200);
}

// ---- HH-03/HH-02: Mcp-Method ----

#[test]
fn a_repeated_or_malformed_mcp_method_is_a_header_mismatch_hh_03() {
    let mut repeated = standard("server/discover", None);
    repeated.push(("MCP-METHOD".to_owned(), "server/discover".to_owned()));
    assert_eq!(
        outcome(&post_owned(&repeated, &discover("1"))),
        (400, Some(-32020))
    );
    for bad in [
        "server/discover\u{1}",
        "server/discover\u{7f}",
        "server/dïscover",
    ] {
        let mut headers = standard("server/discover", None);
        headers[2].1 = bad.to_owned();
        assert_eq!(
            outcome(&post_owned(&headers, &discover("1"))),
            (400, Some(-32020)),
            "{bad:?}"
        );
    }
}

#[test]
fn an_empty_mcp_method_is_a_header_mismatch_hh_02() {
    let mut headers = standard("server/discover", None);
    headers[2].1 = String::new();
    assert_eq!(
        outcome(&post_owned(&headers, &discover("1"))),
        (400, Some(-32020))
    );
}

// ---- HH-04..HH-06: Mcp-Name ----

#[test]
fn mcp_name_is_compared_exactly_hh_04() {
    for wrong in ["Echo", "ech", "echo2", "", "ECHO"] {
        let headers = standard("tools/call", Some(wrong));
        assert_eq!(
            outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
            (400, Some(-32020)),
            "{wrong:?}"
        );
    }
    let headers = standard("tools/call", Some("echo"));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))).0,
        200
    );
}

#[test]
fn a_base64_mcp_name_is_decoded_before_the_comparison_hh_04() {
    let headers = standard("tools/call", Some("=?base64?ZWNobw==?="));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))).0,
        200
    );
    let headers = standard("tools/call", Some("=?base64?ZWNobw?="));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
        (400, Some(-32020))
    );
    let headers = standard("tools/call", Some("=?base64?ZWNobA==?="));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
        (400, Some(-32020))
    );
}

#[test]
fn tabs_and_spaces_around_mcp_name_are_trimmed_hh_05() {
    for padded in [" echo", "echo ", "\techo\t", "  \t echo \t  "] {
        let headers = standard("tools/call", Some(padded));
        assert_eq!(
            outcome(&post_owned(&headers, &call("echo", "{}", "1"))).0,
            200,
            "{padded:?}"
        );
    }
    // interior whitespace is part of the value
    let headers = standard("tools/call", Some("ec ho"));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
        (400, Some(-32020))
    );
}

#[test]
fn mcp_name_is_required_on_tools_call_resources_read_and_prompts_get_hh_06() {
    let cases = [
        ("tools/call", call("echo", "{}", "1")),
        (
            "resources/read",
            rpc(
                "resources/read",
                &format!(r#"{{"uri":"file:///a","_meta":{META}}}"#),
                "1",
            ),
        ),
        (
            "prompts/get",
            rpc(
                "prompts/get",
                &format!(r#"{{"name":"p","_meta":{META}}}"#),
                "1",
            ),
        ),
    ];
    for (method, body) in cases {
        let missing = standard(method, None);
        assert_eq!(
            outcome(&post_owned(&missing, &body)),
            (400, Some(-32020)),
            "{method} without Mcp-Name"
        );
    }
}

#[test]
fn mcp_name_is_not_required_on_methods_that_name_nothing_hh_06() {
    for (method, body) in [
        ("server/discover", discover("1")),
        (
            "tools/list",
            rpc("tools/list", &format!(r#"{{"_meta":{META}}}"#), "1"),
        ),
    ] {
        assert_eq!(
            outcome(&post_owned(&standard(method, None), &body)).0,
            200,
            "{method}"
        );
    }
}

// ---- HH-07/HH-08 ----

#[test]
fn header_names_match_in_every_case_hh_07() {
    for (a, b, c) in [
        ("content-type", "mcp-protocol-version", "mcp-method"),
        ("CONTENT-TYPE", "MCP-PROTOCOL-VERSION", "MCP-METHOD"),
        ("Content-type", "Mcp-protocol-Version", "mCp-MeThOd"),
    ] {
        let headers = vec![
            (a.to_owned(), "application/json".to_owned()),
            (b.to_owned(), "2026-07-28".to_owned()),
            (c.to_owned(), "server/discover".to_owned()),
        ];
        assert_eq!(
            outcome(&post_owned(&headers, &discover("1"))).0,
            200,
            "{b} {c}"
        );
    }
    let headers = vec![
        ("MCP-PROTOCOL-VERSION".to_owned(), "2026-07-28".to_owned()),
        ("MCP-METHOD".to_owned(), "tools/call".to_owned()),
        ("mcp-name".to_owned(), "echo".to_owned()),
    ];
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))).0,
        200
    );
}

#[test]
fn header_values_match_exactly_hh_08() {
    for wrong in [
        "Server/Discover",
        "server/discover/",
        "server/discove",
        "server//discover",
    ] {
        let mut headers = standard("server/discover", None);
        headers[2].1 = wrong.to_owned();
        assert_eq!(
            outcome(&post_owned(&headers, &discover("1"))),
            (400, Some(-32020)),
            "{wrong:?}"
        );
    }
}

// ---- HH-09..HH-12: parameter headers ----

fn execute(arguments: &str, extra: &[(&str, &str)]) -> (u32, Option<i64>) {
    let mut headers = standard("tools/call", Some("execute_sql"));
    for (n, v) in extra {
        headers.push(((*n).to_owned(), (*v).to_owned()));
    }
    outcome(&post_owned(&headers, &call("execute_sql", arguments, "1")))
}

#[test]
fn a_text_parameter_header_must_equal_the_body_value_hh_09() {
    assert_eq!(
        execute(r#"{"region":"eu"}"#, &[("Mcp-Param-Region", "eu")]).0,
        200
    );
    assert_eq!(
        execute(r#"{"region":"eu"}"#, &[("mcp-param-region", "eu")]).0,
        200
    );
    assert_eq!(
        execute(r#"{"region":"eu"}"#, &[("MCP-PARAM-REGION", " eu ")]).0,
        200
    );
    for wrong in ["us", "EU", "eu eu", ""] {
        assert_eq!(
            execute(r#"{"region":"eu"}"#, &[("Mcp-Param-Region", wrong)]),
            (400, Some(-32020)),
            "{wrong:?}"
        );
    }
}

#[test]
fn a_base64_parameter_header_may_carry_what_plain_ascii_cannot_hh_09() {
    // "héllo" is not representable as a plain header value; its Base64 form is.
    assert_eq!(
        execute(
            r#"{"region":"héllo"}"#,
            &[("Mcp-Param-Region", "=?base64?aMOpbGxv?=")]
        )
        .0,
        200
    );
    // a line feed in the value
    assert_eq!(
        execute(
            r#"{"region":"a\nb"}"#,
            &[("Mcp-Param-Region", "=?base64?YQpi?=")]
        )
        .0,
        200
    );
    // the decoded text must match the body
    assert_eq!(
        execute(
            r#"{"region":"héllo"}"#,
            &[("Mcp-Param-Region", "=?base64?aGVsbG8=?=")]
        ),
        (400, Some(-32020))
    );
    // decoded bytes that are not UTF-8 never match
    assert_eq!(
        execute(
            r#"{"region":"x"}"#,
            &[("Mcp-Param-Region", "=?base64?/w==?=")]
        ),
        (400, Some(-32020))
    );
}

#[test]
fn malformed_base64_is_refused_hh_10() {
    for bad in [
        "=?base64?SGVsbG8?=",
        "=?base64?SGVs!!!bG8=?=",
        "=?base64?SGVsbG9=?=",
        "=?base64?SG=sbG8=?=",
        "=?base64?S?=",
        "=?base64?SGVsbG8==?=",
        "=?base64?SGVs bG8=?=",
        "=?base64?SGVsbG8-?=",
        "=?base64?SGVs_G8=?=",
    ] {
        assert_eq!(
            execute(r#"{"region":"Hello"}"#, &[("Mcp-Param-Region", bad)]),
            (400, Some(-32020)),
            "{bad:?}"
        );
    }
}

#[test]
fn the_sentinel_is_case_sensitive_and_otherwise_literal_hh_11() {
    // wrong-case prefix: not a sentinel, so a literal that differs from "Hello"
    assert_eq!(
        execute(
            r#"{"region":"Hello"}"#,
            &[("Mcp-Param-Region", "=?BASE64?SGVsbG8=?=")]
        ),
        (400, Some(-32020))
    );
    // ...and equal to the body when the body spells exactly that text
    assert_eq!(
        execute(
            r#"{"region":"=?BASE64?SGVsbG8=?="}"#,
            &[("Mcp-Param-Region", "=?BASE64?SGVsbG8=?=")]
        )
        .0,
        200
    );
    // a lone prefix, a lone suffix and a truncated sentinel are literals
    for literal in ["=?base64?", "?=", "=?base64?abc", "abc?="] {
        let body = format!(r#"{{"region":"{literal}"}}"#);
        assert_eq!(
            execute(&body, &[("Mcp-Param-Region", literal)]).0,
            200,
            "{literal:?}"
        );
    }
    // the empty payload is a sentinel: it decodes to the empty text
    assert_eq!(
        execute(r#"{"region":""}"#, &[("Mcp-Param-Region", "=?base64??=")]).0,
        200
    );
    assert_eq!(
        execute(
            r#"{"region":"=?base64??="}"#,
            &[("Mcp-Param-Region", "=?base64??=")]
        ),
        (400, Some(-32020))
    );
}

#[test]
fn an_omitted_parameter_header_is_a_mismatch_only_when_the_body_has_a_value_hh_12() {
    assert_eq!(execute(r#"{"region":"eu"}"#, &[]), (400, Some(-32020)));
    assert_eq!(execute(r#"{"region":""}"#, &[]), (400, Some(-32020)));
    // null and absent: the header is not expected
    assert_eq!(execute(r#"{"region":null}"#, &[]).0, 200);
    assert_eq!(execute(r#"{}"#, &[]).0, 200);
    assert_eq!(execute(r#"{"other":"eu"}"#, &[]).0, 200);
}

#[test]
fn a_recognized_parameter_header_with_invalid_bytes_is_refused_even_when_not_expected_hh_12() {
    assert_eq!(
        execute(r#"{}"#, &[("Mcp-Param-Region", "a\u{1}b")]),
        (400, Some(-32020))
    );
    assert_eq!(
        execute(r#"{"region":null}"#, &[("Mcp-Param-Region", "a\u{7f}")]),
        (400, Some(-32020))
    );
}

#[test]
fn a_repeated_parameter_header_is_refused_hh_12() {
    assert_eq!(
        execute(
            r#"{"region":"eu"}"#,
            &[("Mcp-Param-Region", "eu"), ("mcp-param-region", "eu")]
        ),
        (400, Some(-32020))
    );
}

#[test]
fn an_unrecognized_parameter_header_is_ignored_hh_12() {
    assert_eq!(
        execute(
            r#"{"region":"eu"}"#,
            &[("Mcp-Param-Region", "eu"), ("Mcp-Param-Other", "zzz")]
        )
        .0,
        200
    );
    // another tool's rule does not apply to this tool
    let mut headers = standard("tools/call", Some("echo"));
    headers.push(("Mcp-Param-Region".to_owned(), "eu".to_owned()));
    assert_eq!(
        outcome(&post_owned(
            &headers,
            &call("echo", r#"{"region":"zzz"}"#, "1")
        ))
        .0,
        200
    );
}

#[test]
fn an_integer_parameter_header_is_compared_numerically_hh_13() {
    let region = ("Mcp-Param-Region", "eu");
    let ok =
        |arguments: &str, limit: &str| execute(arguments, &[region, ("Mcp-Param-Limit", limit)]).0;
    assert_eq!(ok(r#"{"region":"eu","limit":10}"#, "10"), 200);
    assert_eq!(ok(r#"{"region":"eu","limit":10}"#, " 10 "), 200);
    assert_eq!(ok(r#"{"region":"eu","limit":10.0}"#, "10"), 200);
    assert_eq!(ok(r#"{"region":"eu","limit":1e1}"#, "10"), 200);
    assert_eq!(ok(r#"{"region":"eu","limit":-5}"#, "-5"), 200);
    assert_eq!(ok(r#"{"region":"eu","limit":0}"#, "0"), 200);
    assert_eq!(
        ok(
            r#"{"region":"eu","limit":9007199254740991}"#,
            "9007199254740991"
        ),
        200
    );
    for (body, header) in [
        (r#"{"region":"eu","limit":10}"#, "11"),
        (r#"{"region":"eu","limit":10}"#, "1e1"),
        (r#"{"region":"eu","limit":10}"#, "10.0"),
        (r#"{"region":"eu","limit":10}"#, "+10"),
        (r#"{"region":"eu","limit":10}"#, "ten"),
        (r#"{"region":"eu","limit":10}"#, ""),
        (r#"{"region":"eu","limit":10}"#, "-"),
        (r#"{"region":"eu","limit":"10"}"#, "10"),
        (r#"{"region":"eu","limit":10.5}"#, "10"),
        (
            r#"{"region":"eu","limit":9007199254740993}"#,
            "9007199254740993",
        ),
    ] {
        assert_eq!(ok(body, header), 400, "{body} / {header:?}");
    }
    // omitted while present in the body
    assert_eq!(
        execute(r#"{"region":"eu","limit":10}"#, &[region]),
        (400, Some(-32020))
    );
}

#[test]
fn a_boolean_parameter_header_is_true_or_false_exactly_hh_13() {
    let region = ("Mcp-Param-Region", "eu");
    let run =
        |arguments: &str, flag: &str| execute(arguments, &[region, ("Mcp-Param-Dry-Run", flag)]).0;
    assert_eq!(run(r#"{"region":"eu","dry_run":true}"#, "true"), 200);
    assert_eq!(run(r#"{"region":"eu","dry_run":false}"#, "false"), 200);
    for (body, header) in [
        (r#"{"region":"eu","dry_run":true}"#, "false"),
        (r#"{"region":"eu","dry_run":true}"#, "True"),
        (r#"{"region":"eu","dry_run":true}"#, "TRUE"),
        (r#"{"region":"eu","dry_run":false}"#, "0"),
        (r#"{"region":"eu","dry_run":"true"}"#, "true"),
        (r#"{"region":"eu","dry_run":1}"#, "true"),
    ] {
        assert_eq!(run(body, header), 400, "{body} / {header:?}");
    }
}

// ---- the remaining refusals, each with a negative test ----

#[test]
fn a_malformed_or_repeated_protocol_version_or_name_header_is_refused_hh_01() {
    let mut headers = standard("server/discover", None);
    headers[1].1 = "2026-07-28\u{1}".to_owned();
    assert_eq!(
        outcome(&post_owned(&headers, &discover("1"))),
        (400, Some(-32020))
    );
    let mut headers = standard("server/discover", None);
    headers[1].1 = "2026-07-28\u{80}".to_owned();
    assert_eq!(
        outcome(&post_owned(&headers, &discover("1"))),
        (400, Some(-32020))
    );
}

#[test]
fn a_malformed_or_repeated_mcp_name_is_refused_hh_04() {
    for bad in ["ec\u{1}ho", "echo\u{7f}", "échoé"] {
        let headers = standard("tools/call", Some(bad));
        assert_eq!(
            outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
            (400, Some(-32020)),
            "{bad:?}"
        );
    }
    let mut headers = standard("tools/call", Some("echo"));
    headers.push(("MCP-NAME".to_owned(), "echo".to_owned()));
    assert_eq!(
        outcome(&post_owned(&headers, &call("echo", "{}", "1"))),
        (400, Some(-32020))
    );
}

#[test]
fn a_declared_capability_lets_the_request_through_lc_07() {
    let meta = r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{"sampling":{}}}"#;
    let body = rpc(
        "tools/call",
        &format!(r#"{{"name":"test_missing_capability","arguments":{{}},"_meta":{meta}}}"#),
        "7",
    );
    let (response, json) = send(&post_owned(
        &standard("tools/call", Some("test_missing_capability")),
        &body,
    ));
    assert_eq!(response.status, 200);
    assert_eq!(json["result"]["resultType"], "complete");
}

#[test]
fn the_refusal_for_a_missing_capability_names_the_requirement_as_an_object_lc_07() {
    let body = call("test_missing_capability", "{}", "7");
    let (response, json) = send(&post_owned(
        &standard("tools/call", Some("test_missing_capability")),
        &body,
    ));
    assert_eq!(response.status, 400);
    assert_eq!(
        json["error"]["data"]["requiredCapabilities"],
        json!({"sampling": {}})
    );
}

#[test]
fn an_unsupported_version_lists_every_supported_version_and_echoes_the_request_lc_04() {
    let meta = r#"{"io.modelcontextprotocol/protocolVersion":"1999-01-01","io.modelcontextprotocol/clientCapabilities":{}}"#;
    let body = rpc("tools/list", &format!(r#"{{"_meta":{meta}}}"#), "3");
    let mut headers = standard("tools/list", None);
    headers[1].1 = "1999-01-01".to_owned();
    let (response, json) = send(&post_owned(&headers, &body));
    assert_eq!(response.status, 400);
    assert_eq!(json["error"]["code"], -32022);
    assert_eq!(
        json["error"]["data"],
        json!({"supported": ["2026-07-28"], "requested": "1999-01-01"})
    );
    assert_eq!(json["id"], 3);
}

#[test]
fn a_request_without_a_meta_object_is_refused_for_every_method_lc_01() {
    for method in [
        "server/discover",
        "tools/list",
        "tools/call",
        "prompts/list",
        "no/such",
    ] {
        let name = (method == "tools/call").then_some("echo");
        let body = rpc(method, r#"{"name":"echo"}"#, "9");
        assert_eq!(
            outcome(&post_owned(&standard(method, name), &body)),
            (400, Some(-32602)),
            "{method}"
        );
    }
    for meta in ["null", "[]", "7", r#""x""#, "{}"] {
        let body = rpc("server/discover", &format!(r#"{{"_meta":{meta}}}"#), "9");
        assert_eq!(
            outcome(&post_owned(&standard("server/discover", None), &body)),
            (400, Some(-32602)),
            "_meta {meta}"
        );
    }
    // a protocol version that is not text, and capabilities that are not an object
    for meta in [
        r#"{"io.modelcontextprotocol/protocolVersion":7,"io.modelcontextprotocol/clientCapabilities":{}}"#,
        r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":[]}"#,
        r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":null}"#,
    ] {
        let body = rpc("server/discover", &format!(r#"{{"_meta":{meta}}}"#), "9");
        assert_eq!(
            outcome(&post_owned(&standard("server/discover", None), &body)),
            (400, Some(-32602)),
            "_meta {meta}"
        );
    }
}
