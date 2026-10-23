//! JC-15 and JC-16: the theorems about the encoder and about writing and reading
//! again, which the kernel checks when `lexlean verify` runs. This test does not
//! replace that check; it fails when a theorem the model claims is not declared
//! in its module with the axiom policy the model records, or when any proof is a
//! placeholder.

use std::path::Path;

/// The source of a module of the LexLean project.
fn module(file: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../src")
        .join(file);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is unreadable: {e}", path.display()))
}

/// The text of the theorem `name` in `source`, from its header to the closing brace.
fn theorem<'a>(source: &'a str, file: &str, name: &str) -> &'a str {
    let header = format!("theorem{{\n  name: {name},\n");
    let start = source
        .find(&header)
        .unwrap_or_else(|| panic!("src/{file} declares no theorem `{name}`"));
    let rest = &source[start..];
    let end = rest.find("\n}\n").map_or(rest.len(), |at| at + 2);
    &rest[..end]
}

/// Every theorem of `claimed` is declared in `file` with exactly the axioms listed.
fn declared(file: &str, claimed: &[(&str, &str)]) {
    let source = module(file);
    for (name, axioms) in claimed {
        let text = theorem(&source, file, name);
        let line = format!("  axioms: [{axioms}]\n}}");
        assert!(
            text.ends_with(&line),
            "theorem `{name}` in src/{file} does not end with `axioms: [{axioms}]`"
        );
    }
    // A theorem is checked only if it carries a proof the kernel replays: no
    // placeholder stands in for one.
    assert!(
        !source.contains("sorry"),
        "a placeholder proof in src/{file}"
    );
}

/// The theorems of `JsonWriteLaws`, with their axiom policies.
const WRITER_LAWS: &[(&str, &str)] = &[
    ("forest_encoding_is_faithful", "propext"),
    ("writer_loop_agrees_with_spec", "propext"),
    ("writer_agrees_on_every_value", "Quot.sound, propext"),
];

#[test]
fn the_encoder_agrees_with_its_specification_on_every_value_jc_15() {
    declared("JsonWriteLaws.lex.tex", WRITER_LAWS);
}

/// The instance round trips of `JsonRoundTrip`, with their axiom policies.
const ROUND_TRIPS: &[(&str, &str)] = &[
    (
        "round_trip_string_escapes",
        "Classical.choice, Quot.sound, propext",
    ),
    (
        "round_trip_non_ascii",
        "Classical.choice, Quot.sound, propext",
    ),
    (
        "round_trip_empty_member_name",
        "Classical.choice, Quot.sound, propext",
    ),
    (
        "round_trip_member_order",
        "Classical.choice, Quot.sound, propext",
    ),
    (
        "round_trip_repeated_member_name",
        "Classical.choice, Quot.sound, propext",
    ),
];

#[test]
fn writing_and_reading_again_returns_the_value_for_escapes_and_member_order_jc_16() {
    declared("JsonRoundTrip.lex.tex", ROUND_TRIPS);
}
