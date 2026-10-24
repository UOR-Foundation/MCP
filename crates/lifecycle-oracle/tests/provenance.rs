//! LC-12, LC-13 and LC-14: what the oracle ran, what it ran it against, and that
//! nothing a scenario sends is left unaccounted for.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use common::{
    all_vectors, hex, manifest_sha256, resources_vector_file, root, suite_root, suite_text,
    tools_vector_file, transport_vector_file, vector_file, PIN,
};
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
        serde_json::json!(["allocation", "overflow"]),
        "the export states no recursion effect (LC-16)"
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
    let roots = request["roots"].as_array().expect("roots");
    assert_eq!(roots.len(), 2, "the stress root and the fixture root");
    assert_eq!(roots[1]["root"], "UorMcp.Fixture.fixtureRun");
    assert_eq!(roots[1]["package"]["name"], "mcp_lifecycle");

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
///
/// The caching scenario sends every request through one helper, `queryEndpoint`, so its
/// request sites are the calls of that helper and not the one line inside it.
fn call_sites(text: &str, file: &str) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    if file.ends_with("caching.ts") {
        for (index, line) in text.lines().enumerate() {
            if line.contains("queryEndpoint(") && !line.contains("const queryEndpoint") {
                out.insert(index as u64 + 1);
            }
        }
        return out;
    }
    for (index, line) in text.lines().enumerate() {
        let sends = (line.contains("sendRpc(") && !line.contains("const sendRpc"))
            || (line.contains("listenToStream(") && !line.contains("const listenToStream"))
            || line.contains("sendStatelessRequest(serverUrl")
            || line.contains("this.testCase(")
            || line.contains("this.testBase64Case(")
            || line.contains("this.testMissingCustomHeader(")
            || (line.contains("conn.request<") && line.contains("await"))
            || line.contains("connection.client.callTool(");
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
    let vectors = all_vectors();
    let mut not_run: Vec<Value> = Vec::new();
    for file in [
        vector_file(),
        tools_vector_file(),
        resources_vector_file(),
        transport_vector_file(),
    ] {
        not_run.extend(file["not_run"].as_array().expect("not_run").iter().cloned());
    }
    for scenario_file in [
        "src/scenarios/server/stateless.ts",
        "src/scenarios/server/http-standard-headers.ts",
        "src/scenarios/server/tools.ts",
        "src/scenarios/server/json-schema-2020-12.ts",
        "src/scenarios/server/resources.ts",
        "src/scenarios/server/prompts.ts",
        "src/scenarios/server/caching.ts",
        "src/scenarios/server/utils.ts",
    ] {
        let text = suite_text(scenario_file);
        let mut accounted: BTreeSet<u64> = BTreeSet::new();
        for v in &vectors {
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
        // A listed probe names its file; the lifecycle file's older entries are the
        // probes of `stateless.ts` and name none.
        for n in &not_run {
            let file = n.get("file").and_then(Value::as_str);
            let mine = file.map_or(scenario_file.ends_with("stateless.ts"), |f| {
                f == scenario_file
            });
            if !mine {
                continue;
            }
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
        assert_eq!(
            call_sites(&text, scenario_file),
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
    // The streaming probes of stateless.ts that issue 11 took over are in the transport
    // vectors, listed with the suite file they cite.
    // The diagnostic probes of `stateless.ts` that the fixture's tools (TL-19, issue 13)
    // answer are in the tools vectors.
    for other in [transport_vector_file(), tools_vector_file()] {
        for (key, checks) in [("vectors", "suite_checks"), ("not_run", "checks")] {
            for entry in other[key].as_array().expect(key) {
                let cited = entry["file"].as_str().expect("file");
                if cited.ends_with("stateless.ts") || cited.ends_with("http-standard-headers.ts") {
                    for c in entry[checks].as_array().expect(checks) {
                        known.insert(c.as_str().expect("id").to_owned());
                    }
                }
            }
        }
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

// ---- TL-18: the second root, over a catalog of many tools ----

#[test]
fn the_crowd_package_is_the_recorded_export_of_the_crowd_root_tl_18() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("mcp_crowd");
    let provenance: Value =
        serde_json::from_slice(&std::fs::read(dir.join("provenance.json")).expect("provenance"))
            .expect("provenance.json is JSON");
    assert_eq!(provenance["name"], "mcp_crowd");
    assert_eq!(provenance["profile"], "rust-std");
    assert_eq!(provenance["exports"].as_array().expect("exports").len(), 1);
    assert_eq!(provenance["exports"][0]["name"], "run");
    assert_eq!(
        provenance["effects"],
        serde_json::json!(["allocation", "overflow"]),
        "the export states no recursion effect"
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
    assert_eq!(request["roots"][0]["root"], "UorMcp.Crowd.crowdRun");
    assert_eq!(request["roots"][0]["package"]["name"], "mcp_crowd");
    let source = std::fs::read(dir.join("src").join("lib.rs")).expect("the package source");
    let recorded = provenance["files"]["src/lib.rs"]
        .as_str()
        .expect("src/lib.rs is recorded");
    assert_eq!(recorded, hex(&Sha256::digest(&source)));
    assert_eq!(
        recorded,
        hex(&Sha256::digest(lifecycle_oracle::CROWD_SOURCE.as_bytes())),
        "the oracle compiled a different file from the one recorded"
    );
}

#[test]
fn no_function_of_the_crowd_package_is_on_a_call_cycle_tl_18() {
    let graph = call_graph(lifecycle_oracle::CROWD_SOURCE);
    assert!(
        graph.len() > 300,
        "the graph reads the package: {}",
        graph.len()
    );
    if let Some(cycle) = find_cycle(&graph) {
        panic!("the crowd package recurses: {}", cycle.join(" -> "));
    }
}

// ---- LC-16: no call cycle in the generated package ----

/// The functions of the generated package, `f<number>`, and the functions each one
/// calls. A function whose self calls were all tail calls is rendered as a loop and
/// has no call to itself; a call through a function value goes through the `apply`
/// method of its enum, so any function that calls `.apply(` is given an edge to every
/// function an `apply` can reach.
fn call_graph(source: &str) -> BTreeMap<String, BTreeSet<String>> {
    fn name_at(line: &str) -> Option<String> {
        let rest = line.strip_prefix("pub fn f")?;
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        (!digits.is_empty()).then(|| format!("f{digits}"))
    }
    fn callees(text: &str) -> BTreeSet<String> {
        let bytes = text.as_bytes();
        let mut out = BTreeSet::new();
        let mut i = 0;
        while i < bytes.len() {
            let boundary =
                i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            if bytes[i] == b'f' && boundary {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j > i + 1 && j < bytes.len() && bytes[j] == b'(' {
                    out.insert(text[i..j].to_owned());
                }
            }
            i += 1;
        }
        out
    }
    let mut bodies: BTreeMap<String, String> = BTreeMap::new();
    let mut current: Option<String> = None;
    // the functions each closure type can dispatch to, by the closure type's name
    let mut applied: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut in_apply: Option<String> = None;
    for line in source.lines() {
        if let Some(name) = name_at(line) {
            current = Some(name.clone());
            // the signature names the function; it is not a call
            bodies.entry(name).or_default();
            continue;
        }
        let top_level = !line.is_empty() && !line.starts_with(char::is_whitespace);
        if top_level && !line.starts_with('}') {
            current = None;
        }
        if let Some(kind) = line
            .strip_prefix("impl ")
            .and_then(|rest| rest.strip_suffix(" {"))
        {
            in_apply = Some(kind.to_owned());
        } else if top_level {
            in_apply = None;
        }
        if let Some(kind) = &in_apply {
            applied
                .entry(kind.clone())
                .or_default()
                .extend(callees(line));
        }
        if let Some(name) = &current {
            bodies.get_mut(name).expect("open").push_str(line);
            bodies.get_mut(name).expect("open").push('\n');
        }
    }
    let every: BTreeSet<String> = applied.values().flatten().cloned().collect();
    bodies
        .into_iter()
        .map(|(name, body)| {
            let mut out = callees(&body);
            out.extend(applied_by(&body, &applied, &every));
            (name, out)
        })
        .collect()
}

/// The functions a body can reach through closures. Rust types a closure by its enum, and
/// `let cN: FnK = ..; cN.apply(..)` can reach only the variants of `FnK`; an application
/// whose receiver is not declared that way is read as reaching every closure of the package.
fn applied_by(
    body: &str,
    applied: &BTreeMap<String, BTreeSet<String>>,
    every: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = body;
    while let Some(at) = rest.find(".apply(") {
        let receiver: String = rest[..at]
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let declared = format!("let {receiver}: ");
        let kind = body.find(&declared).map(|i| {
            body[i + declared.len()..]
                .chars()
                .take_while(char::is_ascii_alphanumeric)
                .collect::<String>()
        });
        match kind.and_then(|k| applied.get(&k)) {
            Some(targets) => out.extend(targets.iter().cloned()),
            None => out.extend(every.iter().cloned()),
        }
        rest = &rest[at + ".apply(".len()..];
    }
    out
}

/// A call cycle of the graph, if there is one: the names along it.
fn find_cycle(graph: &BTreeMap<String, BTreeSet<String>>) -> Option<Vec<String>> {
    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Open,
        Done,
    }
    let mut marks: BTreeMap<&str, Mark> = BTreeMap::new();
    for start in graph.keys() {
        if marks.contains_key(start.as_str()) {
            continue;
        }
        // an explicit stack of (function, callees still to visit): the check itself
        // must not recurse on a graph it is showing to be acyclic.
        let mut stack: Vec<(&str, Vec<&str>)> =
            vec![(start, graph[start].iter().map(String::as_str).collect())];
        marks.insert(start, Mark::Open);
        while let Some((name, todo)) = stack.last_mut() {
            let name = *name;
            match todo.pop() {
                None => {
                    marks.insert(name, Mark::Done);
                    stack.pop();
                }
                Some(next) => match marks.get(next) {
                    Some(Mark::Open) => {
                        let mut cycle: Vec<String> = stack
                            .iter()
                            .map(|(n, _)| (*n).to_owned())
                            .skip_while(|n| n != next)
                            .collect();
                        cycle.push(next.to_owned());
                        return Some(cycle);
                    }
                    Some(Mark::Done) => {}
                    None => {
                        let Some(callees) = graph.get(next) else {
                            continue;
                        };
                        marks.insert(next, Mark::Open);
                        stack.push((next, callees.iter().map(String::as_str).collect()));
                    }
                },
            }
        }
    }
    None
}

#[test]
fn no_function_of_the_generated_package_is_on_a_call_cycle_lc_16() {
    let graph = call_graph(lifecycle_oracle::GENERATED_SOURCE);
    assert!(
        graph.len() > 300,
        "the graph reads the package: {} functions",
        graph.len()
    );
    assert!(graph.contains_key("f0"), "the root is in the graph");
    if let Some(cycle) = find_cycle(&graph) {
        panic!(
            "the package recurses: {} (a function whose self calls are all tail calls is a loop; \
             a call that is not one grows the stack with the input)",
            cycle.join(" -> ")
        );
    }
}

#[test]
fn the_call_cycle_check_finds_a_cycle_and_a_self_call_lc_16() {
    let source = "pub fn f1(a: u64) -> u64 {\n    f2(a)\n}\n\npub fn f2(a: u64) -> u64 {\n    f1(a)\n}\n\npub fn f3(a: u64) -> u64 {\n    f3(a)\n}\n\npub fn f4(a: u64) -> u64 {\n    a\n}\n";
    let graph = call_graph(source);
    assert_eq!(graph.len(), 4);
    let cycle = find_cycle(&graph).expect("a cycle");
    assert!(cycle.len() >= 2, "{cycle:?}");
    let acyclic =
        "pub fn f1(a: u64) -> u64 {\n    f2(a)\n}\n\npub fn f2(a: u64) -> u64 {\n    a\n}\n";
    assert!(find_cycle(&call_graph(acyclic)).is_none());
    let through_apply = "pub enum Fn0 {\n    F2,\n}\n\nimpl Fn0 {\n    pub fn apply(&self, p0: u64) -> u64 {\n        match self {\n            Fn0::F2 => f2(p0),\n        }\n    }\n}\n\npub fn f2(a: u64) -> u64 {\n    let c: Fn0 = Fn0::F2;\n    c.apply(a)\n}\n";
    assert!(
        find_cycle(&call_graph(through_apply)).is_some(),
        "a cycle through a function value is a cycle"
    );
    // an application of a closure of one type does not reach the closures of another
    let other_type = "pub enum Fn0 {\n    F2,\n}\n\nimpl Fn0 {\n    pub fn apply(&self, p0: u64) -> u64 {\n        match self {\n            Fn0::F2 => f2(p0),\n        }\n    }\n}\n\npub enum Fn1 {\n    F3,\n}\n\nimpl Fn1 {\n    pub fn apply(&self, p0: u64) -> u64 {\n        match self {\n            Fn1::F3 => f3(p0),\n        }\n    }\n}\n\npub fn f2(a: u64) -> u64 {\n    let c: Fn1 = Fn1::F3;\n    c.apply(a)\n}\n\npub fn f3(a: u64) -> u64 {\n    a\n}\n";
    assert!(
        find_cycle(&call_graph(other_type)).is_none(),
        "a closure is applied only to what its own type can hold"
    );
}
