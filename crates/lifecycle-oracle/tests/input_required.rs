//! Input-required results (SEP-2322), run against the generated lifecycle root: the
//! InputRequiredResult of each kind of request, the retry with `inputResponses`, the
//! tamper-evident `requestState`, and every refusal the issue names.
//!
//! The requests the pinned scenarios send are the vectors of
//! `vectors/input-required.json`; a vector that depends on an earlier answer (the key it
//! asked, the state it handed out) names that vector. The tests here add the requests the
//! scenarios do not send. Each is a falsifying probe authored from the specification text
//! the register cites, never a verdict: the verdict is the suite run against the
//! conformance build (issue 13).

mod common;

use std::collections::{BTreeMap, BTreeSet};

use common::{header, json_of, keyed, post_owned, send, standard, suite_text, validate_against};
use lifecycle_oracle::{encode_request, Request, Response};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const SCENARIOS: &str = "src/scenarios/server/input-required-result.ts";
const ELICIT: &str = "test_input_required_result_elicitation";
const SAMPLING: &str = "test_input_required_result_sampling";
const ROOTS: &str = "test_input_required_result_list_roots";
const STATE: &str = "test_input_required_result_request_state";
const MULTI: &str = "test_input_required_result_multiple_inputs";
const ROUND: &str = "test_input_required_result_multi_round";
const TAMPER: &str = "test_input_required_result_tampered_state";
const CAPS: &str = "test_input_required_result_capabilities";

/// The key the tests give the host: 32 bytes, the floor of `RequestState.minKeyBytes`.
fn key() -> Vec<u8> {
    (0u8..32).collect()
}

// ---- an independent HMAC-SHA-256 (RFC 2104), to check the model against ----

fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut block = if key.len() > 64 {
        Sha256::digest(key).to_vec()
    } else {
        key.to_vec()
    };
    block.resize(64, 0);
    let mut inner = Sha256::new();
    inner.update(block.iter().map(|b| b ^ 0x36).collect::<Vec<u8>>());
    inner.update(message);
    let mut outer = Sha256::new();
    outer.update(block.iter().map(|b| b ^ 0x5c).collect::<Vec<u8>>());
    outer.update(inner.finalize());
    outer.finalize().to_vec()
}

fn unhex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2), "even hexadecimal: {text}");
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).expect("hexadecimal"))
        .collect()
}

fn is_lower_hex(text: &str) -> bool {
    text.bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// What a state is made of: the payload and the tag, both lower-case hexadecimal.
fn split_state(state: &str) -> (Vec<u8>, Vec<u8>) {
    let (payload, tag) = state.split_once('.').expect("a state is <hex>.<hex>");
    assert!(is_lower_hex(payload) && is_lower_hex(tag), "{state}");
    assert_eq!(tag.len(), 64, "the tag is 32 bytes: {state}");
    (unhex(payload), unhex(tag))
}

/// The tag `RequestState.tagOf` states: HMAC over the four-byte big-endian length and the
/// method, the same for the name, and the payload.
fn expected_tag(key: &[u8], method: &str, name: &str, payload: &[u8]) -> Vec<u8> {
    let mut message = Vec::new();
    for part in [method.as_bytes(), name.as_bytes()] {
        message.extend_from_slice(&u32::try_from(part.len()).expect("short").to_be_bytes());
        message.extend_from_slice(part);
    }
    message.extend_from_slice(payload);
    hmac_sha256(key, &message)
}

// ---- requests ----

fn default_meta() -> Value {
    json!({
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientInfo": {"name": "conformance-test-client", "version": "1.0.0"},
        "io.modelcontextprotocol/clientCapabilities": {
            "sampling": {}, "elicitation": {}, "roots": {"listChanged": true}
        }
    })
}

/// `withRequestMeta` of the pinned stateless connection: the defaults, and the keys the
/// params already carry in `_meta` win over them.
fn with_meta(params: &Value, meta: Option<&Value>) -> Value {
    let mut params = params.clone();
    let mut merged = default_meta();
    for source in [params.get("_meta"), meta].into_iter().flatten() {
        for (k, v) in source.as_object().expect("_meta is an object") {
            merged[k] = v.clone();
        }
    }
    params["_meta"] = merged;
    params
}

fn name_of(method: &str, params: &Value) -> Option<String> {
    match method {
        "tools/call" | "prompts/get" => params["name"].as_str().map(str::to_owned),
        _ => None,
    }
}

/// A POST the way `sendStatelessRequest` sends it, with `state_key` as the host's key.
fn stateless(
    method: &str,
    params: &Value,
    meta: Option<&Value>,
    id: u64,
    state_key: &[u8],
) -> Request {
    let mut headers = standard(method, name_of(method, params).as_deref());
    headers.push((
        "Accept".to_owned(),
        "application/json, text/event-stream".to_owned(),
    ));
    let body = json!({
        "jsonrpc": "2.0", "id": id, "method": method, "params": with_meta(params, meta)
    })
    .to_string();
    keyed(post_owned(&headers, &body), state_key)
}

fn tool_call(name: &str, extra: Value) -> Value {
    let mut params = json!({"name": name, "arguments": {}});
    for (k, v) in extra.as_object().expect("extra is an object") {
        params[k] = v.clone();
    }
    params
}

fn call(name: &str, extra: Value) -> (Response, Value) {
    send(&stateless(
        "tools/call",
        &tool_call(name, extra),
        None,
        900,
        &key(),
    ))
}

fn accepted(content: Value) -> Value {
    json!({"action": "accept", "content": content})
}

fn roots_result() -> Value {
    json!({"roots": [{"uri": "file:///test/root", "name": "Test Root"}]})
}

fn request_keys(body: &Value) -> Vec<String> {
    let mut keys: Vec<String> = body["result"]["inputRequests"]
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    keys.sort();
    keys
}

fn assert_refused(response: &Response, body: &Value, status: u32, code: i64, message: &str) {
    assert_eq!(response.status, status, "{body}");
    assert!(body.get("result").is_none(), "{body}");
    assert_eq!(body["error"]["code"], code, "{body}");
    assert_eq!(body["error"]["message"], message, "{body}");
}

const BAD_STATE: &str = "Invalid params: requestState failed verification";
const BAD_RESPONSES: &str = "Invalid params: inputResponses is not a valid InputResponses object";

// ---- the vectors ----

fn vector_file() -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("input-required.json");
    serde_json::from_slice(&std::fs::read(path).expect("the input-required vectors are committed"))
        .expect("the vectors are JSON")
}

fn vector_ids_in(value: &Value, own: &str, out: &mut BTreeSet<String>) {
    let text = value.to_string();
    let mut rest = text.as_str();
    while let Some(at) = rest.find("IV-") {
        let id = &rest[at..at + 5];
        if id != own {
            out.insert(id.to_owned());
        }
        rest = &rest[at + 5..];
    }
}

fn first_key(body: &Value) -> String {
    body["result"]["inputRequests"]
        .as_object()
        .and_then(|o| o.keys().next().cloned())
        .expect("the answer asked for something")
}

fn resolve(value: &Value, bodies: &BTreeMap<String, Value>) -> Value {
    match value {
        Value::String(text) => {
            if let Some(id) = text.strip_prefix("$state:") {
                return bodies[id]["result"]["requestState"].clone();
            }
            if let Some(id) = text.strip_prefix("$tampered:") {
                let state = bodies[id]["result"]["requestState"]
                    .as_str()
                    .expect("state");
                return Value::String(format!("{state}-TAMPERED"));
            }
            value.clone()
        }
        Value::Array(items) => Value::Array(items.iter().map(|v| resolve(v, bodies)).collect()),
        Value::Object(members) => {
            let mut out = serde_json::Map::new();
            for (k, v) in members {
                let key = k
                    .strip_prefix("$key:")
                    .map_or_else(|| k.clone(), |id| first_key(&bodies[id]));
                out.insert(key, resolve(v, bodies));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

fn pointer_check(
    vid: &str,
    c: &Value,
    body: &Value,
    bodies: &BTreeMap<String, Value>,
) -> Result<(), String> {
    let at = c["at"].as_str().ok_or("a check names its pointer")?;
    let found = body.pointer(at);
    if c.get("absent").is_some() {
        return match found {
            None => Ok(()),
            Some(v) => Err(format!("{vid}: {at} should be absent, found {v}")),
        };
    }
    if c.get("ne").is_some() {
        return match found {
            Some(v) if v == &c["ne"] => Err(format!("{vid}: {at} is {v}, which it must not be")),
            _ => Ok(()),
        };
    }
    let found = found.ok_or_else(|| format!("{vid}: {at} is absent from {body}"))?;
    if let Some(want) = c.get("eq") {
        return if found == want {
            Ok(())
        } else {
            Err(format!("{vid}: {at} is {found}, expected {want}"))
        };
    }
    if c.get("present").is_some() {
        return Ok(());
    }
    if let Some(part) = c.get("contains").and_then(Value::as_str) {
        return match found.as_str() {
            Some(text) if text.contains(part) => Ok(()),
            _ => Err(format!(
                "{vid}: {at} is {found}, expected text containing {part:?}"
            )),
        };
    }
    if c.get("nonempty_array").is_some() {
        return match found.as_array() {
            Some(items) if !items.is_empty() => Ok(()),
            _ => Err(format!(
                "{vid}: {at} is {found}, expected a non-empty array"
            )),
        };
    }
    if let Some(want) = c.get("keys").and_then(Value::as_array) {
        let mut want: Vec<String> = want
            .iter()
            .map(|k| k.as_str().expect("key").to_owned())
            .collect();
        want.sort();
        let mut have: Vec<String> = found
            .as_object()
            .ok_or_else(|| format!("{vid}: {at} is not an object"))?
            .keys()
            .cloned()
            .collect();
        have.sort();
        return if have == want {
            Ok(())
        } else {
            Err(format!("{vid}: {at} has keys {have:?}, expected {want:?}"))
        };
    }
    if c.get("state_format").is_some() {
        let text = found
            .as_str()
            .ok_or_else(|| format!("{vid}: {at} is not text"))?;
        let ok = text.split_once('.').is_some_and(|(p, t)| {
            is_lower_hex(p) && !p.is_empty() && is_lower_hex(t) && t.len() == 64
        });
        return if ok {
            Ok(())
        } else {
            Err(format!(
                "{vid}: {at} is {text:?}, expected <hex payload>.<hex tag>"
            ))
        };
    }
    if let Some(pair) = c.get("ne_vector").and_then(Value::as_array) {
        let other = bodies[pair[0].as_str().expect("vector")]
            .pointer(pair[1].as_str().expect("pointer"))
            .ok_or("the other vector has the member")?;
        return if other == found {
            Err(format!("{vid}: {at} equals the one of {}", pair[0]))
        } else {
            Ok(())
        };
    }
    Err(format!("{vid}: unknown check {c}"))
}

/// Run the vectors tagged `id`, and the earlier vectors they read, in file order. A vector
/// marked `pending` is not run. Returns how many tagged vectors ran.
fn run_register(id: &str) -> usize {
    let file = vector_file();
    let state_key = unhex(file["state_key_hex"].as_str().expect("the key"));
    let vectors = file["vectors"].as_array().expect("vectors");
    let mut needed: BTreeSet<String> = BTreeSet::new();
    for v in vectors {
        if v["register"]
            .as_array()
            .expect("register")
            .iter()
            .any(|r| r == id)
        {
            needed.insert(v["id"].as_str().expect("id").to_owned());
        }
    }
    loop {
        let before = needed.len();
        for v in vectors {
            let vid = v["id"].as_str().expect("id");
            if needed.contains(vid) {
                let mut more = BTreeSet::new();
                vector_ids_in(&v["request"]["params"], vid, &mut more);
                needed.extend(more);
            }
        }
        if needed.len() == before {
            break;
        }
    }
    let mut bodies: BTreeMap<String, Value> = BTreeMap::new();
    let mut ran = 0;
    for v in vectors {
        let vid = v["id"].as_str().expect("id");
        if !needed.contains(vid) || v.get("pending").is_some() {
            continue;
        }
        let r = &v["request"];
        let params = resolve(&r["params"], &bodies);
        let request = stateless(
            r["method"].as_str().expect("method"),
            &params,
            r.get("meta"),
            r["id"].as_u64().expect("id"),
            &state_key,
        );
        let (response, body) = send(&request);
        assert_eq!(
            u64::from(response.status),
            v["expect"]["status"].as_u64().expect("status"),
            "{vid} ({} line {}): HTTP status; body {body}",
            v["file"],
            v["line"]
        );
        assert_eq!(
            header(&response, "content-type"),
            Some(&b"application/json"[..]),
            "{vid}: every body is JSON"
        );
        for c in v["expect"]["checks"].as_array().expect("checks") {
            if let Err(why) = pointer_check(vid, c, &body, &bodies) {
                panic!("{why} (scenario {}, line {})", v["scenario"], v["line"]);
            }
        }
        bodies.insert(vid.to_owned(), body);
        if v["register"]
            .as_array()
            .expect("register")
            .iter()
            .any(|x| x == id)
        {
            ran += 1;
        }
    }
    ran
}

// ---- the pinned suite: what is vendored, and that nothing it sends is left out ----

#[test]
fn the_vendored_scenarios_are_the_pinned_files_and_every_request_they_send_is_a_vector_ir_01() {
    let text = suite_text(SCENARIOS);
    let file = vector_file();
    assert_eq!(file["suite"]["commit"], common::PIN);
    let mut accounted: BTreeSet<u64> = BTreeSet::new();
    let mut check_ids: BTreeSet<String> = BTreeSet::new();
    for v in file["vectors"].as_array().expect("vectors") {
        assert_eq!(v["file"], SCENARIOS, "{}", v["id"]);
        let line = v["line"].as_u64().expect("line");
        let quote = v["quote"].as_str().expect("quote");
        let source = text
            .lines()
            .nth(usize::try_from(line).expect("line") - 1)
            .expect("the line exists");
        assert!(
            source.contains(quote),
            "{}: line {line} of {SCENARIOS} does not contain {quote:?}",
            v["id"]
        );
        accounted.insert(line);
        for c in v["suite_checks"].as_array().expect("suite_checks") {
            check_ids.insert(c.as_str().expect("id").to_owned());
        }
        assert!(!v["register"].as_array().expect("register").is_empty());
    }
    let sites: BTreeSet<u64> = text
        .lines()
        .enumerate()
        .filter(|(_, l)| l.contains("sendRpc(") && !l.contains("const sendRpc"))
        .map(|(i, _)| i as u64 + 1)
        .collect();
    assert_eq!(
        sites, accounted,
        "the request sites of {SCENARIOS} and the vectors' lines differ"
    );
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    for token in text.split('\'') {
        if token.starts_with("sep-2322-")
            && token
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            emitted.insert(token.to_owned());
        }
    }
    assert_eq!(
        emitted, check_ids,
        "the check ids the scenarios emit and the vectors cover differ"
    );
    // The helpers and the negative tests are vendored with it, and the reference fixture
    // already was.
    for file in [
        "src/scenarios/server/input-required-result-helpers.ts",
        "src/scenarios/server/negative-mrtr.test.ts",
        "examples/servers/typescript/everything-server.ts",
    ] {
        assert!(!suite_text(file).is_empty(), "{file}");
    }
    assert!(suite_text("src/scenarios/server/negative-mrtr.test.ts")
        .contains("sep-2322-reject-tampered-state"));
}

#[test]
fn the_pinned_requirements_list_the_fourteen_scenarios_as_required_ir_01() {
    let yaml = suite_text("requirements/2026-07-28.yaml");
    let required: Vec<&str> = yaml
        .lines()
        .skip_while(|l| *l != "server:")
        .skip(1)
        .take_while(|l| l.starts_with("  - "))
        .map(|l| l.trim_start_matches("  - "))
        .collect();
    let mut covered: BTreeSet<String> = BTreeSet::new();
    for v in vector_file()["vectors"].as_array().expect("vectors") {
        covered.insert(v["scenario"].as_str().expect("scenario").to_owned());
    }
    let wanted: Vec<&&str> = required
        .iter()
        .filter(|s| s.starts_with("input-required-result-"))
        .collect();
    assert_eq!(wanted.len(), 14, "{wanted:?}");
    for scenario in wanted {
        assert!(covered.contains(*scenario), "{scenario} has no vector");
    }
}

// ---- IR-01 to IR-08: the scenarios ----

#[test]
fn a_tool_that_needs_a_name_asks_for_it_by_elicitation_and_completes_on_the_retry_ir_01() {
    assert_eq!(run_register("IR-01"), 2);
    let (response, body) = call(ELICIT, json!({}));
    assert_eq!(response.status, 200);
    let ask = &body["result"]["inputRequests"]["user_name"];
    assert_eq!(ask["params"]["requestedSchema"]["type"], "object");
    assert_eq!(
        ask["params"]["requestedSchema"]["properties"]["name"]["type"],
        "string"
    );
    assert!(
        ask["params"].get("mode").is_none(),
        "the scenario's request has no mode"
    );
    // the retry's answer is the name the client gave
    let (_, done) = call(
        ELICIT,
        json!({"inputResponses": {"user_name": accepted(json!({"name": "Bob"}))}}),
    );
    assert_eq!(done["result"]["content"][0]["text"], "Hello, Bob!");
    // an answer that declines still completes: the user chose not to say
    let declined = json!({"user_name": {"action": "decline"}});
    let (response, done) = call(ELICIT, json!({"inputResponses": declined}));
    assert_eq!(response.status, 200);
    assert_eq!(done["result"]["content"][0]["text"], "Hello, unknown!");
}

#[test]
fn a_tool_that_needs_a_sample_asks_by_sampling_create_message_ir_02() {
    assert_eq!(run_register("IR-02"), 2);
    let (_, body) = call(SAMPLING, json!({}));
    let ask = &body["result"]["inputRequests"]["capital_question"]["params"];
    assert_eq!(ask["messages"].as_array().map(Vec::len), Some(1));
    assert!(ask.get("modelPreferences").is_none() && ask.get("systemPrompt").is_none());
}

#[test]
fn a_tool_that_needs_the_roots_asks_by_roots_list_and_counts_them_ir_03() {
    assert_eq!(run_register("IR-03"), 2);
    let roots =
        json!({"roots": [{"uri": "file:///a"}, {"uri": "file:///b"}, {"uri": "file:///c"}]});
    let (_, done) = call(ROOTS, json!({"inputResponses": {"client_roots": roots}}));
    assert_eq!(done["result"]["content"][0]["text"], "Found 3 root(s)");
    let (_, none) = call(
        ROOTS,
        json!({"inputResponses": {"client_roots": {"roots": []}}}),
    );
    assert_eq!(none["result"]["content"][0]["text"], "Found 0 root(s)");
}

#[test]
fn the_request_state_is_protected_echoed_and_needed_for_the_retry_to_complete_ir_04() {
    assert_eq!(run_register("IR-04"), 2);
    let (_, first) = call(STATE, json!({}));
    let state = first["result"]["requestState"]
        .as_str()
        .expect("a state")
        .to_owned();
    // the state is the payload and its tag, and the payload is not readable as the plain word
    assert_ne!(state, "request-state");
    let (payload, tag) = split_state(&state);
    assert_eq!(payload, b"request-state");
    assert_eq!(tag, expected_tag(&key(), "tools/call", STATE, &payload));
    let answer = json!({"confirm": accepted(json!({"ok": true}))});
    // without the state the tool asks again; the answer alone does not complete it
    let (response, again) = call(STATE, json!({"inputResponses": answer.clone()}));
    assert_eq!(response.status, 200);
    assert_eq!(again["result"]["resultType"], "input_required");
    assert!(again["result"]["requestState"].is_string());
    // with the state but an answer of ok false, the tool asks again too
    let no = json!({"confirm": accepted(json!({"ok": false}))});
    let (_, again) = call(
        STATE,
        json!({"inputResponses": no, "requestState": state.clone()}),
    );
    assert_eq!(again["result"]["resultType"], "input_required");
    // with both, it completes
    let (_, done) = call(
        STATE,
        json!({"inputResponses": answer, "requestState": state}),
    );
    assert_eq!(
        done["result"]["content"][0]["text"],
        "state-ok: requestState validated"
    );
}

#[test]
fn a_result_can_carry_requests_of_three_kinds_and_a_state_and_completes_when_all_are_answered_ir_05(
) {
    assert_eq!(run_register("IR-05"), 2);
    let (_, first) = call(MULTI, json!({}));
    // the requests travel in the order the tool listed them
    let raw = String::from_utf8_lossy(
        &common::exchange_checked(&stateless(
            "tools/call",
            &tool_call(MULTI, json!({})),
            None,
            901,
            &key(),
        ))
        .expect("a response")
        .body,
    )
    .into_owned();
    let order: Vec<usize> = ["\"user_name\"", "\"greeting\"", "\"client_roots\""]
        .iter()
        .map(|k| raw.find(k).unwrap_or_else(|| panic!("{k} is in {raw}")))
        .collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{raw}");
    assert_eq!(request_keys(&first).len(), 3);
    // the state is bound to this tool: another tool's retry cannot use it
    let state = first["result"]["requestState"].clone();
    let (response, body) = call(ROUND, json!({"requestState": state}));
    assert_refused(&response, &body, 400, -32602, BAD_STATE);
}

#[test]
fn the_state_changes_from_round_to_round_and_the_last_round_completes_ir_06() {
    assert_eq!(run_register("IR-06"), 3);
    let (_, one) = call(ROUND, json!({}));
    let s1 = one["result"]["requestState"]
        .as_str()
        .expect("state")
        .to_owned();
    assert_eq!(split_state(&s1).0, b"1");
    let r1 = json!({"step1": accepted(json!({"name": "Zoë"}))});
    let (_, two) = call(ROUND, json!({"inputResponses": r1, "requestState": s1}));
    let s2 = two["result"]["requestState"]
        .as_str()
        .expect("state")
        .to_owned();
    // the second state carries the first answer, inside the integrity protection
    assert_eq!(split_state(&s2).0, "2Zoë".as_bytes());
    let r2 = json!({"step2": accepted(json!({"color": "green"}))});
    let (_, done) = call(
        ROUND,
        json!({"inputResponses": r2.clone(), "requestState": s2.clone()}),
    );
    assert_eq!(
        done["result"]["content"][0]["text"],
        "Multi-round complete for Zoë who likes green"
    );
    // the first round's state cannot answer the second round's question twice: asked again
    let (_, replay) = call(
        ROUND,
        json!({"inputResponses": r2, "requestState": one["result"]["requestState"].clone()}),
    );
    assert_eq!(replay["result"]["resultType"], "input_required");
    assert_eq!(request_keys(&replay), ["step1"]);
}

#[test]
fn a_missing_response_is_asked_for_again_and_only_for_what_is_missing_ir_07() {
    assert_eq!(run_register("IR-07"), 1);
    // no inputResponses member at all is the first round, not an error
    let (_, first) = call(ELICIT, json!({}));
    assert_eq!(first["result"]["resultType"], "input_required");
    // an empty map is a retry with nothing: asked again
    let (response, again) = call(ELICIT, json!({"inputResponses": {}}));
    assert_eq!(response.status, 200);
    assert_eq!(request_keys(&again), ["user_name"]);
    // the multiple-inputs tool is asked again only for what was not answered
    let (_, multi) = call(MULTI, json!({}));
    let state = multi["result"]["requestState"].clone();
    let partial = json!({"user_name": accepted(json!({"name": "Alice"}))});
    let (response, body) = call(
        MULTI,
        json!({"inputResponses": partial, "requestState": state.clone()}),
    );
    assert_eq!(response.status, 200);
    assert_eq!(request_keys(&body), ["client_roots", "greeting"]);
    assert_eq!(
        body["result"]["requestState"], state,
        "the same state travels on"
    );
    let more =
        json!({"user_name": accepted(json!({"name": "Alice"})), "client_roots": roots_result()});
    let (_, body) = call(
        MULTI,
        json!({"inputResponses": more, "requestState": state}),
    );
    assert_eq!(request_keys(&body), ["greeting"]);
}

#[test]
fn a_prompt_can_ask_for_input_too_because_the_result_is_not_specific_to_tools_ir_08() {
    assert_eq!(run_register("IR-08"), 2);
    let prompt = "test_input_required_result_prompt";
    let ask = |extra: Value| -> (Response, Value) {
        let mut params = json!({"name": prompt});
        for (k, v) in extra.as_object().expect("an object") {
            params[k] = v.clone();
        }
        send(&stateless("prompts/get", &params, None, 920, &key()))
    };
    let (response, first) = ask(json!({}));
    assert_eq!(response.status, 200, "{first}");
    assert_eq!(first["result"]["resultType"], "input_required");
    assert!(first["result"].get("requestState").is_none());
    validate_against("InputRequiredResult", &first["result"]).expect("a conforming result");
    let given = json!({"user_context": accepted(json!({"context": "in a test"}))});
    let (_, done) = ask(json!({"inputResponses": given}));
    assert_eq!(done["result"]["resultType"], "complete");
    assert_eq!(
        done["result"]["messages"][0]["content"]["text"],
        "Prompt with context: in a test"
    );
    // the guard covers a prompt as it covers a tool: a state that does not verify is refused,
    // and so is an entry that is not an object
    let (response, body) = ask(json!({"requestState": "garbage"}));
    assert_refused(&response, &body, 400, -32602, BAD_STATE);
    let (response, body) = ask(json!({"inputResponses": {"user_context": 7}}));
    assert_refused(&response, &body, 400, -32602, BAD_RESPONSES);
    let (response, body) = ask(json!({"inputResponses": {"user_context": {"action": "maybe"}}}));
    assert_refused(&response, &body, 400, -32602, BAD_RESPONSES);
}

// ---- IR-09 and IR-10: the result type, and where it may appear ----

#[test]
fn every_input_required_answer_names_its_result_type_and_the_server_and_conforms_ir_09() {
    assert_eq!(run_register("IR-09"), 1);
    // each tool's first answer, checked against the specification schema by `send`
    for tool in [ELICIT, SAMPLING, ROOTS, STATE, MULTI, ROUND, TAMPER, CAPS] {
        let (response, body) = call(tool, json!({}));
        assert_eq!(response.status, 200, "{tool}: {body}");
        let result = &body["result"];
        assert_eq!(result["resultType"], "input_required", "{tool}");
        assert_eq!(
            result["_meta"]["io.modelcontextprotocol/serverInfo"]["name"], "mcp-core-fixture",
            "{tool}"
        );
        assert!(
            result.get("inputRequests").is_some() || result.get("requestState").is_some(),
            "{tool}: at least one of the two"
        );
        validate_against("InputRequiredResult", result).unwrap_or_else(|e| panic!("{tool}: {e}"));
    }
    // a complete result says so
    let (_, done) = call(
        ELICIT,
        json!({"inputResponses": {"user_name": accepted(json!({"name": "A"}))}}),
    );
    assert_eq!(done["result"]["resultType"], "complete");
}

#[test]
fn input_required_is_never_the_answer_to_a_method_that_does_not_support_it_ir_10() {
    assert_eq!(run_register("IR-10"), 2);
    for method in [
        "tools/list",
        "server/discover",
        "resources/list",
        "prompts/list",
    ] {
        let request = stateless(method, &json!({}), None, 903, &key());
        let (_, body) = send(&request);
        assert_ne!(body["result"]["resultType"], "input_required", "{method}");
    }
    let source =
        std::fs::read_to_string(common::root().join("src/InputGuard.lex.tex")).expect("source");
    for theorem in [
        "input_required_is_never_sent_on_a_method_that_does_not_support_it",
        "tools_list_never_answers_input_required",
    ] {
        assert!(source.contains(&format!("name: {theorem},")), "{theorem}");
    }
}

// ---- IR-11: the state is tamper-evident ----

fn first_state(tool: &str) -> String {
    let (_, body) = call(tool, json!({}));
    body["result"]["requestState"]
        .as_str()
        .expect("a state")
        .to_owned()
}

fn retry_with(tool: &str, state: Value) -> (Response, Value) {
    let answer = json!({"confirm": accepted(json!({"ok": true}))});
    call(
        tool,
        json!({"inputResponses": answer, "requestState": state}),
    )
}

#[test]
fn a_state_that_was_altered_is_refused_with_invalid_params_and_the_tool_does_not_run_ir_11() {
    assert_eq!(run_register("IR-11"), 2);
    let good = first_state(TAMPER);
    let (_, tag_hex) = good.split_once('.').expect("a state");
    let flip = |c: char| if c == '0' { '1' } else { '0' };
    let mut altered = vec![];
    // the last tag nibble, the first payload nibble
    let mut t = good.clone();
    let last = t.pop().expect("a character");
    altered.push(format!("{t}{}", flip(last)));
    let mut chars: Vec<char> = good.chars().collect();
    chars[0] = flip(chars[0]);
    altered.push(chars.into_iter().collect());
    // text appended (the scenario's way), text prepended, the tag cut short, the tag dropped
    altered.push(format!("{good}-TAMPERED"));
    altered.push(format!("x{good}"));
    altered.push(good[..good.len() - 2].to_owned());
    altered.push(good.replace('.', ""));
    altered.push(format!("{}.", good.split_once('.').expect("a state").0));
    altered.push(String::new());
    altered.push(".".to_owned());
    altered.push(good.to_uppercase());
    altered.push(format!("{good}.{tag_hex}"));
    for state in altered {
        let (response, body) = retry_with(TAMPER, Value::String(state.clone()));
        assert_refused(&response, &body, 400, -32602, BAD_STATE);
        assert!(body["error"].get("data").is_none(), "{state}: {body}");
    }
    // the good state works, so the refusals above are about the state
    let (response, body) = retry_with(TAMPER, Value::String(good));
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(
        body["result"]["content"][0]["text"],
        "integrity-ok: state verified"
    );
}

#[test]
fn a_state_sealed_for_another_tool_or_under_another_key_is_refused_ir_11() {
    let good = first_state(TAMPER);
    // the same payload, sealed for the confirm tool: its tool name differs
    let (response, body) = retry_with(STATE, Value::String(good.clone()));
    assert_refused(&response, &body, 400, -32602, BAD_STATE);
    // the host rotated the key between the two requests
    let rotated: Vec<u8> = (1u8..33).collect();
    let answer = json!({"confirm": accepted(json!({"ok": true}))});
    let request = stateless(
        "tools/call",
        &tool_call(
            TAMPER,
            json!({"inputResponses": answer, "requestState": good}),
        ),
        None,
        904,
        &rotated,
    );
    let (response, body) = send(&request);
    assert_refused(&response, &body, 400, -32602, BAD_STATE);
}

#[test]
fn a_state_is_checked_whether_or_not_the_tool_reads_it_and_a_state_that_is_no_text_is_invalid_params_ir_11(
) {
    // the elicitation tool keeps no state, and a presented state is still verified
    let answer = json!({"user_name": accepted(json!({"name": "Alice"}))});
    let (response, body) = call(
        ELICIT,
        json!({"inputResponses": answer, "requestState": "garbage"}),
    );
    assert_refused(&response, &body, 400, -32602, BAD_STATE);
    // a state that is not text is not a state at all: invalid params, from the decoder
    let (response, body) = call(STATE, json!({"requestState": 12345}));
    assert_eq!(response.status, 400, "{body}");
    assert_eq!(body["error"]["code"], -32602);
    let (response, body) = call(STATE, json!({"requestState": null}));
    assert_eq!(response.status, 400, "{body}");
    assert_eq!(body["error"]["code"], -32602);
}

#[test]
fn the_guard_is_a_kernel_checked_gate_and_states_that_a_refused_request_never_runs_ir_11() {
    let source =
        std::fs::read_to_string(common::root().join("src/InputGuard.lex.tex")).expect("source");
    for theorem in [
        "a_state_that_fails_verification_is_refused",
        "a_refused_request_never_reaches_the_tool",
        "a_request_that_is_admitted_is_gated_answered_and_then_settled",
    ] {
        assert!(source.contains(&format!("name: {theorem},")), "{theorem}");
    }
    let state: String = ["RequestState", "SealLaws", "ScopeLaws"]
        .iter()
        .map(|m| {
            std::fs::read_to_string(common::root().join(format!("src/{m}.lex.tex")))
                .expect("source")
        })
        .collect();
    for theorem in [
        "a_state_with_a_changed_tag_is_refused",
        "a_state_with_a_changed_payload_is_refused",
        "a_state_sealed_for_another_name_is_refused",
        "a_state_sealed_for_another_method_is_refused",
        "a_state_sealed_under_another_key_is_refused",
        "a_state_without_a_dot_is_refused",
        "a_state_with_a_short_tag_is_refused",
        "an_empty_state_is_refused",
        "a_short_key_does_not_seal",
        "a_short_key_does_not_open",
    ] {
        assert!(state.contains(&format!("name: {theorem},")), "{theorem}");
    }
}

// ---- IR-12: capabilities ----

#[test]
fn a_tool_asks_only_for_the_capabilities_the_client_declared_ir_12() {
    assert_eq!(run_register("IR-12"), 1);
    let asked = |caps: Value| -> (u32, Value) {
        let meta = json!({"io.modelcontextprotocol/clientCapabilities": caps});
        let request = stateless(
            "tools/call",
            &tool_call(CAPS, json!({})),
            Some(&meta),
            905,
            &key(),
        );
        let (response, body) = send(&request);
        (response.status, body)
    };
    let (status, body) = asked(json!({"elicitation": {}}));
    assert_eq!(status, 200);
    assert_eq!(request_keys(&body), ["elicit_input"]);
    let (_, body) = asked(json!({"elicitation": {}, "sampling": {}}));
    assert_eq!(request_keys(&body), ["elicit_input", "sample_input"]);
    // a client that declares neither is asked nothing, and the tool finishes
    let (status, body) = asked(json!({}));
    assert_eq!(status, 200);
    assert_eq!(body["result"]["resultType"], "complete");
    assert_eq!(
        body["result"]["content"][0]["text"],
        "No supported capabilities declared"
    );
    // declaring roots is not declaring sampling
    let (_, body) = asked(json!({"roots": {}}));
    assert_eq!(body["result"]["resultType"], "complete");
}

#[test]
fn a_tool_that_asks_for_what_the_client_did_not_declare_is_refused_with_the_capability_error_ir_12()
{
    let none = json!({"io.modelcontextprotocol/clientCapabilities": {}});
    let request = stateless(
        "tools/call",
        &tool_call(MULTI, json!({})),
        Some(&none),
        906,
        &key(),
    );
    let (response, body) = send(&request);
    assert_eq!(response.status, 400, "{body}");
    assert_eq!(body["error"]["code"], -32021);
    assert_eq!(
        body["error"]["data"]["requiredCapabilities"],
        json!({"roots": {}, "sampling": {}, "elicitation": {}})
    );
    // one capability short names that one
    let two = json!({"io.modelcontextprotocol/clientCapabilities": {"sampling": {}, "roots": {}}});
    let request = stateless(
        "tools/call",
        &tool_call(MULTI, json!({})),
        Some(&two),
        907,
        &key(),
    );
    let (response, body) = send(&request);
    assert_eq!(response.status, 400, "{body}");
    assert_eq!(body["error"]["code"], -32021);
    // the error carries the state of nothing: the result was not sent
    assert!(body.get("result").is_none());
}

// ---- IR-13 and IR-14: what is ignored, what is validated ----

#[test]
fn entries_the_server_did_not_ask_for_are_ignored_when_they_are_objects_ir_13() {
    assert_eq!(run_register("IR-13"), 1);
    let answers = json!({
        "user_name": accepted(json!({"name": "Alice"})),
        "x": {}, "y": {"anything": [1, 2, 3]}, "z": {"action": "nonsense"}
    });
    let (response, body) = call(ELICIT, json!({"inputResponses": answers}));
    assert_eq!(response.status, 200);
    assert_eq!(body["result"]["content"][0]["text"], "Hello, Alice!");
}

#[test]
fn input_responses_that_do_not_hold_are_refused_with_invalid_params_ir_14() {
    assert_eq!(run_register("IR-14"), 2);
    let cases: Vec<(&str, Value, Value, &str)> = vec![
        ("not an object: a list", json!([]), json!(null), "decoder"),
        ("not an object: text", json!("x"), json!(null), "decoder"),
        ("not an object: a number", json!(1), json!(null), "decoder"),
        (
            "not an object: a boolean",
            json!(true),
            json!(null),
            "decoder",
        ),
        (
            "an entry that is null",
            json!({"user_name": null}),
            json!(null),
            "responses",
        ),
        (
            "an entry that is a list",
            json!({"user_name": []}),
            json!(null),
            "responses",
        ),
        (
            "an entry that is text",
            json!({"x": "y"}),
            json!(null),
            "responses",
        ),
    ];
    for (what, value, _, from) in cases {
        let (response, body) = call(ELICIT, json!({"inputResponses": value}));
        assert_eq!(response.status, 400, "{what}: {body}");
        assert_eq!(body["error"]["code"], -32602, "{what}");
        if from == "responses" {
            assert_eq!(body["error"]["message"], BAD_RESPONSES, "{what}");
        }
    }
    // an entry for an expected key that does not fit its result type
    let misfits = vec![
        (ELICIT, json!({"user_name": {"name": "Alice"}})),
        (ELICIT, json!({"user_name": {"action": "maybe"}})),
        (
            ELICIT,
            json!({"user_name": {"action": "accept", "content": "x"}}),
        ),
        (
            SAMPLING,
            json!({"capital_question": {"role": "assistant", "content": {"type": "text", "text": "x"}}}),
        ),
        (
            SAMPLING,
            json!({"capital_question": {"role": "robot", "content": {"type": "text", "text": "x"}, "model": "m"}}),
        ),
        (
            SAMPLING,
            json!({"capital_question": {"role": "user", "model": "m"}}),
        ),
        (ROOTS, json!({"client_roots": {"roots": "none"}})),
        (ROOTS, json!({"client_roots": {}})),
        (
            ROOTS,
            json!({"client_roots": {"roots": [{"name": "no uri"}]}}),
        ),
        (ROOTS, json!({"client_roots": {"roots": ["file:///a"]}})),
    ];
    for (tool, value) in misfits {
        let (response, body) = call(tool, json!({"inputResponses": value}));
        assert_refused(&response, &body, 400, -32602, BAD_RESPONSES);
    }
    // shapes that do fit are served
    for (tool, value, text) in [
        (
            ELICIT,
            json!({"user_name": {"action": "cancel"}}),
            "Hello, unknown!",
        ),
        (
            SAMPLING,
            json!({"capital_question": {"role": "user", "content": [], "model": "m"}}),
            "Sampling result: no response",
        ),
    ] {
        let (response, body) = call(tool, json!({"inputResponses": value}));
        assert_eq!(response.status, 200, "{body}");
        assert_eq!(body["result"]["content"][0]["text"], text);
    }
}

// ---- IR-15: the HMAC ----

#[test]
fn the_independent_hmac_reproduces_the_rfc_4231_cases_and_the_model_agrees_with_it_ir_15() {
    let cases: [(Vec<u8>, Vec<u8>, &str); 6] = [
        (vec![0x0b; 20], b"Hi There".to_vec(), "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
        (b"Jefe".to_vec(), b"what do ya want for nothing?".to_vec(), "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
        (vec![0xaa; 20], vec![0xdd; 50], "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe"),
        ((1u8..=25).collect(), vec![0xcd; 50], "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b"),
        (vec![0xaa; 131], b"Test Using Larger Than Block-Size Key - Hash Key First".to_vec(), "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"),
        (vec![0xaa; 131], b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.".to_vec(), "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2"),
    ];
    for (key, data, want) in &cases {
        assert_eq!(common::hex(&hmac_sha256(key, data)), *want);
    }
    // the model's theorems state the same six digests
    let source = std::fs::read_to_string(common::root().join("src/Hmac.lex.tex")).expect("source");
    for n in [1, 2, 3, 4, 6, 7] {
        assert!(
            source.contains(&format!("name: rfc4231_case_{n},")),
            "case {n}"
        );
    }
    // and the tag in a state the root writes is the tag the independent HMAC computes, for
    // each tool that keeps a state
    for tool in [STATE, MULTI, ROUND, TAMPER, CAPS] {
        let state = first_state(tool);
        let (payload, tag) = split_state(&state);
        assert_eq!(
            tag,
            expected_tag(&key(), "tools/call", tool, &payload),
            "{tool}"
        );
        // another key, tool or method gives another tag
        assert_ne!(tag, expected_tag(&key(), "tools/call", "other", &payload));
        assert_ne!(tag, expected_tag(&key(), "resources/read", tool, &payload));
        assert_ne!(
            tag,
            expected_tag(
                &(1u8..33).collect::<Vec<u8>>(),
                "tools/call",
                tool,
                &payload
            )
        );
    }
}

#[test]
fn a_key_longer_than_the_hash_block_works_like_any_other_ir_15() {
    let long: Vec<u8> = (0..200u8).collect();
    let request = stateless(
        "tools/call",
        &tool_call(TAMPER, json!({})),
        None,
        908,
        &long,
    );
    let (_, body) = send(&request);
    let state = body["result"]["requestState"]
        .as_str()
        .expect("a state")
        .to_owned();
    let (payload, tag) = split_state(&state);
    assert_eq!(tag, expected_tag(&long, "tools/call", TAMPER, &payload));
    let answer = json!({"confirm": accepted(json!({"ok": true}))});
    let retry = stateless(
        "tools/call",
        &tool_call(
            TAMPER,
            json!({"inputResponses": answer, "requestState": state}),
        ),
        None,
        909,
        &long,
    );
    let (response, body) = send(&retry);
    assert_eq!(response.status, 200, "{body}");
}

// ---- IR-16: the key ----

#[test]
fn a_result_that_needs_a_state_is_not_sent_without_a_key_of_at_least_32_bytes_ir_16() {
    for key in [vec![], vec![7u8; 1], vec![7u8; 31]] {
        for tool in [STATE, MULTI, ROUND, TAMPER, CAPS] {
            let request = stateless("tools/call", &tool_call(tool, json!({})), None, 910, &key);
            let (response, body) = send(&request);
            assert_refused(&response, &body, 500, -32603, "Internal error");
            assert!(body["error"].get("data").is_none(), "{tool}");
        }
        // a tool that keeps no state needs no key
        for tool in [ELICIT, SAMPLING, ROOTS] {
            let request = stateless("tools/call", &tool_call(tool, json!({})), None, 911, &key);
            let (response, body) = send(&request);
            assert_eq!(response.status, 200, "{tool}: {body}");
            assert_eq!(body["result"]["resultType"], "input_required");
            assert!(body["result"].get("requestState").is_none());
        }
        // a state presented with no key is refused: nothing can be verified
        let good = first_state(TAMPER);
        let answer = json!({"confirm": accepted(json!({"ok": true}))});
        let request = stateless(
            "tools/call",
            &tool_call(
                TAMPER,
                json!({"inputResponses": answer, "requestState": good}),
            ),
            None,
            912,
            &key,
        );
        let (response, body) = send(&request);
        assert_refused(&response, &body, 400, -32602, BAD_STATE);
    }
    // exactly 32 bytes is a key
    let request = stateless(
        "tools/call",
        &tool_call(TAMPER, json!({})),
        None,
        913,
        &[9u8; 32],
    );
    let (response, body) = send(&request);
    assert_eq!(response.status, 200, "{body}");
}

#[test]
fn a_tool_call_that_needs_no_state_is_served_with_or_without_a_key_ir_16() {
    let answers = json!({"user_name": accepted(json!({"name": "Alice"}))});
    for key in [vec![], key()] {
        let request = stateless(
            "tools/call",
            &tool_call(ELICIT, json!({"inputResponses": answers})),
            None,
            914,
            &key,
        );
        let (response, body) = send(&request);
        assert_eq!(response.status, 200);
        assert_eq!(body["result"]["content"][0]["text"], "Hello, Alice!");
    }
}

// ---- IR-17: nothing in the result ----

#[test]
fn an_input_required_result_with_neither_requests_nor_state_is_refused_ir_17() {
    let source =
        std::fs::read_to_string(common::root().join("src/InputGuard.lex.tex")).expect("source");
    assert!(source.contains("name: an_input_required_result_with_nothing_in_it_is_refused,"));
    // every answer the fixture gives has at least one of the two members
    for tool in [ELICIT, SAMPLING, ROOTS, STATE, MULTI, ROUND, TAMPER, CAPS] {
        let (_, body) = call(tool, json!({}));
        let result = &body["result"];
        assert!(result.get("inputRequests").is_some() || result.get("requestState").is_some());
    }
    let doc =
        std::fs::read_to_string(common::root().join("INPUT-REQUIRED.md")).expect("the document");
    assert!(doc.contains("emptyInputRequired"));
}

// ---- IR-18: the frame ----

#[test]
fn the_frame_carries_the_key_after_the_count_and_a_cut_or_longer_frame_is_the_fixed_error_ir_18() {
    let request = stateless(
        "tools/call",
        &tool_call(TAMPER, json!({})),
        None,
        915,
        &key(),
    );
    let frame = encode_request(&request);
    let tail = &frame[frame.len() - 36..];
    assert_eq!(&tail[..4], &[0, 0, 0, 32], "the key is length-prefixed");
    assert_eq!(&tail[4..], key().as_slice());
    let ok = common::frame_checked(&frame).expect("a response frame");
    assert_eq!(ok.status, 200);
    // the frame without the key chunk (the frame of an adapter that forgot it)
    let without = &frame[..frame.len() - 36];
    // and the same cut mid-chunk, and one byte more
    let mut longer = frame.clone();
    longer.push(0);
    for cut in [without.to_vec(), frame[..frame.len() - 1].to_vec(), longer] {
        let response = common::frame_checked(&cut).expect("a response frame");
        assert_eq!(response.status, 500, "{} bytes", cut.len());
        let body = json_of(&response);
        assert_eq!(body["error"]["code"], -32603);
        assert_eq!(body["error"]["message"], "Internal error");
    }
}

// ---- IR-19: the encoder ----

#[test]
fn the_requests_the_fixture_sends_are_the_json_the_schema_names_ir_19() {
    for (tool, key, method) in [
        (ELICIT, "user_name", "elicitation/create"),
        (SAMPLING, "capital_question", "sampling/createMessage"),
        (ROOTS, "client_roots", "roots/list"),
    ] {
        let (_, body) = call(tool, json!({}));
        let ask = &body["result"]["inputRequests"][key];
        assert_eq!(ask["method"], method);
        let def = match method {
            "elicitation/create" => "ElicitRequest",
            "sampling/createMessage" => "CreateMessageRequest",
            _ => "ListRootsRequest",
        };
        validate_against(def, ask).unwrap_or_else(|e| panic!("{tool}: {e}"));
        validate_against("InputRequests", &body["result"]["inputRequests"])
            .expect("a map of requests");
    }
    // the encoders of the kinds the fixture does not send are checked as kernel theorems
    let source =
        std::fs::read_to_string(common::root().join("src/InputLaws.lex.tex")).expect("source");
    for theorem in [
        "a_url_elicitation_is_written_with_its_mode_and_url",
        "a_sampling_request_is_written_with_every_member_the_schema_names",
        "a_roots_request_without_params_has_only_its_method",
        "a_tool_result_in_a_sampling_message_is_written",
    ] {
        assert!(source.contains(&format!("name: {theorem},")), "{theorem}");
    }
}

#[test]
fn the_text_of_a_request_is_sanitized_like_the_text_of_a_tool_result_ir_19() {
    let source =
        std::fs::read_to_string(common::root().join("src/ResultJson.lex.tex")).expect("source");
    assert!(
        source.contains("| Methods::ResultBody.inputRequired found => (sanitizedMembers (inputRequiredMembers found))"),
        "the input requests pass through the sanitizer"
    );
}
