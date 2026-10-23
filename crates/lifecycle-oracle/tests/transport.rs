//! SH-01 to SH-10: the Streamable HTTP transport (issue 11).
//!
//! The core decides the bytes, the `Content-Type` and the status of a response: an event
//! stream when the client accepts one and there is something to stream, the refusal of a
//! forbidden `Origin` or `Host`, and the single JSON object otherwise. These tests run the
//! requests of the pinned scenarios `server-sse-multiple-streams` and
//! `dns-rebinding-protection`, and the streaming probes of `server-stateless`, in process
//! against the generated root, and read every stream with an independent reader of the
//! WHATWG event stream format (`lifecycle_oracle::parse_event_stream`). They are falsifying
//! evidence: the suite run against the conformance build (issue 13) is the verdict.

mod common;

use std::collections::BTreeSet;

use common::{
    after_calls, all_vectors, events_of, exchange_checked, exchange_checked_allowing,
    frame_checked, frame_checked_on, header, is_stream, notes_of, post_owned, request_of, rpc,
    run_register, send, standard, suite_text, transport_vector_file, META, PIN,
};
use lifecycle_oracle::{encode_request, encode_request_allowing, Request, Response, STACK_BYTES};
use serde_json::{json, Value};

const BOTH: &str = "application/json, text/event-stream";

/// A tools/call of the progress tool with a token, and an Accept field when given.
fn progress_call(accept: Option<&str>, token: &str, id: u32) -> Request {
    let mut headers = standard("tools/call", Some("test_tool_with_progress"));
    if let Some(accept) = accept {
        headers.push(("Accept".to_owned(), accept.to_owned()));
    }
    let body = format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"test_tool_with_progress","arguments":{{}},"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{{}},"progressToken":"{token}"}}}}}}"#
    );
    post_owned(&headers, &body)
}

/// A server/discover request with `extra` header fields after the standard ones.
fn discover_with(extra: &[(&str, &str)]) -> Request {
    let mut headers = standard("server/discover", None);
    for (name, value) in extra {
        headers.push(((*name).to_owned(), (*value).to_owned()));
    }
    post_owned(
        &headers,
        &rpc("server/discover", &format!(r#"{{"_meta":{META}}}"#), "1"),
    )
}

fn names(allowed: &[&str]) -> Vec<Vec<u8>> {
    allowed.iter().map(|n| n.as_bytes().to_vec()).collect()
}

/// The status and the body of a discover request with the given Host and Origin.
fn judged(host: Option<&str>, origin: Option<&str>, allowed: &[&str]) -> (Response, Value) {
    let mut extra: Vec<(&str, &str)> = Vec::new();
    if let Some(host) = host {
        extra.push(("Host", host));
    }
    if let Some(origin) = origin {
        extra.push(("Origin", origin));
    }
    let response = exchange_checked_allowing(&discover_with(&extra), &names(allowed))
        .expect("a response frame");
    let body = common::json_of(&response);
    (response, body)
}

fn assert_forbidden(response: &Response, body: &Value, message: &str) {
    assert_eq!(response.status, 403, "{body}");
    assert_eq!(
        header(response, "content-type"),
        Some(&b"application/json"[..])
    );
    assert_eq!(body["jsonrpc"], "2.0");
    assert_eq!(body["error"]["code"], -32000);
    assert_eq!(body["error"]["message"], message);
    assert!(body.get("id").is_none(), "a forbidden answer has no id");
    assert!(body.get("result").is_none());
    assert!(body["error"].get("data").is_none(), "nothing is disclosed");
}

fn vector<'a>(file: &'a Value, id: &str) -> &'a Value {
    file["vectors"]
        .as_array()
        .expect("vectors")
        .iter()
        .find(|v| v["id"] == id)
        .unwrap_or_else(|| panic!("vector {id}"))
}

// ---- SH-01: the event stream ----

#[test]
fn a_request_that_accepts_the_stream_and_has_notifications_is_answered_as_a_stream_sh_01() {
    assert_eq!(run_register("SH-01"), 2);
    let response = exchange_checked(&progress_call(Some(BOTH), "t", 8)).expect("a response");
    assert_eq!(response.status, 200);
    assert!(is_stream(&response));
    assert!(response.notifications.is_empty());
    let events = events_of(&response);
    assert_eq!(events.len(), 4, "three notifications and the result");
    for (i, note) in events[..3].iter().enumerate() {
        assert_eq!(note["jsonrpc"], "2.0");
        assert_eq!(note["method"], "notifications/progress");
        assert!(note.get("id").is_none(), "a notification has no id");
        assert_eq!(note["params"]["progressToken"], "t");
        assert_eq!(note["params"]["progress"], [0, 50, 100][i]);
    }
    assert_eq!(events[3]["id"], 8);
    assert_eq!(events[3]["result"]["resultType"], "complete");
}

#[test]
fn the_stream_is_exactly_the_events_of_its_messages_with_the_json_result_last_sh_01() {
    let file = transport_vector_file();
    let vector = vector(&file, "SP-01");
    let streamed = exchange_checked(&request_of(vector)).expect("a response");
    // the same request from a client that does not accept the stream: the result alone
    let mut twin = request_of(vector);
    twin.fields
        .retain(|(n, _)| !n.eq_ignore_ascii_case(b"accept"));
    let json = exchange_checked(&twin).expect("a response");
    assert_eq!(
        header(&json, "content-type"),
        Some(&b"application/json"[..])
    );
    let mut want = String::new();
    for line in vector["expect"]["stream_notifications"]
        .as_array()
        .expect("lines")
    {
        want.push_str(&format!("data: {}\n\n", line.as_str().expect("text")));
    }
    want.push_str(&format!(
        "data: {}\n\n",
        String::from_utf8(json.body).expect("UTF-8")
    ));
    assert_eq!(
        String::from_utf8(streamed.body.clone()).expect("UTF-8"),
        want
    );
}

#[test]
fn the_stream_carries_the_headers_a_stream_needs_and_no_others_sh_01() {
    let response = exchange_checked(&progress_call(Some(BOTH), "t", 8)).expect("a response");
    let fields: Vec<(String, String)> = response
        .fields
        .iter()
        .map(|(n, v)| {
            (
                String::from_utf8(n.clone()).expect("name"),
                String::from_utf8(v.clone()).expect("value"),
            )
        })
        .collect();
    assert_eq!(
        fields,
        vec![
            ("Content-Type".to_owned(), "text/event-stream".to_owned()),
            ("Cache-Control".to_owned(), "no-cache".to_owned()),
            ("X-Accel-Buffering".to_owned(), "no".to_owned()),
        ]
    );
}

#[test]
fn a_stream_has_no_resumption_fields_and_every_line_is_a_data_line_sh_01() {
    for accept in [
        BOTH,
        "text/event-stream",
        "text/event-stream, application/json",
        "application/json, text/event-stream;q=0.5",
        "TEXT/EVENT-STREAM",
        "text/event-stream ; q=1",
    ] {
        let response = exchange_checked(&progress_call(Some(accept), "t", 8)).expect("a response");
        assert!(is_stream(&response), "{accept}");
        let text = String::from_utf8(response.body.clone()).expect("UTF-8");
        assert!(text.ends_with("\n\n"));
        assert!(!text.contains('\r'), "no carriage return is ever written");
        for line in text.split('\n').filter(|l| !l.is_empty()) {
            assert!(line.starts_with("data: "), "{accept}: {line:?}");
        }
        // an event is one data line and a blank line, so the lines pair up
        assert_eq!(text.matches("\n\n").count(), 4, "{accept}");
        assert_eq!(text.matches("\ndata: ").count() + 1, 4, "{accept}");
    }
}

#[test]
fn the_tokens_of_a_stream_are_the_requests_own_sh_01() {
    for (token, id) in [("a", 1), ("", 2), ("tok-\u{e9}", 3), ("0", 4)] {
        let response = exchange_checked(&progress_call(Some(BOTH), token, id)).expect("a response");
        let events = events_of(&response);
        assert_eq!(events.last().expect("the result")["id"], id);
        for note in notes_of(&response) {
            assert_eq!(note["params"]["progressToken"], token);
        }
    }
}

// ---- SH-02: the single JSON object ----

#[test]
fn a_client_that_does_not_accept_the_stream_gets_the_result_alone_as_json_sh_02() {
    for accept in [
        None,
        Some("application/json"),
        Some("*/*"),
        Some("text/*"),
        Some("text/event-streams"),
        Some("xtext/event-stream"),
        Some("text/event-stream;q=0"),
        Some("text/event-stream;q=0.000"),
        Some("application/json, text/event-stream;q=0"),
        Some(""),
    ] {
        let response = exchange_checked(&progress_call(accept, "t", 8)).expect("a response");
        assert_eq!(response.status, 200, "{accept:?}");
        assert_eq!(
            header(&response, "content-type"),
            Some(&b"application/json"[..]),
            "{accept:?}"
        );
        assert!(!is_stream(&response));
        assert!(notes_of(&response).is_empty());
        assert!(response.notifications.is_empty());
        let body = common::json_of(&response);
        assert_eq!(body["id"], 8);
        assert_eq!(body["result"]["resultType"], "complete");
        assert_eq!(
            response.fields.len(),
            1,
            "only the Content-Type: {accept:?}"
        );
    }
}

#[test]
fn an_answer_with_nothing_to_stream_is_json_even_when_the_stream_is_accepted_sh_02() {
    assert_eq!(run_register("SH-02"), 3);
    // the same call from a tool that reports no progress
    let mut headers = standard("tools/call", Some("test_simple_text"));
    headers.push(("Accept".to_owned(), "text/event-stream".to_owned()));
    let body = common::call("test_simple_text", "{}", "5");
    let (response, json) = send(&post_owned(&headers, &body));
    assert_eq!(response.status, 200);
    assert_eq!(
        header(&response, "content-type"),
        Some(&b"application/json"[..])
    );
    assert_eq!(json["id"], 5);
    // and a request that carries a token for a tool that reports none
    let (response, _) = send(&progress_call_for("test_simple_text", Some(BOTH)));
    assert!(!is_stream(&response));
}

/// A tools/call of `tool` with a progress token and the given Accept.
fn progress_call_for(tool: &str, accept: Option<&str>) -> Request {
    let mut headers = standard("tools/call", Some(tool));
    if let Some(accept) = accept {
        headers.push(("Accept".to_owned(), accept.to_owned()));
    }
    let body = format!(
        r#"{{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{{"name":"{tool}","arguments":{{}},"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{{}},"progressToken":"t"}}}}}}"#
    );
    post_owned(&headers, &body)
}

#[test]
fn an_answer_with_a_status_other_than_200_is_never_a_stream_sh_02() {
    let accept = ("Accept".to_owned(), BOTH.to_owned());
    let mut cases: Vec<(Request, u32)> = Vec::new();
    // 400: a header that disagrees with the body, a tool the catalog does not list
    let mut mismatch = progress_call(Some(BOTH), "t", 8);
    mismatch
        .fields
        .retain(|(n, _)| !n.eq_ignore_ascii_case(b"mcp-method"));
    cases.push((mismatch, 400));
    cases.push((progress_call_for("no_such_tool", Some(BOTH)), 400));
    // 429: the host's count is at the limit
    cases.push((after_calls(progress_call(Some(BOTH), "t", 8), 100), 429));
    // 404: a method the core does not register
    let mut listen = standard("subscriptions/listen", None);
    listen.push(accept.clone());
    cases.push((
        post_owned(
            &listen,
            &rpc(
                "subscriptions/listen",
                &format!(r#"{{"_meta":{META}}}"#),
                "1",
            ),
        ),
        404,
    ));
    // 500: a tool whose result the core will not write
    cases.push((progress_call_for("test_bad_payload", Some(BOTH)), 500));
    // 405: a method other than POST
    let mut get = progress_call(Some(BOTH), "t", 8);
    get.method = b"GET".to_vec();
    cases.push((get, 405));
    for (request, status) in cases {
        let response = exchange_checked(&request).expect("a response");
        assert_eq!(response.status, status);
        assert!(!is_stream(&response), "status {status} is never a stream");
        if status != 405 {
            assert_eq!(
                header(&response, "content-type"),
                Some(&b"application/json"[..])
            );
        }
        assert!(response.notifications.is_empty());
    }
}

// ---- SH-03: independent streams ----

#[test]
fn concurrent_posts_are_independent_exchanges_each_with_its_own_stream_sh_03() {
    assert_eq!(run_register("SH-03"), 3);
    let requests: Vec<Request> = vec![
        progress_call(Some(BOTH), "a", 11),
        progress_call(Some(BOTH), "b", 12),
        progress_call(Some(BOTH), "c", 13),
        progress_call(None, "d", 14),
    ];
    let alone: Vec<Response> = requests
        .iter()
        .map(|r| frame_checked(&encode_request(r)).expect("a response"))
        .collect();
    // the same frames on separate threads, repeatedly and interleaved
    let handles: Vec<_> = (0..8)
        .map(|round| {
            let requests = requests.clone();
            std::thread::spawn(move || {
                requests
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(round)
                    .take(requests.len())
                    .map(|(index, r)| {
                        (
                            index,
                            frame_checked(&encode_request(r)).expect("a response"),
                        )
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    for handle in handles {
        for (index, answer) in handle.join().expect("a thread") {
            assert_eq!(
                answer, alone[index],
                "the answer depends on its frame alone"
            );
        }
    }
    // each stream names its own token and id
    for (request, (token, id)) in requests
        .iter()
        .zip([("a", 11), ("b", 12), ("c", 13), ("d", 14)])
    {
        let response = exchange_checked(request).expect("a response");
        if is_stream(&response) {
            for note in notes_of(&response) {
                assert_eq!(note["params"]["progressToken"], token);
            }
        }
        assert_eq!(common::json_of(&response)["id"], id);
    }
}

// ---- SH-04, SH-05, SH-07: DNS rebinding ----

#[test]
fn an_origin_outside_the_allow_list_is_forbidden_before_anything_is_read_sh_04() {
    assert_eq!(run_register("SH-04"), 2);
    let (response, body) = judged(
        Some("evil.example.com"),
        Some("http://evil.example.com"),
        &[],
    );
    assert_forbidden(&response, &body, "Forbidden: invalid Origin");
    // the Origin of a rebinding request is checked ahead of the method, the body and the provider
    let mut get = discover_with(&[("Origin", "http://evil.example.com")]);
    get.method = b"GET".to_vec();
    let response = exchange_checked(&get).expect("a response");
    assert_eq!(response.status, 403, "403, not the 405 of the method");
    let garbage = post_owned(
        &[("Origin".to_owned(), "http://evil.example.com".to_owned())],
        "this is not JSON",
    );
    let response = exchange_checked(&garbage).expect("a response");
    assert_eq!(response.status, 403, "403, not the 400 of the body");
    let limited = after_calls(
        post_owned(
            &{
                let mut h = standard("tools/call", Some("test_simple_text"));
                h.push(("Origin".to_owned(), "http://evil.example.com".to_owned()));
                h
            },
            &common::call("test_simple_text", "{}", "7"),
        ),
        10_000,
    );
    let response = exchange_checked(&limited).expect("a response");
    assert_eq!(response.status, 403, "403, not the 429 of the rate limit");
}

#[test]
fn the_loopback_names_are_allowed_with_any_port_and_either_scheme_sh_04() {
    for (host, origin) in [
        ("127.0.0.1:8080", "http://127.0.0.1:8080"),
        ("127.0.0.1", "http://127.0.0.1"),
        ("localhost:3000", "https://localhost:3000"),
        ("localhost", "http://localhost"),
        ("[::1]:3000", "http://[::1]:3000"),
        ("LocalHost:1", "HTTP://LOCALHOST:65535"),
        (" localhost ", "  http://localhost  "),
    ] {
        let (response, body) = judged(Some(host), Some(origin), &[]);
        assert_eq!(response.status, 200, "{host} {origin}: {body}");
        assert!(body.get("error").is_none());
    }
}

#[test]
fn a_foreign_origin_with_a_loopback_host_is_forbidden_and_says_origin_sh_04() {
    for origin in [
        "http://evil.example.com",
        "https://evil.example.com:8443",
        "http://127.0.0.2",
        "http://0.0.0.0:3000",
        "http://[::2]",
        "http://localhost.evil.example",
        "http://evil.example/localhost",
        "chrome-extension://abcdef",
    ] {
        let (response, body) = judged(Some("127.0.0.1:3000"), Some(origin), &[]);
        assert_forbidden(&response, &body, "Forbidden: invalid Origin");
    }
}

#[test]
fn a_foreign_host_is_forbidden_and_says_host_sh_05() {
    for host in [
        "evil.example.com",
        "evil.example.com:80",
        "127.0.0.1.evil.example",
        "localhost.evil.example",
        "0.0.0.0:3000",
        "[::2]:1",
    ] {
        let (response, body) = judged(Some(host), None, &[]);
        assert_forbidden(&response, &body, "Forbidden: invalid Host");
        let (response, body) = judged(Some(host), Some("http://localhost"), &[]);
        assert_forbidden(&response, &body, "Forbidden: invalid Host");
    }
}

#[test]
fn the_origin_is_judged_before_the_host_sh_05() {
    let (response, body) = judged(
        Some("evil.example.com"),
        Some("http://evil.example.com"),
        &[],
    );
    assert_forbidden(&response, &body, "Forbidden: invalid Origin");
}

#[test]
fn a_request_with_no_origin_or_no_host_is_not_refused_for_its_absence_sh_05() {
    for (host, origin) in [
        (None, None),
        (Some("127.0.0.1:3000"), None),
        (None, Some("http://localhost:3000")),
    ] {
        let (response, body) = judged(host, origin, &[]);
        assert_eq!(response.status, 200, "{host:?} {origin:?}: {body}");
    }
}

#[test]
fn an_origin_or_host_the_core_cannot_judge_is_forbidden_sh_07() {
    let origins = [
        "null",
        "http://localhost/path",
        "http://localhost@evil.example",
        "http://evil.example@localhost",
        "http://localhost:80x",
        "http://localhost:",
        "http://localhost:-1",
        "localhost",
        "//localhost",
        "http:/localhost",
        "http://[::1",
        "http://",
        "",
        "http://localhost\u{1}",
        "http://local\u{7f}host",
        "http://localhost.evil.example",
        "http://127.0.0.1.evil.example",
    ];
    for origin in origins {
        let (response, body) = judged(None, Some(origin), &[]);
        assert_forbidden(&response, &body, "Forbidden: invalid Origin");
    }
    let hosts = [
        "",
        "[::1",
        "[::1]:",
        "[::1]x",
        "localhost:",
        "localhost:80x",
        "localhost\u{1}",
        "local host",
        "localhost/x",
        "user@localhost",
        "localhost.evil.example",
    ];
    for host in hosts {
        let (response, body) = judged(Some(host), None, &[]);
        assert_forbidden(&response, &body, "Forbidden: invalid Host");
    }
}

#[test]
fn a_repeated_origin_or_host_is_forbidden_even_when_every_copy_is_allowed_sh_07() {
    let (response, body) = {
        let r = exchange_checked(&discover_with(&[
            ("Origin", "http://localhost"),
            ("Origin", "http://localhost"),
        ]))
        .expect("a response");
        let b = common::json_of(&r);
        (r, b)
    };
    assert_forbidden(&response, &body, "Forbidden: invalid Origin");
    let r = exchange_checked(&discover_with(&[
        ("Host", "localhost"),
        ("host", "localhost"),
    ]))
    .expect("a response");
    let b = common::json_of(&r);
    assert_forbidden(&r, &b, "Forbidden: invalid Host");
}

#[test]
fn header_names_of_the_gate_are_read_without_regard_to_case_sh_07() {
    for name in ["origin", "ORIGIN", "OrIgIn"] {
        let r = exchange_checked(&discover_with(&[(name, "http://evil.example.com")]))
            .expect("a response");
        assert_eq!(r.status, 403, "{name}");
    }
    for name in ["host", "HOST", "hOsT"] {
        let r =
            exchange_checked(&discover_with(&[(name, "evil.example.com")])).expect("a response");
        assert_eq!(r.status, 403, "{name}");
    }
}

// ---- SH-06: the streaming probes of server-stateless ----

#[test]
fn subscriptions_listen_is_not_found_while_no_subscription_capability_is_advertised_sh_06() {
    assert_eq!(run_register("SH-06"), 3);
    // whatever the client accepts, the probes get the same refusal
    for accept in [None, Some("text/event-stream"), Some(BOTH)] {
        let mut headers = standard("subscriptions/listen", None);
        if let Some(accept) = accept {
            headers.push(("Accept".to_owned(), accept.to_owned()));
        }
        let params = format!(r#"{{"_meta":{META},"notifications":{{"toolsListChanged":true}}}}"#);
        let (response, body) = send(&post_owned(
            &headers,
            &rpc("subscriptions/listen", &params, "5"),
        ));
        assert_eq!(
            (response.status, &body["error"]["code"]),
            (404, &json!(-32601))
        );
        assert!(!is_stream(&response));
    }
}

// ---- SH-08: the allow-list in the frame ----

#[test]
fn a_name_the_host_configured_is_allowed_and_any_other_is_not_sh_08() {
    let request = discover_with(&[
        ("Host", "MCP.example.test:443"),
        ("Origin", "https://mcp.example.test"),
    ]);
    let configured =
        exchange_checked_allowing(&request, &names(&["mcp.example.test"])).expect("a response");
    assert_eq!(configured.status, 200);
    let with_others =
        exchange_checked_allowing(&request, &names(&["a.test", "mcp.example.test", "b.test"]))
            .expect("a response");
    assert_eq!(with_others.status, 200);
    for allowed in [
        names(&[]),
        names(&["other.example.test"]),
        names(&["mcp.example.tes"]),
        names(&["mcp.example.test.evil"]),
        names(&[""]),
    ] {
        let refused = exchange_checked_allowing(&request, &allowed).expect("a response");
        assert_eq!(refused.status, 403, "{allowed:?}");
    }
    // a configured name does not widen the loopback names, and does not shadow them
    let loopback = discover_with(&[("Host", "127.0.0.1:1")]);
    let r =
        exchange_checked_allowing(&loopback, &names(&["mcp.example.test"])).expect("a response");
    assert_eq!(r.status, 200);
}

#[test]
fn a_frame_that_ends_after_the_count_or_names_none_carries_no_allow_list_sh_08() {
    let request = discover_with(&[("Host", "evil.example.com")]);
    let base = encode_request(&request);
    assert_eq!(base, encode_request_allowing(&request, &[]));
    assert_eq!(frame_checked(&base).expect("a response").status, 403);
    // an explicit count of zero is the same frame with nothing added
    let mut zero = base.clone();
    zero.extend_from_slice(&[0, 0, 0, 0]);
    assert_eq!(frame_checked(&zero).expect("a response").status, 403);
    let ok = discover_with(&[("Host", "localhost")]);
    let mut zero = encode_request(&ok);
    zero.extend_from_slice(&[0, 0, 0, 0]);
    assert_eq!(frame_checked(&zero).expect("a response").status, 200);
}

#[test]
fn a_frame_cut_inside_the_allow_list_or_followed_by_bytes_is_unreadable_sh_08() {
    let request = discover_with(&[("Host", "evil.example.com")]);
    let base = encode_request(&request);
    let good = encode_request_allowing(&request, &names(&["evil.example.com", "second.test"]));
    assert!(good.len() > base.len() + 8);
    assert_eq!(frame_checked(&good).expect("a response").status, 200);
    let mut frames: Vec<Vec<u8>> = (base.len() + 1..good.len())
        .map(|cut| good[..cut].to_vec())
        .collect();
    let mut trailing = good.clone();
    trailing.push(0);
    frames.push(trailing);
    let mut short = base.clone();
    short.extend_from_slice(&[0, 0, 0, 1]);
    frames.push(short);
    let mut huge = base.clone();
    huge.extend_from_slice(&[255, 255, 255, 255]);
    frames.push(huge);
    for frame in frames {
        let response = frame_checked(&frame).expect("a response frame");
        assert_eq!(response.status, 500, "frame of {} bytes", frame.len());
        let body: Value = serde_json::from_slice(&response.body).expect("JSON body");
        assert_eq!(body["error"]["code"], -32603);
        assert!(body.get("id").is_none());
    }
}

// ---- SH-09: loops, not recursion ----

#[test]
fn a_very_large_accept_field_and_allow_list_are_judged_on_the_default_stack_sh_09() {
    let mut accept = String::new();
    for i in 0..200_000 {
        accept.push_str(&format!("x/{i},"));
    }
    let mut streamed = progress_call(Some(&format!("{accept}text/event-stream")), "t", 8);
    let response = frame_checked_on(&encode_request(&streamed), STACK_BYTES).expect("a response");
    assert!(is_stream(&response), "the last range is the event stream");
    streamed
        .fields
        .retain(|(n, _)| !n.eq_ignore_ascii_case(b"accept"));
    streamed.fields.push((
        b"Accept".to_vec(),
        format!("{accept}text/plain").into_bytes(),
    ));
    let response = frame_checked_on(&encode_request(&streamed), STACK_BYTES).expect("a response");
    assert!(!is_stream(&response));

    let allowed: Vec<Vec<u8>> = (0..100_000)
        .map(|i| format!("n{i}.example.test").into_bytes())
        .collect();
    let request = discover_with(&[
        ("Host", "n99999.example.test"),
        ("Origin", "http://n0.example.test"),
    ]);
    let response = frame_checked_on(&encode_request_allowing(&request, &allowed), STACK_BYTES)
        .expect("a response");
    assert_eq!(response.status, 200);
    let request = discover_with(&[("Host", "n100000.example.test")]);
    let response = frame_checked_on(&encode_request_allowing(&request, &allowed), STACK_BYTES)
        .expect("a response");
    assert_eq!(response.status, 403);
}

// ---- SH-10: what the scenarios send is accounted for ----

fn line_of(text: &str, line: u64) -> &str {
    text.lines()
        .nth(usize::try_from(line).expect("line fits") - 1)
        .unwrap_or_else(|| panic!("line {line} is past the end of the file"))
}

/// The lines (1-based) of the two transport scenarios that send a request.
fn sites(file: &str, text: &str) -> BTreeSet<u64> {
    let marker = if file.ends_with("sse-multiple-streams.ts") {
        "fetch(serverUrl,"
    } else {
        "await sendRequestWithHostAndOrigin("
    };
    text.lines()
        .enumerate()
        .filter(|(_, l)| l.contains(marker))
        .map(|(i, _)| i as u64 + 1)
        .collect()
}

#[test]
fn every_request_the_transport_scenarios_send_is_a_vector_sh_10() {
    let vectors = all_vectors();
    for file in [
        "src/scenarios/server/sse-multiple-streams.ts",
        "src/scenarios/server/dns-rebinding.ts",
    ] {
        let text = suite_text(file);
        let mut accounted = BTreeSet::new();
        for v in vectors.iter().filter(|v| v["file"] == file) {
            let line = v["line"].as_u64().expect("line");
            let quote = v["quote"].as_str().expect("quote");
            assert!(
                line_of(&text, line).contains(quote),
                "{}: line {line} of {file} does not contain {quote:?}",
                v["id"]
            );
            accounted.insert(line);
        }
        assert_eq!(sites(file, &text), accounted, "{file}");
    }
}

#[test]
fn every_vector_and_listed_probe_of_the_transport_cites_the_pin_and_a_real_line_sh_10() {
    let file = transport_vector_file();
    assert_eq!(file["suite"]["commit"], PIN);
    let mut ids = BTreeSet::new();
    for v in file["vectors"].as_array().expect("vectors") {
        assert!(
            ids.insert(v["id"].as_str().expect("id").to_owned()),
            "{}",
            v["id"]
        );
        let text = suite_text(v["file"].as_str().expect("file"));
        assert!(
            line_of(&text, v["line"].as_u64().expect("line"))
                .contains(v["quote"].as_str().expect("quote")),
            "{}",
            v["id"]
        );
        assert!(!v["register"].as_array().expect("register").is_empty());
    }
    for n in file["not_run"].as_array().expect("not_run") {
        let text = suite_text(n["file"].as_str().expect("file"));
        assert!(
            line_of(&text, n["line"].as_u64().expect("line"))
                .contains(n["quote"].as_str().expect("quote")),
            "line {}",
            n["line"]
        );
        assert!(n["owner"].as_str().expect("owner").starts_with('#'));
        assert!(!n["reason"].as_str().expect("reason").is_empty());
    }
}

#[test]
fn every_check_id_the_transport_scenarios_emit_is_a_vector_check_or_listed_sh_10() {
    let file = transport_vector_file();
    let mut known = BTreeSet::new();
    for v in file["vectors"].as_array().expect("vectors") {
        for c in v["suite_checks"].as_array().expect("suite_checks") {
            known.insert(c.as_str().expect("id").to_owned());
        }
    }
    for u in file["unreached"].as_array().expect("unreached") {
        known.insert(u["id"].as_str().expect("id").to_owned());
        assert!(!u["reason"].as_str().expect("reason").is_empty());
    }
    let mut emitted = BTreeSet::new();
    for scenario in [
        "src/scenarios/server/sse-multiple-streams.ts",
        "src/scenarios/server/dns-rebinding.ts",
    ] {
        let text = suite_text(scenario);
        for part in text.split("id: '").skip(1) {
            let id = part.split('\'').next().expect("a quoted id");
            // `SEP-1699` and the like are specification references, not checks
            if id.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
                emitted.insert(id.to_owned());
            }
        }
    }
    let scenario_checks: BTreeSet<String> = known
        .iter()
        .filter(|id| {
            id.starts_with("server-sse")
                || id.starts_with("server-accepts")
                || id.starts_with("localhost-")
        })
        .cloned()
        .collect();
    assert_eq!(
        emitted, scenario_checks,
        "emitted ids and covered ids differ"
    );
    assert!(emitted.len() >= 5, "{emitted:?}");
}
