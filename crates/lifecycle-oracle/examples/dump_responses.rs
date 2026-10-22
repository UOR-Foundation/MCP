//! Runs every vector in `vectors/suite.json` through the generated root and prints
//! one JSON array of `{id, status, request, body}`. Used by `scripts/wire_check.ts`,
//! which validates each answer against the pinned suite's own wire schema.
//!
//! Run from the repository root:
//! `cargo run -q -p lifecycle-oracle --example dump_responses > responses.json`

use lifecycle_oracle::{exchange, Request};
use serde_json::{json, Value};

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is text"))
}

fn main() {
    let path = "crates/lifecycle-oracle/vectors/suite.json";
    let file: Value = serde_json::from_slice(&std::fs::read(path).expect("the vectors read"))
        .expect("the vectors are JSON");
    let mut out = Vec::new();
    for vector in file["vectors"].as_array().expect("vectors") {
        let sent = &vector["request"];
        let fields = sent["headers"]
            .as_array()
            .expect("headers")
            .iter()
            .map(|pair| {
                (
                    pair[0].as_str().expect("name").as_bytes().to_vec(),
                    pair[1].as_str().expect("value").as_bytes().to_vec(),
                )
            })
            .collect();
        let request = Request {
            method: text(sent, "method").as_bytes().to_vec(),
            target: text(sent, "target").as_bytes().to_vec(),
            fields,
            body: text(sent, "body").as_bytes().to_vec(),
        };
        let response = exchange(&request).expect("the root writes a response frame");
        out.push(json!({
            "id": vector["id"],
            "status": response.status,
            "request": serde_json::from_slice::<Value>(&request.body).expect("a JSON request"),
            "body": serde_json::from_slice::<Value>(&response.body).expect("a JSON answer"),
        }));
    }
    println!("{}", Value::Array(out));
}
