//! PE-01 to PE-09: a provider requests a host effect as a typed outcome (issue 39).
//!
//! The root under test is `UorMcp.StepFixture.stepFixtureRun`: the step serve over a
//! provider that requests host effects on the tools, the resource and the prompt named
//! below, and over the unchanged fixture provider for every other request. Each test acts
//! as PrismPM's generated driver does: it calls the root with a turn, performs the effect
//! the step asks for, appends the outcome and calls the root again. The wire is the one in
//! `PROVIDER-EFFECTS.md`; the sources are `src/HostEffect.lex.tex` and
//! `src/StepServe.lex.tex`. They are falsifying evidence: the kernel-checked laws are
//! `src/StepLaws.lex.tex` (PE-08).

mod common;

use std::collections::BTreeSet;

use common::{
    allowed_hosts_of, all_vectors, call, check_response, json_of, post_owned, request_of, root,
    rpc, standard, after_calls, META,
};
use lifecycle_oracle::{
    decode_response, decode_step, drive, encode_request, encode_request_allowing, encode_turn,
    exchange_frame, exchange_step, DriveError, Driven, Request, Response, Step, StepError,
};
use serde_json::Value;

/// The effect budget the tests give the driver (PrismPM bounds it in the declaration).
const BUDGET: usize = 8;

/// A tools/call of `name` with `arguments` (a JSON text).
fn tool_call(name: &str, arguments: &str) -> Request {
    post_owned(&standard("tools/call", Some(name)), &call(name, arguments, "1"))
}

fn read_effect() -> Request {
    let body = rpc(
        "resources/read",
        &format!(r#"{{"uri":"effect://item","_meta":{META}}}"#),
        "1",
    );
    post_owned(&standard("resources/read", Some("effect://item")), &body)
}

fn prompt_effect() -> Request {
    let body = rpc(
        "prompts/get",
        &format!(r#"{{"name":"effect_prompt","_meta":{META}}}"#),
        "1",
    );
    post_owned(&standard("prompts/get", Some("effect_prompt")), &body)
}

/// A host that answers every effect with the text `o<tag>`.
fn answer_with_tag(tag: u32, _payload: &[u8]) -> Vec<u8> {
    format!("o{tag}").into_bytes()
}

/// A host that panics: a request that must not perform any effect is driven with it.
fn never(tag: u32, payload: &[u8]) -> Vec<u8> {
    panic!(
        "an effect was requested: tag {tag}, payload {:?}",
        String::from_utf8_lossy(payload)
    );
}

fn driven(request: &Request) -> (Driven, Response, Value) {
    let frame = encode_request(request);
    let driven = drive(&frame, BUDGET, answer_with_tag).expect("the root writes steps");
    let response = decode_response(&driven.response).expect("a response frame");
    let body = json_of(&response);
    (driven, response, body)
}

fn first_step(request: &Request) -> Step {
    decode_step(&exchange_step(&encode_turn(&encode_request(request), &[])))
        .expect("the root writes a step")
}

fn done_response(step: Step) -> Response {
    match step {
        Step::Done(bytes) => decode_response(&bytes).expect("a response frame"),
        other => panic!("expected done, found {other:?}"),
    }
}

fn code_of(body: &Value) -> Option<i64> {
    body.pointer("/error/code").and_then(Value::as_i64)
}

fn text_of(body: &Value) -> &str {
    body.pointer("/result/content/0/text")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("no text content in {body}"))
}

fn source(name: &str) -> String {
    std::fs::read_to_string(root().join("src").join(format!("{name}.lex.tex")))
        .unwrap_or_else(|e| panic!("src/{name}.lex.tex reads: {e}"))
}

// ---- PE-01: the typed outcome ----

#[test]
fn a_call_a_read_and_a_prompt_each_ask_for_an_effect_with_a_typed_outcome_pe_01() {
    let call = first_step(&tool_call("provider_effect", "{}"));
    assert_eq!(
        call,
        Step::Perform {
            tag: 3,
            payload: b"call".to_vec()
        },
        "CallOutcome.perform"
    );
    let read = first_step(&read_effect());
    assert_eq!(
        read,
        Step::Perform {
            tag: 4,
            payload: b"effect://item".to_vec()
        },
        "ReadOutcome.perform"
    );
    let prompt = first_step(&prompt_effect());
    assert_eq!(
        prompt,
        Step::Perform {
            tag: 5,
            payload: b"effect_prompt".to_vec()
        },
        "PromptOutcome.perform"
    );
}

#[test]
fn the_three_outcomes_declare_the_constructor_and_answer_of_maps_each_to_the_refusal_pe_01() {
    let extension = source("Extension");
    for outcome in ["CallOutcome", "ReadOutcome", "PromptOutcome"] {
        let at = extension
            .find(&format!("name: {outcome},"))
            .unwrap_or_else(|| panic!("{outcome} is declared"));
        let block = &extension[at..];
        let block = &block[..block.find("\n}").expect("the declaration ends")];
        assert!(
            block.contains("{name: perform, fields: [HostEffect::EffectRequest]}"),
            "{outcome} has no constructor perform that carries an EffectRequest"
        );
    }
    assert_eq!(
        extension
            .matches("ProviderRefusal.effectRequested")
            .count(),
        3,
        "answerOf maps each of the three outcomes to effectRequested"
    );
    let refusal = source("Refusal");
    assert!(refusal.contains("{name: effectRequested, fields: [HostEffect::EffectRequest]}"));
}

#[test]
fn no_result_carries_the_request_of_an_effect_pe_01() {
    // The request an effect carries is a step, not a result: the tool result of the same
    // call, once replayed, holds the outcome and nothing of the request.
    let (driven, response, body) = driven(&tool_call("effect_read", r#"{"key":"alpha"}"#));
    assert_eq!(driven.performed, vec![(1, b"alpha".to_vec())]);
    check_response(Some("tools/call"), &response);
    assert_eq!(text_of(&body), "read:o1");
    let wire = String::from_utf8_lossy(&driven.response).into_owned();
    assert!(
        !wire.contains("alpha"),
        "the payload of the effect appears in the response: {wire}"
    );
}

// ---- PE-02: the turn and the step ----

#[test]
fn a_perform_step_is_the_word_one_the_tag_and_the_payload_pe_02() {
    let written = exchange_step(&encode_turn(
        &encode_request(&tool_call("effect_read", r#"{"key":"alpha"}"#)),
        &[],
    ));
    let mut want = vec![0, 0, 0, 1, 0, 0, 0, 1];
    want.extend_from_slice(b"alpha");
    assert_eq!(written, want);
}

#[test]
fn a_done_step_is_the_word_zero_and_the_response_frame_pe_02() {
    let body = rpc("server/discover", &format!(r#"{{"_meta":{META}}}"#), "1");
    let request = post_owned(&standard("server/discover", None), &body);
    let written = exchange_step(&encode_turn(&encode_request(&request), &[]));
    assert_eq!(&written[..4], &[0, 0, 0, 0]);
    let response = decode_response(&written[4..]).expect("the rest is a response frame");
    assert_eq!(response.status, 200);
}

#[test]
fn the_turn_is_the_request_chunk_the_count_and_the_completed_effects_pe_02() {
    let turn = encode_turn(b"AB", &[(7, b"xyz".to_vec())]);
    let want: Vec<u8> = [
        &[0, 0, 0, 2][..],
        b"AB",
        &[0, 0, 0, 1],
        &[0, 0, 0, 7],
        &[0, 0, 0, 3],
        b"xyz",
    ]
    .concat();
    assert_eq!(turn, want, "the layout of PrismPM's exchange driver");
    assert_eq!(
        decode_step(&[0, 0, 0, 2]),
        Err(StepError::Unknown(2)),
        "a step is one of two words"
    );
    assert_eq!(decode_step(&[0, 0]), Err(StepError::Truncated));
}

fn internal_error_frame() -> Vec<u8> {
    // The frame the byte serve writes for a frame it cannot read.
    exchange_frame(b"")
}

#[test]
fn every_proper_prefix_of_a_turn_is_done_with_the_internal_error_and_never_performs_pe_02() {
    let request = tool_call("effect_chain", r#"{"first":"x","second":"y"}"#);
    let turn = encode_turn(
        &encode_request(&request),
        &[(1, b"ox".to_vec()), (2, b"oy".to_vec())],
    );
    // The whole turn is a turn: its step is done and holds the chained result.
    let whole = done_response(decode_step(&exchange_step(&turn)).expect("a step"));
    assert_eq!(whole.status, 200);
    assert_eq!(text_of(&json_of(&whole)), "chain:ox|oy");
    let internal = internal_error_frame();
    let response = decode_response(&internal).expect("a response frame");
    assert_eq!(response.status, 500);
    assert_eq!(code_of(&json_of(&response)), Some(-32603));
    for len in 0..turn.len() {
        let step = decode_step(&exchange_step(&turn[..len])).expect("the root writes a step");
        assert_eq!(
            step,
            Step::Done(internal.clone()),
            "a turn cut after {len} of {} bytes",
            turn.len()
        );
    }
}

#[test]
fn a_byte_after_the_last_outcome_is_done_with_the_internal_error_pe_02() {
    let request = tool_call("effect_read", r#"{"key":"alpha"}"#);
    let mut turn = encode_turn(&encode_request(&request), &[(1, b"o1".to_vec())]);
    turn.push(0);
    assert_eq!(
        decode_step(&exchange_step(&turn)).expect("a step"),
        Step::Done(internal_error_frame())
    );
}

#[test]
fn a_request_frame_the_byte_serve_cannot_read_is_done_with_the_internal_error_pe_02() {
    assert_eq!(
        decode_step(&exchange_step(&encode_turn(b"junk", &[]))).expect("a step"),
        Step::Done(internal_error_frame())
    );
}

// ---- PE-03: perform, replay, done ----

#[test]
fn a_tool_that_requests_an_effect_is_performed_and_replayed_to_its_final_response_pe_03() {
    let request = tool_call("effect_read", r#"{"key":"alpha"}"#);
    assert_eq!(
        first_step(&request),
        Step::Perform {
            tag: 1,
            payload: b"alpha".to_vec()
        }
    );
    let frame = encode_request(&request);
    let seen = drive(&frame, BUDGET, |tag, payload| {
        assert_eq!((tag, payload), (1, &b"alpha"[..]));
        b"the value".to_vec()
    })
    .expect("the root writes steps");
    assert_eq!(seen.turns, 2, "one turn to ask and one to answer");
    assert_eq!(seen.performed, vec![(1, b"alpha".to_vec())]);
    let response = decode_response(&seen.response).expect("a response frame");
    check_response(Some("tools/call"), &response);
    assert_eq!(response.status, 200);
    let body = json_of(&response);
    assert_eq!(body["result"]["resultType"], "complete");
    assert_eq!(text_of(&body), "read:the value");
}

#[test]
fn a_tool_that_needs_two_effects_requests_the_second_after_the_first_outcome_pe_03() {
    let request = tool_call("effect_chain", r#"{"first":"x","second":"y"}"#);
    let (seen, response, body) = driven(&request);
    assert_eq!(
        seen.performed,
        vec![(1, b"x".to_vec()), (2, b"o1y".to_vec())],
        "the payload of the second effect holds the outcome of the first"
    );
    assert_eq!(seen.turns, 3);
    check_response(Some("tools/call"), &response);
    assert_eq!(text_of(&body), "chain:o1|o2");
}

#[test]
fn the_second_turn_of_a_chain_asks_for_the_second_effect_and_not_the_first_again_pe_03() {
    let frame = encode_request(&tool_call("effect_chain", r#"{"first":"x","second":"y"}"#));
    let step = decode_step(&exchange_step(&encode_turn(&frame, &[(1, b"ox".to_vec())])))
        .expect("a step");
    assert_eq!(
        step,
        Step::Perform {
            tag: 2,
            payload: b"oxy".to_vec()
        }
    );
}

#[test]
fn a_provider_call_a_read_and_a_prompt_are_replayed_to_their_final_responses_pe_03() {
    let (seen, response, body) = driven(&tool_call("provider_effect", "{}"));
    assert_eq!(seen.performed, vec![(3, b"call".to_vec())]);
    check_response(Some("tools/call"), &response);
    assert_eq!(text_of(&body), "call:o3");

    let (seen, response, body) = driven(&read_effect());
    assert_eq!(seen.performed, vec![(4, b"effect://item".to_vec())]);
    check_response(Some("resources/read"), &response);
    assert_eq!(body["result"]["contents"][0]["uri"], "effect://item");
    assert_eq!(body["result"]["contents"][0]["text"], "item:o4");

    let (seen, response, body) = driven(&prompt_effect());
    assert_eq!(seen.performed, vec![(5, b"effect_prompt".to_vec())]);
    check_response(Some("prompts/get"), &response);
    assert_eq!(body["result"]["messages"][0]["content"]["text"], "prompt:o5");
}

#[test]
fn a_replayed_turn_answers_the_same_bytes_pe_03() {
    let frame = encode_request(&tool_call("effect_chain", r#"{"first":"x","second":"y"}"#));
    let turn = encode_turn(&frame, &[(1, b"ox".to_vec()), (2, b"oy".to_vec())]);
    let once = exchange_step(&turn);
    for _ in 0..3 {
        assert_eq!(exchange_step(&turn), once);
    }
    let again = drive(&frame, BUDGET, |tag, _| format!("o{tag}").into_bytes()).expect("steps");
    let twice = drive(&frame, BUDGET, |tag, _| format!("o{tag}").into_bytes()).expect("steps");
    assert_eq!(again, twice);
}

#[test]
fn an_outcome_of_another_tag_is_an_internal_error_and_not_another_effect_pe_03() {
    let frame = encode_request(&tool_call("effect_read", r#"{"key":"alpha"}"#));
    let step = decode_step(&exchange_step(&encode_turn(&frame, &[(9, b"x".to_vec())])))
        .expect("a step");
    let response = done_response(step);
    assert_eq!(response.status, 500);
    assert_eq!(code_of(&json_of(&response)), Some(-32603));
}

// ---- PE-04: a request that fails a check performs no effect ----

/// What a refused request must be answered with: the status, and the JSON-RPC code when
/// the answer is an error.
struct Refused {
    what: &'static str,
    request: Request,
    status: u32,
    code: Option<i64>,
}

fn with_header(mut request: Request, name: &str, value: &str) -> Request {
    request
        .fields
        .push((name.as_bytes().to_vec(), value.as_bytes().to_vec()));
    request
}

fn without_header(mut request: Request, name: &str) -> Request {
    request
        .fields
        .retain(|(n, _)| !n.eq_ignore_ascii_case(name.as_bytes()));
    request
}

fn refused_requests() -> Vec<Refused> {
    let good = || tool_call("effect_read", r#"{"key":"alpha"}"#);
    let old = |text: &str| text.replace("2026-07-28", "2025-11-25");
    let mut old_version = good();
    old_version.body = old(&String::from_utf8(old_version.body).expect("text")).into_bytes();
    for (name, value) in &mut old_version.fields {
        if name.eq_ignore_ascii_case(b"MCP-Protocol-Version") {
            *value = b"2025-11-25".to_vec();
        }
    }
    let mut not_post = good();
    not_post.method = b"GET".to_vec();
    let mut not_json = good();
    not_json.body = b"this is not json".to_vec();
    let mut not_a_request = good();
    not_a_request.body = br#"{"jsonrpc":"2.0","id":1}"#.to_vec();
    let mut no_meta = good();
    no_meta.body = br#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"effect_read","arguments":{"key":"alpha"}}}"#.to_vec();
    let unknown = {
        let body = rpc("tools/frob", &format!(r#"{{"_meta":{META}}}"#), "1");
        post_owned(&standard("tools/frob", None), &body)
    };
    let not_offered = {
        // The fixture declares no tasks extension, so tasks/get is not a method it offers.
        let body = rpc(
            "tasks/get",
            &format!(r#"{{"taskId":"t","_meta":{META}}}"#),
            "1",
        );
        post_owned(&standard("tasks/get", Some("t")), &body)
    };
    let undecodable = {
        let body = rpc(
            "tools/call",
            &format!(r#"{{"name":"effect_read","arguments":"alpha","_meta":{META}}}"#),
            "1",
        );
        post_owned(&standard("tools/call", Some("effect_read")), &body)
    };
    vec![
        Refused {
            what: "a foreign Origin",
            request: with_header(good(), "Origin", "https://evil.example"),
            status: 403,
            code: Some(-32000),
        },
        Refused {
            what: "a foreign Host",
            request: with_header(good(), "Host", "evil.example"),
            status: 403,
            code: Some(-32000),
        },
        Refused {
            what: "a method other than POST",
            request: not_post,
            status: 405,
            code: None,
        },
        Refused {
            what: "a body that is not JSON",
            request: not_json,
            status: 400,
            code: Some(-32700),
        },
        Refused {
            what: "a body that is not a JSON-RPC request",
            request: not_a_request,
            status: 400,
            code: Some(-32600),
        },
        Refused {
            what: "an unsupported protocol version",
            request: old_version,
            status: 400,
            code: Some(-32022),
        },
        Refused {
            what: "a missing Mcp-Method header",
            request: without_header(good(), "Mcp-Method"),
            status: 400,
            code: Some(-32020),
        },
        Refused {
            what: "an Mcp-Name header that differs from the tool",
            request: with_header(without_header(good(), "Mcp-Name"), "Mcp-Name", "echo"),
            status: 400,
            code: Some(-32020),
        },
        Refused {
            what: "a missing _meta",
            request: no_meta,
            status: 400,
            code: Some(-32602),
        },
        Refused {
            what: "a method that is not known",
            request: unknown,
            status: 404,
            code: Some(-32601),
        },
        Refused {
            what: "a method the provider does not offer",
            request: not_offered,
            status: 404,
            code: Some(-32601),
        },
        Refused {
            what: "params that do not decode",
            request: undecodable,
            status: 400,
            code: Some(-32602),
        },
        Refused {
            what: "a tools/call past the rate limit",
            request: after_calls(good(), 100),
            status: 429,
            code: Some(429),
        },
    ]
}

#[test]
fn a_request_that_fails_a_check_of_the_core_performs_no_effect_pe_04() {
    let cases = refused_requests();
    assert!(cases.len() >= 13);
    for case in cases {
        // A host that panics: a single effect request fails the test.
        let frame = encode_request(&case.request);
        let seen = drive(&frame, BUDGET, never)
            .unwrap_or_else(|e| panic!("{}: {e:?}", case.what));
        assert_eq!(seen.turns, 1, "{}", case.what);
        assert!(seen.performed.is_empty(), "{}", case.what);
        let response = decode_response(&seen.response).expect("a response frame");
        assert_eq!(response.status, case.status, "{}", case.what);
        if case.code.is_some() {
            assert_eq!(code_of(&json_of(&response)), case.code, "{}", case.what);
        }
    }
}

#[test]
fn the_refusal_is_the_one_the_byte_serve_gives_for_the_same_frame_pe_04() {
    // The same refused frames, answered by the byte root of the fixture: the step serve
    // writes the response the byte serve writes, because the check that refused is the
    // same check. The tool is the fixture's `echo` so that the byte root can answer it.
    let good = || tool_call("echo", "{}");
    let frames = [
        encode_request(&with_header(good(), "Origin", "https://evil.example")),
        encode_request(&without_header(good(), "Mcp-Method")),
        encode_request(&after_calls(good(), 100)),
        {
            let mut not_json = good();
            not_json.body = b"this is not json".to_vec();
            encode_request(&not_json)
        },
    ];
    for frame in frames {
        let want = exchange_frame(&frame);
        let step = decode_step(&exchange_step(&encode_turn(&frame, &[]))).expect("a step");
        assert_eq!(step, Step::Done(want));
    }
}

#[test]
fn arguments_that_fail_the_input_schema_are_a_tool_error_and_perform_no_effect_pe_04() {
    for arguments in ["{}", r#"{"key":""}"#, r#"{"key":1}"#, r#"{"key":"a","extra":1}"#] {
        let frame = encode_request(&tool_call("effect_read", arguments));
        let seen = drive(&frame, BUDGET, never).expect("the root writes steps");
        assert!(seen.performed.is_empty(), "{arguments}");
        let response = decode_response(&seen.response).expect("a response frame");
        assert_eq!(response.status, 200, "{arguments}");
        let body = json_of(&response);
        assert_eq!(body["result"]["isError"], true, "{arguments}");
        assert!(
            text_of(&body).starts_with("Invalid arguments"),
            "{arguments}: {body}"
        );
    }
}

#[test]
fn an_unknown_tool_is_refused_and_performs_no_effect_pe_04() {
    let frame = encode_request(&tool_call("no_such_tool", "{}"));
    let seen = drive(&frame, BUDGET, never).expect("the root writes steps");
    assert!(seen.performed.is_empty());
    let response = decode_response(&seen.response).expect("a response frame");
    assert_eq!(response.status, 400);
    assert_eq!(code_of(&json_of(&response)), Some(-32602));
}

// ---- PE-05: the step serve agrees with the byte serve ----

#[test]
fn the_step_of_every_vector_is_done_with_the_response_of_the_byte_serve_pe_05() {
    let vectors = all_vectors();
    assert!(vectors.len() > 100, "{} vectors", vectors.len());
    for vector in &vectors {
        let frame = encode_request_allowing(&request_of(vector), &allowed_hosts_of(vector));
        let want = exchange_frame(&frame);
        let step = decode_step(&exchange_step(&encode_turn(&frame, &[]))).expect("a step");
        assert_eq!(
            step,
            Step::Done(want),
            "{} ({} line {})",
            vector["id"],
            vector["file"],
            vector["line"]
        );
    }
}

// ---- PE-06: the byte serve never performs ----

#[test]
fn the_byte_serve_answers_a_provider_that_requests_an_effect_with_the_internal_error_pe_06() {
    for request in [
        tool_call("effect_read", r#"{"key":"alpha"}"#),
        tool_call("provider_effect", "{}"),
        read_effect(),
        prompt_effect(),
    ] {
        let mut request = request;
        request.target = b"/bytes".to_vec();
        // The root answers the target /bytes with the byte serve: a response frame, and
        // not a step.
        let written = exchange_step(&encode_turn(&encode_request(&request), &[]));
        let response = decode_response(&written).expect("a response frame, not a step");
        assert_eq!(response.status, 500);
        let body = json_of(&response);
        assert_eq!(body["jsonrpc"], "2.0");
        assert_eq!(code_of(&body), Some(-32603));
        assert_eq!(body["error"]["message"], "Internal error");
        assert!(body["error"].get("data").is_none(), "{body}");
        assert!(body.get("result").is_none());
        let wire = String::from_utf8_lossy(&written).into_owned();
        assert!(!wire.contains("alpha"), "the request is disclosed: {wire}");
    }
}

#[test]
fn the_byte_serve_answers_a_request_without_an_effect_as_ever_pe_06() {
    let mut request = tool_call("echo", "{}");
    request.target = b"/bytes".to_vec();
    let written = exchange_step(&encode_turn(&encode_request(&request), &[]));
    let response = decode_response(&written).expect("a response frame");
    assert_eq!(response.status, 200);
    assert_eq!(text_of(&json_of(&response)), "ok");
}

// ---- PE-07: the tag is four bytes ----

#[test]
fn the_largest_tag_of_the_encoding_is_performed_pe_07() {
    assert_eq!(
        first_step(&tool_call("effect_edge_ok", "{}")),
        Step::Perform {
            tag: u32::MAX,
            payload: b"edge".to_vec()
        }
    );
}

#[test]
fn a_tag_that_does_not_fit_thirty_two_bits_is_done_with_the_internal_error_pe_07() {
    let step = first_step(&tool_call("effect_edge_over", "{}"));
    let response = done_response(step);
    assert_eq!(response.status, 500);
    assert_eq!(code_of(&json_of(&response)), Some(-32603));
    // Never the wrapped tag 0.
    let written = exchange_step(&encode_turn(
        &encode_request(&tool_call("effect_edge_over", "{}")),
        &[],
    ));
    assert_ne!(&written[..4], &[0, 0, 0, 1]);
}

// ---- PE-08: the laws are stated ----

#[test]
fn the_named_laws_of_the_step_are_declared_pe_08() {
    let laws = source("StepLaws");
    let mut seen = BTreeSet::new();
    for name in [
        "a_done_step_is_the_word_zero_and_the_response",
        "a_perform_step_is_the_word_one_the_tag_and_the_payload",
        "a_turn_with_no_completed_effect_is_read_back",
        "a_turn_with_two_completed_effects_is_read_back",
        "a_turn_cut_inside_the_request_is_a_fault",
        "a_turn_with_a_byte_after_its_last_outcome_is_a_fault",
        "a_call_that_performs_is_the_effect_refusal",
        "a_read_that_performs_is_the_effect_refusal",
        "a_prompt_that_performs_is_the_effect_refusal",
        "a_forbidden_request_never_reaches_the_provider",
        "a_request_that_is_not_a_post_never_reaches_the_provider",
        "a_body_that_is_not_json_never_reaches_the_provider",
        "a_message_that_is_not_a_request_never_reaches_the_provider",
        "a_refused_check_is_never_an_effect",
        "a_handled_request_that_ended_in_an_effect_is_a_perform",
        "a_handled_request_that_did_not_end_in_an_effect_is_done",
        "a_tag_that_does_not_fit_thirty_two_bits_is_not_a_perform",
        "an_effect_request_is_an_internal_error_with_no_data",
        "an_effect_request_is_a_server_error_status",
        "the_byte_serve_answers_an_effect_request_with_the_internal_error",
    ] {
        assert!(
            laws.contains(&format!("name: {name},")),
            "StepLaws declares no theorem {name}"
        );
        assert!(seen.insert(name), "{name} is listed twice");
    }
}

// ---- PE-09: the fixture requests no effect ----

#[test]
fn the_fixture_imports_neither_the_effect_module_nor_the_step_serve_pe_09() {
    let fixture = source("Fixture");
    for module in ["HostEffect", "StepServe", "StepFixture", "StepLaws"] {
        assert!(
            !fixture.contains(&format!("\\importmodule{{{module}}}")),
            "Fixture imports {module}"
        );
    }
    assert!(
        !fixture.contains("effectRequested") && !fixture.contains("EffectRequest"),
        "the fixture provider names an effect request"
    );
    assert!(fixture.contains("Serve::serve (fixtureInfo) (ref fixtureAnswer) input"));
}

#[test]
fn the_conformance_declaration_still_names_fixture_run_as_its_core_pe_09() {
    let declaration = std::fs::read_to_string(root().join("conformance/src/Application.lex.tex"))
        .expect("the declaration reads");
    assert!(declaration.contains("UorMcp.Fixture.fixtureRun"));
    assert!(!declaration.contains("StepFixture"));
    let identity: Value = serde_json::from_slice(
        &std::fs::read(root().join("conformance/core-identity.json")).expect("the identity"),
    )
    .expect("the identity is JSON");
    assert_eq!(identity["root"], "UorMcp.Fixture.fixtureRun");
}

#[test]
fn every_committed_vector_is_answered_done_by_the_step_serve_of_the_fixture_pe_09() {
    let mut kinds = BTreeSet::new();
    for vector in all_vectors() {
        let frame = encode_request_allowing(&request_of(&vector), &allowed_hosts_of(&vector));
        let step = decode_step(&exchange_step(&encode_turn(&frame, &[]))).expect("a step");
        assert!(
            matches!(step, Step::Done(_)),
            "{}: the fixture requested an effect",
            vector["id"]
        );
        kinds.insert(vector["file"].as_str().unwrap_or_default().to_owned());
    }
    assert!(!kinds.is_empty());
}

#[test]
fn the_driver_stops_a_root_that_asks_for_more_effects_than_its_budget_pe_03() {
    let frame = encode_request(&tool_call("effect_chain", r#"{"first":"x","second":"y"}"#));
    assert_eq!(
        drive(&frame, 1, |tag, _| format!("o{tag}").into_bytes()),
        Err(DriveError::Budget)
    );
}
