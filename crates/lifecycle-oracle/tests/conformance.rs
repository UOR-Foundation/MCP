//! CF-01 to CF-07: the conformance build and the oracle run (issue 13).
//!
//! The verdict of the run is the pinned suite's and the pinned schema's, produced by
//! the `conformance` job of CI. These tests check what can be checked without the
//! served build: the declaration, the vectors, the pins, the verdict logic and the
//! schema oracle.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/lifecycle-oracle is two below the root")
        .to_path_buf()
}

fn committed(path: &str) {
    assert!(
        root().join(path).is_file(),
        "{path} is part of the conformance build and is not committed"
    );
}

#[test]
fn the_conformance_build_is_a_server_application_over_the_fixture_core_cf_01() {
    committed("conformance/src/Application.lex.tex");
}

#[test]
fn the_vectors_of_the_declaration_are_the_core_packages_answers_cf_02() {
    committed("conformance/src/Application.lex.tex");
}

#[test]
fn the_server_suite_verdict_is_read_from_the_per_scenario_checks_cf_03() {
    committed("conformance/oracle/verdict.mjs");
}

#[test]
fn the_recorded_transcripts_validate_against_the_pinned_schema_cf_04() {
    committed("conformance/oracle/schema-oracle.mjs");
}

#[test]
fn a_planted_defect_fails_the_run_with_the_suites_diagnostic_cf_05() {
    committed("conformance/oracle/verdict.mjs");
}

#[test]
fn the_core_artifact_is_recorded_by_digest_cf_06() {
    committed("conformance/core-identity.json");
}

#[test]
fn the_pins_of_the_oracle_run_agree_with_their_authority_records_cf_07() {
    committed(".github/prismpm.rev");
}
