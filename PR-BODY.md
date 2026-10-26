Registers the tasks extension `io.modelcontextprotocol/tasks` (SEP-2663) in the core and runs its ten scenarios against the conformance build. It takes over draft #32 (humuhumu33, branch `claude/mcp-12-tasks`, head `2b9fecf`) on a branch of our own, built on their commits, merged with `main` (#40 provider host effects, #30 input-required results) and finished; their branch was not pushed to. #32 closes when this merges.

Part of #12. Eight of the ten scenarios do not pass; see "Not met". They fail one check each, the suite's own, and MCP#44 asks for the decision that fixes it.

## Change

* **The extension** (`TASKS.md`, rows TK-01 to TK-17): `src/Tasks`, `TaskJson`, `TaskServe`, `TaskFixture`, `TaskLaws`. A `tools/call` of a task-supporting tool from a client that declared the extension is answered with a `CreateTaskResult` (`resultType: "task"`); `tasks/get`, `tasks/update` and `tasks/cancel` serve the handle; a tool that requires a task is refused without one, and a client that did not declare the extension gets `-32021`. The core keeps no state: the host passes the clock, a fresh task id and the task table with each request (`TASKS.md`, "The host's side"). A fresh id that a task already holds is an internal error.
* **The table is stored through a host effect** in the step serve (TK-17, `StepServe`, `StepLaws`): the response frame never carries it. The conformance root is the step serve over the fixture (`src/ConformanceFixture.lex.tex`), the fifth effect of the application being `task.store`; `conformance/bindings/mcp-binding` is its host (tool-call window, clock, ids, table).
* **The conformance build** declares the effects and sections, the acceptance vectors and the transcripts (`conformance/src/Application.lex.tex`, rendered by `cargo run -p lifecycle-oracle --example conformance_application -- --write`); `prismpm build` accepts it.
* **The schema oracle** (`conformance/oracle/schema-oracle.mjs`, CF-04) now holds the extension's messages to the pinned ext-tasks schema (`899c1bbc`, digest checked before it runs): requests and results by method, a result with `resultType: "task"` by `CreateTaskResult`. Without the schema those messages are findings. The one rule of the extension it applies is that a server ignores `inputResponses` under a key that is not outstanding; those are counted (1 in the run) and every other part of the request is still validated. Node tests cover each path.
* `crates/*/generated` are re-exported from the final sources (`mcp_crowd` takes `serveTasks` directly, which keeps its call graph free of a closure cycle, TL-18); the pins, `lexlean.lock`, `lexlean.conformance.lock`, `conformance/core-identity.json` and `CONFORMANCE.md` are regenerated.

## Scenarios (pinned suite `232a9014`, run by the conformance job)

| Scenario | Result |
| --- | --- |
| `tasks-required-task-error` | pass (3 of 3) |
| `tasks-lifecycle` | 8 of 9 checks; fails the suite's `wire-schema-valid` |
| `tasks-capability-negotiation` | 4 of 5; same |
| `tasks-wire-fields` | 3 of 4; same |
| `tasks-request-state-removal` | 2 of 3; same |
| `tasks-mrtr-input` | 3 of 4; same |
| `tasks-request-headers` | 4 of 5; same |
| `tasks-dispatch-and-envelope` | 8 of 9; same |
| `tasks-mrtr-composition` | 1 of 2; same |
| `tasks-status-notifications` | skipped by the suite (no request at the pin) |

Required scenarios: 32 of 37 pass, as on `main`. The five that fail are the `input-required-result-*` scenarios that `main` already lists in `conformance/waiting.json` (#30, the key section of the application); they are unchanged by this PR. The conformance job is green: the failing set equals `waiting.json`.

## Validated

* `lexlean fmt --check`, `lexlean lock --check`, `lexlean check` (65 modules), `lexlean verify` (CI job `verify`, green; exact axiom policy)
* `cargo xtask check-model --write`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` (all targets pass locally; CI job `cargo`)
* the exports of `crates/json-oracle` and `crates/lifecycle-oracle`, byte for byte (CI jobs `export json-oracle`, `export lifecycle-oracle`); the packages were produced with the SM-37 compiler named in `.github/lexlean.rev`
* `conformance` job: `prismpm build`, core identity (`program e95e5363...`), binding build/test/fmt/clippy, oracle tests (35 node tests, 4 ignored cargo tests), the suite, the schema oracle (420 messages, 194 held to the extension schema), the scenario table
* `bootstrap`: the SDK gate

## Planted defects (`TASKS.md`, "Planted defects")

Each alone in a copy of the tree, run with `lexlean verify src/TaskLaws.lex.tex`, stopped by the kernel with `LLV7002`, `Tactic rfl failed`:

* a task id is reused (`Tasks.withEntry` keeps the entry that holds the identifier beside the new one): `TaskLaws.lex.tex:367`, `Tasks.withEntry (workingEntry 5) [workingEntry 9] 0` is not `[workingEntry 5]`;
* a cancel after completion is accepted (`Tasks.cancel` skips `settle`): `TaskLaws.lex.tex:283`, `statusAfter (Tasks.cancel (workingEntry 2000) 2000 "n")` is not `completed`;
* a tool that requires a task is answered without one (`TaskServe.fallThrough` runs the tool): `TaskLaws.lex.tex:455`, `fallThrough required tool sync` is not `Except.error missingTasks`;
* and the earlier three (a read that reports a change, a task that outlives its ttl by one instant, the late cancel at its first site).

The Rust-level refusal of a reused id has its own test in `tests/tasks.rs`.

## Not met

* **Eight of the ten acceptance scenarios** (`tasks-lifecycle`, `-capability-negotiation`, `-wire-fields`, `-request-state-removal`, `-mrtr-input`, `-request-headers`, `-dispatch-and-envelope`, `-mrtr-composition`): each passes every check but the suite's `wire-schema-valid`, which at `232a9014` holds the result of every `tools/call` to `CallToolResult` (required `content`); the `CreateTaskResult` the extension requires has none. `modelcontextprotocol/conformance#468` (`cf022cb`), after the pin, accepts it. Moving the pin touches CF-01, the vendored suite, the frozen requirements digest, the vectors and every consumer of the pin, so it is left as a decision (MCP#44); the eight are in `conformance/waiting.json` with that issue. The core is not changed to satisfy a check that contradicts the extension.
* `tasks-status-notifications`: skipped by the suite at the pin (it awaits a `subscriptions/listen` rewrite); the notifications are modeled and tested here, their stream is #11's.
* The limits in `TASKS.md`: a task that parks on a sampling request has no wire form; `subscriptions/listen` by task id is #11's; overlapping requests race on the table (the later store wins; the suite sends its requests one after another); the conformance application carries a table of at most 512 KiB.

Lock: `lexlean.lock` and `lexlean.conformance.lock` were made with the SM-37 compiler pinned in `.github/lexlean.rev`.

Credit: the first implementation, the vectors and the first tests are Ilya Paveliev's (#32).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01F4DJnZorC2Jd4MJfN9DbZk
