# JSON text codec

The codec reads and writes JSON text (RFC 8259) as bytes. It is LexLean source in
language 1.2 over the existing `Json` type in `src/Json.lex.tex`, which is
unchanged; so are Dispatch and Validation. Issue #18; register rows JC-01 to JC-12.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Utf8.lex.tex` | RFC 3629 validator and its theorems |
| `src/JsonText.lex.tex` | the typed refusals (`JsonRefusal`, fifteen kinds) and the bytes/list bridge |
| `src/JsonRead.lex.tex` | the parser: a state machine over bytes with an explicit frame stack |
| `src/JsonWrite.lex.tex` | the encoder: a work list, no recursion on `Json` |
| `src/JsonCodec.lex.tex` | the production root `codecRun` |
| `src/JsonGate.lex.tex`, `src/JsonExamples.lex.tex`, `src/JsonRoundTrip.lex.tex` | kernel-checked theorems |
| `crates/json-oracle/generated/mcp_json_codec` | LexLean's export of `codecRun`, unedited |
| `crates/json-oracle/corpus/JSONTestSuite-1ef36fa01286573e846ac449e8683f8833c5b26a` | the vendored corpus |

Regenerate the package after any change to the sources (it verifies the project
first):

```
lexlean production build --request crates/json-oracle/generated/request.json
```

with that request naming root `UorMcp.JsonCodec.codecRun`, package
`mcp_json_codec` 1.0.0, target `rust-std`. Copy `Cargo.toml`, `package.json`,
`provenance.json` and `src/lib.rs` from `.lexlean/production/<id>/packages/rust-std/mcp_json_codec`
to `crates/json-oracle/generated/mcp_json_codec`. The lock was made with LexLean
`release/v0.4` at `9796067`.

## Decisions

| Question | Choice |
| --- | --- |
| Input | bytes; a byte string that is not well-formed UTF-8 (RFC 3629) is refused before any JSON is read |
| Byte order mark | refused as `unexpectedByte` at offset 0 (RFC 8259 section 8.1 lets a parser do so) |
| Lone surrogate escape | refused as `loneSurrogate`; a `Json` text is a `String` of scalar values, so it has no value to decode to (RFC 8259 section 8.2) |
| Surrogate pair escape | one code point |
| Raw control character in a string | refused as `controlInString` |
| Numbers | the section 6 grammar, mapped exactly to `Decimal {mantissa, exponent}`: the digits are the mantissa, the exponent is the written exponent less the fraction digits |
| Number bound | at most 18 significant digits (leading zeros do not count) and an effective exponent in -9999..9999, else `numberOutOfRange`; the encoder refuses the same bounds as `encodeNumberOutOfRange` |
| Negative zero | the mantissa is an `Int`, so `-0` is `0`; the sign is not kept |
| Nesting bound | 512 levels of array or object; 513 is `tooDeep`. The parser has no recursion and no fuel, so depth is a declared bound and not a stack limit |
| Duplicate member names | all kept, in source order, in `Json.object`'s member list; `Json.lookup` answers the first |
| Encoder output | compact; `"` and `\` escaped, `\b \t \n \f \r` short, other control characters `\u00xx` in lower case, everything else raw; a number is `<mantissa>` or `<mantissa>e<exponent>` |
| Encoder budget | 2^26 work steps, then `encodeBudget` |

## The root

`codecRun(mode, input)` takes a `Nat` and `Bytes` and returns a list of `Nat`, so the
export boundary is first-order data. Word 0 is the tag: 0 accepted (the rest is the
canonical encoding of the parsed value), 1 refused by the parser, 2 refused by the
encoder (words 1 and 2 are the refusal code and its offset), 3 no such member,
4 not an object.

| Mode | Does |
| --- | --- |
| 0 | parse, report the canonical text |
| 1 | the UTF-8 verdict alone |
| 2 | encode a synthetic value (nesting, digits, exponent, ASCII string, budget) |
| 3 | parse an object and report its first member named `a` |
| 4 | parse without the UTF-8 gate, to exercise `internalText` |

## Evidence

| ID | Evidence |
| --- | --- |
| JC-01 | theorems in `Utf8`; `utf8.rs`: about 880 000 byte strings against `std::str::from_utf8` and an independent Table 3-7 reference |
| JC-02 | `gate_refuses_every_invalid_input`; every one- and two-byte string through the parser |
| JC-03 to JC-05 | `corpus.rs` over the 318 files, with the recorded tables in `tables/` |
| JC-06 to JC-10 | `text.rs`; `JsonExamples`; `encoder_refuses_*` |
| JC-11 | `accepted_corpus_texts_reach_a_canonical_fixed_point_jc_11`; `JsonRoundTrip` |
| JC-12 | `provenance.rs` |

Commands:

```
lexlean fmt --check && lexlean lock --check && lexlean check && lexlean build && lexlean verify
RUSTUP_TOOLCHAIN=stable cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p xtask -- validate
```

## Planted defects

Each was planted alone in the sources of a clean tree, run, and reverted.

| Defect | Planted in | Gate | Command | Diagnostic |
| --- | --- | --- | --- | --- |
| a lone high surrogate is dropped instead of refused | `JsonRead` `wantSlash` | kernel | `lexlean verify` | `Tactic rfl failed` for `JsonCodec.parseReport (JsonRead.readList [34, 92, 117, 68, 56, 48, 48, 34])` against `[1, 6, 7]` |
| overlong leads C0 and C1 accepted | `Utf8` first lead test `byte <? 194` made `192` | kernel | `lexlean verify` | `omega could not prove the goal`, counterexample `192 <= byte <= 193`, in `bad_lead_refused` |
| a high surrogate followed by a non-surrogate escape is accepted | `JsonRead` `unitDone`, last branch | oracle | `cargo test -p json-oracle` | `i_string_1st_valid_surrogate_2nd_invalid.json: expected refused:loneSurrogate, got accepted` (`every_i_case_matches_the_recorded_table_jc_05`), `n_string_incomplete_surrogate_escape_invalid.json: expected refused:loneSurrogate, got refused:badEscape` (`..._jc_04`), and `a_lone_surrogate_is_refused_as_lone_surrogate_jc_06` |
| the third byte of a four-byte sequence may be up to 255 | `Utf8` phase three bound `191` made `255` | oracle | `cargo test -p json-oracle` | `class of [f0, 90, c0]`: left `"truncated"`, right `"invalid"` (`the_validator_agrees_with_table_3_7_and_the_standard_library_jc_01`) |
| the solidus escape is dropped | `JsonRead` `escapeByte` | oracle | `cargo test -p json-oracle` | `y_string_allowed_escapes.json: refused:badEscape` (`every_y_case_is_accepted_jc_03`) and `string_escapes_decode_and_encode_jc_06` |
| the generated package is edited by hand | `generated/.../src/lib.rs` | provenance | `cargo test -p json-oracle --test provenance` | `src/lib.rs is not the file the export recorded: regenerate it, never edit it` |
| a corpus file is altered | `test_parsing/y_structure_lonely_null.json` | pin and checksum | same | `the vendored files are not the files the authority records` |
| a test is renamed so no test ends in `jc_11` | `tests/corpus.rs` | meta-gate | `cargo test -p repo-conformance` | `CM-02: JC-11 is registered but no test name ends in jc_11` |

The first two were stopped by `lexlean verify` before an export was possible; the next three
pass `verify` because no theorem covers them (a successful string parse cannot be
kernel-evaluated) and are stopped by the oracle after `lexlean production build` re-exported the package.

## Not met

* The universal statement "for every `Json` value that encodes, parsing the encoding returns
  it" is not proved. `Json` is nested through `List` and LexLean 1.2 has no induction
  principle over such a type, so the encoder and parser are machines over lists and a
  proof by induction on the value has nothing to induct on. What is checked: the property
  as kernel-checked instances (eight values, and all `Bool`), and as a canonical fixed point
  over every accepted corpus text. The precondition of the instances is that the encoder
  accepts the value (number and depth bounds), which the tests exercise on both sides.
* A successful parse of a string is not kernel-evaluated: the decoder uses the opaque
  `utf8_decode`, which the kernel cannot reduce. String success is covered by the oracle.
* The gate theorem fixes the refusal kind, not the offset; the offset is checked by tests.
* The Utf8 theorems are universal for the lead byte and second byte and for complete
  two-byte sequences; complete three- and four-byte sequences are checked by the differential
  test and not by a theorem.
* `cargo deny` is not installed in this environment, so `just vv`'s `deny` step was not run.
  The prismpm steps (`prismpm.lock`, `template check`) cannot run without the SDK.

## Findings about the template's rules for a generated package

* `deny.toml`: the package has no dependencies, so it adds nothing to the graph. It is
  not a workspace member and nothing depends on it; it is compiled as a module of the
  dev-only `json-oracle` crate.
* `xtask audit-limits` (R5) rejects `Result<` in the `src/` of any crate that is not
  `publish = false`. Every fallible generated function returns `R<T>`, so the package could
  not live in a shipped crate. The oracle crate is `publish = false`.
* `xtask audit-deferral` (R4) scans every `.rs`, `.md` and `.toml` under `crates/`,
  generated files included. The generated source contains none of its markers.
* The package is not `rustfmt`-clean, so the include carries `#[rustfmt::skip]`. Its own
  manifest sets `warnings = deny` and `clippy::all = deny` with ten named exceptions and it
  is clean under them as a crate; included as a module it inherits the workspace table, so
  the include names those ten exceptions plus `dead_code`, `unused` and `missing_docs`.
* `crates/model/src/registry.rs` documents an authority as "never re-derived, vendored, or
  gated on", and `model/authorities.toml` allows a checksum "over the committed artifact".
  The issue asks for the corpus to be vendored at a pin, so `A-JSONTESTSUITE` records a
  checksum of the vendored manifest and the Git tree of the pinned directory.
* `xtask validate` runs `check-model`, `audit-limits`, `audit-deferral` and
  `audit-dependency-updates`, then stops at `bootstrap::audit`: `prismpm.lock is required`.
  This is the expected state until the SDK binding.
