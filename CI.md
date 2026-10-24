# Continuous integration without the SDK

`.github/workflows/lexlean.yml` runs every gate of `just vv` that does not need
the PrismPM SDK image, with the compiler and the production exports the SDK image
does not run. The template's `bootstrap` workflow is unchanged and remains the
acceptance boundary (`VERIFICATION.md`): it runs `just vv` inside the locked SDK
image (`prismpm.lock`, MCP#4) and is green on `main`, so it must be green on a pull
request. This workflow does not replace it and is not a reduced path to release.

The SDK image's `just vv` runs `cargo test --workspace` with the pinned suite absent
(the template's locked-dependency rule R6 forbids fetching it there), so no test of
the workspace may need the installed conformance suite, and none may need node
beyond what the SDK image supplies to `xtask`. The tests that run a node oracle or
read the installed suite are `#[ignore]`d with a reason naming the conformance job;
that job runs them with `cargo test -p lifecycle-oracle --test conformance --
--ignored` (`run_oracle_tests`) and is green only when as many passed as the source
ignores (CF-08).

## Jobs and gates

| Job | Gate |
| --- | --- |
| `cargo / fmt clippy audits deny test` | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo check --workspace --all-features --all-targets --locked`; `cargo xtask check-model`, `audit-limits`, `audit-deferral` and `audit-dependency-updates`; `cargo deny --all-features check`; `cargo test --workspace --locked --no-fail-fast`, run as the SDK gate runs it, with no installed conformance suite |
| `lexlean / verify` | `lexlean fmt --check`, `lexlean lock --check`, `lexlean check`, `lexlean verify` |
| `lexlean / export <crate>` | one job per `crates/*/generated/request.json`, found by `find` and never listed: `lexlean production build --request <it>` in a clean `.lexlean/production`, which first compares the request's `expect` pins with the project (`LLC0204`, before any Lean process runs) and its target digests with the compiler's (`LLC0203`); then every exported `rust-std` package is compared with `diff -r` against the package committed beside the request, in both directions (a committed package the request no longer exports, or an export nobody committed, fails by name) |
| `prepare / compiler and toolchains` | builds `lexlean`, installs the Lean toolchain, checks that the compiler is the one `lexlean.lock` was made with, and publishes the job matrix |

## Not run here

- `cargo xtask audit-bootstrap` (and so `xtask validate`), `prismpm template check`
  and `prismpm lock check`; `just vv` runs them in the SDK image.
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

## The conformance job and the PrismPM pin

`conformance / server suite and schema oracle` builds the conformance server
application (`conformance/`, [CONFORMANCE-RUN.md](CONFORMANCE-RUN.md)) and runs the MCP
project's own oracles over it. Its steps, in order: `prismpm` at `.github/prismpm.rev`;
the copies of PrismPM's files equal the files at that revision (`sdk_copies_agree`);
`prismpm build`; the core is the one `conformance/core-identity.json` records; the
consumer workspace builds and its binding passes `cargo test`, `cargo fmt` and `cargo
clippy`; the pinned suite is installed (`conformance/oracle/fetch-suite.sh`, `npm ci`);
the oracle scripts pass their own tests; the workspace tests that need node and the
suite run with `--ignored` (`run_oracle_tests`); the suite's server runner is run over
loopback; the schema oracle validates the recorded transcript against the pinned
2026-07-28 `schema.json`; the per-scenario table is written to the step summary.

**The server-suite step is exact, not lenient.** It passes only when the set of failing
scenarios equals the set `conformance/waiting.json` lists as waiting, each entry naming
its issue and pull request (MCP#10 and PR #30, MCP#12 and PR #32). A failing scenario
that is not listed fails the job (`FAILING and not named in waiting.json`, first in the
annotation); a listed scenario that passes fails the job (`named as waiting but not
failing`, so the entry is deleted in the pull request that delivers it); an entry that
names neither an issue nor a pull request fails the job. The waiting scenarios are
failures in the report, the step summary and `report.json` (`valid: false`), and the
required scenarios passing are counted without them (27 of 37 today); the list is a
classification of known failures, never a pass. A scenario the suite itself skips at the
pin (`suite_skipped`) is reported as skipped and never counted as passing. A red job is
therefore always a regression, a delivery that left a stale entry, or a broken oracle,
and a merge queue on `main` is not blocked by a known failure.

`.github/prismpm.rev` is the only place the PrismPM revision is written. It names
PrismPM#339 at its head, and it moves to the PrismPM SDK release before MCP v0.1.0.

**One LexLean pin drives both tools.** PrismPM vendors its own LexLean (`cee0fbade` at
#339's head), which cannot read this repository's sources and has no `nested_induction`.
Development imports commits, so `install_prismpm` builds `prismpm` from a copy of the PrismPM
tree at the pinned revision whose `vendor/lexlean` is the package `cargo package` makes of
the LexLean tree at `.github/lexlean.rev`. Nothing in PrismPM changes, `Cargo.lock` is
PrismPM's own (`--locked`), and the binary is accepted only when `$PRISMPM_ROOT/pins` names
both revisions, which are also the cache key. The profile is debug, because PrismPM's release
profile is killed for memory on a 7 GB runner. The compiler that certifies the core is then the
compiler `lexlean.lock` was made with, and CI checks it twice: `lexlean.conformance.lock` and
`lexlean.lock` record the same `compiler_semantics`, and the production result `prismpm build`
writes names that same compiler (`conformance/oracle/identity.mjs`).

**Release.** MCP v0.1.0 pins a PrismPM SDK release, and that release must pin a LexLean release
that contains SM-37. Until then the pair above is a development pin.

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
head of a throwaway pull request (#29, based on this branch, closed unmerged; its branch `claude/plant-hand-edited-package` is still on the remote, delete it).

| | |
| --- | --- |
| failing step | `lexlean / export json-oracle`, step "5. the committed packages are the export of their sources" |
| diagnostic | `export of crates/json-oracle/generated/request.json failed (exit 1)`, then `== crates/json-oracle/generated/mcp_json_codec differs from the export of crates/json-oracle/generated/request.json ==` and the `diff -ru` hunk with ` // hand edit` on the committed side and an empty line on the export's |
| also failing | `cargo test`: `error: 1 target failed: -p json-oracle --test provenance` |
| passing | `lexlean / verify` |
| run | commit `d2fca58`, run 39953428770, jobs 120415836688 (export, 12 min 1 s) and 120413275623 (cargo) |
