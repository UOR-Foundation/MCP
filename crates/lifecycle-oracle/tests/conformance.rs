//! CF-01 to CF-07: the conformance build and the oracle run (issue 13).
//!
//! The verdict of the run is the pinned suite's and the pinned schema's, produced by
//! the `conformance` job of CI. These tests check what can be checked without the
//! served build: the declaration, the vectors, the pins, the verdict logic, the
//! schema oracle and the record of the core.
//!
//! The tests that run a node oracle, or read the installed suite, are ignored in the
//! workspace gate (`cargo test --workspace`, which `just vv` runs inside the locked SDK
//! image, with neither node nor the suite: R6 forbids fetching it there). The
//! `conformance` job runs them with `--ignored` after installing the pinned suite
//! (`conformance/oracle/fetch-suite.sh DIR`, then `MCP_CONFORMANCE_SUITE_ROOT=DIR`), and
//! fails unless as many passed as are ignored (`run_oracle_tests` in
//! `.github/lexlean-ci/lib.sh`). A missing tool is a failure there, never a skip. CF-08
//! keeps the split honest.

mod common;

use std::collections::BTreeSet;
use std::process::Command;

use common::{hex, manifest_of, root, PIN, SPEC_PIN};
use repo_model::Model;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// SHA-256 of `requirements/2026-07-28.yaml` at the pinned commit.
const REQUIREMENTS_SHA256: &str =
    "ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57";

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|error| {
        panic!("{path} is part of the conformance build and does not read: {error}")
    })
}

fn json(path: &str) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|error| panic!("{path} is JSON: {error}"))
}

fn authority(id: &str) -> repo_model::AuthorityRow {
    let model = Model::load(&root().join("model")).expect("the model loads");
    model
        .authorities
        .authority
        .into_iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("{id} is a cited authority"))
}

/// Runs `node --test FILES` from the root; a failure shows the whole report.
fn node_tests(files: &[&str], needs_suite: bool) {
    let mut command = Command::new("node");
    command.current_dir(root()).arg("--test").args(files);
    if needs_suite {
        let suite = std::env::var("MCP_CONFORMANCE_SUITE_ROOT").expect(
            "MCP_CONFORMANCE_SUITE_ROOT names the installed conformance suite \
             (conformance/oracle/fetch-suite.sh DIR installs it)",
        );
        command.env("MCP_CONFORMANCE_SUITE_ROOT", suite);
    }
    let output = command
        .output()
        .expect("node runs; the conformance oracles are node programs");
    assert!(
        output.status.success(),
        "node --test {files:?} failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The members of a `members = [ ... ]` list, in order.
fn members(manifest: &str) -> Vec<String> {
    let start = manifest.find("members = [").expect("a members list") + "members = [".len();
    let end = start + manifest[start..].find(']').expect("the list ends");
    manifest[start..end]
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

/// The server scenarios of the frozen requirements: (required, not scored).
fn server_scenarios() -> (Vec<String>, Vec<String>) {
    let yaml = common::suite_text("requirements/2026-07-28.yaml");
    let required: Vec<String> = yaml
        .lines()
        .skip_while(|line| *line != "server:")
        .skip(1)
        .take_while(|line| line.starts_with("  - "))
        .map(|line| line.trim_start_matches("  - ").to_string())
        .collect();
    let mut unscored = Vec::new();
    let mut scenario = None;
    for line in yaml
        .lines()
        .skip_while(|line| *line != "not_scored:")
        .skip(1)
    {
        if let Some(name) = line.strip_prefix("  - scenario: ") {
            scenario = Some(name.to_string());
        } else if line == "    leg: server" {
            unscored.push(scenario.clone().expect("a leg follows its scenario"));
        }
    }
    (required, unscored)
}

#[test]
fn the_conformance_build_is_a_server_application_over_the_fixture_core_cf_01() {
    let application = read("conformance/src/Application.lex.tex");
    for needle in [
        "result: Foundation.Server.V1.Model::ServerApplication,",
        "profile := \"prismpm/server-application/1\"",
        "root := \"UorMcp.ConformanceFixture.conformanceRun\"",
        "package := \"mcp_conformance_core\"",
        "cargoName := \"mcp-conformance\"",
        "bindings/mcp-binding",
        "mcp_binding::Native",
        "127.0.0.1",
    ] {
        assert!(
            application.contains(needle),
            "Application.lex.tex does not declare {needle}"
        );
    }
    assert!(
        !application.contains("0.0.0.0"),
        "the build serves loopback only"
    );

    // The core is the conformance root of this repository, a production root: the step
    // serve over the fixture provider, which asks the host to store the task table.
    let fixture = read("src/ConformanceFixture.lex.tex");
    assert!(fixture.contains("name: conformanceRun,"));
    assert!(fixture.contains("production: {effects: [allocation, overflow], targets: [rust-std]}"));
    assert!(fixture.contains("StepServe::serveStep (conformanceInfo)"));

    // The declaration is part of a LexLean project and a PrismPM project of its own.
    let project = read("lexlean.conformance.toml");
    assert!(project.contains(
        "entrypoints = [\"conformance/src/Application.lex.tex\", \"src/ConformanceFixture.lex.tex\"]"
    ));
    let prismpm = read("prismpm.toml");
    assert!(prismpm.contains("lexlean_project = \"lexlean.conformance.toml\""));
    // The main project does not see it, so its gates are the ones they were.
    assert!(!read("lexlean.toml").contains("Application.lex.tex"));

    // The consumer workspace lists exactly the generated packages and the binding, and
    // the root workspace leaves it out.
    assert_eq!(
        members(&read("conformance/Cargo.toml")),
        [
            "mcp_conformance_core",
            "mcp-conformance-host-api",
            "mcp-conformance-host",
            "bindings/mcp-binding"
        ]
    );
    assert!(read("Cargo.toml").contains("exclude = [\"conformance\"]"));

    // The binding depends on the generated host API and nothing else (R6).
    let binding = read("conformance/bindings/mcp-binding/Cargo.toml");
    let dependencies: Vec<&str> = binding
        .lines()
        .skip_while(|line| *line != "[dependencies]")
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(
        dependencies,
        ["mcp-conformance-host-api = { path = \"../../mcp-conformance-host-api\" }"]
    );
    let lock = read("conformance/Cargo.lock");
    assert!(
        !lock.lines().any(|line| line.starts_with("source = ")),
        "the consumer workspace resolves nothing from a registry or a repository"
    );

    // The generated packages are not committed.
    let ignored = read(".gitignore");
    for package in [
        "/conformance/mcp_conformance_core/",
        "/conformance/mcp-conformance-host-api/",
        "/conformance/mcp-conformance-host/",
        "/conformance/target/",
    ] {
        assert!(
            ignored.lines().any(|line| line == package),
            "{package} is ignored"
        );
    }
}

#[test]
fn the_vectors_of_the_declaration_are_the_core_packages_answers_cf_02() {
    // The declaration is rendered from the vectors of the pinned scenarios, with the
    // answer of the committed core package to each, so a declaration that is not the
    // rendering is not what the core answers.
    let rendered = common::application::render();
    assert_eq!(
        read("conformance/src/Application.lex.tex"),
        rendered,
        "conformance/src/Application.lex.tex is not the rendering: \
         cargo run -p lifecycle-oracle --example conformance_application -- --write"
    );
    assert_eq!(common::application::SERVED.len(), 11);
    let tasks = common::application::task_vectors();
    assert_eq!(
        rendered.matches("AcceptanceVector {request :=").count(),
        common::application::SERVED.len() + tasks.len(),
        "one acceptance vector for each served vector and for each task vector"
    );
    // The core asks the host for one effect, the store of the task table, and the
    // declaration exercises it: a transcript step for each vector that wrote a table.
    assert_eq!(
        rendered
            .matches("HostStep {effect := \"task.store\"")
            .count(),
        tasks.iter().map(|vector| vector.steps.len()).sum::<usize>()
    );
    assert!(
        tasks.iter().any(|vector| !vector.steps.is_empty()),
        "a transcript exercises task.store"
    );
    assert!(
        tasks.iter().any(|vector| vector.steps.is_empty()),
        "a task vector that writes no table asks for no effect"
    );
    // The tag of an effect is its position in the declaration: the store is the fifth, and
    // the conformance root names that tag.
    let declared: Vec<usize> = [
        "tool.calls",
        "clock.now",
        "task.id",
        "task.table",
        "task.store",
    ]
    .iter()
    .map(|name| {
        rendered
            .find(&format!("HostEffect {{name := \"{name}\""))
            .unwrap_or_else(|| panic!("the declaration holds the effect {name}"))
    })
    .collect();
    assert!(declared.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(common::application::STORE_TAG, 4);
    assert!(read("src/ConformanceFixture.lex.tex")
        .contains("name: storeEffectTag,\n  parameters: [],\n  result: Nat,\n  body: 4,"));
    // No served vector depends on the host's count, which starts at zero in each run.
    for id in common::application::SERVED {
        let vector = common::application::vector(id);
        assert!(!vector.frame_request.is_empty(), "{id} has a request");
    }
}

#[test]
#[ignore = "runs in the conformance job with node and the pinned suite, not in the workspace gate"]
fn the_verdict_and_recorder_node_tests_pass_cf_03() {
    node_tests(
        &[
            "conformance/oracle/test/verdict.test.mjs",
            "conformance/oracle/test/recorder.test.mjs",
        ],
        false,
    );
}

#[test]
fn the_server_suite_verdict_is_read_from_the_per_scenario_checks_cf_03() {
    // The scenarios the oracle holds the build to are the frozen requirements': 37
    // required and 13 not scored.
    let (required, unscored) = server_scenarios();
    assert_eq!(required.len(), 37, "required server scenarios");
    assert_eq!(unscored.len(), 13, "not-scored server scenarios");
    let all: BTreeSet<&String> = required.iter().chain(unscored.iter()).collect();
    assert_eq!(all.len(), 50, "the scenarios are distinct");

    // What is not a pass is named in a committed file, and every name is a scenario.
    let waiting = json("conformance/waiting.json");
    assert_eq!(waiting["spec"], "2026-07-28");
    for (name, entry) in waiting["waiting"].as_object().expect("waiting") {
        assert!(
            all.contains(&name),
            "{name} is not a scenario of the frozen requirements"
        );
        let pr = &entry["pr"];
        assert!(
            pr.is_null() || [30, 32].contains(&pr.as_u64().expect("a pull request number")),
            "{name} waits on an unknown pull request"
        );
        assert!(entry["reason"]
            .as_str()
            .is_some_and(|reason| !reason.is_empty()));
        if name.starts_with("input-required-result-") {
            assert_eq!(
                entry["pr"], 30,
                "{name} is delivered by the input-required results"
            );
        }
        if name.starts_with("tasks-") {
            assert_eq!(
                entry["pr"], 32,
                "{name} is delivered by the tasks extension"
            );
        }
    }
    for (name, entry) in waiting["suite_skipped"].as_object().expect("suite_skipped") {
        assert!(
            unscored.contains(name),
            "{name} is skipped by the suite, so it is not scored"
        );
        assert!(entry["reason"]
            .as_str()
            .is_some_and(|reason| !reason.is_empty()));
    }
    assert!(
        waiting["waiting"]
            .as_object()
            .expect("waiting")
            .keys()
            .all(|name| waiting["suite_skipped"].get(name).is_none()),
        "a scenario is waiting or skipped by the suite, not both"
    );
    // The gate of the job: every waiting entry names its issue and its pull request, and
    // the job passes only when the failing scenarios are exactly these.
    for (name, entry) in waiting["waiting"].as_object().expect("waiting") {
        let issue = entry["issue"].as_u64().expect("an issue number");
        let expected = if name.starts_with("tasks-") { 12 } else { 10 };
        assert_eq!(issue, expected, "{name} names the issue that delivers it");
    }
    let verdict = read("conformance/oracle/verdict.mjs");
    for needle in ["matchesWaiting", "unexplained", "stale", "unnamed"] {
        assert!(
            verdict.contains(needle),
            "the verdict logic does not decide {needle}"
        );
    }
    let suite = read("conformance/oracle/server-suite.mjs");
    assert!(
        suite.contains("if (!matchesWaiting)"),
        "the exit status is the gate of the waiting list"
    );
}

#[test]
#[ignore = "runs in the conformance job with node and the pinned suite, not in the workspace gate"]
fn the_schema_oracle_node_tests_pass_cf_04() {
    node_tests(&["conformance/oracle/test/schema-oracle.test.mjs"], true);
}

#[test]
fn the_recorded_transcripts_validate_against_the_pinned_schema_cf_04() {
    // The oracle reads the schema the authority records.
    let schema = root().join(format!(
        "crates/lifecycle-oracle/suite/mcp-spec-{SPEC_PIN}/schema/2026-07-28/schema.json"
    ));
    let oracle = read("conformance/oracle/schema-oracle.mjs");
    assert!(oracle.contains("const SPEC = '2026-07-28';"));
    assert!(schema.is_file(), "the pinned schema is vendored");
    let ci = read(".github/workflows/lexlean.yml");
    let helpers = read(".github/lexlean-ci/lib.sh");
    assert!(
        helpers.contains(&format!(
            "mcp-spec-{SPEC_PIN}/schema/2026-07-28/schema.json"
        )),
        "CI validates the transcripts against the pinned schema"
    );
    assert!(ci.contains("run_schema_oracle"));
}

#[test]
#[ignore = "runs in the conformance job with node and the pinned suite, not in the workspace gate"]
fn the_verdict_logic_refuses_a_planted_defect_cf_05() {
    // The verdict logic names the scenario and the check for every failure, a scenario
    // without a passing check is not a pass, and a listed scenario that passes or an
    // unlisted one that fails fails the run: these are the node tests.
    node_tests(&["conformance/oracle/test/verdict.test.mjs"], false);
}

#[test]
fn a_planted_defect_fails_the_run_with_the_suites_diagnostic_cf_05() {
    // The defect that was planted, and the diagnostic the suite gave, are recorded
    // with the commands that reproduce them.
    let record = read("CONFORMANCE-RUN.md");
    for needle in [
        "## Planted defects",
        "tools-call-simple-text",
        "content array is empty",
        "FAILING and not named in waiting.json",
        "named as waiting but not failing",
    ] {
        assert!(
            record.contains(needle),
            "CONFORMANCE-RUN.md does not record {needle:?}"
        );
    }
}

#[test]
fn the_core_artifact_is_recorded_by_digest_cf_06() {
    let identity = json("conformance/core-identity.json");
    assert_eq!(identity["spec"], "mcp-conformance/core-identity/1");
    assert_eq!(identity["root"], "UorMcp.ConformanceFixture.conformanceRun");
    assert_eq!(identity["package"]["name"], "mcp_conformance_core");
    assert_eq!(identity["package"]["version"], "1.0.0");
    let program = identity["program_sha256"]
        .as_str()
        .expect("a program digest");
    assert_eq!(program.len(), 64);
    assert!(program
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));

    // The program is the one this repository already exports for the same root: the
    // compiled program does not depend on the compiler that exported it.
    let provenance = json("crates/lifecycle-oracle/generated/mcp_conformance/provenance.json");
    assert_eq!(
        provenance["program"], program,
        "the core is not the committed export of the root"
    );
    assert_eq!(identity["same_program_as"]["package"], "mcp_conformance");
    assert_eq!(identity["same_program_as"]["program_sha256"], program);

    // The record is of the sources the build was made from.
    let lock = std::fs::read(root().join("lexlean.conformance.lock")).expect("the lock");
    assert_eq!(
        identity["source"]["lock_sha256"],
        hex(&Sha256::digest(&lock)),
        "core-identity.json was not made from this lock: rebuild and `identity.mjs --write`"
    );
    // One compiler pin: the lock of the conformance project and the lock of the main
    // project record the same compiler, and the record names it.
    let semantics = |lock: &str| -> String {
        read(lock)
            .lines()
            .find_map(|line| {
                line.strip_prefix("compiler_semantics = \"")
                    .map(|rest| rest.trim_end_matches('"').to_string())
            })
            .unwrap_or_else(|| panic!("{lock} records a compiler"))
    };
    assert_eq!(
        semantics("lexlean.conformance.lock"),
        semantics("lexlean.lock"),
        "both locks are made by the compiler of .github/lexlean.rev"
    );
    assert_eq!(
        identity["compiler"]["semantics"],
        semantics("lexlean.lock"),
        "the core was exported by the compiler of .github/lexlean.rev"
    );
    let files = identity["package"]["files"].as_array().expect("files");
    let paths: Vec<&str> = files
        .iter()
        .map(|f| f["path"].as_str().expect("a path"))
        .collect();
    assert_eq!(
        paths,
        [
            "Cargo.toml",
            "package.json",
            "provenance.json",
            "src/lib.rs"
        ]
    );
    for file in files {
        let digest = file["sha256"].as_str().expect("a digest");
        assert_eq!(digest.len(), 64);
        assert!(file["byte_length"].as_u64().is_some_and(|n| n > 0));
    }
    // CI rebuilds the core and compares the record (identity.mjs --check).
    let ci = read(".github/workflows/lexlean.yml");
    let helpers = read(".github/lexlean-ci/lib.sh");
    assert!(helpers.contains("conformance/oracle/identity.mjs"));
    assert!(helpers.contains("--check conformance/core-identity.json"));
    assert!(ci.contains("check_core_identity"));
}

#[test]
#[ignore = "runs in the conformance job with node and the pinned suite, not in the workspace gate"]
fn the_identity_checker_refuses_another_core_cf_06() {
    // The checker itself is exercised: a package that disagrees with the result is refused.
    let dir = std::env::temp_dir().join(format!("identity-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("src")).expect("a scratch directory");
    for (name, bytes) in [
        ("Cargo.toml", "x"),
        ("package.json", "x"),
        ("provenance.json", "{}"),
        ("src/lib.rs", "x"),
    ] {
        std::fs::write(dir.join(name), bytes).expect("a scratch file");
    }
    let result = dir.join("result.json");
    std::fs::write(&result, "{}").expect("a scratch result");
    let output = Command::new("node")
        .current_dir(root())
        .arg("conformance/oracle/identity.mjs")
        .args(["--result"])
        .arg(&result)
        .args(["--lock", "lexlean.conformance.lock", "--package"])
        .arg(&dir)
        .args([
            "--reference",
            "crates/lifecycle-oracle/generated/mcp_conformance/provenance.json",
            "--check",
            "conformance/core-identity.json",
        ])
        .output()
        .expect("node runs");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(
        output.status.code(),
        Some(1),
        "a result that is not the record is refused"
    );
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("the production result must have exactly one root"));
}

#[test]
fn the_pins_of_the_oracle_run_agree_with_their_authority_records_cf_07() {
    // The conformance suite: one commit, written in the fetch script, the vectors and
    // the authority.
    let fetch = read("conformance/oracle/fetch-suite.sh");
    assert!(fetch.contains(&format!("COMMIT={PIN}\n")));
    assert!(authority("A-MCP-CONFORMANCE").citation.contains(PIN));
    assert_eq!(PIN, "232a9014457eaf4c59559916e3d616d2f7f28f05");

    // The frozen requirements: the digest the runner refuses to run without is the
    // digest of the vendored file and the authority's.
    let runner = read("conformance/oracle/server-suite.mjs");
    assert!(runner.contains(&format!(
        "const FROZEN_REQUIREMENTS_SHA256 = '{REQUIREMENTS_SHA256}';"
    )));
    let yaml =
        std::fs::read(common::suite_root().join("requirements/2026-07-28.yaml")).expect("yaml");
    assert_eq!(hex(&Sha256::digest(&yaml)), REQUIREMENTS_SHA256);
    assert!(authority("A-MCP-CONFORMANCE")
        .statement
        .contains(REQUIREMENTS_SHA256));
    let (required, unscored) = server_scenarios();
    assert_eq!((required.len(), unscored.len()), (37, 13));
    assert!(runner.contains("`mcp-conformance/232a9014+${REVISION}`"));

    // The schema: the pinned specification commit, whose digest the authority records.
    let schema = authority("A-MCP-SCHEMA-2026-07-28");
    assert!(schema.citation.contains(SPEC_PIN));
    assert_eq!(
        manifest_of(&common::spec_root()),
        schema.checksum,
        "the vendored schema is not the file the authority records"
    );

    // PrismPM: one revision, a full commit hash, written once; the files this repository
    // copies from it are the files the authority records.
    let pin = read(".github/prismpm.rev");
    let revisions: Vec<&str> = pin
        .lines()
        .filter(|line| line.starts_with("rev = "))
        .collect();
    assert_eq!(
        revisions.len(),
        1,
        ".github/prismpm.rev holds one `rev = ` line"
    );
    let revision = revisions[0].trim_start_matches("rev = ");
    assert_eq!(revision.len(), 40);
    assert!(revision
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    assert!(
        pin.contains("The pin moves to the PrismPM SDK release before MCP v0.1.0"),
        "the pin says when it moves"
    );
    assert!(
        pin.contains("pin a LexLean release that contains SM-37"),
        "the pin says what the release must pin"
    );
    let ci = read(".github/workflows/lexlean.yml") + &read(".github/lexlean-ci/lib.sh");
    assert!(
        !ci.contains(revision),
        "the revision is written in .github/prismpm.rev only"
    );
    let prismpm = authority("A-PRISMPM-SERVER-APPLICATION");
    assert!(prismpm
        .citation
        .contains("f5927bf46b92179d807ee6bf987f024b2f5a4e7e"));
    assert_eq!(
        revision, "f5927bf46b92179d807ee6bf987f024b2f5a4e7e",
        "the pin is the commit the authority cites"
    );
    let mut lines = Vec::new();
    for dir in ["conformance/src/Foundation", "language/prism.arch"] {
        for (name, bytes) in common::files_under(&root().join(dir)) {
            lines.push(format!("{}  {dir}/{name}\n", hex(&Sha256::digest(&bytes))));
        }
    }
    lines.sort_by(|a, b| a.as_bytes()[66..].cmp(&b.as_bytes()[66..]));
    let manifest: String = lines.concat();
    assert_eq!(
        hex(&Sha256::digest(manifest.as_bytes())),
        prismpm.checksum,
        "the copies of the PrismPM files are not the files the authority records"
    );
    // CI compares the copies with the PrismPM files at the pinned revision.
    assert!(ci.contains("sdk_copies_agree"));
}

/// One test function of a source file: its name, the text of its attributes and of its
/// body.
struct TestFunction {
    name: String,
    attributes: String,
    body: String,
}

/// The `#[test]` functions of a source file, at the top level.
fn test_functions(source: &str) -> Vec<TestFunction> {
    let mut found = Vec::new();
    let mut attributes = String::new();
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("#[") {
            attributes.push_str(line);
            attributes.push('\n');
        } else if let Some(rest) = line.strip_prefix("fn ") {
            let name = rest.split('(').next().unwrap_or_default().to_string();
            let mut body = String::from(line);
            for next in lines.by_ref() {
                body.push('\n');
                body.push_str(next);
                if next == "}" {
                    break;
                }
            }
            if attributes.contains("#[test]") {
                found.push(TestFunction {
                    name,
                    attributes: std::mem::take(&mut attributes),
                    body,
                });
            }
            attributes.clear();
        } else if !line.starts_with("//") {
            attributes.clear();
        }
    }
    found
}

/// Whether a test body runs node or reads the installed suite. The needles are written
/// in pieces so that this file does not match itself.
fn needs_node_or_suite(body: &str) -> bool {
    [
        concat!("Command::new(", "\"node\")"),
        concat!("node_", "tests("),
        concat!("MCP_CONFORMANCE_", "SUITE_ROOT"),
    ]
    .iter()
    .any(|needle| body.contains(needle))
}

/// What the workspace gate refuses in a test source: a test that needs node or the
/// installed suite and is not ignored for the conformance job, and an ignored test
/// whose reason does not name that job.
fn split_refusals(file: &str, source: &str) -> Vec<String> {
    let mut refusals = Vec::new();
    for test in test_functions(source) {
        let ignored = test.attributes.contains("#[ignore");
        let needs_node = needs_node_or_suite(&test.body);
        if needs_node && !ignored {
            refusals.push(format!(
                "{file}: {} needs node or the installed suite and is not ignored: the workspace gate runs without them",
                test.name
            ));
        }
        if needs_node && ignored && !test.attributes.contains("conformance job") {
            refusals.push(format!(
                "{file}: {} is ignored without naming the conformance job that runs it",
                test.name
            ));
        }
    }
    refusals
}

#[test]
fn the_workspace_gate_needs_neither_node_nor_the_suite_and_the_job_runs_the_rest_cf_08() {
    // The negative cases: the split refuses both ways it can be broken.
    let call = concat!("node_", "tests(&[\"x\"], true);");
    let source = |attributes: &str| {
        format!("#[test]\n{attributes}fn drives_the_oracle_cf_99() {{\n    {call}\n}}\n")
    };
    let unignored = source("");
    let unnamed = source("#[ignore = \"slow\"]\n");
    let honest = source("#[ignore = \"runs in the conformance job\"]\n");
    assert_eq!(split_refusals("x.rs", &unignored).len(), 1);
    assert!(split_refusals("x.rs", &unignored)[0].contains("needs node or the installed suite"));
    assert_eq!(split_refusals("x.rs", &unnamed).len(), 1);
    assert!(split_refusals("x.rs", &unnamed)[0].contains("without naming the conformance job"));
    assert!(split_refusals("x.rs", &honest).is_empty());

    // Every test source of the workspace obeys it, and the ignored tests are these.
    let mut ignored = BTreeSet::new();
    for entry in std::fs::read_dir(root().join("crates")).expect("crates") {
        let tests = entry.expect("a crate").path().join("tests");
        let Ok(files) = std::fs::read_dir(&tests) else {
            continue;
        };
        for file in files {
            let path = file.expect("a test source").path();
            if path.extension().is_none_or(|extension| extension != "rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("a test source reads");
            let name = path.strip_prefix(root()).expect("under the root");
            let refusals = split_refusals(&name.display().to_string(), &source);
            assert!(refusals.is_empty(), "{}", refusals.join("\n"));
            for test in test_functions(&source) {
                let needs_node = needs_node_or_suite(&test.body);
                if test.attributes.contains("#[ignore") && needs_node {
                    ignored.insert(format!("{}::{}", name.display(), test.name));
                }
            }
        }
    }
    let expected: BTreeSet<String> = [
        "the_verdict_and_recorder_node_tests_pass_cf_03",
        "the_schema_oracle_node_tests_pass_cf_04",
        "the_verdict_logic_refuses_a_planted_defect_cf_05",
        "the_identity_checker_refuses_another_core_cf_06",
    ]
    .iter()
    .map(|name| format!("crates/lifecycle-oracle/tests/conformance.rs::{name}"))
    .collect();
    assert_eq!(
        ignored, expected,
        "the tests ignored in the workspace are the oracle tests the conformance job runs"
    );

    // The conformance job runs exactly these, with --ignored, and requires as many
    // passes as there are ignored tests; a run that ran none is not green.
    let helpers = read(".github/lexlean-ci/lib.sh");
    let ci = read(".github/workflows/lexlean.yml");
    let start = helpers
        .find("run_oracle_tests() {")
        .expect("run_oracle_tests is defined");
    let function = &helpers[start
        ..helpers[start..]
            .find("\n}\n")
            .map_or(helpers.len(), |n| start + n)];
    for needle in [
        "--test conformance",
        "-- --ignored",
        concat!("MCP_CONFORMANCE_", "SUITE_ROOT:?"),
        "grep -c",
        "ignored tests, expected",
    ] {
        assert!(
            function.contains(needle),
            "run_oracle_tests does not contain {needle:?}"
        );
    }
    assert!(
        ci.contains("run_oracle_tests"),
        "a job runs the oracle tests"
    );
    // The cargo job is the workspace gate: it installs no suite and sets no root.
    let cargo_job = &ci[ci.find("  cargo:").expect("the cargo job")
        ..ci.find("  lexlean:").expect("the lexlean job")];
    assert!(!cargo_job.contains("install_suite"));
    assert!(!cargo_job.contains("run_oracle_tests"));
}
