# JSON text codec

The codec reads and writes JSON text (RFC 8259) as bytes. It is LexLean source in
language 1.2 over the existing `Json` type in `src/Json.lex.tex`, which is
unchanged; so are Dispatch and Validation. Issue #18; register rows JC-01 to JC-12, and
JC-13 and JC-14 from issues #23 and #18.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Utf8.lex.tex` | RFC 3629 validator and its theorems, for one- and two-byte sequences and, by lead-byte class of Table 3-7, for complete three- and four-byte sequences |
| `src/JsonText.lex.tex` | the typed refusals (`JsonRefusal`, fifteen kinds) and the bytes/list bridge, `listToBytes`, a binary-counter merge of blocks (see "Reading a string") |
| `src/JsonRead.lex.tex` | the parser: a state machine over bytes with an explicit frame stack |
| `src/JsonWrite.lex.tex` | the encoder: a work list, no recursion on `Json` |
| `src/JsonCodec.lex.tex` | the production root `codecRun` |
| `src/JsonGate.lex.tex`, `src/JsonExamples.lex.tex`, `src/JsonRoundTrip.lex.tex` | kernel-checked theorems |
| `crates/json-oracle/generated/mcp_json_codec` | LexLean's export of `codecRun`, unedited |
| `crates/json-oracle/corpus/JSONTestSuite-1ef36fa01286573e846ac449e8683f8833c5b26a` | the vendored corpus |

Regenerate the pins and the package in the same commit as any change to the sources, the
lock or the pinned compiler (it verifies the project first):

```
cargo xtask export
```

The request `crates/json-oracle/generated/request.json` names root `UorMcp.JsonCodec.codecRun`,
package `mcp_json_codec` 1.0.0, target `rust-std`, and pins in `expect` the source ID, the
semantic ID and the SHA-256 of `lexlean.lock` it was exported from (SPEC section 24.7).
`cargo xtask export` writes those pins and the target digest, runs
`lexlean production build --request` and replaces the package in
`crates/json-oracle/generated/mcp_json_codec` with the export. The lock was made with the compiler
pinned in `.github/lexlean.rev` (LexLean PR #292 at `0b0c95d96`, on PR #284, on
`release/v0.4`; see [CI.md](CI.md)).

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

## Reading a string in linear time

`JsonText.listToBytes` turns the list of byte values the reader collected for a string (or a
member name) into `Bytes`. LexLean 1.2 has no primitive from a list to `Bytes` (the closed
primitive list of SPEC section 17.10 has `append`, `slice`, `index`, `length` and the rest, and
`append` copies both operands), so it used one `append` per byte, which is quadratic: in the
stress run of #22 a 768 KiB string took 41 s and a 2 MiB string 301 s. It now keeps a stack of
blocks and merges the way a binary counter carries (`pushBlock`: a new block of one byte
absorbs the top of the stack while the top has the same length, `gatherBlocks` is the loop over
the list, `joinBlocks` prepends the at most log2 n distinct-sized blocks that are left). Each
byte is copied once per level, so the cost is O(n log n) with n appends, and every self call is
still a tail call, so the package keeps its property of no recursion on the stack (JC-12,
LC-16). The same audit covered every other accumulating function of the codec, the encoder and
`Base64.groups`: the only append to the end of a growing accumulator in a loop was this one.

JC-13 states the bound and checks it through both exported roots, in the dev profile at
`opt-level = 1`. The read finishes within `0.5 s + 5 microseconds per byte of the text`, and a
read of the next length takes no more than twice the growth of the length times the time of the
one before (lengths grow 4 and then 2). A read still running at its bound is reported as a
failure without waiting for it.

| Shape | 256 KiB | 1 MiB | 2 MiB | Growth (length 4x, then 2x) |
| --- | --- | --- | --- | --- |
| string, codec root | 171 ms | 730 ms | 1.42 s | 4.3x, 1.9x |
| escapes and multi-byte characters, codec root | 106 ms | 504 ms | 891 ms | 4.7x, 1.8x |
| member name, codec root | 177 ms | 770 ms | 1.48 s | 4.4x, 1.9x |
| string argument of a `tools/call`, lifecycle root | 140 ms | 449 ms | 893 ms | 3.2x, 2.0x |
| argument name of a `tools/call`, lifecycle root | 141 ms | 489 ms | 964 ms | 3.5x, 2.0x |
| the validator alone (no bytes built) | 9 ms | 36 ms | 99 ms | |
| bound | 1.81 s | 5.74 s | 10.99 s | |

Control: the package of #20 (`c4b3d67`), swapped in with the same tests and the same bound, fails
them. The member name and the plain string are not read in 1.81 s at 256 KiB in the codec root
(`not finished within the bound of 1.81075s`), the string with escapes reads 256 KiB in 0.57 s and
is not finished at 1 MiB within 5.74 s, and through the lifecycle root both a string argument and an
argument name of 256 KiB are not answered within 1.81 s. The earlier figures of that package:
768 KiB in 41 s and 2 MiB in 301 s (#22), and, with the first looser bound of 20 microseconds a
byte, 1 MiB of the escapes shape in 20.1 s. The growth of the old reads is the square of the
growth of the length (a quadratic read takes 4 times as long when the string is twice as long and
16 times as long when it is four times as long); the checked growth is at most twice the length's.

## Three- and four-byte sequences

The validator is `Utf8.utf8Scan`, a state machine over the byte list. `Utf8` states, for
each lead-byte class of Table 3-7, what it does to a three- or four-byte sequence, as
universal theorems over every byte value (all with exact axiom policies, `propext` and
`Quot.sound`, and `Classical.choice` for the two `..._zero_iff` facts):

| Class (lead byte) | Second byte | `lead_*_step` | sequence and complete theorems |
| --- | --- | --- | --- |
| E0 | A0..BF | `lead_e0_step` | `three_byte_e0_sequence`, `three_byte_e0_complete` |
| E1..EC | 80..BF | `lead_mid3_step` | `three_byte_mid3_*` |
| ED | 80..9F | `lead_ed_step` | `three_byte_ed_*` |
| EE..EF | 80..BF | `lead_hi3_step` | `three_byte_hi3_*` |
| F0 | 90..BF | `lead_f0_step` | `four_byte_f0_*` |
| F1..F3 | 80..BF | `lead_mid4_step` | `four_byte_mid4_*` |
| F4 | 80..8F | `lead_f4_step` | `four_byte_f4_*` |

* `lead_*_step`: for a lead byte in the class and any tail `t`, the scan of `lead :: t` in the
  ready state is the scan of `t` in the phase and bounds of the table row.
* `*_sequence`: for a lead byte in the class, continuation bytes `b2 b3 [b4]` and any `rest`, the
  scan of the sequence followed by `rest` is `if inRange b2 lo hi then (if inRange b3 128 191
  then ... scan rest ... else invalidCode (length (b3 :: ...))) else invalidCode (length (b2 :: ...))`
  with the row's `lo hi`. A sequence the table admits continues with `rest`; one it does not is
  refused with the code of the first byte it does not admit, so the verdict names that byte.
* `*_complete`: the same for the sequence alone, where the value is `0` or
  `invalidCode 1`, `2` or `3`.
* `three_byte_complete_zero_iff`, `four_byte_complete_zero_iff`: for all `lo hi` and all
  continuation bytes, that conditional is `0` exactly when every `inRange` flag holds.

Together: the validator accepts a complete three- or four-byte sequence exactly when every
byte is in the bounds of Table 3-7, and refuses each other one at the first byte outside them.

How the proof is built, and why it is built so. The first attempt proved each statement in
one `linear_arithmetic` step, unfolding `utf8Scan` over the whole sequence. That fails: with the
lead byte a variable, unfolding the ready state at each of three or four cells gives a nested
conditional with every branch of every cell (an ASCII lead recurses into the next cell, a
two-byte lead into the next, and so on), and `simp` stops with "maximum number of steps
exceeded" inside the `repeat' split` that `linear_arithmetic` lowers to, which reports nothing,
so `omega` sees a goal with unsplit conditionals and answers with a counterexample that is only
a branch it was never told was impossible. The theorems are therefore built from two cheap
facts: the step lemma, whose tail is a variable so only the ready state's ten branches unfold
(`linear_arithmetic`), and the equation after the lead byte, which holds by unfolding with a
known phase and no hypothesis, so `apply` of the step lemma at the tail `b2 :: b3 :: rest` proves
it by definitional unfolding. The statements are not weakened.

What language 1.2 does not give, so that the two facts stay two theorems: a proof form that
composes a conditional theorem with another (`simplify` rewrites by unconditional rules, a
conditional rule's hypothesis is discharged only by `simp` on the closed set, and a theorem
cannot have a hypothesis as a parameter, `LLP2001`; `apply` is one `exact`;
`linear_arithmetic` unfolds definitions and not theorems, SPEC 17.10 and 17.12). With one it
would be the single statement `scan [b1, b2, b3] = 0 <-> flags` for a variable lead byte.

## Evidence

| ID | Evidence |
| --- | --- |
| JC-01 | theorems in `Utf8`; `utf8.rs`: about 880 000 byte strings against `std::str::from_utf8` and an independent Table 3-7 reference |
| JC-13 | `stress.rs` (this crate) and `crates/lifecycle-oracle/tests/linear.rs`: strings of 256 KiB, 1 MiB and 2 MiB through both exported roots |
| JC-14 | the 23 theorems of `Utf8` listed under "Three- and four-byte sequences"; `utf8.rs`: every three-byte sequence, and every four-byte sequence with an edge byte in the third or fourth place, against Table 3-7 and the standard library |
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
| a quadratic append is put back: `listToBytes` appends one byte at a time again (#23) | `JsonText` `listToBytes` | oracle, after `lexlean production build` re-exported the package | `cargo test -p json-oracle --test stress` | a member name of 256 KiB `not finished within the bound of 1.81075s`; a plain string reads 256 KiB in 1.80 s (bound 1.81 s) and `reading 1048576 bytes was not finished within the bound of 5.74289s`; the same for the string with escapes at 1 MiB (`a_long_member_name_..._jc_13`, `a_long_string_is_read_..._jc_13`, `a_long_string_of_escapes_..._jc_13` fail; the validator alone passes) |
| a three-byte form is overlong: E0 accepts a second byte from 80 (`E0 80 80`) | `Utf8` the E0 lead arm `160` made `128` | kernel | `lexlean verify` (run in a project holding `src/Utf8.lex.tex` alone) | `omega could not prove the goal` in `table_3_7_second_byte` (counterexample `128 <= b2 <= 159`) and in `lead_e0_step` (the scan in phase two with bounds 160 and 128 differ) |
| the same, in the generated package | `generated/.../src/lib.rs`, the E0 bounds `(160u64, (191u64` made `128u64` by hand | oracle | `cargo test -p json-oracle --test utf8` | `the_validator_names_each_forbidden_form_jc_01`: left `Valid`, right `Invalid(1)`; `every_complete_three_byte_sequence_is_judged_as_table_3_7_judges_it_jc_14`: `[e0, 80, 00]` left `Invalid(2)`, right `Invalid(1)`; `the_generated_package_is_the_recorded_export_of_the_codec_root_jc_12` also fails because the file was edited |
| the third byte of a three-byte sequence may be up to 255 (no earlier theorem covers it) | `Utf8` the `Phase.two` arm's next bounds `128 191` made `128 255` | kernel | `lexlean verify` (same project) | `Type mismatch` in exactly the seven `*_sequence` theorems (four three-byte, three four-byte), whose statement the unfolded phase two no longer matches; no earlier theorem fails |
| the same, in the generated package | `generated/.../src/lib.rs`, the transition to phase one `(128u64, (191u64` made `255u64` by hand | oracle | `cargo test -p json-oracle --test utf8` | `class of [e0, a0, c0]`: left `"valid"`, right `"invalid"` (JC-01 and both JC-14 sweeps); the provenance test fails too |
| a corpus file is altered | `test_parsing/y_structure_lonely_null.json` | pin and checksum | same | `the vendored files are not the files the authority records` |
| a test is renamed so no test ends in `jc_11` | `tests/corpus.rs` | meta-gate | `cargo test -p repo-conformance` | `CM-02: JC-11 is registered but no test name ends in jc_11` |

The first two of the original rows were stopped by `lexlean verify` before an export was possible; the next
three passed `verify` because no theorem covered them (a successful string parse cannot be
kernel-evaluated) and were stopped by the oracle after `lexlean production build` re-exported the package.
The fourth of them, a four-byte bound, is now also caught by the theorems of #18's open item.
A defect the theorems stop cannot be exported (the export verifies first), so the oracle rows for the
Utf8 defects edit the generated source by hand to what the defective source would export; they show that
the oracle sweeps would also have stopped it, and that is all they show. The quadratic append passes
`verify` (no theorem is about time), so its row is a real re-export.

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
* The Utf8 theorems for complete three- and four-byte sequences are two statements per
  class and not one: the scan of the sequence is the nested conditional of the table's bounds
  (`*_sequence`, `*_complete`), and that conditional is zero exactly when every bound holds
  (`*_zero_iff`). The single theorem `scan [b1, b2, b3] = 0 <-> flags` for a variable lead
  byte in a class is not stated: it composes a theorem that has a hypothesis on the lead
  byte (`lead_*_step`) with another inside one proof, and the closed proof forms of language 1.2
  cannot do that (see "Three- and four-byte sequences"); a one-step `linear_arithmetic` proof
  of it exhausts `simp`'s step budget. Nothing was weakened to get around it.
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
