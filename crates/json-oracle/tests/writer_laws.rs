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
        if axioms.is_empty() {
            // The exact empty set: the theorem states no `axioms` member.
            assert!(
                !text.contains("axioms:"),
                "theorem `{name}` in src/{file} declares axioms; the model records none"
            );
        } else {
            let line = format!("  axioms: [{axioms}]\n}}");
            assert!(
                text.ends_with(&line),
                "theorem `{name}` in src/{file} does not end with `axioms: [{axioms}]`"
            );
        }
    }
    // A theorem is checked only if it carries a proof the kernel replays: no
    // placeholder stands in for one.
    assert!(
        !source.contains("sorry"),
        "a placeholder proof in src/{file}"
    );
}

/// The theorems of `JsonWriteLaws` about the encoder's work loop, with their
/// axiom policies: the headline theorems, then the lemmas they are built from.
const WRITER_LAWS: &[(&str, &str)] = &[
    ("loop_step", "Quot.sound, propext"),
    ("continue_advanced", "Quot.sound, propext"),
    ("value_step_number", "Quot.sound, propext"),
    ("continue_number", "Quot.sound, propext"),
    ("value_step_array", "Quot.sound, propext"),
    ("value_step_object", "Quot.sound, propext"),
    ("continue_array", "Quot.sound, propext"),
    ("continue_object", "Quot.sound, propext"),
    ("failure_or", ""),
    ("orElse_gate", ""),
    ("orElse_number", ""),
    ("failure_gate", ""),
    ("failure_number", ""),
    ("orElse_none", ""),
    ("failure_none_passes", ""),
    ("failure_some_stops", ""),
    ("plus_assoc", "propext"),
    ("reverse_cat", "propext"),
    ("writer_loop_agrees_with_outline", "Quot.sound, propext"),
    ("jsons_append", "propext"),
    ("members_append", "propext"),
    ("members_relabel", "propext"),
    ("forest_encoding_is_faithful", "propext"),
    ("write_with_finalize", "Quot.sound, propext"),
    ("finalize_failure", ""),
    ("reverse_reverse", "propext"),
    ("loop_finished", "Quot.sound, propext"),
    ("writer_agrees_with_outline", "Quot.sound, propext"),
    ("writer_agrees_on_every_value", "Quot.sound, propext"),
];

/// The headline theorems of JC-15, which must be among those declared.
const HEADLINE: &[&str] = &[
    "forest_encoding_is_faithful",
    "writer_loop_agrees_with_outline",
    "writer_agrees_with_outline",
    "writer_agrees_on_every_value",
];

#[test]
fn the_encoder_agrees_with_its_specification_on_every_value_jc_15() {
    for name in HEADLINE {
        assert!(
            WRITER_LAWS.iter().any(|(declared, _)| declared == name),
            "the model does not list the headline theorem `{name}`"
        );
    }
    declared("JsonWriteLaws.lex.tex", WRITER_LAWS);
}

/// The theorems of `JsonWriteLaws` about the bytes the encoder writes.
const WRITTEN_BYTES: &[(&str, &str)] = &[
    ("escape_every_control_byte", ""),
    ("escape_quote_and_backslash", ""),
    ("escape_leaves_other_bytes", ""),
    ("encoder_keeps_item_order", "Quot.sound, propext"),
];

#[test]
fn the_encoder_escapes_every_control_byte_and_keeps_item_order_jc_16() {
    declared("JsonWriteLaws.lex.tex", WRITTEN_BYTES);
}
