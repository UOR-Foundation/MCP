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
| The declaration, a PrismPM `ServerApplication` whose core is `UorMcp.ConformanceFixture.conformanceRun`, the step serve over the fixture provider | `conformance/src/Application.lex.tex`, generated from the vectors of the pinned scenarios: `cargo run -p lifecycle-oracle --example conformance_application -- --write` |
| The LexLean and PrismPM projects the declaration is built in | `lexlean.conformance.toml`, `lexlean.conformance.lock`, `prismpm.toml`; the main project (`lexlean.toml`) does not see them |
| The copies of PrismPM's `Foundation.Bytes`, `Foundation.Server.V1.Model` and the `prism.arch` lexicon | `conformance/src/Foundation`, `language/prism.arch`; byte for byte the files at the revision of `.github/prismpm.rev` (authority `A-PRISMPM-SERVER-APPLICATION`) |
| The binding: the host effects `tool.calls` (the count of `tools/call` requests in the last 60 s), `clock.now`, `task.id` and `task.table`, which fill the request frame, and `task.store`, which the core requests to store the task table | `conformance/bindings/mcp-binding` |
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

The `conformance` job of `.github/workflows/lexlean.yml` runs these steps, and
`MCP_CONFORMANCE_SUITE_ROOT="$SUITE" cargo test -p lifecycle-oracle --test conformance -- --ignored`
before the server suite.

**The workspace gate does not run the node oracles.** `just vv` runs `cargo test --workspace` in
the locked SDK image, where the pinned suite is not installed (R6 forbids fetching it there). The
tests behind CF-03 to CF-06 that run a node program, or read the installed suite, are therefore
`#[ignore]`d with a reason that names the conformance job, and the pure-Rust checks of the same
claims (the declaration, the vectors, the waiting list, the digests, the pins and the authorities)
stay in the gate. The job runs the ignored tests with `--ignored` (`run_oracle_tests`) and is green
only when as many passed as `conformance.rs` ignores; a run that selected none, or skipped some,
fails. CF-08 refuses a test that needs node or the suite without the ignore, an ignore without
that reason, an ignored test the job does not run, and a job that does not require the count.

## The verdict

The suite's runner exits 1 for a failing scenario and for a harness error alike, and exits 0
when a scenario it was asked for produced nothing, so its exit status is not the verdict.
`server-suite.mjs` reads `checks.json` of every scenario of the frozen requirements file
(`requirements/2026-07-28.yaml`, SHA-256 `ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57`;
the runner refuses any other file) and decides:

* a scenario passes when it produced a check, no check failed, and at least one succeeded;
* a scenario with no checks, or whose checks are all skipped or warnings, is a failure;
* every failing check is reported as `scenario: check id: message`, the suite's own text;
* the job passes (exit 0) only when the set of failing scenarios equals the set
  `conformance/waiting.json` lists as waiting, each entry naming its issue and pull request
  (MCP#10 and #30; MCP#44, which has no pull request). A failing scenario that is not listed fails the job
  (`FAILING and not named in waiting.json`, printed first); a listed scenario that does not
  fail fails the job (`named as waiting but not failing`); an entry that names neither an issue
  nor a pull request fails the job. The last lines of the diagnostic group the failing scenarios
  by the pull request that is to deliver what they need.
* a waiting scenario is still a failure: it is `fail` in the table and in `report.json`, which says
  `valid: false`, the step summary groups it under the pull request it waits on, and the scenarios
  passing are counted without it (32 of 37 required today). The list classifies; it never turns a
  failure into a pass, and it is shortened in the pull request that delivers a scenario.

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

Run of the core of this commit (core digest below), spec 2026-07-28, 117 HTTP exchanges,
236 JSON-RPC messages, 60 refused probes, 1 transport refusal, every message valid.

Required: 32 of 37 pass. Not scored: 4 of 13 pass, 1 is skipped by the suite, 8 fail. Every failing scenario waits on #30 (five required) or #44 (eight `tasks-*`).

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
| 24 | required | `input-required-result-basic-elicitation` | pass | 3 of 3 |  |
| 25 | required | `input-required-result-basic-sampling` | pass | 3 of 3 |  |
| 26 | required | `input-required-result-basic-list-roots` | pass | 3 of 3 |  |
| 27 | required | `input-required-result-request-state` | **fail** | 1 of 2 | #10 (PR #30) |
| 28 | required | `input-required-result-multiple-input-requests` | **fail** | 1 of 2 | #10 (PR #30) |
| 29 | required | `input-required-result-multi-round` | **fail** | 1 of 2 | #10 (PR #30) |
| 30 | required | `input-required-result-missing-input-response` | pass | 2 of 2 |  |
| 31 | required | `input-required-result-non-tool-request` | pass | 3 of 3 |  |
| 32 | required | `input-required-result-result-type` | pass | 2 of 2 |  |
| 33 | required | `input-required-result-unsupported-methods` | pass | 2 of 2 |  |
| 34 | required | `input-required-result-tampered-state` | **fail** | 1 of 2 | #10 (PR #30) |
| 35 | required | `input-required-result-capability-check` | **fail** | 1 of 2 | #10 (PR #30) |
| 36 | required | `input-required-result-ignore-extra-params` | pass | 2 of 2 |  |
| 37 | required | `input-required-result-validate-input` | pass | 3 of 3 |  |
| 38 | not scored | `tasks-lifecycle` | **fail** | 8 of 9 | #44 (the suite's wire-schema-valid) |
| 39 | not scored | `tasks-capability-negotiation` | **fail** | 4 of 5 | #44 (the suite's wire-schema-valid) |
| 40 | not scored | `tasks-wire-fields` | **fail** | 3 of 4 | #44 (the suite's wire-schema-valid) |
| 41 | not scored | `tasks-request-state-removal` | **fail** | 2 of 3 | #44 (the suite's wire-schema-valid) |
| 42 | not scored | `tasks-mrtr-input` | **fail** | 3 of 4 | #44 (the suite's wire-schema-valid) |
| 43 | not scored | `tasks-request-headers` | **fail** | 4 of 5 | #44 (the suite's wire-schema-valid) |
| 44 | not scored | `tasks-dispatch-and-envelope` | **fail** | 8 of 9 | #44 (the suite's wire-schema-valid) |
| 45 | not scored | `tasks-status-notifications` | skipped by the suite | 0 of 1 (1 skipped) |  |
| 46 | not scored | `tasks-required-task-error` | pass | 3 of 3 |  |
| 47 | not scored | `tasks-mrtr-composition` | **fail** | 1 of 2 | #44 (the suite's wire-schema-valid) |
| 48 | not scored | `json-schema-2020-12` | pass | 8 of 8 |  |
| 49 | not scored | `http-header-validation` | pass | 14 of 14 |  |
| 50 | not scored | `http-custom-header-server-validation` | pass | 10 of 10 |  |

Two passes need a caution. `server-sse-multiple-streams` passes with an info: the core answers
JSON, and event streams are optional there. `server-stateless` passes with 5 of its 30 checks skipped by
the suite: the checks that need a `subscriptions/listen` stream. The suite skips them for a server that
advertises no `listChanged` or `subscribe` capability (`stateless.ts`, the comment above
`advertisesSubscriptions`), and the specification schema makes those capabilities and the list-changed
notifications optional (`ServerCapabilities.tools.listChanged`: "Whether this server supports notifications
for changes to the tool list"; `ToolListChangedNotification`: "An optional notification"). The core
advertises none and does not register `subscriptions/listen`, so the skips stand; they are listed in the
report with their check ids.

### Not met

* Five `input-required-result-*` scenarios in the table (required): `request-state`,
  `multiple-input-requests`, `multi-round`, `tampered-state` and `capability-check`. Their tools
  are registered by MCP#10, PR #30, and ask for a `requestState`, which the core writes only
  with the host's key (`LIFECYCLE.md`, "key"). The conformance application declares one frame
  section, the count of tool calls, and no key, so the core refuses them with an internal error.
  The key section of the application is the open work. The other nine `input-required-result-*`
  scenarios pass.
* Eight `tasks-*` scenarios (not scored) fail one check each, the suite's own `wire-schema-valid`,
  and pass every other: `lifecycle`, `capability-negotiation`, `wire-fields`,
  `request-state-removal`, `mrtr-input`, `request-headers`, `dispatch-and-envelope` and
  `mrtr-composition` (8 of 9, 4 of 5, 3 of 4, 2 of 3, 3 of 4, 4 of 5, 8 of 9 and 1 of 2 checks).
  The check holds the result of every `tools/call` to the core schema's `CallToolResult`, which
  requires `content`; a task-creating call is answered with the extension's `CreateTaskResult`
  (`resultType: "task"`, `Result & Task`), which has none, and the suite at the pinned commit
  `232a9014` knows no such result. `modelcontextprotocol/conformance#468` (`cf022cb`, after the
  pin) accepts it. MCP#44 is the decision to move the pin; the core is not changed to satisfy a
  check that the extension contradicts. The schema oracle, which holds the messages of the
  extension to the extension's schema, finds every one of them valid.
  `tasks-required-task-error` passes (3 of 3). `tasks-status-notifications` is skipped by the
  suite.

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
`7cbffcf5422d15c36bdaf4d9284c13df5e7651f5c60fd3b31ca2959943cd7b77`, with the package, its file
digests, the source and semantic IDs and the lock it was exported from, and the compiler that
exported it. That program digest is the one the committed export `mcp_lifecycle` of the same root
records: the compiled program does not depend on the compiler that exported it, while the
generated `src/lib.rs` does. CI rebuilds the core and the job fails unless the record is
reproduced; lexlean-mcp checks the program digest.

## Planted defects

Each was planted alone against the green state (the 19 waiting scenarios failing and every
other scenario passing, exit 0), the build and the run repeated. A reverted tree is the clean run
above. "Green" below means the exit status of `server-suite.mjs`, which is the status of step 9a.

| Defect | Planted in | Run | Diagnostic |
| --- | --- | --- | --- |
| `test_simple_text` answers an empty content array instead of its text block | `src/Fixture.lex.tex`, the `callTool` of the fixture provider; production build and host rebuilt, core digest `8f1335055a9149073644484f22f1aa59f6feb1190b647ebe34e0d21b17a1b01c` | exit 1; 26 of 37 required pass; `check_core_identity` fails as well: `the program differs from the committed export mcp_lifecycle` | `FAILING and not named in waiting.json (1): tools-call-simple-text`, then `tools-call-simple-text: tools-call-simple-text: content array is empty; No text content found` |
| a failing scenario is not listed | `conformance/waiting.json` without `tasks-lifecycle`, same build | exit 1 | `FAILING and not named in waiting.json (1): tasks-lifecycle`, then the suite's check messages, and `the failing scenarios are not the ones .../waiting.json names` |
| a listed scenario passes (stale) | `conformance/waiting.json` with `caching` added, same build | exit 1 | `named as waiting but not failing (1): caching` |
| an entry names neither an issue nor a pull request | `conformance/waiting.json`, `tasks-lifecycle` with only a reason | exit 1 | `named as waiting without an issue or a pull request (1): tasks-lifecycle` |
| the split is broken by dropping `#[ignore]` from `the_schema_oracle_node_tests_pass_cf_04` | `crates/lifecycle-oracle/tests/conformance.rs`; `cargo test -p lifecycle-oracle --test conformance` with `MCP_CONFORMANCE_SUITE_ROOT` unset, as in the SDK image | exit 101; `the_schema_oracle_node_tests_pass_cf_04` fails and so does CF-08 | `MCP_CONFORMANCE_SUITE_ROOT names the installed conformance suite (conformance/oracle/fetch-suite.sh DIR installs it): NotPresent` (the failure that turned `bootstrap` red), and CF-08: `the_schema_oracle_node_tests_pass_cf_04 needs node or the installed suite and is not ignored: the workspace gate runs without them` |
| the conformance job selects fewer ignored tests than the source ignores | `run_oracle_tests` with the filter `cf_03` after `-- --ignored` | exit 1 | `1 passed of the ignored tests, expected 4` |

The first defect passes `lexlean check` and the production build of the conformance project, whose
entrypoints are the declaration and the fixture root, and so is found by the suite and by the
core identity record, not by the kernel.

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
