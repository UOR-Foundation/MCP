//! LC-15: every response the root writes in these tests is checked against the
//! specification's own JSON Schema, and the check is shown to refuse what it should.
//!
//! A JSON Schema validator is falsifying evidence: it can show that a response is not
//! the shape the specification names, and it cannot show that the core is right. The
//! oracle for the protocol is MCP#6, not this file.

mod common;

use std::path::Path;

use common::{
    files_under, manifest_of, root, send, spec_root, validate_against, validate_body, vector_file,
    SPEC_PIN,
};
use repo_model::Model;
use serde_json::{json, Value};

fn authority(id: &str) -> repo_model::AuthorityRow {
    let model = Model::load(&root().join("model")).expect("the model loads");
    model
        .authorities
        .authority
        .into_iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("{id} is a cited authority"))
}

#[test]
fn the_vendored_schema_is_the_pinned_specification_and_matches_its_recorded_checksum_lc_15() {
    assert!(
        spec_root().ends_with(format!("mcp-spec-{SPEC_PIN}")),
        "the vendored directory names the pin"
    );
    let a = authority("A-MCP-SCHEMA-2026-07-28");
    assert!(
        a.citation.contains(SPEC_PIN),
        "the citation names the pinned commit"
    );
    assert_eq!(
        manifest_of(&spec_root()),
        a.checksum,
        "the vendored files are not the files the authority records"
    );
    let names: Vec<String> = files_under(&spec_root())
        .into_iter()
        .map(|(n, _)| n)
        .collect();
    assert_eq!(names, ["LICENSE", "schema/2026-07-28/schema.json"]);
    let licence = std::fs::read_to_string(spec_root().join("LICENSE")).expect("licence");
    assert!(
        licence.contains("Apache License, Version 2.0"),
        "the licence travels with it"
    );
}

#[test]
fn every_vector_response_validates_against_the_schema_lc_15() {
    let file = vector_file();
    let vectors = file["vectors"].as_array().expect("vectors");
    assert!(vectors.len() >= 32);
    for vector in vectors {
        // `send` checks the response against the schema and panics when it refuses.
        let (response, body) = send(&common::request_of(vector));
        assert!(response.status >= 200, "{}: {body}", vector["id"]);
    }
}

#[test]
fn no_test_reaches_the_root_without_the_schema_check_lc_15() {
    // Every test file other than this one and the shared helpers goes through
    // `common::exchange_checked`, `common::frame_checked` or `common::send`.
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut seen = 0;
    for entry in std::fs::read_dir(&tests).expect("tests directory") {
        let path = entry.expect("entry").path();
        let is_test = path.extension().is_some_and(|e| e == "rs");
        if !is_test || path.file_name().is_some_and(|n| n == "schema.rs") {
            continue;
        }
        seen += 1;
        let text = std::fs::read_to_string(&path).expect("source reads");
        for forbidden in [
            "lifecycle_oracle::exchange",
            "exchange_frame(",
            "decode_response(",
        ] {
            assert!(
                !text.contains(forbidden),
                "{} calls {forbidden}, which skips the schema check",
                path.display()
            );
        }
    }
    assert!(seen >= 3, "the test files were found");
}

#[test]
fn the_schema_accepts_an_error_without_an_id_and_with_one_lc_15() {
    let bare = json!({"jsonrpc": "2.0", "error": {"code": -32700, "message": "Parse error"}});
    validate_body(None, &bare).expect("an error response without an id is valid");
    let with_id = json!({"jsonrpc": "2.0", "id": 1, "error": {"code": -32600, "message": "x"}});
    validate_body(None, &with_id).expect("an error response with an id is valid");
}

#[test]
fn the_schema_refuses_an_error_response_with_a_null_id_lc_15() {
    for code in [-32700, -32600, -32603] {
        let body = json!({"jsonrpc": "2.0", "id": null, "error": {"code": code, "message": "x"}});
        let refused = validate_body(None, &body).expect_err("a null id is not a RequestId");
        assert!(refused.contains("JSONRPCResponse"), "{refused}");
    }
}

#[test]
fn the_schema_refuses_a_discover_result_without_a_server_info_lc_15() {
    let good = send(&common::post_owned(
        &common::standard("server/discover", None),
        &common::rpc(
            "server/discover",
            &format!(r#"{{"_meta":{}}}"#, common::META),
            "1",
        ),
    ))
    .1;
    validate_body(Some("server/discover"), &good).expect("the root's own answer is valid");
    let mut broken = good.clone();
    broken["result"]
        .as_object_mut()
        .expect("result")
        .retain(|k, _| !k.contains("serverInfo") && k != "resultType");
    validate_body(Some("server/discover"), &broken)
        .expect_err("a result without serverInfo or resultType is not a DiscoverResult");
}

#[test]
fn the_schema_refuses_an_unsupported_version_error_without_the_supported_list_lc_15() {
    let body = json!({
        "jsonrpc": "2.0", "id": 1,
        "error": {"code": -32022, "message": "x", "data": {"requested": "1999-01-01"}}
    });
    validate_against("UnsupportedProtocolVersionError", &body)
        .expect_err("data.supported is required");
    let good = json!({
        "jsonrpc": "2.0", "id": 1,
        "error": {"code": -32022, "message": "x",
                  "data": {"requested": "1999-01-01", "supported": ["2026-07-28"]}}
    });
    validate_body(None, &good).expect("with the list it is valid");
}

#[test]
fn the_schema_refuses_a_header_mismatch_with_another_code_lc_15() {
    let body = json!({"jsonrpc": "2.0", "id": 1, "error": {"code": -32602, "message": "x"}});
    validate_against("HeaderMismatchError", &body).expect_err("the code must be -32020");
    assert!(matches!(body["error"]["code"], Value::Number(_)));
}
