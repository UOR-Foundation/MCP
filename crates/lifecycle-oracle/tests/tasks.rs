//! The tasks extension, run against the generated lifecycle root.
//!
//! The core keeps no state, so the host of this test does what the adapter of PrismPM
//! issue 216 does for a socket: it holds the task table between requests, supplies the
//! clock and the identifier of a task a request may create, and stores the table the
//! root returns (`TASKS.md`, "The host's side"). Time is a number the test chooses, so
//! a task that "sleeps for two seconds" is a task the test asks about two seconds later.
//!
//! The requests the ten pinned scenarios send are the steps of `vectors/tasks.json`;
//! the tests here add the requests the scenarios do not send. Each is a falsifying probe
//! authored from the extension text the register cites, never a verdict.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use common::{
    check, ext_root, header, json_of, root, standard, suite_text, EXT_TASKS_PIN, META, PIN,
};
use lifecycle_oracle::{Request, Response, TaskHost};
use serde_json::{json, Value};

const TASKS: &str = "io.modelcontextprotocol/tasks";
const TASKS_DIR: &str = "src/scenarios/server/tasks/";

/// The first instant of every script, in the host's millisecond counter.
const BASE_MS: u64 = 1_790_000_000_000;

fn caps_with_tasks() -> Value {
    json!({"elicitation": {}, "sampling": {}, "extensions": {TASKS: {}}})
}

// ---- the host ----

/// Days since 1970-01-01 to a civil date (Howard Hinnant's `civil_from_days`).
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// An instant of the counter as ISO 8601 text.
fn iso(ms: u64) -> String {
    let secs = ms / 1000;
    let milli = ms % 1000;
    let (year, month, day) = civil(i64::try_from(secs / 86_400).expect("a day count"));
    let rem = secs % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{milli:03}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// The adapter's side of the contract: the table, the clock and the identifiers.
struct Host {
    table: Vec<u8>,
    at: u64,
    issued: u32,
}

impl Host {
    fn new() -> Self {
        Host {
            table: Vec::new(),
            at: 0,
            issued: 0,
        }
    }

    fn wait(&mut self, ms: u64) {
        self.at += ms;
    }

    /// The host fields of a request sent now.
    fn fields(&mut self) -> TaskHost {
        self.issued += 1;
        TaskHost {
            now_ms: BASE_MS + self.at,
            now: iso(BASE_MS + self.at),
            fresh_id: format!("task-{:04}", self.issued),
            table: self.table.clone(),
        }
    }

    /// Send a request through the root, check the response against the schemas, and store
    /// the table when the root returned one.
    fn send(&mut self, mut request: Request) -> (Response, Value) {
        request.host = self.fields();
        let response = common::exchange_checked(&request).expect("the root writes a response");
        if let Some(table) = &response.tasks {
            self.table.clone_from(table);
        }
        let body = json_of(&response);
        (response, body)
    }

    fn rpc(
        &mut self,
        method: &str,
        params: &Value,
        caps: &Value,
        headers: &[(&str, &str)],
    ) -> (Response, Value) {
        self.send(request_of(method, params, caps, headers))
    }

    /// A tool call by a client that declared `caps`.
    fn call(&mut self, caps: &Value, name: &str, arguments: &Value) -> (Response, Value) {
        self.rpc(
            "tools/call",
            &json!({"name": name, "arguments": arguments}),
            caps,
            &[],
        )
    }

    fn get(&mut self, task: &str) -> (Response, Value) {
        self.rpc(
            "tasks/get",
            &json!({"taskId": task}),
            &caps_with_tasks(),
            &[],
        )
    }

    fn cancel(&mut self, task: &str) -> (Response, Value) {
        self.rpc(
            "tasks/cancel",
            &json!({"taskId": task}),
            &caps_with_tasks(),
            &[],
        )
    }

    fn update(&mut self, task: &str, answers: &Value) -> (Response, Value) {
        self.rpc(
            "tasks/update",
            &json!({"taskId": task, "inputResponses": answers}),
            &caps_with_tasks(),
            &[],
        )
    }

    /// Create a task of `name` as a client that declared the extension; its id.
    fn create(&mut self, name: &str, arguments: &Value) -> String {
        let (response, body) = self.call(&caps_with_tasks(), name, arguments);
        assert_eq!(response.status, 200, "{body}");
        assert_eq!(body["result"]["resultType"], "task", "{body}");
        body["result"]["taskId"]
            .as_str()
            .expect("a task id")
            .to_owned()
    }
}

/// The name a routing header carries for `method` and `params`.
fn name_for(method: &str, params: &Value) -> Option<String> {
    let field = match method {
        "tools/call" => "name",
        "tasks/get" | "tasks/update" | "tasks/cancel" | "tasks/result" => "taskId",
        _ => return None,
    };
    params.get(field)?.as_str().map(str::to_owned)
}

/// A POST of `method` with `params`, a conformant `_meta` for a client that declared
/// `caps` (an `_meta` the params already carry is kept member by member), and the
/// standard headers with `headers` replacing or adding to them.
fn request_of(method: &str, params: &Value, caps: &Value, headers: &[(&str, &str)]) -> Request {
    let mut params = params.clone();
    let object = params.as_object_mut().expect("params are an object");
    let meta = object
        .entry("_meta")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .expect("_meta is an object");
    meta.entry("io.modelcontextprotocol/protocolVersion")
        .or_insert_with(|| json!("2026-07-28"));
    meta.entry("io.modelcontextprotocol/clientCapabilities")
        .or_insert_with(|| caps.clone());
    let name = name_for(method, &params);
    let mut fields = standard(method, name.as_deref());
    // a client that accepts a stream receives the notifications of a task before the answer
    fields.push((
        "Accept".to_owned(),
        "application/json, text/event-stream".to_owned(),
    ));
    for (key, value) in headers {
        match fields.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(key)) {
            Some(field) => field.1 = (*value).to_owned(),
            None => fields.push(((*key).to_owned(), (*value).to_owned())),
        }
    }
    common::post_owned(&fields, &common::rpc(method, &params.to_string(), "9"))
}

fn notes(response: &Response) -> Vec<Value> {
    common::notes_of(response)
}

fn status_of(body: &Value) -> &str {
    body["result"]["status"].as_str().expect("a status")
}

fn assert_refused(response: &Response, body: &Value, status: u32, code: i64) {
    assert_eq!(response.status, status, "{body}");
    assert_eq!(body["error"]["code"], code, "{body}");
}

fn assert_missing_tasks(response: &Response, body: &Value) {
    assert_refused(response, body, 400, -32021);
    assert_eq!(
        body["error"]["data"]["requiredCapabilities"]["extensions"][TASKS],
        json!({}),
        "{body}"
    );
}

// ---- the vectors ----

fn task_vector_file() -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("tasks.json");
    serde_json::from_slice(&std::fs::read(path).expect("the task vectors are committed"))
        .expect("the task vectors are JSON")
}

/// Replace `${name}` in every string and every member name of `value`.
fn substituted(value: &Value, vars: &BTreeMap<String, String>) -> Value {
    fn text(s: &str, vars: &BTreeMap<String, String>) -> String {
        let mut out = s.to_owned();
        for (name, value) in vars {
            out = out.replace(&format!("${{{name}}}"), value);
        }
        out
    }
    match value {
        Value::String(s) => Value::String(text(s, vars)),
        Value::Array(items) => Value::Array(items.iter().map(|v| substituted(v, vars)).collect()),
        Value::Object(members) => Value::Object(
            members
                .iter()
                .map(|(k, v)| (text(k, vars), substituted(v, vars)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// One check of a step against the response body. The kinds the shared evaluator
/// has (`eq`, `present`, `absent`, `is_object`, `nonempty_array`) are its; the others are
/// the questions a task response raises.
fn evaluate(
    step: &str,
    c: &Value,
    body: &Value,
    vars: &BTreeMap<String, String>,
) -> Result<(), String> {
    let at = c["at"].as_str().ok_or("a check names its pointer")?;
    let found = body.pointer(at);
    let need = || found.ok_or_else(|| format!("{step}: {at} is absent from {body}"));
    if c.get("type").is_some() {
        let value = need()?;
        let kind = c["type"].as_str().ok_or("type names a kind")?;
        let holds = match kind {
            "string" => value.is_string(),
            "integer" => value.is_i64() || value.is_u64(),
            "object" => value.is_object(),
            "array" => value.is_array(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            other => return Err(format!("{step}: unknown type {other}")),
        };
        return if holds {
            Ok(())
        } else {
            Err(format!("{step}: {at} is {value}, expected {kind}"))
        };
    }
    if c.get("one_of").is_some() {
        let value = need()?;
        let options = c["one_of"].as_array().ok_or("one_of lists values")?;
        return if options.contains(value) {
            Ok(())
        } else {
            Err(format!(
                "{step}: {at} is {value}, expected one of {options:?}"
            ))
        };
    }
    if c.get("iso8601").is_some() {
        let value = need()?;
        let text = value
            .as_str()
            .ok_or_else(|| format!("{step}: {at} is {value}"))?;
        let bytes = text.as_bytes();
        let shaped = bytes.len() >= 19
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes[10] == b'T'
            && bytes[13] == b':'
            && bytes[16] == b':'
            && bytes[..4].iter().all(u8::is_ascii_digit);
        return if shaped {
            Ok(())
        } else {
            Err(format!(
                "{step}: {at} is {text:?}, not an ISO 8601 timestamp"
            ))
        };
    }
    if c.get("contains").is_some() {
        let value = need()?;
        let want = c["contains"].as_str().ok_or("contains names text")?;
        return if value.as_str().is_some_and(|t| t.contains(want)) {
            Ok(())
        } else {
            Err(format!(
                "{step}: {at} is {value}, expected it to contain {want:?}"
            ))
        };
    }
    for key in ["positive_integer_or_null", "positive_integer"] {
        if c.get(key).is_some() {
            let value = need()?;
            let integer = value.as_i64().is_some_and(|n| n > 0);
            let allowed_null = key == "positive_integer_or_null" && value.is_null();
            return if integer || allowed_null {
                Ok(())
            } else {
                Err(format!("{step}: {at} is {value}, expected {key}"))
            };
        }
    }
    if let Some(n) = c.get("min_keys") {
        let value = need()?;
        let have = value.as_object().map_or(0, serde_json::Map::len);
        let want = usize::try_from(n.as_u64().ok_or("min_keys is a count")?).expect("a count");
        return if have >= want {
            Ok(())
        } else {
            Err(format!(
                "{step}: {at} has {have} members, expected at least {want}"
            ))
        };
    }
    if let Some(var) = c.get("eq_var") {
        let value = need()?;
        let var = var.as_str().ok_or("eq_var names a variable")?;
        let want = vars
            .get(var)
            .ok_or_else(|| format!("{step}: variable {var} is not captured"))?;
        return if value.as_str() == Some(want.as_str()) {
            Ok(())
        } else {
            Err(format!("{step}: {at} is {value}, expected {want:?}"))
        };
    }
    if let Some(var) = c.get("keys_exclude_var") {
        let value = need()?;
        let var = var.as_str().ok_or("names a variable")?;
        let key = vars
            .get(var)
            .ok_or_else(|| format!("{step}: variable {var} is not captured"))?;
        let has = value.as_object().is_some_and(|m| m.contains_key(key));
        return if has {
            Err(format!("{step}: {at} still has the answered key {key:?}"))
        } else {
            Ok(())
        };
    }
    if let Some(members) = c.get("each_value_has") {
        let value = need()?;
        for (key, entry) in value.as_object().ok_or("an object")? {
            for member in members.as_array().ok_or("members")? {
                let member = member.as_str().ok_or("a member name")?;
                if entry.get(member).is_none() {
                    return Err(format!("{step}: {at}/{key} lacks {member}"));
                }
            }
        }
        return Ok(());
    }
    check(step, c, body, &BTreeMap::new())
}

/// The status a wait loop of the scenarios stops at.
fn waited_for(poll: &Value, body: &Value) -> bool {
    let status = body.pointer("/result/status").and_then(Value::as_str);
    let terminal = matches!(status, Some("completed" | "failed" | "cancelled"));
    match poll["kind"].as_str() {
        Some("terminal") => terminal,
        Some("status") => terminal || status == poll["status"].as_str(),
        other => panic!("unknown wait {other:?}"),
    }
}

/// Run one script: every step in order, against one host, asserting every step.
fn run_script(script: &Value) {
    let sessions = script["sessions"].as_object().expect("sessions");
    let default_session = sessions.keys().next().expect("a session").clone();
    let mut host = Host::new();
    let mut vars: BTreeMap<String, String> = BTreeMap::new();
    for step in script["steps"].as_array().expect("steps") {
        let id = step["id"].as_str().expect("id");
        host.wait(step.get("after_ms").and_then(Value::as_u64).unwrap_or(0));
        let session = step
            .get("session")
            .and_then(Value::as_str)
            .unwrap_or(&default_session);
        let caps = &sessions[session];
        let method = step["method"].as_str().expect("method");
        let params = substituted(&step["params"], &vars);
        let overrides: Vec<(String, String)> = step
            .get("headers")
            .and_then(Value::as_object)
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            substituted(v, &vars).as_str().expect("text").to_owned(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        let borrowed: Vec<(&str, &str)> = overrides
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let mut waited = 0;
        let (response, body) = loop {
            let (response, body) = host.rpc(method, &params, caps, &borrowed);
            let Some(poll) = step.get("poll") else {
                break (response, body);
            };
            if waited_for(poll, &body) {
                break (response, body);
            }
            // the scenario's wait gives up after its timeout; a task that has not settled
            // by then is a failure of the root, reported with the last answer
            assert!(
                waited < poll["timeout_ms"].as_u64().expect("a timeout"),
                "{id}: the wait ended with {body}"
            );
            let interval = poll["interval_ms"].as_u64().expect("an interval");
            host.wait(interval);
            waited += interval;
        };
        let want = step["expect"]["status"].as_u64().expect("status");
        assert_eq!(
            u64::from(response.status),
            want,
            "{id} ({} line {}): HTTP status; body {body}",
            step["file"],
            step["line"]
        );
        assert!(
            matches!(
                header(&response, "content-type"),
                Some(b"application/json" | b"text/event-stream")
            ),
            "{id}: every body is JSON or an event stream"
        );
        for c in step["expect"]["checks"].as_array().expect("checks") {
            if let Err(why) = evaluate(id, &substituted(c, &vars), &body, &vars) {
                panic!(
                    "{why} (scenario {}, {} line {})",
                    script["scenario"], step["file"], step["line"]
                );
            }
        }
        if let Some(captures) = step.get("capture").and_then(Value::as_object) {
            for (name, at) in captures {
                let value = body
                    .pointer(at.as_str().expect("a pointer"))
                    .and_then(Value::as_str)
                    .unwrap_or_else(|| panic!("{id}: {at} is not text in {body}"));
                vars.insert(name.clone(), value.to_owned());
            }
        }
        if let Some(capture) = step.get("capture_key") {
            let at = capture["at"].as_str().expect("a pointer");
            let index = usize::try_from(capture["index"].as_u64().expect("an index")).expect("n");
            let key = body
                .pointer(at)
                .and_then(Value::as_object)
                .and_then(|m| m.keys().nth(index))
                .unwrap_or_else(|| panic!("{id}: no key {index} at {at} in {body}"));
            vars.insert(
                capture["var"].as_str().expect("a variable").to_owned(),
                key.clone(),
            );
        }
    }
}

/// Run every script that has a step tagged `id`, and return how many steps carry the tag.
fn run_register(id: &str) -> usize {
    let file = task_vector_file();
    let mut tagged = 0;
    for script in file["scripts"].as_array().expect("scripts") {
        let count = script["steps"]
            .as_array()
            .expect("steps")
            .iter()
            .filter(|s| {
                s["register"]
                    .as_array()
                    .expect("register")
                    .iter()
                    .any(|r| r == id)
            })
            .count();
        if count > 0 {
            run_script(script);
            tagged += count;
        }
    }
    assert!(tagged > 0, "no step is tagged {id}");
    tagged
}

// ---- TK-01: the extension is declared ----

#[test]
fn the_server_declares_the_extension_and_not_a_tasks_slot_tk_01() {
    assert!(run_register("TK-01") >= 1);
    let mut host = Host::new();
    // the declaration is the server's, whatever the client declared
    for caps in [json!({}), caps_with_tasks()] {
        let (response, body) = host.rpc("server/discover", &json!({}), &caps, &[]);
        assert_eq!(response.status, 200);
        assert_eq!(
            body["result"]["capabilities"]["extensions"][TASKS],
            json!({}),
            "{body}"
        );
        assert!(
            body["result"]["capabilities"].get("tasks").is_none(),
            "{body}"
        );
        assert_eq!(body["result"]["capabilities"]["tools"], json!({}), "{body}");
    }
}

#[test]
fn the_task_tools_are_listed_after_the_earlier_ones_tk_01() {
    let mut host = Host::new();
    let (_, body) = host.rpc("tools/list", &json!({}), &json!({}), &[]);
    let names: Vec<&str> = body["result"]["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .map(|t| t["name"].as_str().expect("name"))
        .collect();
    let tail = [
        "greet",
        "slow_compute",
        "failing_job",
        "protocol_error_job",
        "confirm_delete",
        "multi_input",
        "test_tool_with_task",
    ];
    assert_eq!(&names[names.len() - 7..], &tail);
    for tool in body["result"]["tools"].as_array().expect("tools") {
        // task support is the server's decision per request: no tool declares it
        assert!(tool.get("execution").is_none(), "{tool}");
    }
}

// ---- TK-02: the task methods are gated on the extension ----

#[test]
fn each_task_method_from_a_client_without_the_extension_is_a_missing_capability_tk_02() {
    assert!(run_register("TK-02") >= 3);
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    let before = host.table.clone();
    for (method, params) in [
        ("tasks/get", json!({"taskId": id})),
        ("tasks/update", json!({"taskId": id, "inputResponses": {}})),
        ("tasks/cancel", json!({"taskId": id})),
    ] {
        let (response, body) = host.rpc(method, &params, &json!({}), &[]);
        assert_missing_tasks(&response, &body);
        assert!(
            response.tasks.is_none(),
            "{method} must not touch the table"
        );
    }
    assert_eq!(host.table, before);
    let (_, body) = host.get(&id);
    assert_eq!(
        status_of(&body),
        "working",
        "the refused cancel changed nothing"
    );
}

#[test]
fn the_capability_is_checked_before_the_task_is_looked_up_tk_02() {
    let mut host = Host::new();
    let (response, body) = host.rpc(
        "tasks/get",
        &json!({"taskId": "no-such-task"}),
        &json!({}),
        &[],
    );
    assert_missing_tasks(&response, &body);
    let (response, body) = host.get("no-such-task");
    assert_refused(&response, &body, 400, -32602);
}

#[test]
fn the_extension_declared_in_the_request_meta_alone_opens_the_task_methods_tk_02() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    // a client that declared nothing at session level but declares the extension in
    // this request's own _meta (SEP-2575 per-request capabilities)
    let params = json!({
        "taskId": id,
        "_meta": {"io.modelcontextprotocol/clientCapabilities": {"extensions": {TASKS: {}}}}
    });
    let (response, body) = host.rpc("tasks/get", &params, &json!({}), &[]);
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(status_of(&body), "working");
}

// ---- TK-03: creation is the server's decision, and needs the extension ----

#[test]
fn a_task_is_created_only_for_a_client_that_declared_the_extension_tk_03() {
    assert!(run_register("TK-03") >= 6);
    let mut host = Host::new();
    let (_, with) = host.call(
        &caps_with_tasks(),
        "slow_compute",
        &json!({"seconds": 5, "label": "x"}),
    );
    let (_, without) = host.call(
        &json!({}),
        "slow_compute",
        &json!({"seconds": 5, "label": "x"}),
    );
    assert_eq!(with["result"]["resultType"], "task");
    assert_eq!(without["result"]["resultType"], "complete");
    assert!(without["result"].get("taskId").is_none());
    // what the sync path answers is what the task finishes with
    host.wait(5_000);
    let (_, done) = host.get(with["result"]["taskId"].as_str().expect("id"));
    assert_eq!(
        done["result"]["result"]["content"],
        without["result"]["content"]
    );
    assert_eq!(without["result"].get("isError"), None);
}

#[test]
fn a_sync_only_tool_never_becomes_a_task_tk_03() {
    let mut host = Host::new();
    for task_hint in [json!(null), json!({"ttl": 1000, "pollInterval": 50})] {
        let mut params = json!({"name": "greet", "arguments": {"name": "Ada"}});
        if !task_hint.is_null() {
            params["task"] = task_hint;
        }
        let (response, body) = host.rpc("tools/call", &params, &caps_with_tasks(), &[]);
        assert_eq!(response.status, 200);
        assert_eq!(body["result"]["resultType"], "complete", "{body}");
        assert_eq!(body["result"]["content"][0]["text"], "Hello, Ada!");
        assert!(
            response.tasks.is_none(),
            "a sync call leaves the table alone"
        );
    }
}

#[test]
fn arguments_are_validated_before_a_task_is_created_tk_03() {
    let mut host = Host::new();
    let (response, body) = host.call(&caps_with_tasks(), "slow_compute", &json!({"seconds": -1}));
    assert_eq!(response.status, 200);
    assert_eq!(body["result"]["resultType"], "complete", "{body}");
    assert_eq!(body["result"]["isError"], true);
    assert!(response.tasks.is_none());
    let (response, body) = host.call(&caps_with_tasks(), "greet", &json!({}));
    assert_eq!(body["result"]["isError"], true, "{body}");
    assert!(response.tasks.is_none());
}

#[test]
fn the_rate_limit_still_applies_before_a_task_is_created_tk_03() {
    let mut host = Host::new();
    let mut request = request_of(
        "tools/call",
        &json!({"name": "slow_compute", "arguments": {"seconds": 1}}),
        &caps_with_tasks(),
        &[],
    );
    request.tool_calls = 100;
    let (response, body) = host.send(request);
    assert_refused(&response, &body, 429, 429);
    assert!(response.tasks.is_none());
}

// ---- TK-04: a required task is refused to a client without the extension ----

#[test]
fn a_task_that_must_be_a_task_is_a_missing_capability_without_the_extension_tk_04() {
    assert!(run_register("TK-04") >= 1);
    let mut host = Host::new();
    for caps in [
        json!({}),
        json!({"elicitation": {}}),
        json!({"extensions": {"other/ext": {}}}),
    ] {
        let (response, body) = host.call(&caps, "failing_job", &json!({}));
        assert_missing_tasks(&response, &body);
        assert!(response.tasks.is_none(), "nothing was created");
    }
    // the same tool for a client that declared the extension
    let (response, body) = host.call(&caps_with_tasks(), "failing_job", &json!({}));
    assert_eq!(response.status, 200);
    assert_eq!(body["result"]["resultType"], "task");
}

#[test]
fn the_composed_tool_is_refused_on_its_last_round_without_the_extension_tk_04() {
    let mut host = Host::new();
    let caps = json!({"elicitation": {}});
    let params = json!({
        "name": "test_tool_with_task",
        "arguments": {},
        "inputResponses": {"user_name": {"action": "accept", "content": {"name": "Bo"}}}
    });
    let (response, body) = host.rpc("tools/call", &params, &caps, &[]);
    assert_missing_tasks(&response, &body);
}

// ---- TK-05: the shape of a created task ----

#[test]
fn a_created_task_is_the_flat_shape_with_the_hosts_identifier_and_clock_tk_05() {
    assert!(run_register("TK-05") >= 10);
    let mut host = Host::new();
    host.wait(1_234);
    let (response, body) = host.call(&caps_with_tasks(), "slow_compute", &json!({"seconds": 3}));
    assert_eq!(response.status, 200);
    let result = &body["result"];
    assert_eq!(result["resultType"], "task");
    assert_eq!(result["taskId"], "task-0001", "the id is the host's");
    assert_eq!(result["status"], "working");
    assert_eq!(result["createdAt"], iso(BASE_MS + 1_234));
    assert_eq!(result["lastUpdatedAt"], result["createdAt"]);
    assert_eq!(result["ttlMs"], 60_000);
    assert_eq!(result["pollIntervalMs"], 1_000);
    assert!(result.get("statusMessage").is_none());
    for member in [
        "task",
        "result",
        "error",
        "inputRequests",
        "requestState",
        "ttl",
        "pollInterval",
    ] {
        assert!(result.get(member).is_none(), "{member} in {result}");
    }
    assert!(
        notes(&response).is_empty(),
        "the client learns the id from the result"
    );
    assert!(response.tasks.is_some(), "the table gained the task");
}

#[test]
fn the_task_exists_when_the_result_is_returned_tk_05() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    let (response, body) = host.get(&id);
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(body["result"]["taskId"], id);
    // a second task does not replace the first
    let other = host.create("slow_compute", &json!({"seconds": 60}));
    assert_ne!(id, other);
    assert_eq!(host.get(&id).1["result"]["taskId"], id);
    assert_eq!(host.get(&other).1["result"]["taskId"], other);
}

#[test]
fn a_host_that_supplies_no_identifier_gets_a_server_error_and_no_task_tk_05() {
    let mut host = Host::new();
    let mut request = request_of(
        "tools/call",
        &json!({"name": "slow_compute", "arguments": {"seconds": 1}}),
        &caps_with_tasks(),
        &[],
    );
    request.host = host.fields();
    request.host.fresh_id.clear();
    let response = common::exchange_checked(&request).expect("a response");
    let body = json_of(&response);
    assert_refused(&response, &body, 500, -32603);
    assert_eq!(body["error"]["message"], "Internal error");
    assert!(body["error"].get("data").is_none());
    assert!(response.tasks.is_none());
}

// ---- TK-06: wire fields and lifetime ----

#[test]
fn a_task_is_kept_for_its_ttl_and_may_be_dropped_after_it_tk_06() {
    assert!(run_register("TK-06") >= 4);
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(59_999);
    let (response, body) = host.get(&id);
    assert_eq!(
        response.status, 200,
        "within its ttl the task answers: {body}"
    );
    assert_eq!(status_of(&body), "completed");
    host.wait(1);
    let (response, body) = host.get(&id);
    assert_refused(&response, &body, 400, -32602);
}

#[test]
fn a_task_that_expired_is_dropped_from_the_table_by_the_next_change_tk_06() {
    let mut host = Host::new();
    let old = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(60_000);
    let new = host.create("slow_compute", &json!({"seconds": 1}));
    let table: Value = serde_json::from_slice(&host.table).expect("the table is JSON");
    let ids: Vec<&str> = table
        .as_array()
        .expect("an array")
        .iter()
        .map(|e| e["task"]["taskId"].as_str().expect("id"))
        .collect();
    assert_eq!(ids, [new.as_str()], "{old} is gone");
}

#[test]
fn a_result_carries_no_related_task_meta_and_no_request_state_tk_06() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(1_000);
    let (_, body) = host.get(&id);
    let text = body.to_string();
    assert!(!text.contains("related-task"), "{text}");
    assert!(!text.contains("requestState"), "{text}");
}

// ---- TK-07: what tasks/get answers ----

#[test]
fn tasks_get_answers_the_detailed_task_of_the_moment_tk_07() {
    assert!(run_register("TK-07") >= 8);
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 2, "label": "a"}));
    host.wait(1_999);
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "working");
    assert_eq!(body["result"]["resultType"], "complete");
    assert!(body["result"].get("result").is_none());
    host.wait(1);
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "completed");
    assert_eq!(body["result"]["lastUpdatedAt"], iso(BASE_MS + 2_000));
    assert_eq!(body["result"]["createdAt"], iso(BASE_MS));
    assert_eq!(
        body["result"]["result"]["content"][0]["text"],
        "slow_compute a finished after 2 seconds"
    );
    // the inlined result is the tool's result: no envelope of its own
    assert!(body["result"]["result"].get("resultType").is_none());
}

#[test]
fn a_tool_error_completes_and_a_protocol_error_fails_tk_07() {
    let mut host = Host::new();
    let tool_error = host.create("failing_job", &json!({}));
    let protocol_error = host.create("protocol_error_job", &json!({}));
    host.wait(1_000);
    let (_, body) = host.get(&tool_error);
    assert_eq!(status_of(&body), "completed");
    assert_eq!(body["result"]["result"]["isError"], true);
    assert!(body["result"].get("error").is_none());
    let (_, body) = host.get(&protocol_error);
    assert_eq!(status_of(&body), "failed");
    assert_eq!(body["result"]["error"]["code"], -32603);
    assert_eq!(
        body["result"]["error"]["message"],
        "protocol_error_job failed"
    );
    assert!(body["result"].get("result").is_none());
}

#[test]
fn a_terminal_task_does_not_change_when_asked_again_tk_07() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(5_000);
    let (_, first) = host.get(&id);
    host.wait(5_000);
    let (_, second) = host.get(&id);
    assert_eq!(first["result"], second["result"]);
}

// ---- TK-08: unknown tasks and the removed methods ----

#[test]
fn an_unknown_task_is_invalid_params_for_each_task_method_tk_08() {
    assert!(run_register("TK-08") >= 3);
    let mut host = Host::new();
    for (method, params) in [
        ("tasks/get", json!({"taskId": "ghost"})),
        (
            "tasks/update",
            json!({"taskId": "ghost", "inputResponses": {}}),
        ),
        ("tasks/cancel", json!({"taskId": "ghost"})),
    ] {
        let (response, body) = host.rpc(method, &params, &caps_with_tasks(), &[]);
        assert_refused(&response, &body, 400, -32602);
        assert!(response.tasks.is_none());
    }
}

#[test]
fn there_is_no_task_list_and_no_task_result_method_tk_08() {
    let mut host = Host::new();
    host.create("slow_compute", &json!({"seconds": 1}));
    for method in ["tasks/list", "tasks/result", "tasks/delete"] {
        let (response, body) = host.rpc(
            method,
            &json!({"taskId": "task-0001"}),
            &caps_with_tasks(),
            &[],
        );
        assert_refused(&response, &body, 404, -32601);
    }
    // a client without the extension is told the same: the methods do not exist
    let (response, body) = host.rpc("tasks/list", &json!({}), &json!({}), &[]);
    assert_refused(&response, &body, 404, -32601);
}

// ---- TK-09: cancellation ----

#[test]
fn cancel_acknowledges_with_an_empty_result_and_the_task_is_cancelled_tk_09() {
    assert!(run_register("TK-09") >= 8);
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    host.wait(10);
    let (response, body) = host.cancel(&id);
    assert_eq!(response.status, 200);
    let mut keys: Vec<&String> = body["result"]
        .as_object()
        .expect("an object")
        .keys()
        .collect();
    keys.sort();
    assert_eq!(keys, ["_meta", "resultType"], "{body}");
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "cancelled");
    assert_eq!(body["result"]["lastUpdatedAt"], iso(BASE_MS + 10));
    assert!(body["result"].get("result").is_none());
}

#[test]
fn cancelling_a_terminal_task_is_acknowledged_and_changes_nothing_tk_09() {
    let mut host = Host::new();
    let done = host.create("slow_compute", &json!({"seconds": 1}));
    let failed = host.create("protocol_error_job", &json!({}));
    host.wait(2_000);
    let before: Vec<Value> = [&done, &failed]
        .iter()
        .map(|id| host.get(id).1["result"].clone())
        .collect();
    host.wait(1_000);
    for id in [&done, &failed] {
        let (response, body) = host.cancel(id);
        assert_eq!(response.status, 200, "{body}");
        assert_eq!(body["result"]["resultType"], "complete");
    }
    let after: Vec<Value> = [&done, &failed]
        .iter()
        .map(|id| host.get(id).1["result"].clone())
        .collect();
    assert_eq!(before, after);
    // a second cancel of a cancelled task is the same
    let live = host.create("slow_compute", &json!({"seconds": 60}));
    host.cancel(&live);
    let (_, first) = host.get(&live);
    host.wait(1_000);
    host.cancel(&live);
    assert_eq!(host.get(&live).1["result"], first["result"]);
}

#[test]
fn a_task_that_finished_before_the_cancel_arrived_stays_finished_tk_09() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(1_500);
    // nothing asked about it since it finished: the cancel finds the task finished
    host.cancel(&id);
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "completed", "{body}");
}

// ---- TK-10: input ----

#[test]
fn a_waiting_task_surfaces_its_requests_and_resumes_when_they_are_answered_tk_10() {
    assert!(run_register("TK-10") >= 10);
    let mut host = Host::new();
    let id = host.create("confirm_delete", &json!({"filename": "a.txt"}));
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "input_required");
    let requests = body["result"]["inputRequests"].as_object().expect("a map");
    assert_eq!(requests.len(), 1);
    let (key, request) = requests.iter().next().expect("one");
    assert_eq!(request["method"], "elicitation/create");
    assert_eq!(request["params"]["message"], "Delete a.txt");
    assert_eq!(request["params"]["mode"], "form");
    assert_eq!(
        request["params"]["requestedSchema"]["properties"]["confirm"]["type"],
        "boolean"
    );
    let (response, body) = host.update(
        &id,
        &json!({key: {"action": "accept", "content": {"confirm": true}}}),
    );
    assert_eq!(response.status, 200);
    assert_eq!(body["result"]["resultType"], "complete");
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "completed");
    assert_eq!(
        body["result"]["result"]["content"][0]["text"],
        "Deleted a.txt"
    );
    assert!(body["result"].get("inputRequests").is_none());
}

#[test]
fn an_answer_to_a_key_that_is_not_outstanding_is_ignored_tk_10() {
    let mut host = Host::new();
    let id = host.create("multi_input", &json!({}));
    let (_, body) = host.get(&id);
    let keys: Vec<String> = body["result"]["inputRequests"]
        .as_object()
        .expect("a map")
        .keys()
        .cloned()
        .collect();
    assert_eq!(
        keys,
        ["first", "second"],
        "two distinct keys, in the order asked"
    );
    let (_, body) = host.update(&id, &json!({"never-issued": {"action": "accept"}}));
    assert_eq!(body["result"]["resultType"], "complete");
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "input_required");
    assert_eq!(
        body["result"]["inputRequests"]
            .as_object()
            .expect("a map")
            .len(),
        2
    );
    // answer one; answering it again is ignored, and the other stays pending
    host.update(&id, &json!({"first": {"action": "accept"}}));
    host.update(&id, &json!({"first": {"action": "decline"}}));
    let (_, body) = host.get(&id);
    assert_eq!(status_of(&body), "input_required");
    let left: Vec<&String> = body["result"]["inputRequests"]
        .as_object()
        .expect("a map")
        .keys()
        .collect();
    assert_eq!(left, ["second"]);
    host.update(&id, &json!({"second": {"action": "accept"}}));
    assert_eq!(status_of(&host.get(&id).1), "completed");
}

#[test]
fn an_update_changes_a_task_only_while_it_waits_for_input_tk_10() {
    let mut host = Host::new();
    let working = host.create("slow_compute", &json!({"seconds": 60}));
    let finished = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(1_000);
    let before = host.get(&finished).1["result"].clone();
    for id in [&working, &finished] {
        let (response, body) = host.update(id, &json!({"k": {"action": "accept"}}));
        assert_eq!(response.status, 200, "{body}");
        assert_eq!(body["result"]["resultType"], "complete");
    }
    assert_eq!(status_of(&host.get(&working).1), "working");
    assert_eq!(host.get(&finished).1["result"], before);
    // a cancelled task that was waiting does not resume
    let waiting = host.create("confirm_delete", &json!({"filename": "x"}));
    host.cancel(&waiting);
    host.update(&waiting, &json!({"confirm": {"action": "accept"}}));
    assert_eq!(status_of(&host.get(&waiting).1), "cancelled");
}

#[test]
fn a_task_that_was_never_read_still_accepts_its_answers_tk_10() {
    let mut host = Host::new();
    let id = host.create("confirm_delete", &json!({"filename": "b"}));
    // the client answers without ever having asked: the update settles the task first
    host.update(&id, &json!({"confirm": {"action": "accept"}}));
    assert_eq!(status_of(&host.get(&id).1), "completed");
}

// ---- TK-11: routing headers ----

#[test]
fn the_task_methods_need_the_task_id_in_the_name_header_tk_11() {
    assert!(run_register("TK-11") >= 5);
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    for (method, params) in [
        ("tasks/get", json!({"taskId": id})),
        ("tasks/update", json!({"taskId": id, "inputResponses": {}})),
        ("tasks/cancel", json!({"taskId": id})),
    ] {
        let name = common::standard(method, None);
        let (response, body) = host.send(common::post_owned(
            &name,
            &common::rpc(
                method,
                &format!(r#"{{"taskId":"{id}","inputResponses":{{}},"_meta":{META}}}"#),
                "5",
            ),
        ));
        assert_refused(&response, &body, 400, -32020);
        assert!(response.tasks.is_none());
        let (response, body) = host.rpc(
            method,
            &params,
            &caps_with_tasks(),
            &[("Mcp-Name", "other")],
        );
        assert_refused(&response, &body, 400, -32020);
        let (response, body) = host.rpc(
            method,
            &params,
            &caps_with_tasks(),
            &[("Mcp-Method", "tasks/result")],
        );
        assert_refused(&response, &body, 400, -32020);
    }
    assert_eq!(
        status_of(&host.get(&id).1),
        "working",
        "refused requests changed nothing"
    );
}

// ---- TK-12: status notifications ----

#[test]
fn a_change_the_core_makes_is_written_as_a_task_notification_beside_the_body_tk_12() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 2, "label": "n"}));
    let (response, _) = host.get(&id);
    assert!(notes(&response).is_empty(), "nothing changed yet");
    host.wait(2_000);
    let (response, body) = host.get(&id);
    let sent = notes(&response);
    assert_eq!(sent.len(), 1, "{sent:?}");
    assert_eq!(sent[0]["jsonrpc"], "2.0");
    assert_eq!(sent[0]["method"], "notifications/tasks");
    // "identical to what tasks/get would have returned at that moment": the result
    // without the envelope
    let mut expected = body["result"].clone();
    let object = expected.as_object_mut().expect("an object");
    object.remove("resultType");
    object.remove("_meta");
    assert_eq!(sent[0]["params"], expected);
    assert_eq!(sent[0]["params"]["status"], "completed");
    // the same state again is not a change
    let (response, _) = host.get(&id);
    assert!(notes(&response).is_empty());
    // a client that does not accept a stream receives the JSON body alone, and the table
    // still changes
    let other = host.create("slow_compute", &json!({"seconds": 1}));
    host.wait(1_000);
    let mut plain = request_of(
        "tasks/get",
        &json!({"taskId": other}),
        &caps_with_tasks(),
        &[("Accept", "application/json")],
    );
    plain.host = host.fields();
    let response = common::exchange_checked(&plain).expect("a response");
    assert_eq!(
        header(&response, "content-type"),
        Some(&b"application/json"[..])
    );
    assert!(notes(&response).is_empty());
    assert!(response.tasks.is_some());
    assert_eq!(json_of(&response)["result"]["status"], "completed");
}

#[test]
fn cancel_update_and_creation_notify_only_when_they_change_a_task_tk_12() {
    let mut host = Host::new();
    let (response, _) = host.call(&caps_with_tasks(), "multi_input", &json!({}));
    assert!(notes(&response).is_empty(), "creation");
    let id = "task-0001";
    let (response, _) = host.get(id);
    assert_eq!(notes(&response).len(), 1, "parking is a change");
    assert_eq!(notes(&response)[0]["params"]["status"], "input_required");
    let (response, _) = host.update(id, &json!({"stranger": {"action": "accept"}}));
    assert!(notes(&response).is_empty(), "an ignored answer");
    let (response, _) = host.update(id, &json!({"first": {"action": "accept"}}));
    let sent = notes(&response);
    assert_eq!(sent.len(), 1, "a partial answer");
    assert_eq!(sent[0]["params"]["status"], "input_required");
    assert_eq!(
        sent[0]["params"]["inputRequests"]
            .as_object()
            .expect("a map")
            .keys()
            .collect::<Vec<_>>(),
        ["second"]
    );
    let (response, _) = host.update(id, &json!({"second": {"action": "accept"}}));
    let sent = notes(&response);
    assert_eq!(sent.len(), 1);
    assert_eq!(
        sent[0]["params"]["status"], "completed",
        "the last answer resumes and finishes"
    );
    let (response, _) = host.cancel(id);
    assert!(notes(&response).is_empty(), "a terminal task");
    let other = host.create("slow_compute", &json!({"seconds": 60}));
    let (response, _) = host.cancel(&other);
    let sent = notes(&response);
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0]["params"]["status"], "cancelled");
    assert_eq!(sent[0]["params"]["taskId"], other);
}

#[test]
fn a_task_notification_is_sanitized_like_every_other_text_tk_12() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 1, "label": "bell\u{7}"}));
    host.wait(1_000);
    let (response, body) = host.get(&id);
    let text = body["result"]["result"]["content"][0]["text"]
        .as_str()
        .expect("text")
        .to_owned();
    assert!(
        text.contains('\u{fffd}') && !text.contains('\u{7}'),
        "{text:?}"
    );
    assert_eq!(
        notes(&response)[0]["params"]["result"]["content"][0]["text"],
        text
    );
}

// ---- TK-13: input before the task ----

#[test]
fn a_tool_asks_first_and_becomes_a_task_on_the_last_round_tk_13() {
    assert!(run_register("TK-13") >= 3);
    let mut host = Host::new();
    let caps = json!({"elicitation": {}, "extensions": {TASKS: {}}});
    let (response, first) = host.rpc(
        "tools/call",
        &json!({"name": "test_tool_with_task", "arguments": {}}),
        &caps,
        &[],
    );
    assert_eq!(response.status, 200, "{first}");
    assert_eq!(first["result"]["resultType"], "input_required");
    assert!(first["result"].get("taskId").is_none());
    assert!(response.tasks.is_none(), "no task before the last round");
    // an answer that does not carry the name asks again
    let (_, again) = host.rpc(
        "tools/call",
        &json!({"name": "test_tool_with_task", "arguments": {},
                "inputResponses": {"user_name": {"action": "decline"}}}),
        &caps,
        &[],
    );
    assert_eq!(again["result"]["resultType"], "input_required", "{again}");
}

// ---- TK-14: the laws are stated ----

fn source(name: &str) -> String {
    std::fs::read_to_string(root().join("src").join(format!("{name}.lex.tex")))
        .unwrap_or_else(|e| panic!("src/{name}.lex.tex reads: {e}"))
}

#[test]
fn every_transition_of_a_task_has_a_theorem_in_the_laws_module_tk_14() {
    let laws = source("TaskLaws");
    let theorems = laws.matches("\ntheorem{").count();
    assert!(theorems >= 20, "{theorems} theorems");
    for transition in [
        "settle",
        "answer",
        "cancel",
        "touch",
        "withEntry",
        "findEntry",
        "isExpired",
    ] {
        assert!(
            laws.contains(&format!("Tasks::{transition}")),
            "no theorem in TaskLaws states Tasks::{transition}"
        );
    }
    for state in [
        "working",
        "input_required",
        "completed",
        "failed",
        "cancelled",
    ] {
        assert!(
            laws.contains(&format!("DetailedTask.{state}")),
            "no theorem in TaskLaws states a {state} task"
        );
    }
    let all = laws + &source("Refusal") + &source("Status");
    for refusal in ["unknownTask", "missingCapability", "internal"] {
        assert!(
            all.contains(&format!("ProviderRefusal.{refusal}")),
            "{refusal}"
        );
    }
}

// ---- TK-15: the host's side of the boundary ----

#[test]
fn a_table_that_cannot_be_read_is_a_server_error_that_leaves_the_table_alone_tk_15() {
    let mut host = Host::new();
    let id = host.create("slow_compute", &json!({"seconds": 60}));
    let good = host.table.clone();
    for broken in [
        b"not json".to_vec(),
        b"{}".to_vec(),
        b"[1]".to_vec(),
        b"[{\"task\":{}}]".to_vec(),
    ] {
        host.table = broken.clone();
        let (response, body) = host.get(&id);
        assert_refused(&response, &body, 500, -32603);
        assert_eq!(body["error"]["message"], "Internal error");
        assert!(
            response.tasks.is_none(),
            "the root never rewrites a table it cannot read"
        );
        let (response, body) =
            host.call(&caps_with_tasks(), "slow_compute", &json!({"seconds": 1}));
        assert_refused(&response, &body, 500, -32603);
        assert!(response.tasks.is_none());
        // a request that does not use tasks does not care
        let (response, _) = host.rpc("tools/list", &json!({}), &json!({}), &[]);
        assert_eq!(response.status, 200);
        assert_eq!(host.table, broken);
    }
    host.table = good;
    assert_eq!(host.get(&id).0.status, 200);
}

#[test]
fn the_table_is_returned_only_when_a_task_was_touched_tk_15() {
    let mut host = Host::new();
    let (response, _) = host.rpc("tools/list", &json!({}), &json!({}), &[]);
    assert!(response.tasks.is_none());
    let (response, _) = host.call(&json!({}), "greet", &json!({"name": "x"}));
    assert!(response.tasks.is_none());
    let id = host.create("slow_compute", &json!({"seconds": 1}));
    for (response, _) in [
        host.get(&id),
        host.cancel(&id),
        host.update(&id, &json!({})),
    ] {
        assert!(response.tasks.is_some());
    }
    // a refused request does not return one
    let (response, _) = host.get("ghost");
    assert!(response.tasks.is_none());
    // the table the root returns is the one it reads back
    let table = host.table.clone();
    let (response, _) = host.get(&id);
    assert_eq!(response.tasks.as_deref(), Some(table.as_slice()));
}

#[test]
fn many_tasks_are_served_on_the_standard_stack_tk_15() {
    let mut host = Host::new();
    let first = host.create("slow_compute", &json!({"seconds": 1}));
    let mut last = first.clone();
    for _ in 0..300 {
        last = host.create("slow_compute", &json!({"seconds": 1}));
    }
    assert_eq!(host.get(&first).1["result"]["taskId"], first);
    assert_eq!(host.get(&last).1["result"]["taskId"], last);
    let table: Value = serde_json::from_slice(&host.table).expect("JSON");
    assert_eq!(table.as_array().expect("an array").len(), 301);
}

#[test]
fn a_frame_whose_host_section_is_cut_or_not_text_is_a_server_error_tk_15() {
    let mut host = Host::new();
    let mut request = request_of("tools/list", &json!({}), &json!({}), &[]);
    request.host = host.fields();
    request.host.now = "AAAA".to_owned();
    let frame = lifecycle_oracle::encode_request(&request);
    let internal = |frame: &[u8]| {
        let response = common::frame_checked(frame).expect("a response frame");
        assert_eq!(response.status, 500);
        assert_eq!(json_of(&response)["error"]["code"], -32603);
        assert!(response.tasks.is_none());
    };
    // the clock text is not UTF-8
    let at = frame
        .windows(4)
        .position(|w| w == b"AAAA")
        .expect("the text is in the frame");
    let mut bad = frame.clone();
    bad[at] = 0xff;
    internal(&bad);
    // the frame ends inside the host section
    for cut in 1..=12 {
        internal(&frame[..frame.len() - cut]);
    }
    // bytes after the table
    let mut long = frame.clone();
    long.push(0);
    internal(&long);
    // the frame as sent is fine
    let response = common::frame_checked(&frame).expect("a response frame");
    assert_eq!(response.status, 200);
}

#[test]
fn a_frame_that_ends_after_the_tool_calls_count_is_served_without_a_host_section_tk_15() {
    // the conformance build's adapter writes the count and nothing after it
    let mut host = Host::new();
    let mut request = request_of("tools/list", &json!({}), &json!({}), &[]);
    request.host = host.fields();
    let frame = lifecycle_oracle::encode_request(&request);
    let section = 8
        + (4 + request.host.now.len())
        + (4 + request.host.fresh_id.len())
        + (4 + request.host.table.len());
    let short = &frame[..frame.len() - section];
    // the decoder refuses a flag of 0, so a frame that decodes with no table ended after its body
    let response = common::frame_checked(short).expect("a response frame");
    assert_eq!(response.status, 200);
    assert!(response.tasks.is_none());
    let body = json_of(&response);
    assert_eq!(
        body["result"]["tools"].as_array().expect("tools").len(),
        35,
        "{body}"
    );
    // a call that would make a task has no identifier to give it: a server error, no table
    let mut call = request_of(
        "tools/call",
        &json!({"name": "slow_compute", "arguments": {"seconds": 1}}),
        &caps_with_tasks(),
        &[],
    );
    call.host = host.fields();
    let frame = lifecycle_oracle::encode_request(&call);
    let section = 8
        + (4 + call.host.now.len())
        + (4 + call.host.fresh_id.len())
        + (4 + call.host.table.len());
    let response =
        common::frame_checked(&frame[..frame.len() - section]).expect("a response frame");
    let body = json_of(&response);
    assert_refused(&response, &body, 500, -32603);
    assert!(response.tasks.is_none());
}

// ---- TK-16: the vectors and what they were taken from ----

fn tasks_authority() -> repo_model::AuthorityRow {
    let model = repo_model::Model::load(&root().join("model")).expect("the model loads");
    model
        .authorities
        .authority
        .into_iter()
        .find(|a| a.id == "A-MCP-EXT-TASKS")
        .expect("A-MCP-EXT-TASKS is a cited authority")
}

#[test]
fn the_vendored_extension_is_the_pinned_commit_and_matches_its_recorded_checksum_tk_16() {
    assert!(
        ext_root().ends_with(format!("ext-tasks-{EXT_TASKS_PIN}")),
        "the vendored directory names the pin"
    );
    let a = tasks_authority();
    assert!(a.citation.contains(EXT_TASKS_PIN));
    assert_eq!(
        common::manifest_of(&ext_root()),
        a.checksum,
        "the vendored files are not the files the authority records"
    );
    let text = std::fs::read_to_string(ext_root().join("specification/2026-07-28/tasks.md"))
        .expect("the extension text");
    assert!(text.contains("io.modelcontextprotocol/tasks"));
    let licence = std::fs::read_to_string(ext_root().join("LICENSE")).expect("licence");
    assert!(licence.contains("Apache License"));
}

#[test]
fn the_vectors_cite_the_pins_the_register_records_tk_16() {
    let file = task_vector_file();
    assert_eq!(file["suite"]["commit"], PIN);
    assert_eq!(file["extension"]["commit"], EXT_TASKS_PIN);
    assert_eq!(
        file["extension"]["repository"],
        "https://github.com/modelcontextprotocol/ext-tasks"
    );
}

#[test]
fn the_pinned_requirements_run_the_ten_scenarios_and_do_not_score_them_tk_16() {
    let yaml = suite_text("requirements/2026-07-28.yaml");
    let not_scored = yaml.find("not_scored:").expect("a not_scored section");
    for scenario in [
        "tasks-lifecycle",
        "tasks-capability-negotiation",
        "tasks-wire-fields",
        "tasks-request-state-removal",
        "tasks-mrtr-input",
        "tasks-request-headers",
        "tasks-dispatch-and-envelope",
        "tasks-status-notifications",
        "tasks-required-task-error",
        "tasks-mrtr-composition",
    ] {
        let at = yaml
            .find(&format!(
                "  - scenario: {scenario}\n    leg: server\n    reason: extension"
            ))
            .unwrap_or_else(|| panic!("{scenario} is recorded as an unscored extension scenario"));
        assert!(at > not_scored, "{scenario} is under not_scored");
        let scripted = task_vector_file()["scripts"]
            .as_array()
            .expect("scripts")
            .iter()
            .any(|s| s["scenario"] == scenario)
            || task_vector_file()["unreached"]
                .as_array()
                .expect("unreached")
                .iter()
                .any(|u| u["id"] == scenario);
        assert!(
            scripted,
            "{scenario} is neither scripted nor listed as unreached"
        );
    }
}

fn line_of(text: &str, line: u64) -> &str {
    text.lines()
        .nth(usize::try_from(line).expect("line fits") - 1)
        .unwrap_or_else(|| panic!("line {line} is past the end of the file"))
}

/// The lines (1-based) of a scenario file that send a request.
fn request_sites(text: &str) -> BTreeSet<u64> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| {
            let t = line.trim_start();
            !(t.starts_with('*') || t.starts_with("//") || t.starts_with("/*"))
                && (line.contains(".request(")
                    || line.contains(".request<")
                    || line.contains(".discover()"))
        })
        .map(|(i, _)| i as u64 + 1)
        .collect()
}

#[test]
fn every_request_a_tasks_scenario_sends_is_a_step_of_a_script_tk_16() {
    let file = task_vector_file();
    let mut accounted: BTreeMap<String, BTreeSet<u64>> = BTreeMap::new();
    for script in file["scripts"].as_array().expect("scripts") {
        for step in script["steps"].as_array().expect("steps") {
            let path = step["file"].as_str().expect("file").to_owned();
            let text = suite_text(&path);
            let line = step["line"].as_u64().expect("line");
            let quote = step["quote"].as_str().expect("quote");
            assert!(
                line_of(&text, line).contains(quote),
                "{}: line {line} of {path} does not contain {quote:?}",
                step["id"]
            );
            accounted.entry(path).or_default().insert(line);
        }
    }
    let mut seen = 0;
    for entry in std::fs::read_dir(suite_root_tasks()).expect("the tasks directory") {
        let name = entry
            .expect("an entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        let path = format!("{TASKS_DIR}{name}");
        let sites = request_sites(&suite_text(&path));
        let have = accounted.get(&path).cloned().unwrap_or_default();
        assert_eq!(
            sites, have,
            "{path}: the request sites and the accounted lines differ"
        );
        seen += 1;
    }
    assert_eq!(seen, 12, "the twelve vendored scenario files");
}

fn suite_root_tasks() -> std::path::PathBuf {
    common::suite_root().join(TASKS_DIR)
}

#[test]
fn every_check_id_a_tasks_scenario_emits_is_covered_or_listed_tk_16() {
    let file = task_vector_file();
    let mut known: BTreeSet<String> = BTreeSet::new();
    for script in file["scripts"].as_array().expect("scripts") {
        for step in script["steps"].as_array().expect("steps") {
            for c in step["suite_checks"].as_array().expect("suite_checks") {
                known.insert(c.as_str().expect("id").to_owned());
            }
        }
    }
    for u in file["unreached"].as_array().expect("unreached") {
        known.insert(u["id"].as_str().expect("id").to_owned());
        assert!(!u["reason"].as_str().expect("reason").is_empty());
    }
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    for entry in std::fs::read_dir(suite_root_tasks()).expect("the tasks directory") {
        let name = entry
            .expect("an entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name == "helpers.ts" || name == "mrtr-helpers.ts" {
            continue;
        }
        for line in suite_text(&format!("{TASKS_DIR}{name}")).lines() {
            let t = line.trim();
            for prefix in [
                "const id = '",
                "const id2 = '",
                "id: '",
                "'tasks-status-notifications'",
            ] {
                if let Some(rest) = t.strip_prefix(prefix) {
                    let id = if prefix.starts_with('\'') {
                        "tasks-status-notifications"
                    } else {
                        rest.split('\'').next().expect("an id")
                    };
                    emitted.insert(id.to_owned());
                }
            }
        }
    }
    assert!(emitted.len() >= 35, "{} ids", emitted.len());
    assert_eq!(
        emitted, known,
        "the ids the scenarios emit and the ids the vectors cover differ"
    );
}

#[test]
fn the_vectors_state_the_register_ids_they_stand_for_tk_16() {
    let file = task_vector_file();
    let mut tagged: BTreeSet<String> = BTreeSet::new();
    for script in file["scripts"].as_array().expect("scripts") {
        assert!(
            !script["steps"].as_array().expect("steps").is_empty(),
            "{}",
            script["id"]
        );
        for step in script["steps"].as_array().expect("steps") {
            for r in step["register"].as_array().expect("register") {
                tagged.insert(r.as_str().expect("id").to_owned());
            }
        }
    }
    for id in [
        "TK-01", "TK-02", "TK-03", "TK-04", "TK-05", "TK-06", "TK-07", "TK-08", "TK-09", "TK-10",
        "TK-11", "TK-13",
    ] {
        assert!(tagged.contains(id), "no step is tagged {id}");
    }
    assert!(
        !tagged.contains("TK-12"),
        "the scenario of TK-12 is skipped upstream and has no step"
    );
}

#[test]
fn the_scripts_run_from_first_step_to_last_tk_16() {
    let file = task_vector_file();
    let scripts = file["scripts"].as_array().expect("scripts");
    assert_eq!(
        scripts.len(),
        9,
        "nine scripts; the tenth scenario is skipped upstream"
    );
    for script in scripts {
        run_script(script);
    }
}
