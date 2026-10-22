//! Shared by the lifecycle oracle tests: the vectors, how one runs against the
//! generated root, and the pins the register records.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use boon::{Compiler, SchemaIndex, Schemas};
use lifecycle_oracle::{decode_response, exchange_frame, FrameError, Request, Response};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// The commit of the conformance suite the vectors are taken from.
pub const PIN: &str = "232a9014457eaf4c59559916e3d616d2f7f28f05";

/// The commit of the MCP specification whose `schema/2026-07-28/schema.json` every
/// response is checked against.
pub const SPEC_PIN: &str = "80041ab0a139a8f4fb3adbc99f7ae9113ea59671";

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
    }
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
    }
}

/// The value of the first field named `name`, compared without regard to case.
pub fn header<'a>(response: &'a Response, name: &str) -> Option<&'a [u8]> {
    response
        .fields
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name.as_bytes()))
        .map(|(_, v)| v.as_slice())
}

/// The response body as JSON.
pub fn json_of(response: &Response) -> Value {
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

fn checker() -> &'static Checker {
    static CHECKER: OnceLock<Checker> = OnceLock::new();
    CHECKER.get_or_init(|| {
        let path = spec_root().join("schema/2026-07-28/schema.json");
        let schema: Value = serde_json::from_slice(&std::fs::read(&path).expect("schema reads"))
            .expect("the schema is JSON");
        let mut schemas = Schemas::new();
        let mut compiler = Compiler::new();
        compiler
            .add_resource(SCHEMA_URL, schema.clone())
            .expect("the schema is a resource");
        let mut defs = BTreeMap::new();
        for name in schema["$defs"].as_object().expect("$defs").keys() {
            let index = compiler
                .compile(&format!("{SCHEMA_URL}#/$defs/{name}"), &mut schemas)
                .unwrap_or_else(|e| panic!("{name} compiles: {e}"));
            defs.insert(name.clone(), index);
        }
        Checker {
            schemas,
            schema,
            defs,
        }
    })
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
    if body.get("result").is_some() {
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
fn method_of(request: &Request) -> Option<String> {
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
    let response = lifecycle_oracle::exchange(request)?;
    check_response(method_of(request).as_deref(), &response);
    Ok(response)
}

/// Run one frame through the generated root, checking the response against the schema.
pub fn frame_checked(frame: &[u8]) -> Result<Response, FrameError> {
    let response = decode_response(&exchange_frame(frame))?;
    check_response(None, &response);
    Ok(response)
}

/// Run one request through the generated root and parse the body.
pub fn send(request: &Request) -> (Response, Value) {
    let response = exchange_checked(request).expect("the root writes a response frame");
    let body = json_of(&response);
    (response, body)
}

/// One check of a vector against the response body and the bodies of earlier vectors.
fn check(
    vector: &str,
    check: &Value,
    body: &Value,
    earlier: &std::collections::BTreeMap<String, Value>,
) -> Result<(), String> {
    let at = check["at"].as_str().ok_or("a check names its pointer")?;
    let found = body.pointer(at);
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
    let file = vector_file();
    let mut earlier = std::collections::BTreeMap::new();
    let mut ran = 0;
    // Vectors that other vectors read (ST-05) run whatever their own tags.
    for vector in file["vectors"].as_array().expect("vectors") {
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
        let (response, body) = send(&request_of(vector));
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
        assert_eq!(
            header(&response, "content-type"),
            Some(&b"application/json"[..]),
            "{vid}: every body is JSON"
        );
        for c in vector["expect"]["checks"].as_array().expect("checks") {
            if let Err(why) = check(vid, c, &body, &earlier) {
                panic!(
                    "{why} (scenario {}, line {})",
                    vector["scenario"], vector["line"]
                );
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
