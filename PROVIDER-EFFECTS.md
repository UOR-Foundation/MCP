# Provider host effects

A provider can ask the host to perform an effect and be asked again with the outcome. The
request is a typed outcome of a call, a read or a prompt, the core carries it to the root
unchanged, and a second serving entry, `StepServe.serveStep`, writes it in the step wire of
PrismPM's replay driver. The core stays pure: it performs nothing, it only says what it
needs. Issue #39; register rows PE-01 to PE-09.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/HostEffect.lex.tex` | `EffectRequest` (a tag and a payload), `Completed` (a tag and an outcome), `Step` (`done` or `perform`), and `tagFits`, the check that a tag is four bytes |
| `src/Extension.lex.tex` | extended: the constructor `perform` on `CallOutcome`, `ReadOutcome` and `PromptOutcome`, and `callAnswer`, `readAnswer` and `promptAnswer`, which `answerOf` uses and which turn a `perform` into the refusal `effectRequested` |
| `src/Refusal.lex.tex`, `src/Status.lex.tex` | extended: the provider refusal `effectRequested`, whose code is -32603, whose status is 500 and whose data is empty |
| `src/StepServe.lex.tex` | `serveStep`: the turn read, the same pre-provider checks as `Serve.serve` in the same order, the provider asked with the completed effects of the turn and the task host, and the step written; the store of the task table when the response wrote one (TK-17) |
| `src/StepFixture.lex.tex` | the root `stepFixtureRun`: the step serve over a provider that requests effects on a few tools, one resource and one prompt, and over the unchanged fixture provider for every other request |
| `src/StepLaws.lex.tex` | kernel-checked theorems about the above (PE-08) |
| `crates/lifecycle-oracle/generated/mcp_step` | LexLean's export of `stepFixtureRun`, unedited |
| `crates/lifecycle-oracle/tests/effects.rs` | PE-01 to PE-09, run through a driver that performs the effects as PrismPM's generated driver does |

The fixture provider requests no effect of its own, and `Serve.serve` is the byte root it was: a
wrapper over `Serve.serveTasks`, which also takes the task host and returns the task table in its
response frame ([TASKS.md](TASKS.md#the-hosts-side)). The step serve has one effect request of its
own, the store of that table (TK-17): `StepServe.storeRequest` is an `EffectRequest` under the tag
`ProviderInfo.taskStoreTag`, made when a response wrote a table and not yet stored, and
`StepServe.Asked` carries the task host with the completed effects. The conformance build's core is
`UorMcp.ConformanceFixture.conformanceRun`, the step serve over the fixture provider with the tag of
the store effect of its declaration.

## The typed outcome

```
EffectRequest { tag : Nat, payload : Bytes }
Completed     { tag : Nat, outcome : Bytes }
Step          = done (List Nat) | perform EffectRequest
```

A provider asks for an effect by returning `CallOutcome.perform request`,
`ReadOutcome.perform request` or `PromptOutcome.perform request`. `Extension.answerOf`
turns each of the three into `Result.error (ProviderRefusal.effectRequested request)`. The
refusal is the transport: it passes `Dispatch` and `Tools.gate` as every provider refusal
does, so the checks that run before the provider (the transport gate, the method, the
headers, the version, `_meta`, the params, the capability and the rate limit) run first and
unchanged, and an effect request is never a result body, a content block or a text. No
marker can collide with what a tool returns.

The tag is a `Nat`, and `tagFits` holds below 2^32, because the wire carries four bytes and
`UInt32` is a PrismPM type that the `mcp.core` lexicon does not have. A request whose tag
does not fit is not a `perform`: the step is `done` with the internal-error response, and
the tag is never wrapped.

A provider that is called with the completed effects of its turn builds its answer from
them. `StepServe.Asked` holds the log, `completed : List Completed` in the order the effects
were performed, and the `request : ClientRequest`; `serveStep` takes
`answerWith : Fn Asked (Result ResultBody ProviderRefusal)`.

## The wire

Every word is four bytes, big-endian, as in PrismPM's `server_host/exchange.rs.in` at the
commit pinned by `prismpm.lock`.

```
turn = u32 length, request frame,
       u32 count, count times (u32 tag, u32 length, outcome)
step = u32 0, response frame                      done
     | u32 1, u32 tag, payload                    perform
```

The request frame is the one `Serve.serve` reads. A turn with a request frame that cannot be
read, one that is cut short, or one with a byte after its last outcome is `done` with the
internal-error response frame, the same one the byte serve writes for a frame it cannot
read, and never a `perform`. The root that declares no effect is the plain frame function
and has no step; `stepFixtureRun` is the root that does.

A host drives a turn by calling the root with the request and no outcome, performing the
effect of a `perform` step, appending `(tag, outcome)` to the log and calling the root
again, until the step is `done`. The core is pure, so the same turn is answered with the same
bytes. A provider that needs two effects asks for the second once the first outcome is in
the log, and the host's budget for the number of effects bounds the exchange.

## The byte serve

`Serve.serve` is untouched and cannot perform: an effect request that reaches it is a
provider refusal like any other, answered as an internal error (500, -32603, message
`Internal error`, no data), and the response discloses nothing of the request. The step serve
and the byte serve share the checks and the responses they write: `serveStep` is
`Serve.serve` with the answer of the provider asked once more, and a request that asks for
no effect is answered by both with the same bytes. PE-05 runs that over every committed
vector.

## What is checked

| Claim | Where |
| --- | --- |
| PE-01 the typed outcome and its transport | `effects.rs`, `StepLaws` |
| PE-02 the turn and the step | `effects.rs`, `StepLaws` (the turn read back, the faults, the words) |
| PE-03 perform, replay, done | `effects.rs` |
| PE-04 a request that fails a check performs no effect | `effects.rs` with a host that panics on any effect; `StepLaws` for the checks that precede the provider |
| PE-05 the step serve agrees with the byte serve | `effects.rs` over all vectors; `StepLaws` for the step of a handled request |
| PE-06 the byte serve never performs | `effects.rs`; `StepLaws` |
| PE-07 the tag is four bytes | `effects.rs`; `StepLaws` |
| PE-08 the laws are stated | `StepLaws` and the check of its theorem names in `effects.rs` |
| PE-09 the fixture provider requests no effect | `effects.rs`: the fixture imports none of the new modules; the conformance declaration names `conformanceRun`, whose only effect request is the store of a task table |
| TK-17 the task table is stored through one effect | `effects.rs` and `StepLaws` ([TASKS.md](TASKS.md#the-hosts-side)) |

The tests are falsifying evidence. The theorems in `StepLaws` are the kernel-checked part:
they are about the step and the wire as data, the pre-provider checks that end in `done`
without the provider being applied, and the byte serve's answer to an effect request.

## Planted defects

Each defect was planted alone in a copy of `src/StepServe.lex.tex`, with the lock and the
check made on the copy, and `lexlean verify --all` run on it. Each fails in `UorMcp.StepLaws`
with `LLV7002` (Lean rejected the module, the tactic `rfl` failed), at the theorem named. The
unplanted tree passes `verify --all` (the `verify` job).

| Planted defect | Change | Theorem that fails |
|---|---|---|
| the provider is called before a failing check | in `serveGatedStep`, the branch `Option.some refusal` serves the request with `serveOpenStep` instead of ending in `done (forbiddenResponse refusal)` | the law that a request failing the transport gate ends in `done` of the forbidden response (`src/StepLaws.lex.tex`, line 119 of the copy): `serveGatedStep info answer request (some refusal)` is not definitionally `done (writeResponse (forbiddenResponse refusal))` |
| an effect outcome is dropped on replay | in `readCompleted`, the outcome read for a tag is replaced by `0x` | the law on `readCompleted` at `succ` (line 48): the left side is not the read of the tag and of the length-prefixed outcome |
| the byte serve and the step serve disagree | in `stepOfHandled`, the case where no effect is requested returns `Serve.internalResponse` instead of `Serve.respond accepts handled` | the law that `stepOfHandled` with no effect requested is `done (writeResponse (respond accepts handled))` (line 215), failing in both the `ok` and the `error` case |

To reproduce one, make the change in a copy, then run `lexlean lock`, `lexlean check` and
`lexlean verify --all` in it.

## Limits

* A record of functions cannot be an exported root's parameter in LexLean 1.2, and
  `Extension.answerOf` is not an executable definition, so the oracle does not reach
  `answerOf`. `StepFixture` drives the three outcomes through `Extension.callAnswer`,
  `readAnswer` and `promptAnswer`, which `answerOf` calls for `tools/call`,
  `resources/read` and `prompts/get`; that `answerOf` calls them is the reading of its source
  that `the_three_outcomes_declare_the_constructor_and_answer_of_maps_each_to_the_refusal_pe_01`
  checks.
* The kernel cannot evaluate a slice of a byte literal (it stops at the maximum recursion depth),
  so the laws about reading a turn are stated on `readCompleted`, `turnAfter`, `afterCount` and
  `closed`, and the reading of concrete bytes is tested by `effects.rs`.
* The three declared production roots together exceed the 64 MiB `max_total_source_bytes` that
  `lexlean.toml` configured (67 944 171 bytes), because a production build generates the
  certificates of every declared root. The limit is 128 MiB.

## Regenerate

`stepFixtureRun` is the third root of `crates/lifecycle-oracle/generated/request.json`. After
any change to the sources, regenerate the packages as [LIFECYCLE.md](LIFECYCLE.md)
describes and update `conformance/core-identity.json` with the record that
`check_core_identity` prints when it differs.
