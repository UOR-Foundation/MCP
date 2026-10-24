# Input-required results

When a tool needs something from the client, the core answers `tools/call`, `resources/read` or
`prompts/get` with an `InputRequiredResult` instead of a result, and the client retries the same
request with `inputResponses`. The server keeps nothing between the two requests: what it must
remember travels in `requestState`, which the core protects with an HMAC so that a client cannot
alter it. This is SEP-2322 of revision 2026-07-28, in LexLean source (`src/*.lex.tex`); the host
adapter (PrismPM issue 216) moves bytes and holds the key. Issue #10; register rows IR-01 to IR-19.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Sha256.lex.tex` | SHA-256 (FIPS 180-4), with the three FIPS examples as theorems |
| `src/Hmac.lex.tex` | HMAC-SHA-256 (RFC 2104), a comparison of two tags that looks at every byte, and the RFC 4231 test cases 1, 2, 3, 4, 6 and 7 as theorems |
| `src/RequestState.lex.tex` | what a `requestState` is: `seal` and `open` over bytes, hexadecimal, the key floor, and theorems for each way a state can fail |
| `src/InputGuard.lex.tex` | the guard around a provider's answer function: opens the state a request presents, validates `inputResponses`, then seals the state a result hands out and checks where and how a result may be input-required |
| `src/Interact.lex.tex` | the engine of a tool that may ask: a step is a finished result or a request for input; the argument check of `Tools` is shared |
| `src/InputFixture.lex.tex` | the eight tools and the one prompt of the conformance fixture that ask for input |
| `src/InputLaws.lex.tex` | kernel-checked theorems about those tools and the prompt, the capability gate and the encoders |
| `src/Refusal`, `Status`, `ResultJson`, `Http`, `Frame`, `Serve`, `Tools`, `Fixture`, `ToolLaws` | extended: five typed refusals with their codes and statuses, the JSON of an `InputRequiredResult`, the key in the request frame, the guard applied in `Serve`, `Tools.checkArguments` split out of `argumentsStep`, the eight tools in the catalog |
| `crates/lifecycle-oracle/generated/mcp_lifecycle`, `mcp_crowd` | LexLean's exports of the two roots, unedited |
| `crates/lifecycle-oracle/vectors/input-required.json` | the requests of the pinned scenarios, with file and line at the pinned suite commit |
| `crates/lifecycle-oracle/tests/input_required.rs` | IR-01 to IR-19 |
| `crates/lifecycle-oracle/suite/conformance-232a9014457eaf4c59559916e3d616d2f7f28f05` | now eleven vendored suite files: also `input-required-result.ts`, `input-required-result-helpers.ts` and `negative-mrtr.test.ts` |

Regenerate the packages after any change to the sources, as [LIFECYCLE.md](LIFECYCLE.md)
describes.

## What a request does

After the request has been read (`_meta`, headers, version, route and params, as in LIFECYCLE.md),
a `tools/call`, `resources/read` or `prompts/get` runs in this order:

1. **Open the state.** If the request carries a `requestState`, the guard opens it with the host's
   key, for this method and this tool name (or resource uri). A state that does not open is
   `invalidRequestState`, and nothing below runs. This is done whether or not the tool reads the
   state: the specification says to treat the whole of it as attacker-controlled input.
2. **Rate limit.** `tools/call` only, as in [TOOLS.md](TOOLS.md#the-rate-limit). A request whose
   state fails is refused before it is counted against the limit by the core; the host has already
   counted it, because it counts what it admits.
3. **Check the shape of `inputResponses`.** Every entry must be a JSON object, or the request is
   `invalidInputResponses`. Keys the server did not ask for are not looked at further.
4. **Run the provider.** The provider sees the request with the verified payload in place of the
   text the client sent. It never sees a state that failed.
5. **Settle the answer.** If the provider answered `input_required`: on a method that does not
   support it, that is `misplacedInputRequired`; with neither requests nor state, `emptyInputRequired`;
   with a state and no usable key, `requestStateKeyMissing`; otherwise the state is sealed.
6. **Check the capabilities.** Every request in `inputRequests` needs a client capability
   (elicitation, sampling, roots). If the client did not declare one, the answer is
   `missingCapability`, -32021, HTTP 400, as for any undeclared capability (LC-07), and the
   missing capabilities are in `data.requiredCapabilities`.
7. **Write** the result with `resultType` `input_required`, the server identity under `_meta`, and
   sanitized text.

Steps 1 to 5 are one function, `InputGuard.guardedGate`, which `Dispatch.run` applies to the provider's
answer function: it opens, then gates and runs the provider, then settles. A provider cannot skip it and
does not call it. The key reaches it as data, in `ProviderInfo.requestStateKey`, which `Serve` fills
from the frame; no function value wraps the provider (a wrapper of the same function type makes the
package's call graph cyclic, which `no_function_of_the_generated_package_is_on_a_call_cycle_lc_16` refuses).

## The `requestState`

The state a client sees is two lower-case hexadecimal strings joined by a full stop:

```
<hex of the payload> . <hex of the 32-byte tag>
tag = HMAC-SHA-256(key, len(method) || method || len(name) || name || payload)
```

with each length four bytes big-endian. The payload is whatever text the tool chose (the fixture's
tools write `request-state`, `multiple-inputs`, `1`, `2<name>`, `tamper-test`, `capabilities-test`). The
method and the name are the method of the request (`tools/call`) and the tool name, the resource
uri or the prompt name. So a state is valid for one method and one name, under one key, and for the
payload it carries, and for nothing else. Opening is strict: lower-case hexadecimal only, exactly one
full stop, a 32-byte tag, and the tags are compared by looking at every byte. The scenario's tamper
(appending text) is not hexadecimal and fails before the tags are compared; a flipped nibble fails at
the comparison.

**The key.** The issue names the secret reference `request-state-key`. The core keeps no secrets,
so the host supplies the key in the request frame, as it supplies the rate-limit count
([LIFECYCLE.md](LIFECYCLE.md#the-boundary-encoding)), and the core computes and checks the HMAC. A
key shorter than 32 bytes, the size of the hash, is no key: nothing is sealed with it and nothing is
opened with it. An adapter that wants the replay limits the specification asks for (a short expiry,
the authenticated principal) derives the key it sends from them: a key that is a function of the
principal and the time window makes a state from another principal or an old window fail
verification with no change to the core.

**What is not bound.** The state is not bound to the `arguments` of the request, to a principal or
to a time, because the core has none of the three: arguments could be compared but a client may
reorder an object's members between retries, a principal and a clock belong to the host. A tool that
needs the arguments to be the same in the next round puts what it needs in its payload. The
specification's single-use warning is the host's too: a state can be replayed within its key's life.

**Why the model has its own SHA-256.** `LexLeanStd.Sha256` exists in LexLean's standard library at the
pinned revision (SL-02), but a project imports a standard-library module as a *source package*, which
LexLean allows only for a project that is a member of a workspace and whose workspace file is in a
proper ancestor directory. This repository's project is its root. So `Sha256.lex.tex` is the
standard library's module adapted to this project (the same definitions, the same FIPS 180-4
theorems, no production-root markers), and `Hmac.lex.tex` is new. Both are cross-checked: the FIPS
and RFC 4231 digests are kernel-checked, and the oracle recomputes the tag of a sealed state with the
`sha2` crate's SHA-256 under its own HMAC.

## Refusals

| Refusal | When | JSON-RPC | HTTP | Data |
| --- | --- | --- | --- | --- |
| `invalidRequestState` | a presented `requestState` does not open: altered, extended, cut short, not hexadecimal, sealed for another method or name, sealed under another key, presented with no key | -32602 `Invalid params: requestState failed verification` | 400 | none |
| `invalidInputResponses` | an entry of `inputResponses` is not an object; an entry for a key the tool expects does not have the shape of its result type | -32602 `Invalid params: inputResponses is not a valid InputResponses object` | 400 | none |
| (decoder) | `inputResponses` that is not an object, or `requestState` that is not text | -32602 | 400 | none |
| `missingCapability` | a result asks for a capability the client did not declare | -32021 | 400 | `requiredCapabilities` |
| `misplacedInputRequired` | a provider answered `input_required` on a method other than `tools/call`, `resources/read`, `prompts/get` | -32603 `Internal error` | 500 | none |
| `emptyInputRequired` | a result with neither `inputRequests` nor `requestState` | -32603 `Internal error` | 500 | none |
| `requestStateKeyMissing` | a result with a state and no key of 32 bytes | -32603 `Internal error` | 500 | none |

The three internal errors say nothing of the cause: they are defects of the provider or of the
adapter's configuration, not of the client, and the client is told only that the server failed.
`invalidRequestState` does not say which check failed, so a client cannot learn from the message
whether it got the payload or the tag wrong.

## Decisions

| Question | Choice |
| --- | --- |
| A retry that lacks what the server needs | not a refusal: a new `InputRequiredResult` that asks again, and only for the keys still missing (the specification: SHOULD ask again rather than fail). The issue lists it among the refusals; the scenario `input-required-result-missing-input-response` and the specification say 200, so the typed outcome is the result, not an error. |
| Extra keys in `inputResponses` | ignored when they are objects |
| `inputResponses` of the wrong shape | refused, -32602, never completed (SHOULD validate; the scenario accepts an error or a new request, and the pinned reference answers `Hello, unknown!`, which the scenario scores as a warning) |
| An answer that declines or cancels | a valid ElicitResult, and the fixture's tool completes with `unknown` as the name, as the reference does |
| Keys of the requests | the scenario texts', `capital_question` and `client_roots`; the reference server uses `sample_request` and `roots_request`, and the scenarios read whatever key is first |
| State a tool does not read | verified anyway |
| Non-tool request | `prompts/get` of `test_input_required_result_prompt` (IR-08), served by `InputFixture.answerPrompt` and routed by `Fixture.fixtureServe` by its name. The prompt is not in the `prompts/list` of the resources fixture (#9), whose catalog and assertions stay as they are; the reference server lists it. `resources/read` is covered by the same guard |
| `tasks/result` | the suite's description lists it as supporting `input_required`; the specification page and `Methods.supportsInputRequired` do not, and the tasks method set of this core has no `tasks/result` (#12) |
| `resultType` | always written, `input_required` or `complete` |
| `mode` of an elicitation form | omitted when the tool does not set it, as in the scenario's example |

## The fixture tools

| Tool | First answer | Completes with |
| --- | --- | --- |
| `test_input_required_result_elicitation` | `user_name` by `elicitation/create` | `Hello, <name>!` |
| `test_input_required_result_sampling` | `capital_question` by `sampling/createMessage`, `maxTokens` 100 | `Sampling result: <text>` |
| `test_input_required_result_list_roots` | `client_roots` by `roots/list` | `Found <n> root(s)` |
| `test_input_required_result_request_state` | `confirm`, state `request-state` | `state-ok: requestState validated`, with the state and `ok: true` |
| `test_input_required_result_multiple_inputs` | `user_name`, `greeting`, `client_roots`, state `multiple-inputs` | `Name: <n>; Greeting: <g>; Roots: <k>`, asking again for what is missing |
| `test_input_required_result_multi_round` | `step1`, state `1`; then `step2`, state `2<name>` | `Multi-round complete for <name> who likes <color>` |
| `test_input_required_result_tampered_state` | `confirm`, state `tamper-test` | `integrity-ok: state verified` |
| `test_input_required_result_capabilities` | `elicit_input` if elicitation is declared, `sample_input` if sampling is | `capabilities-ok: received <keys>`, or `No supported capabilities declared` |

## Kernel theorems

`lexlean verify` checks, among others: HMAC-SHA-256 on the six RFC 4231 cases and SHA-256 on the
three FIPS examples; that a sealed state opens to its payload, and that a state with a changed tag or
payload, with text appended, sealed for another name, another method or under another key, without a
full stop, with a short tag, in upper case, or empty, does not; that a key under 32 bytes neither seals
nor opens; that a refused request never reaches the answer function (`a_refused_request_never_reaches_the_tool`,
for every answer function and every limit); that input-required is never sent on a method that does not support it, for all
twelve methods; that an empty result, a state without a key and a stray result are the typed refusals;
what each fixture tool asks and when it finishes; the capability gate; the code and the HTTP status of each new
refusal and that the status follows the code; and the JSON of the request kinds the fixture does not send.

LexLean's proof backend cannot evaluate `utf8_encode`, `utf8_decode` or `join`, so the multi-round tool, which
reads its state through them, has no theorem (the oracle covers it), and the state is stated over
bytes (`sealBytes`, `openBytes`) and the String wrappers (`sealState`, `openState`) and the text of a
finished result are covered by the oracle, not by theorems.

## Evidence

| ID | Evidence |
| --- | --- |
| IR-01 to IR-07, IR-09 to IR-14 | `tests/input_required.rs` over the vectors of `vectors/input-required.json` (the requests of the fourteen scenarios, 25 call sites, 23 of them run), and the probes beyond them; every response is validated against the specification's JSON Schema |
| IR-08 | the two vectors for `prompts/get`; `InputLaws.lex.tex` (the prompt asks, finishes, and refuses a response that does not fit); `tests/input_required.rs` (the guard on a prompt) |
| IR-15 | `Hmac.lex.tex` (RFC 4231) and `Sha256.lex.tex` (FIPS) theorems; `tests/input_required.rs`: an independent HMAC reproduces the RFC cases and the tag of every state the root writes |
| IR-16, IR-17, IR-18 | `tests/input_required.rs`, `InputGuard.lex.tex` |
| IR-19 | `InputLaws.lex.tex` (the encoders), `tests/input_required.rs` (the requests of the fixture against `ElicitRequest`, `CreateMessageRequest`, `ListRootsRequest`) |

Commands:

```
lexlean fmt --check && lexlean lock --check && lexlean check && lexlean verify
RUSTUP_TOOLCHAIN=stable cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p xtask -- check-model
```

## The vectors are falsifying evidence

`vectors/input-required.json` holds 25 vectors, one for each `sendRpc` call in
`input-required-result.ts` (the call in the loop of `unsupported-methods` is two vectors). A vector that
depends on an earlier answer names it: `$key:IV-01` is the first key that answer asked, `$state:IV-07` its
`requestState`, `$tampered:IV-20` that state with `-TAMPERED` appended, as the scenario builds it. Each cites
the scenario file, the suite commit, the line and a quoted fragment of it, and the check identifiers it
stands for. `tests/input_required.rs` checks every citation against the vendored file, that every call
site is a vector, and that the check identifiers of the file and of the vectors are the same set.

A vector the root fails is a defect of the root. A vector it passes shows that this request meets these
checks in process. It is not the verdict of the suite: the suite run against the conformance build (issue
#13) is, and it has not been done.

## Planted defects

PLANTED

## Not met

* The acceptance of #10 is that the fourteen scenarios pass against the conformance build (#13). That
  build does not exist yet and the scenarios have not been run. Not met.
* The suite validates each request and response against its own wire schema; here every response is
  validated against the specification's JSON Schema by another validator, which is falsifying evidence.
* Replay: the state is not bound to the arguments, a principal or a time ([The `requestState`](#the-requeststate)).
  The host binds them through the key, or the specification's single-use warning is the host's to meet.
* The model's SHA-256 is a copy, not an import; the first of the Findings says why.
* A successful `utf8_decode` is not kernel-evaluated, so no theorem states the text of a finished result
  or the string wrappers of the state; the oracle covers them.

## Findings

* A project at a repository root cannot import `LexLeanStd` as a source package (a workspace file must be in
  a proper ancestor of the project), so SHA-256 is carried here.
* The pinned reference server and the scenario text disagree on the keys of the sampling and roots requests
  (`sample_request` and `roots_request` against `capital_question` and `client_roots`); both pass, because
  the scenarios use the first key.
* `negative-mrtr.test.ts` runs three scenarios against a deliberately broken server, `sep-2322-mrtr-broken-server.ts`,
  that is not part of the vendored set; those tests are vendored for citation and are not run here.
* Glossary surfaces must not contain numerals (`LLR3006`), so the modules `Sha256` and `Hmac` are titled
  `Secure hash` and `Keyed message authentication`.
* A binder named `decide` is refused (`LLT4001`), as it would capture the Lean tactic.
* `serde_json` keeps object members sorted, so the oracle compares the keys of `inputRequests` as sets and
  reads their order from the body text.
