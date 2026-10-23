//! Writes the two recorded tables the corpus tests compare against:
//! `tables/n_refusals.tsv` and `tables/i_outcomes.tsv`.
//!
//! Run from the repository root with
//! `cargo run -p json-oracle --example record_tables`. A recorded row is a
//! reviewed decision: read the diff of the tables before committing it.

use std::fmt::Write as _;
use std::path::Path;

use json_oracle::{parse, Parse};

fn outcome(result: &Parse) -> String {
    match result {
        Parse::Value(_) => "accepted".to_owned(),
        Parse::Refused(d) => format!("refused:{}", d.kind.name()),
        Parse::Unwritable(d) => format!("unwritable:{}", d.kind.name()),
    }
}

fn main() {
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join("JSONTestSuite-1ef36fa01286573e846ac449e8683f8833c5b26a")
        .join("test_parsing");
    let mut names: Vec<String> = std::fs::read_dir(&corpus)
        .expect("the vendored corpus is present")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for (prefix, file, what) in [
        (
            "n_",
            "n_refusals.tsv",
            "the refusal each must-reject case receives",
        ),
        (
            "i_",
            "i_outcomes.tsv",
            "the outcome of each implementation-defined case",
        ),
    ] {
        let mut out = format!(
            "# JSONTestSuite 1ef36fa01286573e846ac449e8683f8833c5b26a test_parsing: {what}.\n\
             # Recorded by `cargo run -p json-oracle --example record_tables`; one reviewed row per case.\n"
        );
        for name in names.iter().filter(|n| n.starts_with(prefix)) {
            let bytes = std::fs::read(corpus.join(name)).expect("a corpus file reads");
            writeln!(out, "{name}\t{}", outcome(&parse(&bytes))).expect("a string write");
        }
        std::fs::write(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tables")
                .join(file),
            out,
        )
        .expect("the table is written");
    }
}
