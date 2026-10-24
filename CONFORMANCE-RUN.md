# The conformance build and the oracle run

Issue 13. The core is accepted on the MCP project's own oracles: the server suite of
`modelcontextprotocol/conformance`, run over loopback against the served core, and the
2026-07-28 `schema.json`, run over every message of the transcripts of that run. This file
says what is built, how to reproduce the run, what the run says today, and what a planted
defect looks like. The IDs are CF-01 to CF-07 in `CONFORMANCE.md`; a passing scenario here
is evidence for what it checks, not a proof about the core.

## What is built

| Part | Where |
| --- | --- |
| The declaration, a PrismPM `ServerApplication` whose core is `UorMcp.Fixture.fixtureRun` | `conformance/src/Application.lex.tex`, generated from the vectors of the pinned scenarios: `cargo run -p lifecycle-oracle --example conformance_application -- --write` |
| The LexLean and PrismPM projects the declaration is built in | `lexlean.conformance.toml`, `lexlean.conformance.lock`, `prismpm.toml`; the main project (`lexlean.toml`) does not see them |
| The copies of PrismPM's `Foundation.Bytes`, `Foundation.Server.V1.Model` and the `prism.arch` lexicon | `conformance/src/Foundation`, `language/prism.arch`; byte for byte the files at the revision of `.github/prismpm.rev` (authority `A-PRISMPM-SERVER-APPLICATION`) |
| The binding: the host effect `tool.calls`, the count of `tools/call` requests in the last 60 s | `conformance/bindings/mcp-binding` |
| The consumer workspace: the generated core and host packages beside the binding | `conformance/Cargo.toml`; the three generated packages are not committed |
| The oracles | `conformance/oracle/`: `server-suite.mjs`, `recorder.mjs`, `verdict.mjs`, `schema-oracle.mjs`, `identity.mjs`, `fetch-suite.sh` |
| What is not a pass, and which pull request it waits on | `conformance/waiting.json` |
| The identity of the core | `conformance/core-identity.json` |

`prismpm build` (PrismPM at `.github/prismpm.rev`) certifies the core and generates three
packages: `mcp_conformance_core`, `mcp-conformance-host-api` and `mcp-conformance-host`.
`conformance/lay-out.sh` copies them into the consumer workspace, where `cargo build
--release` builds `mcp-conformance-host`. `mcp-conformance-host serve` listens on
`127.0.0.1` and prints `listening 127.0.0.1:PORT`.

The adapter enforces only what PrismPM's model states: loopback, the origins `localhost`,
`127.0.0.1` and `[::1]`, `POST`, the content type `application/json`, and the bounds. The
`Accept` field is not enforced, so what the core answers is what the suite sees.

## Reproduce

```sh
bash conformance/oracle/fetch-suite.sh "$SUITE"            # the pinned commit, then npm ci
( . .github/lexlean-ci/lib.sh && PRISMPM_ROOT="$PRISMPM" install_prismpm )   # PrismPM at .github/prismpm.rev, built with the LexLean of .github/lexlean.rev
PATH="$PRISMPM/bin:$PATH" prismpm build                    # prints: build published: <id>
bash conformance/lay-out.sh .prism/build/<id> production-result.json
(cd conformance && cargo build --release --locked)
node conformance/oracle/server-suite.mjs --suite-root "$SUITE" --out out \
    "$PWD/conformance/target/release/mcp-conformance-host" serve
node conformance/oracle/schema-oracle.mjs --suite-root "$SUITE" \
    crates/lifecycle-oracle/suite/mcp-spec-80041ab0a139a8f4fb3adbc99f7ae9113ea59671/schema/2026-07-28/schema.json out/transcript.json
node conformance/oracle/identity.mjs --result production-result.json --lock lexlean.conformance.lock \
    --package conformance/mcp_conformance_core \
    --reference crates/lifecycle-oracle/generated/mcp_lifecycle/provenance.json --check conformance/core-identity.json
```

The `conformance` job of `.github/workflows/lexlean.yml` runs these steps. The cargo job
installs the same suite so that the node tests behind CF-03 to CF-05 run in `cargo test`;
they fail, and do not skip, when `node` or the suite is missing.

## The verdict

The suite's runner exits 1 for a failing scenario and for a harness error alike, and exits 0
when a scenario it was asked for produced nothing, so its exit status is not the verdict.
`server-suite.mjs` reads `checks.json` of every scenario of the frozen requirements file
(`requirements/2026-07-28.yaml`, SHA-256 `ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57`;
the runner refuses any other file) and decides:

* a scenario passes when it produced a check, no check failed, and at least one succeeded;
* a scenario with no checks, or whose checks are all skipped or warnings, is a failure;
* every failing check is reported as `scenario: check id: message`, the suite's own text;
* the run fails (exit 1) while any scenario fails; the last lines of the diagnostic group the
  failing scenarios by the pull request that is to deliver what they need, from
  `conformance/waiting.json`. That file classifies; it never turns a failure into a pass, and
  a scenario in it that does not fail makes the run fail.

One scenario cannot pass on any server at this pin: `tasks-status-notifications` sends no
request and reports one `SKIPPED` check (`src/scenarios/server/tasks/notifications.ts`, pending a
`subscriptions/listen` rewrite). It is named in `conformance/waiting.json` under
`suite_skipped`, reported as `skipped by the suite`, not counted as passing, and held to the rule
above as soon as the suite stops skipping it.

## The schema oracle

`schema-oracle.mjs` validates every JSON-RPC message of `transcript.json` with the Ajv that
the suite locks, against `schema.json` at the pinned specification commit. A request or
notification is held to the definition its method names; a response to the definition of the
request it answers; a typed error to the definition of its code. Two kinds of message are
counted, not excused:

* A probe the server refused. The suite sends requests that are invalid on purpose (no
  `_meta`, an unknown method, a mistyped field). The server's JSON-RPC error answer is validated
  like every other response; the probe itself is not asserted valid. A request the server
  answers with a result is never a refused probe and must validate.
* A transport refusal. A 403, 406 or 415 whose body is not JSON-RPC is the HTTP refusal of a
  request before the core sees it (the DNS-rebinding probe). Any other body that is not JSON is
  a finding.

## What passes today

Run of the core of this commit (core digest below), spec 2026-07-28, 113 HTTP exchanges,
228 JSON-RPC messages, 67 refused probes, 1 transport refusal, every message valid.

Required: 27 of 37 pass. Not scored: 3 of 13 pass, 1 is skipped by the suite, 9 fail. Every failing scenario waits on #30 or #32.

| # | kind | scenario | result | checks | waits on |
|---|---|---|---|---|---|
| 1 | required | `server-stateless` | pass | 25 of 30 (5 skipped) |  |
| 2 | required | `completion-complete` | pass | 2 of 2 |  |
| 3 | required | `tools-list` | pass | 3 of 3 |  |
| 4 | required | `tools-call-simple-text` | pass | 2 of 2 |  |
| 5 | required | `tools-call-image` | pass | 2 of 2 |  |
| 6 | required | `tools-call-audio` | pass | 2 of 2 |  |
| 7 | required | `tools-call-embedded-resource` | pass | 2 of 2 |  |
| 8 | required | `tools-call-mixed-content` | pass | 2 of 2 |  |
| 9 | required | `tools-call-error` | pass | 2 of 2 |  |
| 10 | required | `tools-call-with-progress` | pass | 2 of 2 |  |
| 11 | required | `server-sse-multiple-streams` | pass | 1 of 2 (1 info) |  |
| 12 | required | `resources-list` | pass | 2 of 2 |  |
| 13 | required | `resources-read-text` | pass | 2 of 2 |  |
| 14 | required | `resources-read-binary` | pass | 2 of 2 |  |
| 15 | required | `resources-templates-read` | pass | 2 of 2 |  |
| 16 | required | `sep-2164-resource-not-found` | pass | 4 of 4 |  |
| 17 | required | `prompts-list` | pass | 2 of 2 |  |
| 18 | required | `prompts-get-simple` | pass | 2 of 2 |  |
| 19 | required | `prompts-get-with-args` | pass | 2 of 2 |  |
| 20 | required | `prompts-get-embedded-resource` | pass | 2 of 2 |  |
| 21 | required | `prompts-get-with-image` | pass | 2 of 2 |  |
| 22 | required | `dns-rebinding-protection` | pass | 2 of 2 |  |
| 23 | required | `caching` | pass | 8 of 8 |  |
| 24 | required | `input-required-result-basic-elicitation` | **fail** | 1 of 2 | #10 (PR #30) |
| 25 | required | `input-required-result-basic-sampling` | **fail** | 1 of 2 | #10 (PR #30) |
| 26 | required | `input-required-result-basic-list-roots` | **fail** | 1 of 2 | #10 (PR #30) |
| 27 | required | `input-required-result-request-state` | **fail** | 1 of 2 | #10 (PR #30) |
| 28 | required | `input-required-result-multiple-input-requests` | **fail** | 1 of 2 | #10 (PR #30) |
| 29 | required | `input-required-result-multi-round` | **fail** | 1 of 2 | #10 (PR #30) |
| 30 | required | `input-required-result-missing-input-response` | pass | 1 of 2 (1 warning) |  |
| 31 | required | `input-required-result-non-tool-request` | **fail** | 1 of 2 | #10 (PR #30) |
| 32 | required | `input-required-result-result-type` | **fail** | 1 of 2 | #10 (PR #30) |
| 33 | required | `input-required-result-unsupported-methods` | pass | 2 of 2 |  |
| 34 | required | `input-required-result-tampered-state` | **fail** | 1 of 2 | #10 (PR #30) |
| 35 | required | `input-required-result-capability-check` | **fail** | 1 of 2 | #10 (PR #30) |
| 36 | required | `input-required-result-ignore-extra-params` | pass | 1 of 2 (1 warning) |  |
| 37 | required | `input-required-result-validate-input` | pass | 3 of 3 |  |
| 38 | not scored | `tasks-lifecycle` | **fail** | 1 of 9 | #12 (PR #32) |
| 39 | not scored | `tasks-capability-negotiation` | **fail** | 1 of 5 | #12 (PR #32) |
| 40 | not scored | `tasks-wire-fields` | **fail** | 1 of 4 | #12 (PR #32) |
| 41 | not scored | `tasks-request-state-removal` | **fail** | 1 of 2 | #12 (PR #32) |
| 42 | not scored | `tasks-mrtr-input` | **fail** | 1 of 4 | #12 (PR #32) |
| 43 | not scored | `tasks-request-headers` | **fail** | 2 of 5 | #12 (PR #32) |
| 44 | not scored | `tasks-dispatch-and-envelope` | **fail** | 3 of 9 | #12 (PR #32) |
| 45 | not scored | `tasks-status-notifications` | skipped by the suite | 0 of 1 (1 skipped) |  |
| 46 | not scored | `tasks-required-task-error` | **fail** | 1 of 2 | #12 (PR #32) |
| 47 | not scored | `tasks-mrtr-composition` | **fail** | 1 of 2 | #12 (PR #32) |
| 48 | not scored | `json-schema-2020-12` | pass | 8 of 8 |  |
| 49 | not scored | `http-header-validation` | pass | 14 of 14 |  |
| 50 | not scored | `http-custom-header-server-validation` | pass | 10 of 10 |  |

Four passes need a caution. `input-required-result-missing-input-response` and
`input-required-result-ignore-extra-params` pass with one warning each, because the tool they
call is not listed yet and a refusal is an acceptable answer to those probes; they pass
vacuously until MCP#10 lands. `server-sse-multiple-streams` passes with an info: the core answers
JSON, and event streams are optional there. `server-stateless` passes with 5 of its 30 checks skipped by
the suite: the checks that need a `subscriptions/listen` stream. The suite skips them for a server that
advertises no `listChanged` or `subscribe` capability (`stateless.ts`, the comment above
`advertisesSubscriptions`), and the specification schema makes those capabilities and the list-changed
notifications optional (`ServerCapabilities.tools.listChanged`: "Whether this server supports notifications
for changes to the tool list"; `ToolListChangedNotification`: "An optional notification"). The core
advertises none and does not register `subscriptions/listen`, so the skips stand; they are listed in the
report with their check ids.

### Not met

* The ten `input-required-result-*` scenarios in the table (required): the tools and the
  prompt they call are registered by MCP#10, PR #30.
* Nine `tasks-*` scenarios (not scored): the tools and the `tasks/*` methods they call are
  registered by MCP#12, PR #32. `tasks-status-notifications` is skipped by the suite.

### `server-stateless` and the two diagnostic tools

The scenario calls `test_streaming_elicitation` (line 887) and `test_logging_tool` (line 947) and reports a
call that is rejected, or a tool the server does not list, as not testable, which fails the check. The
conformance fixture lists both (TL-19) and each answers one text result, as in the suite's reference server.
Neither is an input-required tool: the first probe fails only on a frame with a method other than
`notifications/*`, the second only on a `notifications/message` for a request that set no `logLevel`. The
core writes progress only for a request that carries a `progressToken`, declares no `logging` capability and
writes no log message, so both probes pass. The core digest changed with the fixture catalog.

## The identity of the core

`conformance/core-identity.json` records the core by the SHA-256 of its compiled program,
`5da88084314a27a48dfe69a936f6df4d64c3e936103b625a93e04888da98dfc8`, with the package, its file
digests, the source and semantic IDs and the lock it was exported from, and the compiler that
exported it. That program digest is the one the committed export `mcp_lifecycle` of the same root
records: the compiled program does not depend on the compiler that exported it, while the
generated `src/lib.rs` does. CI rebuilds the core and the job fails unless the record is
reproduced; lexlean-mcp checks the program digest.

## Planted defects

Each was planted alone in the sources of a clean tree (a `git worktree` of this commit), the
production build and the host rebuilt, and the run repeated. A reverted tree is the clean run
above.

| Defect | Planted in | Core digest | Run | Diagnostic |
| --- | --- | --- | --- | --- |
| `test_simple_text` answers an empty content array instead of its text block | `src/Fixture.lex.tex`, the `callTool` of the fixture provider | `8f1335055a9149073644484f22f1aa59f6feb1190b647ebe34e0d21b17a1b01c`, not the recorded one | 26 of 37 required pass, `tools-call-simple-text` fails and is `unexplained` in the classification | `tools-call-simple-text: tools-call-simple-text: content array is empty; No text content found` |

The defect passes `lexlean check` and the production build of the conformance project, whose
entrypoints are the declaration and the fixture root, and so is found by the suite and by the
core identity record, not by the kernel. `check_core_identity` fails on it as well, since the
program digest moves.

## The PrismPM pin, and what slice 3 still needs

`.github/prismpm.rev` names PrismPM#339 at its head, `f5927bf46b92179d807ee6bf987f024b2f5a4e7e`.
Nothing in PrismPM changes. #339 vendors LexLean `cee0fbade`, which cannot read this repository's
sources (`LLP2001` on `term_cases`, `src/Json.lex.tex:230`) and has no `nested_induction`, so for
development `install_prismpm` builds `prismpm` from a copy of the PrismPM tree whose
`vendor/lexlean` is the package `cargo package` makes of the LexLean tree at `.github/lexlean.rev`
(`0b0c95d96`, SM-37). One pin drives both tools. `Cargo.lock` is PrismPM's own (`--locked`). The
compiler that certifies the core is then the compiler both locks were made with, and CI checks that:
`lexlean.lock` and `lexlean.conformance.lock` record the same `compiler_semantics`, and so does the
production result of `prismpm build`. The generated `src/lib.rs` of the core is byte for byte the
`src/lib.rs` of the committed export `mcp_lifecycle`.

Release: MCP v0.1.0 pins a PrismPM SDK release, and that release must pin a LexLean release that
contains SM-37 (tracked in MCP#16).

For PrismPM slice 3:

* the vendored LexLean has to read MCP's sources, which needs SM-37 (`nested_induction`) or a
  release that contains it;
* the build controller reads the LexLean build root as `.lexlean`, so `lexlean.conformance.toml`
  sets `build_root = ".lexlean"`;
* `prismpm verify` of a server application is still PP2014;
* the release profile of `prismpm` (thin LTO, one codegen unit) is killed for memory on a 7 GB
  runner, so CI installs the debug profile;
* the response frame must carry no notifications; the core writes the event-stream bytes into the
  response body, which satisfies it.
