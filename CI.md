# Continuous integration without the SDK

`.github/workflows/lexlean.yml` runs every gate of `just vv` that does not need
the PrismPM SDK, so a pull request has real evidence before `prismpm.lock`
exists (#4). The template's `bootstrap` workflow is unchanged and remains the
acceptance boundary (`VERIFICATION.md`); until the SDK binding lands it fails at
"Run the template gate through the locked SDK" on every pull request and on
`main`. This workflow does not replace it and is not a reduced path to release.

## Jobs and gates

| Job | Gate |
| --- | --- |
| `cargo / fmt clippy audits deny test` | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo check --workspace --all-features --all-targets --locked`; `cargo xtask check-model`, `audit-limits`, `audit-deferral` and `audit-dependency-updates`; `cargo deny --all-features check`; `cargo test --workspace --locked --no-fail-fast` |
| `lexlean / verify` | `lexlean fmt --check`, `lexlean lock --check`, `lexlean check`, `lexlean verify` |
| `lexlean / export <crate>` | one job per `crates/*/generated/request.json`, found by `find` and never listed: `lexlean production build --request <it>` in a clean `.lexlean/production`, which first compares the request's `expect` pins with the project (`LLC0204`, before any Lean process runs) and its target digests with the compiler's (`LLC0203`); then every exported `rust-std` package is compared with `diff -r` against the package committed beside the request, in both directions (a committed package the request no longer exports, or an export nobody committed, fails by name) |
| `prepare / compiler and toolchains` | builds `lexlean`, installs the Lean toolchain, checks that the compiler is the one `lexlean.lock` was made with, and publishes the job matrix |

## Not run here

- `cargo xtask audit-bootstrap` (and so `xtask validate`) stops at
  "prismpm.lock is required", after `audit-dependency-updates` has run.
- `prismpm template check` and `prismpm lock check` need the SDK image.
- `cargo test --workspace` already includes the `repo-conformance` scenarios that
  `just bdd` runs.

These stay in `just vv` and in the `bootstrap` workflow.

## The LexLean pin

`.github/lexlean.rev` is the only place the LexLean revision is written. The
workflow builds `lexlean` at that commit with `cargo install --git ... --rev ...
--locked` and caches the result under a key that holds the revision. A cache
restored for another revision cannot pass, because the installed binary is
accepted only when cargo's own record (`.crates.toml`) names exactly that
revision.

`lexlean --version` prints the language-1.0 compiler-semantics ID, which a
language-1.2 lock does not record, so it is not the comparison. The step "The
compiler is the one lexlean.lock was made with" locks a copy of the committed
tree with the built binary and requires the copy's `compiler_semantics` to equal
the one in `lexlean.lock`; `lexlean lock --check` (LLC0102) then repeats the
comparison in the `verify` job. Move `.github/lexlean.rev` and `lexlean.lock` in
one commit, and run `cargo xtask export` (below) in it: a new compiler can change
the descriptor digest of a target as well as the packages it exports.

The pin names a commit that is not yet a LexLean release (the SM-37
`nested_induction` form, stacked on DF-19 in LexLean#292). The pin moves to a
LexLean release before an MCP release is cut.

Every third-party action is pinned by its full commit SHA, and the elan and
cargo-deny archives are pinned by version and SHA-256 and checked before they
are unpacked.

## Request pins and the one regeneration command

Every `crates/*/generated/request.json` pins what its packages were exported
from, in `expect` (SPEC section 24.7):

| field | the value it pins |
| --- | --- |
| `source_id` | the source ID `lexlean check` states for the tree |
| `semantic_id` | the semantic ID `lexlean check` states for the tree |
| `lock_sha256` | the SHA-256 of `lexlean.lock` |

and each requested target carries the descriptor digest `lexlean production
targets` states for it. `lexlean production build --request` refuses a request
whose pins the project no longer matches before it runs Lean: `LLC0204` for a
stale `expect`, `LLC0203` for a target digest that is not the compiler's. So a
source edit that is not followed by a re-export cannot pass, in the job that
builds and in the job that compares.

A commit that changes a `src/*.lex.tex`, a lexicon entry, `lexlean.toml`,
`lexlean.lock` or `.github/lexlean.rev` changes the pins and the packages in the
same commit. The one command is

```sh
cargo xtask export
```

which, with the `lexlean` of `.github/lexlean.rev` on `PATH` (or named by
`LEXLEAN`), rewrites the pins and target digests of every request from `lexlean
check`, `lexlean.lock` and `lexlean production targets`, runs `lexlean production
build --request` for each, and replaces the packages beside the request with the
export. It writes nothing else. The production build verifies the project first,
so a successful run is also a successful `lexlean verify`. The export job in CI
is the same build followed by the byte comparison.

## Reading a failure

The job log cannot always be downloaded. Each step runs its command through
`gate` (`.github/lexlean-ci/lib.sh`), which:

- on failure, writes the last 40 lines of the command's output as one
  `::error::` annotation titled `<gate> failed (exit N)`;
- on success, writes a `::notice::` with the gate's duration and its last line,
  so a gate that executed nothing cannot look like one that passed.

Read them with

```sh
gh api repos/UOR-Foundation/MCP/check-runs/<job id>/annotations
```

## Planted defects

Each was planted alone in a tree that passes, and reverted.

### A source edit without a re-export (#21)

One definition, `definition{name: plantedConstant, parameters: [], result: Nat, body: 7}`, added to
`src/JsonText.lex.tex` in the tree of commit `cd390ef` (consistent pins), and nothing re-exported.

| | |
| --- | --- |
| command | `lexlean production build --request crates/json-oracle/generated/request.json` |
| diagnostic | `error[LLC0204]: the request pins the source ID 7e3c5254...a2a1; the project's is b9de8e63...6f71`, exit 2, after 9 s: before any Lean process runs, so neither the build nor the comparison is reached |
| where CI stops it | `lexlean / export json-oracle` and `lexlean / export lifecycle-oracle`, step "5. the committed packages are the export of their sources": `export of <request> failed (exit 2)` with the `LLC0204` line as the tail of its output; `lexlean / verify` passes, since the edit is a valid definition |

The same stale pin is what `cargo xtask export` rewrites: after it, the same command builds.

### A hand-edited package (#21)

One comment, ` // hand edit`, appended to line 10 of
`crates/json-oracle/generated/mcp_json_codec/src/lib.rs` in the tree of commit `e821637`
(a blank line there, so the line becomes the comment). It changes no behavior. Pushed as the
head of a throwaway pull request (#29, based on this branch, closed unmerged, branch deleted).

| | |
| --- | --- |
| failing step | `lexlean / export json-oracle`, step "5. the committed packages are the export of their sources" |
| diagnostic | `export of crates/json-oracle/generated/request.json failed (exit 1)`, then `== crates/json-oracle/generated/mcp_json_codec differs from the export of crates/json-oracle/generated/request.json ==` and the `diff -ru` hunk with ` // hand edit` on the committed side and an empty line on the export's |
| also failing | `cargo test`: `error: 1 target failed: -p json-oracle --test provenance` |
| passing | `lexlean / verify` |
| run | commit `d2fca58`, run 39953428770, jobs 120415836688 (export, 12 min 1 s) and 120413275623 (cargo) |
