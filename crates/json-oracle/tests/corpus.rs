//! JC-03, JC-04, JC-05, JC-11: the codec against the pinned JSONTestSuite corpus.
//!
//! Every case is run through the package generated from the LexLean codec. The
//! expectations for `n_` and `i_` are recorded tables, not predicates, so a change
//! of diagnostic is a visible change of the table.

mod common;

use common::{kind, table};
use json_oracle::{parse, Parse};

/// The outcome of a parse as the tables spell it.
fn outcome(result: &Parse) -> String {
    match result {
        Parse::Value(_) => "accepted".to_owned(),
        Parse::Refused(d) => format!("refused:{}", d.kind.name()),
        Parse::Unwritable(d) => format!("unwritable:{}", d.kind.name()),
    }
}

#[test]
fn every_y_case_is_accepted_jc_03() {
    let cases = kind("y_");
    assert_eq!(cases.len(), 95, "the pinned corpus has 95 y_ files");
    let failures: Vec<String> = cases
        .iter()
        .filter_map(|(name, bytes)| match parse(bytes) {
            Parse::Value(_) => None,
            other => Some(format!("{name}: {}", outcome(&other))),
        })
        .collect();
    assert!(
        failures.is_empty(),
        "y_ cases not accepted:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_n_case_is_refused_with_its_recorded_kind_jc_04() {
    let cases = kind("n_");
    assert_eq!(cases.len(), 188, "the pinned corpus has 188 n_ files");
    let recorded = table("n_refusals.tsv");
    let names: Vec<&str> = cases.iter().map(|(n, _)| n.as_str()).collect();
    let rows: Vec<&str> = recorded.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        rows, names,
        "the n_ table lists exactly the n_ files, in order"
    );
    let mut wrong = Vec::new();
    for ((name, bytes), (_, expected)) in cases.iter().zip(&recorded) {
        let got = outcome(&parse(bytes));
        if !got.starts_with("refused:") {
            wrong.push(format!("{name}: not refused ({got})"));
        } else if &got != expected {
            wrong.push(format!("{name}: expected {expected}, got {got}"));
        }
    }
    assert!(wrong.is_empty(), "n_ cases:\n{}", wrong.join("\n"));
}

#[test]
fn every_i_case_matches_the_recorded_table_jc_05() {
    let cases = kind("i_");
    assert_eq!(cases.len(), 35, "the pinned corpus has 35 i_ files");
    let recorded = table("i_outcomes.tsv");
    let names: Vec<&str> = cases.iter().map(|(n, _)| n.as_str()).collect();
    let rows: Vec<&str> = recorded.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        rows, names,
        "the i_ table lists exactly the i_ files, in order"
    );
    let wrong: Vec<String> = cases
        .iter()
        .zip(&recorded)
        .filter_map(|((name, bytes), (_, expected))| {
            let got = outcome(&parse(bytes));
            (&got != expected).then(|| format!("{name}: expected {expected}, got {got}"))
        })
        .collect();
    assert!(wrong.is_empty(), "i_ cases:\n{}", wrong.join("\n"));
}

#[test]
fn accepted_corpus_texts_reach_a_canonical_fixed_point_jc_11() {
    let mut checked = 0;
    for prefix in ["y_", "i_"] {
        for (name, bytes) in kind(prefix) {
            let Parse::Value(canonical) = parse(&bytes) else {
                continue;
            };
            // Encoding the parsed value and parsing that text gives a value
            // that encodes to the same bytes.
            match parse(&canonical) {
                Parse::Value(again) => {
                    assert_eq!(again, canonical, "{name}: canonical bytes moved")
                }
                other => panic!("{name}: the canonical text is refused: {}", outcome(&other)),
            }
            checked += 1;
        }
    }
    assert!(checked >= 95, "only {checked} accepted texts were checked");
}
