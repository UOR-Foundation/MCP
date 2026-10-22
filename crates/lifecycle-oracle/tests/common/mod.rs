//! Shared by the lifecycle oracle tests: the vectors, how one runs against the
//! generated root, and the pins the register records.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use lifecycle_oracle::{exchange, Request, Response};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// The commit of the conformance suite the vectors are taken from.
pub const PIN: &str = "232a9014457eaf4c59559916e3d616d2f7f28f05";

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

/// Every vendored suite file as (path under the pin directory, bytes), in byte order.
pub fn suite_files() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    walk(&suite_root(), &suite_root(), &mut out);
    out.sort();
    out
}

/// SHA-256 over the manifest: one `<sha256 of file>  <path>` line per vendored file.
pub fn manifest_sha256() -> String {
    let mut manifest = String::new();
    for (name, bytes) in suite_files() {
        manifest.push_str(&format!("{}  {name}\n", hex(&Sha256::digest(&bytes))));
    }
    hex(&Sha256::digest(manifest.as_bytes()))
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

/// Run one request through the generated root and parse the body.
pub fn send(request: &Request) -> (Response, Value) {
    let response = exchange(request).expect("the root writes a response frame");
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
