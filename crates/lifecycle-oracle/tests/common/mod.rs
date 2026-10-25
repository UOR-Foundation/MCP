//! Shared by the lifecycle oracle tests: the vectors, how one runs against the
//! generated root, and the pins the register records.
#![allow(dead_code)]

pub mod application;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use boon::{Compiler, SchemaIndex, Schemas};
use lifecycle_oracle::{
    decode_response, encode_request, exchange_crowd_frame_on, exchange_frame_on,
    parse_event_stream, FrameError, Request, Response,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// The commit of the conformance suite the vectors are taken from.
pub const PIN: &str = "232a9014457eaf4c59559916e3d616d2f7f28f05";

/// The commit of the MCP specification whose `schema/2026-07-28/schema.json` every
/// response is checked against.
pub const SPEC_PIN: &str = "80041ab0a139a8f4fb3adbc99f7ae9113ea59671";

/// The commit of the `ext-tasks` repository whose `schema/2026-07-28/schema.json` types the
/// task results and the task status notification (`TASKS.md`, "Authorities"). The
/// specification's own schema has no task type: the extension keeps its schema apart.
pub const EXT_TASKS_PIN: &str = "899c1bbc869567670093face3591b39c28ce3219";

/// The workspace root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/lifecycle-oracle is two below the root")
        .to_path_buf()
}

/// The vendored files of the pinned suite; its name carries the pin.
pub fn suite_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("suite")
        .join(format!("conformance-{PIN}"))
}

/// Lower-case hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn walk(dir: &Path, base: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(dir).expect("the vendored suite reads") {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            walk(&path, base, out);
        } else {
            let name = path
                .strip_prefix(base)
                .expect("under the base")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((name, std::fs::read(&path).expect("a suite file reads")));
        }
    }
}

/// Every vendored file under `dir` as (path under `dir`, bytes), in byte order.
pub fn files_under(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

/// Every vendored suite file as (path under the pin directory, bytes), in byte order.
pub fn suite_files() -> Vec<(String, Vec<u8>)> {
    files_under(&suite_root())
}

/// SHA-256 over the manifest of `dir`: one `<sha256 of file>  <path>` line per file.
pub fn manifest_of(dir: &Path) -> String {
    let mut manifest = String::new();
    for (name, bytes) in files_under(dir) {
        manifest.push_str(&format!("{}  {name}\n", hex(&Sha256::digest(&bytes))));
    }
    hex(&Sha256::digest(manifest.as_bytes()))
}

/// SHA-256 over the manifest of the vendored conformance suite.
pub fn manifest_sha256() -> String {
    manifest_of(&suite_root())
}

/// The vendored copy of the specification's schema; its directory name carries the pin.
pub fn spec_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("suite")
        .join(format!("mcp-spec-{SPEC_PIN}"))
}

/// The vendored copy of the tasks extension's schema and text; its directory name carries
/// the pin.
pub fn ext_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("suite")
        .join(format!("ext-tasks-{EXT_TASKS_PIN}"))
}

/// The text of one vendored suite file.
pub fn suite_text(path: &str) -> String {
    std::fs::read_to_string(suite_root().join(path)).expect("the vendored file reads")
}

/// The vectors file, parsed.
pub fn vector_file() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("suite.json");
    serde_json::from_slice(&std::fs::read(path).expect("the vectors are committed"))
        .expect("the vectors are JSON")
}

/// The tools vectors file, parsed: the requests of the pinned tools scenarios.
pub fn tools_vector_file() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("tools.json");
    serde_json::from_slice(&std::fs::read(path).expect("the tools vectors are committed"))
        .expect("the tools vectors are JSON")
}

/// The resources vectors file, parsed: the requests of the pinned resource, prompt,
/// completion and caching scenarios.
pub fn resources_vector_file() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("resources.json");
    serde_json::from_slice(&std::fs::read(path).expect("the resources vectors are committed"))
        .expect("the resources vectors are JSON")
}

/// The transport vectors file, parsed: the requests of the pinned transport scenarios and
/// the streaming probes of `server-stateless` (issue 11).
pub fn transport_vector_file() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("vectors")
        .join("transport.json");
    serde_json::from_slice(&std::fs::read(path).expect("the transport vectors are committed"))
        .expect("the transport vectors are JSON")
}

/// Every vector of all the files, the lifecycle vectors first, in file order.
pub fn all_vectors() -> Vec<Value> {
    let mut out = Vec::new();
    for file in [
        vector_file(),
        tools_vector_file(),
        resources_vector_file(),
        transport_vector_file(),
    ] {
        out.extend(file["vectors"].as_array().expect("vectors").iter().cloned());
    }
    out
}

/// A plain POST with the headers a conformant client sends and the given body.
pub fn post(headers: &[(&str, &str)], body: &str) -> Request {
    Request {
        method: b"POST".to_vec(),
        target: b"/mcp".to_vec(),
        fields: headers
            .iter()
            .map(|(n, v)| (n.as_bytes().to_vec(), v.as_bytes().to_vec()))
            .collect(),
        body: body.as_bytes().to_vec(),
        tool_calls: 0,
        host: lifecycle_oracle::TaskHost::default(),
        state_key: Vec::new(),
    }
}

/// `request`, sent when the host holds `key` as the `requestState` key.
pub fn keyed(mut request: Request, key: &[u8]) -> Request {
    request.state_key = key.to_vec();
    request
}

/// `request`, sent when the host has already counted `tool_calls` tool calls for the
/// client in the current window.
pub fn after_calls(mut request: Request, tool_calls: u32) -> Request {
    request.tool_calls = tool_calls;
    request
}

/// The request a vector sends.
pub fn request_of(vector: &Value) -> Request {
    let request = &vector["request"];
    Request {
        method: request["method"]
            .as_str()
            .expect("method")
            .as_bytes()
            .to_vec(),
        target: request["target"]
            .as_str()
            .expect("target")
            .as_bytes()
            .to_vec(),
        fields: request["headers"]
            .as_array()
            .expect("headers")
            .iter()
            .map(|pair| {
                (
                    pair[0].as_str().expect("name").as_bytes().to_vec(),
                    pair[1].as_str().expect("value").as_bytes().to_vec(),
                )
            })
            .collect(),
        body: request["body"].as_str().expect("body").as_bytes().to_vec(),
        tool_calls: request.get("tool_calls").map_or(0, |n| {
            u32::try_from(n.as_u64().expect("tool_calls")).expect("u32")
        }),
        host: lifecycle_oracle::TaskHost::default(),
        state_key: Vec::new(),
    }
}

/// The names the host configured for a vector (`request.allowed_hosts`), none by default.
pub fn allowed_hosts_of(vector: &Value) -> Vec<Vec<u8>> {
    vector["request"]
        .get("allowed_hosts")
        .and_then(Value::as_array)
        .map_or_else(Vec::new, |names| {
            names
                .iter()
                .map(|n| n.as_str().expect("a name").as_bytes().to_vec())
                .collect()
        })
}

/// True when the response is an event stream.
pub fn is_stream(response: &Response) -> bool {
    header(response, "content-type") == Some(&b"text/event-stream"[..])
}

/// The events of a stream response, read as a client reads them, after the checks that make
/// a stream the core's own: status 200, every event of the default type with only a `data`
/// line, no `id`, `event` or `retry` field (resumption is not supported), and the stream
/// ending at the end of an event.
pub fn events_of(response: &Response) -> Vec<Value> {
    assert!(is_stream(response), "the response is not an event stream");
    assert_eq!(response.status, 200, "only a 200 answer is a stream");
    assert!(
        response.body.ends_with(b"\n\n"),
        "a stream ends with a blank line"
    );
    let parsed = parse_event_stream(&response.body).expect("a stream is UTF-8");
    assert!(!parsed.dangling, "the stream ends inside an event");
    let text = String::from_utf8_lossy(&response.body).into_owned();
    for line in text.split('\n').filter(|l| !l.is_empty()) {
        assert!(
            line.starts_with("data: "),
            "a line that is not a data line: {line:?}"
        );
    }
    parsed
        .events
        .iter()
        .map(|event| {
            assert_eq!(event.kind, "message");
            assert!(event.id.is_none() && event.retry.is_none());
            serde_json::from_str(&event.data)
                .unwrap_or_else(|e| panic!("an event is not JSON ({e}): {}", event.data))
        })
        .collect()
}

/// The progress notifications a response delivered: every event of a stream but the last,
/// and none for a single JSON body.
pub fn notes_of(response: &Response) -> Vec<Value> {
    if !is_stream(response) {
        return Vec::new();
    }
    let mut events = events_of(response);
    events.pop();
    events
}

/// The value of the first field named `name`, compared without regard to case.
pub fn header<'a>(response: &'a Response, name: &str) -> Option<&'a [u8]> {
    response
        .fields
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name.as_bytes()))
        .map(|(_, v)| v.as_slice())
}

/// The response body as JSON; for an event stream, the last event, which is the JSON-RPC
/// response the notifications lead up to.
pub fn json_of(response: &Response) -> Value {
    if is_stream(response) {
        return events_of(response)
            .pop()
            .expect("a stream carries the response");
    }
    serde_json::from_slice(&response.body).unwrap_or_else(|e| {
        panic!(
            "the root's body is not JSON ({e}): {}",
            String::from_utf8_lossy(&response.body)
        )
    })
}

/// The vendored schema, compiled once, with the definitions a response is checked against.
struct Checker {
    schemas: Schemas,
    schema: Value,
    defs: BTreeMap<String, SchemaIndex>,
}

const SCHEMA_URL: &str = "https://schema.invalid/mcp/2026-07-28/schema.json";

const EXT_SCHEMA_URL: &str = "https://schema.invalid/mcp-ext-tasks/2026-07-28/schema.json";

fn compile_checker(path: &Path, url: &str) -> Checker {
    let schema: Value = serde_json::from_slice(&std::fs::read(path).expect("schema reads"))
        .expect("the schema is JSON");
    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();
    compiler
        .add_resource(url, schema.clone())
        .expect("the schema is a resource");
    let mut defs = BTreeMap::new();
    for name in schema["$defs"].as_object().expect("$defs").keys() {
        let index = compiler
            .compile(&format!("{url}#/$defs/{name}"), &mut schemas)
            .unwrap_or_else(|e| panic!("{name} compiles: {e}"));
        defs.insert(name.clone(), index);
    }
    Checker {
        schemas,
        schema,
        defs,
    }
}

fn checker() -> &'static Checker {
    static CHECKER: OnceLock<Checker> = OnceLock::new();
    CHECKER.get_or_init(|| {
        compile_checker(
            &spec_root().join("schema/2026-07-28/schema.json"),
            SCHEMA_URL,
        )
    })
}

fn ext_checker() -> &'static Checker {
    static CHECKER: OnceLock<Checker> = OnceLock::new();
    CHECKER.get_or_init(|| {
        compile_checker(
            &ext_root().join("schema/2026-07-28/schema.json"),
            EXT_SCHEMA_URL,
        )
    })
}

/// Validate `value` against the definition `name` of the vendored tasks extension schema.
pub fn validate_ext(name: &str, value: &Value) -> Result<(), String> {
    let c = ext_checker();
    let index = *c
        .defs
        .get(name)
        .unwrap_or_else(|| panic!("the tasks extension schema defines {name}"));
    c.schemas
        .validate(value, index)
        .map_err(|e| format!("{name} (ext-tasks): {e:#}"))
}

/// Validate `value` against the definition `name` of the vendored schema.
pub fn validate_against(name: &str, value: &Value) -> Result<(), String> {
    let c = checker();
    let index = *c
        .defs
        .get(name)
        .unwrap_or_else(|| panic!("the schema defines {name}"));
    c.schemas
        .validate(value, index)
        .map_err(|e| format!("{name}: {e:#}"))
}

/// The definition that types the error a response carries, by JSON-RPC code.
fn error_def(code: i64) -> Option<&'static str> {
    Some(match code {
        -32700 => "ParseError",
        -32600 => "InvalidRequestError",
        -32601 => "MethodNotFoundError",
        -32602 => "InvalidParamsError",
        -32603 => "InternalError",
        -32020 => "HeaderMismatchError",
        -32021 => "MissingRequiredClientCapabilityError",
        -32022 => "UnsupportedProtocolVersionError",
        // The rate-limit code is outside the JSON-RPC range the MCP schema types
        // (RFC 6585 section 4; `Status.tooManyRequestsStatus`), so the schema has no
        // definition for it: the body is checked as a JSON-RPC response only.
        429 => "JSONRPCErrorResponse",
        // The refusal of a forbidden Origin or Host: a server-defined code in the
        // implementation range (-32019 to -32000), which the schema does not type.
        -32000 => "JSONRPCErrorResponse",
        _ => return None,
    })
}

/// The definition that types the success response to `method`, where the schema has one
/// for a method this core answers.
fn result_def(method: &str) -> Option<&'static str> {
    Some(match method {
        "server/discover" => "DiscoverResultResponse",
        "tools/list" => "ListToolsResultResponse",
        "tools/call" => "CallToolResultResponse",
        "resources/list" => "ListResourcesResultResponse",
        "resources/read" => "ReadResourceResultResponse",
        "resources/templates/list" => "ListResourceTemplatesResultResponse",
        "prompts/list" => "ListPromptsResultResponse",
        "prompts/get" => "GetPromptResultResponse",
        "completion/complete" => "CompleteResultResponse",
        _ => return None,
    })
}

/// Check a response body against the specification's JSON Schema: as a JSON-RPC
/// response, then as the typed response the schema names for the request's method (a
/// success) or for the error code (a failure). `method` is the method of the request
/// when it could be read.
///
/// This is falsifying evidence from a JSON Schema validator, not the oracle for the
/// protocol (MCP#6 binds that).
pub fn validate_body(method: Option<&str>, body: &Value) -> Result<(), String> {
    validate_against("JSONRPCResponse", body)?;
    if let Some(result) = body.get("result") {
        // A task result is typed by the extension's schema, not by the specification's:
        // `tools/call` may answer a CreateTaskResult, and the three task methods have
        // their own results.
        let ext = match (method, result.get("resultType").and_then(Value::as_str)) {
            (Some("tools/call"), Some("task")) => Some("CreateTaskResult"),
            (Some("tasks/get"), _) => Some("GetTaskResult"),
            (Some("tasks/update"), _) => Some("UpdateTaskResult"),
            (Some("tasks/cancel"), _) => Some("CancelTaskResult"),
            _ => None,
        };
        if let Some(def) = ext {
            return validate_ext(def, result);
        }
        // An interim result: the specification's schema types it.
        if method == Some("tools/call")
            && result.get("resultType").and_then(Value::as_str) == Some("input_required")
        {
            return validate_against("InputRequiredResult", result);
        }
        if let Some(def) = method.and_then(result_def) {
            validate_against(def, body)?;
        }
        return Ok(());
    }
    let code = body
        .pointer("/error/code")
        .and_then(Value::as_i64)
        .ok_or("an error response carries an integer code")?;
    let def =
        error_def(code).ok_or_else(|| format!("the schema types no error with code {code}"))?;
    let wrapped = checker().schema["$defs"][def]["properties"]
        .as_object()
        .is_some_and(|p| p.contains_key("error"));
    if wrapped {
        validate_against(def, body)
    } else {
        validate_against(def, &body["error"])
    }
}

/// The method of a request body, when it is a JSON object with a text `method`.
pub fn method_of(request: &Request) -> Option<String> {
    serde_json::from_slice::<Value>(&request.body)
        .ok()?
        .get("method")?
        .as_str()
        .map(str::to_owned)
}

/// When `SCHEMA_CHECK_LOG` names a file, append one line per response checked, so a
/// run can state how many responses the schema saw.
fn log_check(response: &Response) {
    use std::io::Write;
    if let Some(path) = std::env::var_os("SCHEMA_CHECK_LOG") {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(file, "{}", response.status);
        }
    }
}

/// Check one response; a response the schema refuses fails the test that produced it.
/// The 405 the core writes for a method other than POST has no body, which is the one
/// response that is not a JSON-RPC message.
pub fn check_response(method: Option<&str>, response: &Response) {
    log_check(response);
    if response.status == 405 {
        assert!(response.body.is_empty(), "a 405 has no body");
        return;
    }
    assert!(
        response.notifications.is_empty(),
        "the notification field of the frame is reserved and empty since issue 11"
    );
    for note in &notes_of(response) {
        if note["method"] == "notifications/tasks" {
            // The specification's schema has no task type; the extension's schema types
            // the notification, and the generic JSON-RPC shape is checked as well.
            for check in [
                validate_against("JSONRPCNotification", note),
                validate_ext("TaskStatusNotification", note),
            ] {
                if let Err(why) = check {
                    panic!(
                        "the schema refuses the root's notification: {why}
notification: {note}"
                    );
                }
            }
            continue;
        }
        for def in ["JSONRPCNotification", "ProgressNotification"] {
            if let Err(why) = validate_against(def, note) {
                panic!("the schema refuses the root's notification: {why}\nnotification: {note}");
            }
        }
    }
    let body = json_of(response);
    if let Err(why) = validate_body(method, &body) {
        panic!(
            "the schema refuses the root's response (status {}): {why}\nbody: {body}",
            response.status
        );
    }
}

/// Run one request through the generated root, checking the response against the schema.
///
/// # Errors
///
/// Returns [`FrameError`] when the root's answer is not a response frame.
pub fn exchange_checked(request: &Request) -> Result<Response, FrameError> {
    exchange_checked_allowing(request, &[])
}

/// [`exchange_checked`] for a host that configured `allowed_hosts`.
///
/// # Errors
///
/// Returns [`FrameError`] when the root's answer is not a response frame.
pub fn exchange_checked_allowing(
    request: &Request,
    allowed_hosts: &[Vec<u8>],
) -> Result<Response, FrameError> {
    let response = lifecycle_oracle::exchange_allowing(request, allowed_hosts)?;
    check_response(method_of(request).as_deref(), &response);
    Ok(response)
}

/// Run one frame through the generated root, checking the response against the schema.
pub fn frame_checked(frame: &[u8]) -> Result<Response, FrameError> {
    frame_checked_on(frame, lifecycle_oracle::STACK_BYTES)
}

/// [`frame_checked`] on a thread with `stack_bytes` of stack.
pub fn frame_checked_on(frame: &[u8], stack_bytes: usize) -> Result<Response, FrameError> {
    let response = decode_response(&exchange_frame_on(frame, stack_bytes))?;
    check_response(None, &response);
    Ok(response)
}

/// Decode a response frame that the step root wrote and check it against the schema.
///
/// # Panics
///
/// Panics when the bytes are not a response frame or the schema refuses it.
pub fn step_response_checked(frame: &[u8]) -> Response {
    let response = decode_response(frame).expect("a response frame");
    check_response(None, &response);
    response
}

/// The frame the byte root writes for a frame: the answer the step root must give when
/// no effect is requested.
pub fn byte_root_frame(frame: &[u8]) -> Vec<u8> {
    lifecycle_oracle::exchange_frame(frame)
}

/// Run one request through the second generated root, the one over a catalog of many
/// tools, checking the response against the schema.
pub fn crowd_checked(request: &Request) -> Response {
    let frame = exchange_crowd_frame_on(&encode_request(request), lifecycle_oracle::STACK_BYTES);
    let response = decode_response(&frame).expect("the crowd root writes a response frame");
    check_response(method_of(request).as_deref(), &response);
    response
}

/// Run the request of a vector, with the allow-list it names, and parse the body.
pub fn send_vector(vector: &Value) -> (Response, Value) {
    let response = exchange_checked_allowing(&request_of(vector), &allowed_hosts_of(vector))
        .expect("the root writes a response frame");
    let body = json_of(&response);
    (response, body)
}

/// Run one request through the generated root and parse the body.
pub fn send(request: &Request) -> (Response, Value) {
    let response = exchange_checked(request).expect("the root writes a response frame");
    let body = json_of(&response);
    (response, body)
}

/// One check of a vector against the response body and the bodies of earlier vectors.
pub fn check(
    vector: &str,
    check: &Value,
    body: &Value,
    earlier: &std::collections::BTreeMap<String, Value>,
) -> Result<(), String> {
    let at = check["at"].as_str().ok_or("a check names its pointer")?;
    // A check with a `tool` member points into that tool of a tools/list result.
    let base = match check.get("tool").and_then(Value::as_str) {
        None => Some(body),
        Some(tool) => body
            .pointer("/result/tools")
            .and_then(Value::as_array)
            .and_then(|tools| tools.iter().find(|t| t["name"] == tool)),
    };
    let found = base.and_then(|b| b.pointer(at));
    if check.get("absent").is_some() {
        return match found {
            None => Ok(()),
            Some(v) => Err(format!("{vector}: {at} should be absent, found {v}")),
        };
    }
    let found = found.ok_or_else(|| format!("{vector}: {at} is absent from {body}"))?;
    if let Some(want) = check.get("eq") {
        return if found == want {
            Ok(())
        } else {
            Err(format!("{vector}: {at} is {found}, expected {want}"))
        };
    }
    if check.get("present").is_some() {
        return Ok(());
    }
    if check.get("is_array").is_some() {
        return if found.is_array() {
            Ok(())
        } else {
            Err(format!("{vector}: {at} is {found}, expected an array"))
        };
    }
    if check.get("nonempty_string").is_some() {
        return match found.as_str() {
            Some(text) if !text.is_empty() => Ok(()),
            _ => Err(format!(
                "{vector}: {at} is {found}, expected a non-empty string"
            )),
        };
    }
    if let Some(part) = check.get("contains").and_then(Value::as_str) {
        return match found.as_str() {
            Some(text) if text.contains(part) => Ok(()),
            _ => Err(format!(
                "{vector}: {at} is {found}, expected text with {part:?}"
            )),
        };
    }
    if check.get("non_negative_integer").is_some() {
        return if found.is_u64() {
            Ok(())
        } else {
            Err(format!(
                "{vector}: {at} is {found}, expected a non-negative integer"
            ))
        };
    }
    if let Some(allowed) = check.get("one_of").and_then(Value::as_array) {
        return if allowed.contains(found) {
            Ok(())
        } else {
            Err(format!(
                "{vector}: {at} is {found}, expected one of {allowed:?}"
            ))
        };
    }
    if check.get("nonempty_array").is_some() {
        return match found.as_array() {
            Some(items) if !items.is_empty() => Ok(()),
            _ => Err(format!(
                "{vector}: {at} is {found}, expected a non-empty array"
            )),
        };
    }
    if check.get("is_object").is_some() {
        return if found.is_object() {
            Ok(())
        } else {
            Err(format!("{vector}: {at} is {found}, expected an object"))
        };
    }
    if let Some(keys) = check.get("each_has").and_then(Value::as_array) {
        let items = found
            .as_array()
            .ok_or_else(|| format!("{vector}: {at} is not an array"))?;
        for item in items {
            for key in keys {
                let key = key.as_str().ok_or("each_has names members")?;
                if item.get(key).is_none() {
                    return Err(format!("{vector}: {item} lacks {key}"));
                }
            }
        }
        return Ok(());
    }
    if let Some(rule) = check.get("each_name_charset") {
        let allowed = rule["allowed"].as_str().ok_or("allowed")?;
        // `allowed` is written as a character class body with ranges, for example
        // `A-Za-z0-9_./-`; a `-` last or first is a hyphen.
        let chars: Vec<char> = allowed.chars().collect();
        let mut ranges: Vec<(char, char)> = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            if i + 2 < chars.len() && chars[i + 1] == '-' {
                ranges.push((chars[i], chars[i + 2]));
                i += 3;
            } else {
                ranges.push((chars[i], chars[i]));
                i += 1;
            }
        }
        let min = rule["min"].as_u64().ok_or("min")?;
        let max = rule["max"].as_u64().ok_or("max")?;
        let items = found
            .as_array()
            .ok_or_else(|| format!("{vector}: {at} is not an array"))?;
        for item in items {
            let name = item["name"].as_str().ok_or("a tool has a text name")?;
            let length = name.chars().count() as u64;
            let inside = name
                .chars()
                .all(|c| ranges.iter().any(|(lo, hi)| *lo <= c && c <= *hi));
            if !(min..=max).contains(&length) || !inside {
                return Err(format!(
                    "{vector}: the tool name {name:?} is outside the format"
                ));
            }
        }
        return Ok(());
    }
    if let Some(from) = check.get("each_in_result_of").and_then(Value::as_str) {
        let result_at = check["result_at"].as_str().ok_or("result_at")?;
        let other = earlier
            .get(from)
            .ok_or_else(|| format!("{vector}: vector {from} has not run"))?;
        let allowed = other
            .pointer(result_at)
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{vector}: {from} {result_at} is not an array"))?;
        let items = found
            .as_array()
            .ok_or_else(|| format!("{vector}: {at} is not an array"))?;
        return if items.iter().all(|i| allowed.contains(i)) {
            Ok(())
        } else {
            Err(format!(
                "{vector}: {at} {found} is not within {from} {result_at}"
            ))
        };
    }
    Err(format!("{vector}: unknown check {check}"))
}

/// Run every vector tagged with the register id `id`, in file order, against the
/// generated root; every vector's earlier dependants run first.
pub fn run_register(id: &str) -> usize {
    let mut earlier = std::collections::BTreeMap::new();
    let mut ran = 0;
    // Vectors that other vectors read (ST-05) run whatever their own tags.
    for vector in &all_vectors() {
        let vid = vector["id"].as_str().expect("id");
        let tagged = vector["register"]
            .as_array()
            .expect("register")
            .iter()
            .any(|r| r == id);
        let needed = vid == "ST-05";
        if !tagged && !needed {
            continue;
        }
        let (response, body) = send_vector(vector);
        earlier.insert(vid.to_owned(), body.clone());
        if !tagged {
            continue;
        }
        ran += 1;
        let want = vector["expect"]["status"].as_u64().expect("status");
        assert_eq!(
            u64::from(response.status),
            want,
            "{vid} ({} line {}): HTTP status; body {body}",
            vector["file"],
            vector["line"]
        );
        let media = vector["expect"]
            .get("content_type")
            .and_then(Value::as_str)
            .unwrap_or("application/json");
        assert_eq!(
            header(&response, "content-type"),
            Some(media.as_bytes()),
            "{vid}: the media type of the answer"
        );
        for c in vector["expect"]["checks"].as_array().expect("checks") {
            if let Err(why) = check(vid, c, &body, &earlier) {
                panic!(
                    "{why} (scenario {}, line {})",
                    vector["scenario"], vector["line"]
                );
            }
        }
        // Each expected notification is a subset of the params of the notification
        // the core produced at that position, and there are no others.
        let want = vector["expect"].get("notifications");
        let got: Vec<Value> = notes_of(&response);
        let want = want.map_or(&[][..], |w| w.as_array().expect("notifications").as_slice());
        assert_eq!(
            got.len(),
            want.len(),
            "{vid}: the number of notifications; got {got:?}"
        );
        for (note, expected) in got.iter().zip(want) {
            assert_eq!(note["jsonrpc"], "2.0", "{vid}");
            assert_eq!(note["method"], "notifications/progress", "{vid}");
            for (key, value) in expected.as_object().expect("an object") {
                assert_eq!(&note["params"][key], value, "{vid}: params.{key} of {note}");
            }
        }
    }
    assert!(ran > 0, "no vector is tagged {id}");
    ran
}

/// The `_meta` of a conformant request.
pub const META: &str = r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

/// The headers a conformant client sends for `method`, with `name` when it carries one.
pub fn standard(method: &str, name: Option<&str>) -> Vec<(String, String)> {
    let mut headers = vec![
        ("Content-Type".to_owned(), "application/json".to_owned()),
        ("MCP-Protocol-Version".to_owned(), "2026-07-28".to_owned()),
        ("Mcp-Method".to_owned(), method.to_owned()),
    ];
    if let Some(name) = name {
        headers.push(("Mcp-Name".to_owned(), name.to_owned()));
    }
    headers
}

/// A POST of `body` with owned header pairs.
pub fn post_owned(headers: &[(String, String)], body: &str) -> Request {
    let borrowed: Vec<(&str, &str)> = headers
        .iter()
        .map(|(n, v)| (n.as_str(), v.as_str()))
        .collect();
    post(&borrowed, body)
}

/// A request body for `method` with `params` (a JSON text) and `id` (a JSON text).
pub fn rpc(method: &str, params: &str, id: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#)
}

/// A `tools/call` of `name` with `arguments` (a JSON text) and a conformant `_meta`.
pub fn call(name: &str, arguments: &str, id: &str) -> String {
    rpc(
        "tools/call",
        &format!(r#"{{"name":"{name}","arguments":{arguments},"_meta":{META}}}"#),
        id,
    )
}

/// Run a request and return the status with the code of the error it carries, if any.
pub fn outcome(request: &Request) -> (u32, Option<i64>) {
    let (response, body) = send(request);
    (
        response.status,
        body.pointer("/error/code").and_then(Value::as_i64),
    )
}

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
pub fn iso(ms: u64) -> String {
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

/// A POST of `method` with `params` by a client that declared the tasks extension.
pub fn tasks_request(method: &str, params: &Value, name: &str) -> Request {
    let mut params = params.clone();
    params["_meta"] = json!({
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientCapabilities": {
            "elicitation": {},
            "sampling": {},
            "extensions": {"io.modelcontextprotocol/tasks": {}}
        }
    });
    let mut fields = standard(method, Some(name));
    fields.push((
        "Accept".to_owned(),
        "application/json, text/event-stream".to_owned(),
    ));
    post_owned(&fields, &rpc(method, &params.to_string(), "9"))
}
