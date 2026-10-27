# Tasks

The core registers the tasks extension `io.modelcontextprotocol/tasks` (SEP-2663) for
revision 2026-07-28: a `tools/call` may answer a task handle instead of a result, and
`tasks/get`, `tasks/update` and `tasks/cancel` serve the handle. It is LexLean source in
language 1.2 (`src/*.lex.tex`); the host adapter (PrismPM issue 216) moves bytes and holds
the state the core cannot. Issue #12; register rows TK-01 to TK-17.

The extension is optional by definition: the pinned requirements run its ten scenarios and do
not score them (`reason: extension`). The verdict of the suite over the conformance build is in
`CONFORMANCE-RUN.md` and `conformance/waiting.json`. Nine of the ten pass against the conformance
build; `tasks-status-notifications` is skipped by the suite, which sends no request ("Not met" below).
Eight of the nine failed one check each, the suite's own `wire-schema-valid`, until MCP#44 moved the
suite pin to `cf022cbb`, which contains `modelcontextprotocol/conformance#468`: that commit accepts the
`CreateTaskResult` of the extension (`resultType: "task"`, no `content`) as an extension result
instead of holding it to `CallToolResult`. The core did not change.

## Authorities

| Authority | Pin | What it gives |
| --- | --- | --- |
| the extension text and schema | `modelcontextprotocol/ext-tasks` at `899c1bbc869567670093face3591b39c28ce3219` (2026-10-09), `specification/2026-07-28/tasks.md` and `schema/2026-07-28/schema.json` | the normative text, and the types of `CreateTaskResult`, `GetTaskResult`, the two acknowledgements and `notifications/tasks`. The specification's own `schema.json` (80041ab) has no task type: the extension keeps its schema apart, so the responses of this layer are checked against both schemas |
| the proposal | `docs/seps/2663-tasks-extension.mdx` of `modelcontextprotocol/modelcontextprotocol` at `80041ab0a139a8f4fb3adbc99f7ae9113ea59671` | the design as accepted; not vendored, because the extension text above is the specification and the proposal says it is a historical record |
| the scenarios | `modelcontextprotocol/conformance` at `cf022cbb541f9d939068d7f84758b9728be7fc23` | `src/scenarios/server/tasks/*.ts` (twelve files) and `input-required-result-helpers.ts`, vendored beside the earlier files; authority row `A-MCP-CONFORMANCE` records the new manifest checksum |

The pin of `ext-tasks` is the last commit at or before the pinned specification commit
(2026-10-20). Its next commit (`b3d28ac`, 2026-10-21) changes one sentence of the overview.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Tasks.lex.tex` | the task states, `DetailedTask`, `CreateTaskResult`, and the state machine: a `TaskEntry` is a task, the instant it was created and the `TaskPlan` it follows; `settle`, `answer` and `cancel` are the only ways an entry changes; `withEntry`, `findEntry` and `isExpired` are the table |
| `src/TaskJson.lex.tex` | a task as JSON (the wire form `tasks/get` answers and the notification carries), and the host's table as JSON, written and read |
| `src/TaskServe.lex.tex` | `tasks/get`, `tasks/update`, `tasks/cancel`, and the decision of `tools/call` whether the answer is a task |
| `src/TaskFixture.lex.tex` | the seven tools the scenarios name, and what each does |
| `src/TaskLaws.lex.tex` | kernel-checked theorems about the above |
| `src/Methods`, `Dispatch`, `Tools`, `ResultJson`, `Http`, `Frame`, `Serve`, `Fixture` | extended: one more result body (`stored`), the host's clock and table in the frame, `Fixture.fixtureRun` over `Serve.serveTasks` |
| `src/StepServe.lex.tex`, `StepFixture`, `StepLaws`, `Extension` | extended: the store effect (`storeRequest`, `storing`), `Asked.host`, `ProviderInfo.taskStoreTag`, and the theorems of the store |
| `src/ConformanceFixture.lex.tex` | the root of the conformance build: the step serve over the fixture provider, with the store tag 4 (the fifth effect the conformance declaration lists) |
| `crates/lifecycle-oracle/generated/mcp_lifecycle` | LexLean's export of `fixtureRun`, unedited |
| `crates/lifecycle-oracle/vectors/tasks.json` | the requests of the nine scenarios that send any, in the order they send them, with file and line at the pinned suite commit |
| `crates/lifecycle-oracle/tests/tasks.rs` | TK-01 to TK-16: the host of the test, the vectors, and the probes beyond them |
| `crates/lifecycle-oracle/suite/ext-tasks-899c1bb…` | the vendored extension text, schema and licence |

Regenerate the packages after any change to the sources, as [LIFECYCLE.md](LIFECYCLE.md)
describes.

## The host's side

The core keeps no state. A task outlives the request that made it, so the state it needs
arrives with each request and leaves with its answer, as the rate-limit count does
([TOOLS.md](TOOLS.md#the-rate-limit)). The request frame carries, after `tool_calls`
([LIFECYCLE.md](LIFECYCLE.md#the-boundary-encoding)):

| Field | What the host owes |
| --- | --- |
| `now_ms` | a monotonic counter of milliseconds, eight bytes. Only differences matter, so a process clock works |
| `now` | the same instant as ISO 8601 text; the core writes it into `createdAt` and `lastUpdatedAt` and cannot check that the two agree |
| `fresh_task_id` | an identifier that no task has had and that a client cannot guess; the core cannot make one and uses it only when this request creates a task |
| `task_table` | the table as the previous response returned it, or empty |

What the core returns depends on the root the host serves:

| Root | How the host gets the table |
| --- | --- |
| the byte serve (`Serve.serveTasks`, exported as `UorMcp.Fixture.fixtureRun`) | the response frame ends with the flag 1 and the table, when the request created or touched a task; the host stores it from then on |
| the step serve (`StepServe.serveStep`, exported as `UorMcp.ConformanceFixture.conformanceRun`) | the core asks for one effect, the store of the table, under the tag its `ProviderInfo.taskStoreTag` names (TK-17). The payload is the table preceded by its length in four bytes; once the host's outcome is in the log the response is written without the table |

PrismPM's adapter reads a response frame that ends at the body and refuses one with a byte after
it, so a root served by PrismPM's adapter cannot return the table in the frame. The conformance
build declares the three sections that fill the frame (`clock.now`, `task.id` and `task.table`
beside `tool.calls`) and the effect `task.store` that the core requests; its binding holds the
table in memory (`conformance/bindings/mcp-binding`).

The core returns a table when a request created or touched a task, and never when it refused the
request. The table is a JSON array with one object per task: the task as `tasks/get` would write it,
the instant it was created, and the plan the work follows. It is the host's to keep and the core's to
read: the host must hand back exactly what it was given.

The host is also the work. A task in this model is a plan: what the task asks the client for,
when it asks, when it finishes and how. The fixture's plans are functions of the call, so the
core can answer "what is this task now" from the plan and the clock without a thread of its own.
A host that runs real work reports its progress to the core the way the fixture's plans do, by
the clock and by the table; the adapter of PrismPM#216 owns that part.

The host expires nothing: a task whose `ttlMs` has elapsed is dropped from the table by the next
request that returns a table, and until then it answers as an unknown task (`-32602`) too, because
a server MAY delete it at any time after the ttl and MUST NOT before.

## What a call does

`tools/call` runs the steps of [TOOLS.md](TOOLS.md#what-a-call-does) up to the arguments, and
then, for a tool that supports tasks:

1. **Validate the arguments.** An invalid argument is the result with `isError`, as for any tool.
   No task is created.
2. **Decide.** The server alone decides; the request carries no hint, and a legacy `task` member
   is ignored. A client has declared the extension when `io.modelcontextprotocol/tasks` is in the
   capabilities of the request's own `_meta` (SEP-2575: the declaration is per request).

   | The tool | Client declared the extension | Client did not |
   | --- | --- | --- |
   | does not support tasks (`greet`) | the result | the result |
   | supports tasks (`slow_compute`, `confirm_delete`, `multi_input`, `protocol_error_job`) | a `CreateTaskResult` | the result it would have returned at once |
   | needs a task (`failing_job`, `test_tool_with_task`) | a `CreateTaskResult` | `missingCapability` (-32021) naming the extension |
3. **Create.** The task is the `working` seed of the tool's plan, with the host's identifier and
   instant, `ttlMs` 60 000 and `pollIntervalMs` 1 000. It is put in the table the response returns,
   so `tasks/get` for the identifier resolves as soon as the result is returned. The result is the
   flat `CreateTaskResult`: `resultType` `task` and the task's members at the top level, with no
   nested `task`, `result`, `error`, `inputRequests` or `requestState`.

A tool that asks for input before it becomes a task (`test_tool_with_task`) answers its first
round with an `input_required` result and its last with the `CreateTaskResult`. This is the
multi round-trip of issue #10 composed with tasks; see [Not met](#not-met).

## What a task does

A task moves only through `settle`, `answer` and `cancel`, and only forward:

| Step | When | Effect |
| --- | --- | --- |
| `settle` | a `working` task whose plan parks it, once `parkAfterMs` has passed | `input_required`, with the plan's `inputRequests` (once) |
| `settle` | a `working` task with nothing to ask, once `finishAfterMs` has passed | `completed` with the plan's result, or `failed` with its JSON-RPC error |
| `answer` | an `input_required` task | the answered keys leave `inputRequests`; the task stays `input_required` while any key remains and returns to `working`, and settles at once, when none does |
| `cancel` | a `working` or `input_required` task | `cancelled` |
| any | a task in a terminal state (`completed`, `failed`, `cancelled`) | nothing |

`settle` is lazy: nothing runs between requests, so a task that has finished is found out by the
first request that asks about it, which also writes the change. `tasks/update` and `tasks/cancel`
settle first, so a cancel that arrives after the work finished finds a `completed` task and
leaves it (the extension allows a task to reach a terminal state other than `cancelled`). The
fixture honours every cancel of a task that is still running.

`tasks/get` answers the `DetailedTask` of the moment, with `resultType` `complete`. `tasks/update`
and `tasks/cancel` answer an empty result with `resultType` `complete`, for every known task,
whatever its state: an update of a task that is not waiting, an answer to a key nobody asked,
and a cancel of a finished task are acknowledged and change nothing. An unknown task is
`invalidParams` (`-32602`). The methods `tasks/result` and `tasks/list` do not exist
(`methodNotFound`).

The `result` of a completed task is what the tool would have returned at once, sanitized
once, when the plan is made ([TOOLS.md](TOOLS.md#output-sanitization)), without `resultType` or
`_meta`. A tool that ran and reported an error is `completed` with `isError` true; `failed` is for a
JSON-RPC error only.

## Status notifications

A change the core makes to a task while it answers a request is written, beside the body, as one
`notifications/tasks` notification whose params are the complete task, exactly what `tasks/get`
would have returned at that moment. That is one notification when a request settles a task, when
an `answer` removes a key or resumes the task, and when a `cancel` takes effect. Creating a task,
reading an unchanged task, an ignored answer, a refused request and a cancel of a terminal task
write none. The notifications travel in the frame's notification list, as the progress
notifications of TL-13 do; `Content-Type` stays `application/json`. Their delivery, on the
`subscriptions/listen` stream to the clients that listed the task id, is issue #11's.

The pinned scenario `tasks-status-notifications` sends no request at this commit: it returns one
skipped check, because the harness observes the POST response while the extension sends
notifications on the listen stream. TK-12 therefore has no vector, and says so.

## Refusals

| Refusal | JSON-RPC | HTTP | When |
| --- | --- | --- | --- |
| `missingCapability` (existing) | -32021, `data.requiredCapabilities.extensions` holds the extension | 400 | a task method from a client that did not declare the extension, checked before the task is looked up; `tools/call` of a tool that needs a task |
| `unknownTask` (existing) | -32602 | 400 | `tasks/get`, `tasks/update` or `tasks/cancel` for a task the table does not hold, or whose ttl has elapsed |
| `methodNotFound` (existing) | -32601 | 404 | `tasks/result`, `tasks/list`, and any other `tasks/` method |
| `headerMismatch` (existing) | -32020 | 400 | `Mcp-Name` missing or different from `params.taskId` on a task method; `Mcp-Method` different from the method |
| `internal` | -32603, no `data`, message `Internal error` | 500 | a task table the core cannot read (`the task table is not readable`); a host that supplied no identifier for a task the request creates (`the host supplied no task id`) or the identifier of a task the table still holds (`the host supplied a task id that a task already has`); a task that cannot be written (a parked request that has no wire form) |
| the fixed internal body | -32603, no `id` | 500 | a frame whose host section is cut, whose text is not UTF-8, or that has bytes after the table |

Each is stated in a theorem or a test: the new `internal` texts are the `ProviderRefusal.internal`
values of `TaskServe`, and the refusals that were already modeled gain the cases above.

## Decisions

| Question | Choice |
| --- | --- |
| Where task state lives | the host's table, in the frame; the core is a function of the request and the table |
| How a task finishes | by the plan and the host's clock, found out lazily; nothing runs between requests |
| What a task id is | the host's `fresh_task_id`; an empty one is refused, and so is one that a live task of the table has, because a creation would replace that task |
| `ttlMs` and `pollIntervalMs` | 60 000 and 1 000 for every fixture task; `ttlMs` is an `Int` in `TaskFields`, as is `pollIntervalMs`, because a millisecond clock is an `Int` and a conversion from `Nat` counts one successor at a time |
| A task of a tool that needs one | `missingCapability` for a client without the extension, on the call; the issue's "typed refusal for a tool that does not support tasks" has no case, because the extension is server-directed and a client cannot ask for a task |
| `tasks/cancel` of a terminal task | acknowledged, unchanged: the scenario `tasks-lifecycle` requires the idempotent acknowledgement and reserves `-32602` for unknown ids |
| `tasks/update` of a task that is not waiting | acknowledged, unchanged |
| Order of the checks | the request is validated as in [LIFECYCLE.md](LIFECYCLE.md); then the rate limit; for a task method, the extension, then the table, then the task |
| `statusMessage` | never written; the extension makes it optional |
| Input requests a task can park on | elicitation (form and URL) and roots; sampling requests have no wire form here yet and a plan that parks on one is a server error, until #10 lands its encoder |

## Kernel theorems

`TaskLaws.lex.tex` states, and `lexlean verify` checks:

* a task in a terminal state is not changed by `settle`, by `answer` or by `cancel`;
* `answer` changes a task only while it is `input_required`: a working task, and each terminal
  task, is returned as it was;
* `cancel` of a waiting task, and of a working task whose time has not come, yields `cancelled`; of a
  working task whose time has come, `completed`;
* a working task stays working until its time, to the millisecond, and a plan parks it at its time;
* answering the last request resumes the task and answering one of two leaves the other; an
  answer to a key nobody asked changes nothing;
* a step keeps the task's identifier, creation instant, ttl and poll interval;
* a task is expired exactly when its ttl has elapsed, and never when it has none;
* a table finds a task by its identifier while it lives, and keeps one entry per identifier;
* a task is written without a result, an error or inputs unless its state carries one, never with a
  `requestState`, and with a `null` ttl when it has none;
* an unknown task is `-32602` with status 400, a missing capability is `-32021` with status 400 and an
  unreadable table is `-32603` with no data.

LexLean's proof backend cannot evaluate `utf8_decode` and the other primitives listed in
[TOOLS.md](TOOLS.md#kernel-theorems), so the table's JSON text, the host's text fields and the reading of
a table are covered by the tests of `tests/tasks.rs`, which run LexLean's export.

## Evidence

| ID | Evidence |
| --- | --- |
| TK-01 to TK-11, TK-13 | `tests/tasks.rs` over the steps of `vectors/tasks.json` (the requests of nine scenarios), and the probes beyond them; every response and notification is validated against the specification's schema and the extension's |
| TK-12 | `tests/tasks.rs` probes only; the scenario has no request at the pin |
| TK-14 | `TaskLaws.lex.tex`, checked by `lexlean verify`, and the test that it states every transition and every refusal |
| TK-15 | `tests/tasks.rs`: unreadable tables, a missing identifier, an identifier a live task has, a cut or non-UTF-8 host section, the table returned only when touched, 301 tasks on the standard stack |
| TK-17 | `tests/effects.rs`: the store of the table through the step root of the tests and through the conformance root, no effect for a refusal or a failed check, the table of a read stored again, the table stored equal to the one the byte serve returns; `StepLaws`, checked by `lexlean verify`, for the store request and the tag |
| TK-16 | `tests/tasks.rs`: the vendored files against the checksums of the authorities, every request site and every check id of the vendored scenarios against the vectors |

## The vectors

`vectors/tasks.json` holds nine scripts, one for each tasks scenario that sends a request, with 71 steps
in all. A step is a request the scenario sends, built as the scenario builds it, with the checks the
scenario makes on the answer; a step that waits is a wait loop of the scenario (`waitForTerminal` or
`waitForStatus`, polling every 200 ms) and cites its line in `helpers.ts`. A script runs on the host
of the test, which advances its clock by the delays of the scenario and the intervals of its waits
instead of sleeping. Each step cites the scenario file, its line at the suite commit, a fragment of that
line and the check identifiers it stands for; `tests/tasks.rs` checks every citation against the vendored
files, checks that every request site of the vendored scenarios is a step, and checks that every check
identifier the scenarios emit is a step's or is listed with its reason.

The tenth scenario, `tasks-status-notifications`, sends no request at this commit and is listed under
`unreached`, with eight identifiers that are emitted only when a connection cannot be opened.

A vector the root fails is a defect of the root. A vector it passes shows that this request meets these
checks in process, on a clock the test chooses. It is not the verdict of the suite.

The reference fixture of the suite does not define the seven tools: the scenarios' descriptions name
them (`greet`, `slow_compute`, `failing_job`, `protocol_error_job`, `confirm_delete`, `multi_input` and
`test_tool_with_task`) and say what each must do. `TaskFixture` follows those descriptions and nothing
else; where a description leaves a value open (the text of a result, the key of an input request) the
fixture picks one and `tests/tasks.rs` states it.

## Planted defects

Each defect was planted alone in `src/Tasks.lex.tex` of a tree on which every gate passed, run
with `lexlean verify src/TaskLaws.lex.tex`, and reverted with `git checkout`. Each took about a
minute and each is stopped by the kernel.

| Defect | Planted in | Command | Diagnostic |
| --- | --- | --- | --- |
| reading a task that has nothing to settle reports a change | `Tasks.kept`: `changed := true` for `changed := false` | `lexlean verify src/TaskLaws.lex.tex` | `LLV7002`, `Tactic rfl failed` in `a_working_task_stays_working_until_its_time` (`TaskLaws.lex.tex:287`): `(Tasks.settle (workingEntry 2000) 1999 "n").changed` is not definitionally equal to `false` |
| a task outlives its ttl by one instant | `Tasks.isExpired`: `limit < elapsed` for `elapsed >= limit` | same | `LLV7002`, `Tactic rfl failed` in three theorems: `a_task_is_expired_only_when_its_ttl_has_elapsed` (`TaskLaws.lex.tex:343`, `Tasks.isExpired (workingEntry 0) 60000` is not `true`), `an_entry_is_found_by_its_identifier_while_it_lives` (`:353`) and `an_expired_entry_leaves_the_table_with_the_next_change` (`:371`) |
| a cancel that arrives after the plan finished cancels the task | `Tasks.cancel`: `cancelSettled (kept entry) now` for `cancelSettled (settle entry nowMs now) now` | same | `LLV7002`, `Tactic rfl failed` in `a_working_task_that_finished_is_not_cancelled` (`TaskLaws.lex.tex:279`): `statusAfter (Tasks.cancel (workingEntry 2000) 2000 "n")` is not definitionally equal to `Tasks.TaskStatus.completed` |

| a task id is reused: the entry already holding the identifier survives beside the new one, so the table holds the identifier twice | `Tasks.withEntry`: `survivorsOnto ""` for `survivorsOnto (idOf entry)` | same | `LLV7002`, `Tactic rfl failed` in `TaskLaws.lex.tex:367`: `Tasks.withEntry (workingEntry 5) [workingEntry 9] 0` is not definitionally equal to `[workingEntry 5]` |
| a cancel after completion is accepted (the plant in the row above, re-run on the merged tree) | `Tasks.cancel`: `cancelSettled (kept entry) now` | same | `LLV7002`, `Tactic rfl failed` at `TaskLaws.lex.tex:283`: `statusAfter (Tasks.cancel (workingEntry 2000) 2000 "n")` is not `Tasks.TaskStatus.completed` |
| a tool that requires a task is answered without one | `TaskServe.fallThrough`: `Tools::checkExecuted tool sync` for `missingTasks` in the `TaskSupport.required` arm | same | `LLV7002`, `Tactic rfl failed` at `TaskLaws.lex.tex:455` in `a_tool_that_needs_a_task_is_not_answered_without_one`: `TaskServe.fallThrough TaskServe.TaskSupport.required tool sync` is not `Except.error TaskServe.missingTasks` |

The id-reuse defect, the late cancel and the required-without-task defect were planted on the merged
tree, each alone in a copy of the clone, and stopped by the kernel with `lexlean verify
src/TaskLaws.lex.tex`. The Rust-level refusal of a reused identifier (`TaskServe.idInUse`, an internal
error) is covered by the id-in-use test of `tests/tasks.rs`; no defect was planted in the encoder
(`TaskJson`).

Restored result: every defect was reverted with `git checkout`, and on the final tree
`lexlean check` reports 51 modules, `cargo fmt --all -- --check` and `cargo clippy --workspace
--all-targets --locked -- -D warnings` pass, and `cargo test --workspace --locked` passes with the
three packages equal to a fresh export.

## Not met

* `tasks-status-notifications` is skipped by the suite at the pinned commit (it awaits a
  `subscriptions/listen` rewrite); its row has no vector. The notifications are modeled and tested
  here, and their stream is #11's.
* A task whose plan parks on a sampling request has no wire form; the fixture never parks on one.
* The extension's `subscriptions/listen` subscription by task id, and the `-32021` it returns to a
  client that lists task ids without declaring the extension, are #11's.
* The host adapter keeps the table, the clock and the identifiers (PrismPM#216). The conformance
  host (`conformance/bindings/mcp-binding`) is the adapter's host for the run; the tests drive the
  core with a host of their own. Two requests that overlap read the table before either stores it, and
  the later store wins; the suite sends its requests one after another.
* The conformance application declares the table at most 512 KiB in either direction
  (`TABLE_MAXIMUM` in `tests/common/application.rs`); a larger table is outside what the run carries.
* The five `input-required-result-*` scenarios that wait on #30 (`conformance/waiting.json`, as on
  `main`) need the key of `requestState`, which the conformance application does not carry; they are
  not this change's.
* The suite validates each response against its own wire schema; here every response is validated
  against the specification's schema and the extension's by another validator. That is falsifying
  evidence, not the oracle for the protocol.
