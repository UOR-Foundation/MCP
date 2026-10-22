//! LC-12, LC-13 and LC-14: what the oracle ran, what it ran it against, and that
//! nothing a scenario sends is left unaccounted for.

mod common;

use std::collections::BTreeSet;
use std::path::Path;

use common::{hex, manifest_sha256, root, suite_root, suite_text, vector_file, PIN};
use repo_model::Model;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// SHA-256 of `requirements/2026-07-28.yaml` at the pinned commit, which is the file
/// byte for byte at the 0.2.0-alpha.10 anchor the file's own comment names.
const REQUIREMENTS_SHA256: &str =
    "ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57";

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
fn the_vendored_suite_is_the_pinned_commit_and_matches_its_recorded_checksum_lc_12() {
    assert!(
        suite_root().ends_with(format!("conformance-{PIN}")),
        "the vendored directory names the pin"
    );
    let a = authority("A-MCP-CONFORMANCE");
    assert!(
        a.citation.contains(PIN),
        "the citation names the pinned commit"
    );
    assert!(
        a.statement.contains(REQUIREMENTS_SHA256),
        "the authority records the requirements checksum"
    );
    assert_eq!(
        manifest_sha256(),
        a.checksum,
        "the vendored files are not the files the authority records"
    );
    let yaml = std::fs::read(suite_root().join("requirements/2026-07-28.yaml")).expect("yaml");
    assert_eq!(hex(&Sha256::digest(&yaml)), REQUIREMENTS_SHA256);
    let licence = std::fs::read_to_string(suite_root().join("LICENSE")).expect("licence");
    assert!(
        licence.contains("Apache License, Version 2.0"),
        "the licence travels with it"
    );
}

#[test]
fn the_vectors_cite_the_pin_the_register_records_lc_12() {
    let file = vector_file();
    assert_eq!(file["suite"]["commit"], PIN);
    assert_eq!(file["suite"]["requirements_sha256"], REQUIREMENTS_SHA256);
    assert_eq!(
        file["suite"]["repository"],
        "https://github.com/modelcontextprotocol/conformance"
    );
}

#[test]
fn the_pinned_requirements_name_the_scenarios_and_their_standing_lc_12() {
    let yaml = suite_text("requirements/2026-07-28.yaml");
    let server: Vec<&str> = yaml
        .lines()
        .skip_while(|l| *l != "server:")
        .skip(1)
        .take_while(|l| l.starts_with("  - "))
        .collect();
    assert!(
        server.contains(&"  - server-stateless"),
        "server-stateless is a required server scenario"
    );
    for scenario in [
        "http-header-validation",
        "http-custom-header-server-validation",
    ] {
        let at = yaml
            .find(&format!(
                "  - scenario: {scenario}\n    leg: server\n    reason: pending"
            ))
            .unwrap_or_else(|| {
                panic!("{scenario} is recorded as a pending, unscored server scenario")
            });
        assert!(
            at > yaml.find("not_scored:").expect("a not_scored section"),
            "{scenario} is under not_scored"
        );
    }
}

#[test]
fn the_generated_package_is_the_recorded_export_of_the_lifecycle_root_lc_13() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("mcp_lifecycle");
    let provenance: Value =
        serde_json::from_slice(&std::fs::read(dir.join("provenance.json")).expect("provenance"))
            .expect("provenance.json is JSON");
    assert_eq!(provenance["name"], "mcp_lifecycle");
    assert_eq!(provenance["profile"], "rust-std");
    assert_eq!(provenance["spec"], "lexlean/rust-provenance/1");
    assert_eq!(provenance["exports"].as_array().expect("exports").len(), 1);
    assert_eq!(provenance["exports"][0]["name"], "run");
    assert_eq!(
        provenance["effects"],
        serde_json::json!(["allocation", "overflow", "recursion"])
    );
    let request: Value = serde_json::from_slice(
        &std::fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("generated")
                .join("request.json"),
        )
        .expect("request.json"),
    )
    .expect("request.json is JSON");
    assert_eq!(request["roots"][0]["root"], "UorMcp.Fixture.fixtureRun");
    assert_eq!(request["roots"][0]["package"]["name"], "mcp_lifecycle");

    let source = std::fs::read(dir.join("src").join("lib.rs")).expect("the package source");
    let recorded = provenance["files"]["src/lib.rs"]
        .as_str()
        .expect("src/lib.rs is recorded");
    assert_eq!(
        recorded,
        hex(&Sha256::digest(&source)),
        "src/lib.rs is not the file the export recorded: regenerate it, never edit it"
    );
    assert_eq!(
        recorded,
        hex(&Sha256::digest(
            lifecycle_oracle::GENERATED_SOURCE.as_bytes()
        )),
        "the oracle compiled a different file from the one recorded"
    );
    let manifest = std::fs::read(dir.join("Cargo.toml")).expect("the package manifest");
    assert_eq!(
        provenance["files"]["Cargo.toml"]
            .as_str()
            .expect("Cargo.toml is recorded"),
        hex(&Sha256::digest(&manifest))
    );
}

#[test]
fn the_oracle_reaches_the_generated_package_lc_13() {
    // A run through the package, not a re-implementation: the answer names the server
    // the fixture declares.
    let body = common::rpc(
        "server/discover",
        &format!(r#"{{"_meta":{}}}"#, common::META),
        "1",
    );
    let request = common::post_owned(&common::standard("server/discover", None), &body);
    let (_, json) = common::send(&request);
    assert_eq!(
        json["result"]["_meta"]["io.modelcontextprotocol/serverInfo"]["name"],
        "mcp-core-fixture"
    );
}

/// The lines of `text` (1-based) that send a request: a call of `sendRpc`,
/// `listenToStream`, `sendStatelessRequest(serverUrl`, or one of the scenario's case
/// runners, other than the line that defines it.
fn call_sites(text: &str) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    for (index, line) in text.lines().enumerate() {
        let sends = (line.contains("sendRpc(") && !line.contains("const sendRpc"))
            || (line.contains("listenToStream(") && !line.contains("const listenToStream"))
            || line.contains("sendStatelessRequest(serverUrl")
            || line.contains("this.testCase(")
            || line.contains("this.testBase64Case(")
            || line.contains("this.testMissingCustomHeader(");
        if sends {
            out.insert(index as u64 + 1);
        }
    }
    out
}

fn line_of(text: &str, line: u64) -> &str {
    text.lines()
        .nth(usize::try_from(line).expect("line fits") - 1)
        .unwrap_or_else(|| panic!("line {line} is past the end of the file"))
}

#[test]
fn every_request_a_scenario_sends_is_a_vector_or_a_listed_probe_that_is_not_run_lc_14() {
    let file = vector_file();
    for scenario_file in [
        "src/scenarios/server/stateless.ts",
        "src/scenarios/server/http-standard-headers.ts",
    ] {
        let text = suite_text(scenario_file);
        let mut accounted: BTreeSet<u64> = BTreeSet::new();
        for v in file["vectors"].as_array().expect("vectors") {
            if v["file"] == scenario_file {
                let line = v["line"].as_u64().expect("line");
                let quote = v["quote"].as_str().expect("quote");
                assert!(
                    line_of(&text, line).contains(quote),
                    "{}: line {line} of {scenario_file} does not contain {quote:?}",
                    v["id"]
                );
                accounted.insert(line);
            }
        }
        if scenario_file.ends_with("stateless.ts") {
            for n in file["not_run"].as_array().expect("not_run") {
                let line = n["line"].as_u64().expect("line");
                let quote = n["quote"].as_str().expect("quote");
                assert!(
                    line_of(&text, line).contains(quote),
                    "line {line} of {scenario_file} does not contain {quote:?}"
                );
                assert!(n["owner"].as_str().expect("owner").starts_with('#'));
                assert!(!n["reason"].as_str().expect("reason").is_empty());
                accounted.insert(line);
            }
        }
        assert_eq!(
            call_sites(&text),
            accounted,
            "{scenario_file}: the request sites and the accounted lines differ"
        );
    }
}

#[test]
fn every_check_id_a_scenario_emits_is_covered_or_listed_lc_14() {
    let file = vector_file();
    let mut known: BTreeSet<String> = BTreeSet::new();
    for v in file["vectors"].as_array().expect("vectors") {
        for c in v["suite_checks"].as_array().expect("suite_checks") {
            known.insert(c.as_str().expect("id").to_owned());
        }
    }
    for n in file["not_run"].as_array().expect("not_run") {
        for c in n["checks"].as_array().expect("checks") {
            known.insert(c.as_str().expect("id").to_owned());
        }
    }
    for u in file["unreached"].as_array().expect("unreached") {
        known.insert(u["id"].as_str().expect("id").to_owned());
        assert!(!u["reason"].as_str().expect("reason").is_empty());
    }
    let mut emitted: BTreeSet<String> = BTreeSet::new();
    for scenario_file in [
        "src/scenarios/server/stateless.ts",
        "src/scenarios/server/http-standard-headers.ts",
    ] {
        let text = suite_text(scenario_file);
        for token in text.split(['\'', '`']) {
            if (token.starts_with("sep-2575-") || token.starts_with("sep-2243-"))
                && token
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                emitted.insert(token.to_owned());
            }
        }
    }
    // The two templated ids are `<prefix>-<slug>`; their prefixes appear literally.
    let templated = [
        "sep-2575-request-meta-invalid-",
        "sep-2575-http-server-method-not-found-404-",
    ];
    let missing: Vec<&String> = emitted
        .iter()
        .filter(|id| !known.contains(*id) && !templated.contains(&id.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "check ids no vector covers: {missing:?}"
    );
    for id in &known {
        let literal = emitted.contains(id);
        let from_template = templated.iter().any(|t| id.starts_with(t));
        assert!(
            literal || from_template,
            "{id} is not an id the scenarios emit"
        );
    }
}
