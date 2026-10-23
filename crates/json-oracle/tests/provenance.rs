//! JC-12: the oracle runs the generated package over the pinned corpus, and both
//! are the files the model records.

mod common;

use std::path::Path;

use common::{corpus_root, hex, manifest_sha256, root, PIN};
use repo_model::Model;
use sha2::{Digest, Sha256};

fn generated() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("mcp_json_codec")
}

#[test]
fn the_corpus_is_the_pinned_commit_and_matches_its_recorded_checksum_jc_12() {
    assert!(
        corpus_root().ends_with(format!("JSONTestSuite-{PIN}")),
        "the vendored directory names the pin"
    );
    let model = Model::load(&root().join("model")).expect("the model loads");
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-JSONTESTSUITE")
        .expect("the corpus is a cited authority");
    assert!(
        authority.citation.contains(PIN),
        "the citation names the pinned commit"
    );
    assert_eq!(
        manifest_sha256(),
        authority.checksum,
        "the vendored files are not the files the authority records"
    );
    assert_eq!(
        common::corpus().len(),
        318,
        "the pinned test_parsing has 318 files"
    );
}

#[test]
fn the_corpus_licence_travels_with_it_jc_12() {
    let licence =
        std::fs::read_to_string(corpus_root().join("LICENSE")).expect("the licence is vendored");
    assert!(
        licence.contains("MIT License"),
        "the corpus is MIT licensed"
    );
}

#[test]
fn the_generated_package_is_the_recorded_export_of_the_codec_root_jc_12() {
    let dir = generated();
    let provenance: serde_json::Value = serde_json::from_slice(
        &std::fs::read(dir.join("provenance.json")).expect("provenance.json is committed"),
    )
    .expect("provenance.json is JSON");
    assert_eq!(provenance["name"], "mcp_json_codec");
    assert_eq!(provenance["profile"], "rust-std");
    assert_eq!(provenance["spec"], "lexlean/rust-provenance/1");
    let exports = provenance["exports"].as_array().expect("exports is a list");
    assert_eq!(exports.len(), 1, "one root is exported");
    assert_eq!(exports[0]["name"], "run");
    assert_eq!(
        provenance["effects"],
        serde_json::json!(["allocation", "overflow"])
    );

    // The export records the digest of every file it wrote; the oracle compiled
    // exactly the recorded src/lib.rs, not an edited copy.
    let source =
        std::fs::read(dir.join("src").join("lib.rs")).expect("the package source is committed");
    let recorded = provenance["files"]["src/lib.rs"]
        .as_str()
        .expect("the export records src/lib.rs");
    assert_eq!(
        recorded,
        hex(&Sha256::digest(&source)),
        "src/lib.rs is not the file the export recorded: regenerate it, never edit it"
    );
    assert_eq!(
        recorded,
        hex(&Sha256::digest(json_oracle::GENERATED_SOURCE.as_bytes())),
        "the oracle compiled a different file from the one recorded"
    );
    let manifest =
        std::fs::read(dir.join("Cargo.toml")).expect("the package manifest is committed");
    assert_eq!(
        provenance["files"]["Cargo.toml"]
            .as_str()
            .expect("the export records Cargo.toml"),
        hex(&Sha256::digest(&manifest)),
        "Cargo.toml is not the file the export recorded"
    );
}

#[test]
fn the_oracle_reaches_the_generated_package_jc_12() {
    // A run through the package, not a re-implementation: the diagnostic names
    // the byte the generated validator stopped at.
    match json_oracle::parse(b"[\"\xF4\x90\x80\x80\"]") {
        json_oracle::Parse::Refused(d) => {
            assert_eq!((d.kind, d.offset), (json_oracle::Refusal::InvalidUtf8, 3));
        }
        other => panic!("{other:?}"),
    }
}
