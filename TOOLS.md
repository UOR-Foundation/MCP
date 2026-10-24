# Tools

The core lists and calls the tools of a provider's catalog, validates the arguments of a
call and the result of a tool against their JSON Schema, reports progress, limits the
rate of calls and sanitizes what it writes, for revision 2026-07-28. It is LexLean
source in language 1.2 (`src/*.lex.tex`); the host adapter (PrismPM issue 216) moves
bytes and nothing else. Issue #8; register rows TL-01 to TL-18; TL-19 (#13) adds the two diagnostic tools of `server-stateless`.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Numeric.lex.tex` | exact comparison of `Decimal`s, used by schema bounds and by the progress check |
| `src/JsonEq.lex.tex` | deep equality of two `Json` values on a worklist; object members compare without regard to order |
| `src/Schema.lex.tex` | a JSON Schema 2020-12 validator: a stack machine, so it never recurses |
| `src/Admission.lex.tex` | the admission of a catalog (`AdmitFault`), and `rulesOf`, which derives the `Mcp-Param` rules from `x-mcp-header` |
| `src/Sanitize.lex.tex` | the sanitization of text and of a `Json` tree, and the Base64 check of a payload |
| `src/Tools.lex.tex` | `tools/list`, `tools/call`, the argument, output and progress checks, and the rate-limit gate |
| `src/ToolLaws.lex.tex` | kernel-checked theorems about the above; hand-maintained source like the other modules, with no generator behind it |
| `src/Fixture.lex.tex` | the catalog of the pinned suite's fixture tools and the root `fixtureRun` |
| `src/Crowd.lex.tex` | a catalog of 60 generated tools and the root `crowdRun`, for the stress tests |
| `src/Methods`, `Dispatch`, `Refusal`, `Status`, `JsonRpc`, `ResultJson`, `RpcWire`, `Http`, `Frame`, `Serve`, `Extension` | extended: progress steps travel with a result, the new refusals have codes and statuses, the result encoders cover every content block, and the frames carry the rate-limit count and the notifications |
| `crates/lifecycle-oracle/generated/mcp_lifecycle`, `mcp_crowd` | LexLean's exports of the two roots, unedited |
| `crates/lifecycle-oracle/vectors/tools.json` | the requests of the tool scenarios, with file and line at the pinned suite commit |
| `crates/lifecycle-oracle/tests/tools.rs`, `stress.rs`, `provenance.rs` | TL-01 to TL-19 |
| `crates/lifecycle-oracle/suite/conformance-232a9014457eaf4c59559916e3d616d2f7f28f05` | now eight vendored suite files: also `tools.ts`, `json-schema-2020-12.ts` and `everything-server.ts` |

Regenerate the packages after any change to the sources, as [LIFECYCLE.md](LIFECYCLE.md)
describes; `generated/request.json` names both roots.

## What a call does

`tools/call` runs after the request has been read (`_meta`, headers, version, route and
params, as in LIFECYCLE.md), in this order:

1. **Rate limit.** If the host's count of calls is at or over the limit, the call is refused
   as `rateLimited` and nothing below runs ([The rate limit](#the-rate-limit)).
2. **Admit the catalog.** The provider's answer function admits the whole catalog before it
   serves any request that reaches it. A catalog with a fault is an internal error (-32603,
   HTTP 500) with a fixed message; the fixture's catalog has none.
3. **Find the tool.** A name the catalog does not list is `unknownTool`: JSON-RPC -32602,
   HTTP 400. (A request with no text `name`, or with `arguments` that are not an object,
   fails the params check and is -32602 as well.)
4. **Validate the arguments** (an absent `arguments` member is the empty object) against
   the tool's `inputSchema`. A failure is not a protocol error: the answer is a result with
   `isError: true` and one text block, `Invalid arguments: <pointer> fails <keyword>`,
   where the pointer is the RFC 6901 pointer of the first failing instance (`the arguments
   object` for the root) and the keyword is the schema keyword that failed. For
   `additionalProperties: false` the failing instance is the object that carries the
   undeclared member, not the member; for a composition (`anyOf`, `oneOf`, `not`) it is the
   instance the composition is applied to, with the composition's keyword; inside `if`,
   `then` and `else` it is the keyword that failed in the branch taken. The tool does
   not run.
5. **Run the tool.** The tool is a function from the call to a result and its progress
   steps, or to a refusal (for example `missingCapability`).
6. **Check the result.** Every image, audio and blob payload must be canonical Base64
   (RFC 4648); a tool that declares an `outputSchema` and does not fail must return
   `structuredContent` that validates against it; the progress steps must strictly
   increase. Any failure is `badToolOutput`: -32603, HTTP 500, message `Internal error`,
   and no `data`, so nothing of the tool's output reaches the client.
7. **Sanitize and write** the result ([Output sanitization](#output-sanitization)). The
   progress steps become notifications if, and only if, the request carries a
   `progressToken`.

The tools of a provider are data and a function: `handleWith(ProviderInfo, answer)` is
unchanged, and the fixture's `answer` is `Tools.answerList` and `Tools.answerCall` over its
catalog and its behaviour function. The catalog is `List Tool`, so a provider lists its
tools by listing them; the order of `tools/list` is the order of the list, and a theorem
states it for the fixture (`the_catalog_lists_its_tools_in_the_declared_order`), and a second
states that `tools/list` answers with that catalog unchanged
(`tools_list_answers_with_the_catalog_in_its_declared_order`). The
catalog is one page: a `cursor` is `invalidCursor` (-32602).

## Schemas

`Schema.validate` implements the JSON Schema 2020-12 vocabulary a tool schema needs:
`type` (a name or a list of names, `integer` accepting a whole number written with a
fraction), `enum`, `const`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`,
`minLength`, `maxLength` (in code points), `minItems`, `maxItems`, `minProperties`,
`maxProperties`, `required`, `properties`, `additionalProperties`, `items`, `allOf`,
`anyOf`, `oneOf`, `not`, `if`/`then`/`else`, and `$ref` to `#`, `#/$defs/<name>` or an
`#anchor` declared by `$anchor`. The validator is a stack machine with a fuel bound
(67 108 864 steps), so a schema cannot make it recurse or loop: a self-referencing schema
ends as `exhausted`, an internal error.

The keywords of the vocabulary that the validator does not implement (`pattern`,
`patternProperties`, `propertyNames`, `multipleOf`, `uniqueItems`, `contains`,
`minContains`, `maxContains`, `prefixItems`, `dependentRequired`, `dependentSchemas`,
`dependencies`, `additionalItems`, `unevaluatedProperties`, `unevaluatedItems`,
`$dynamicRef`, `$dynamicAnchor`, `$id`, `$vocabulary`, `contentEncoding`,
`contentMediaType`, `contentSchema`) are refused when the catalog is admitted
(`keywordUnsupported`), never ignored: a schema the core cannot enforce is not published.
Annotation keywords (`title`, `description`, `default`, `examples`, `format`, `$comment`,
`deprecated`, `readOnly`, `writeOnly`) and any `x-` keyword are carried and not enforced.
`tools/list` returns every keyword of the schema as the provider declared it, so the
focal schema of `json-schema-2020-12` comes back member for member.

## Admission

A catalog is admitted by the provider's answer function on every request that reaches it, and a fault is an
`AdmitFault` (a typed refusal, answered as an internal error with a fixed text for the
operator, never with the schema):

| Fault | When |
| --- | --- |
| `nameInvalid` | a name that is not 1 to 64 characters of A-Z, a-z, 0-9, `_`, `-` and `.`: the intersection of the specification (1 to 128, no solidus) and the suite's check (1 to 64, solidus allowed) |
| `nameDuplicate` | two tools with one name |
| `dialectUnsupported` | a `$schema` other than `https://json-schema.org/draft/2020-12/schema` |
| `rootNotObject` | an `inputSchema` whose `type` is not `object` |
| `keywordUnsupported`, `keywordMalformed` | a keyword in the list above, or a keyword whose value has the wrong shape |
| `referenceUnresolved` | a `$ref` that is not `#`, `#/$defs/<name>` or an `$anchor` of the schema |
| `headerNotText`, `headerEmpty`, `headerNotToken`, `headerType`, `headerUnreachable`, `headerDuplicate` | an `x-mcp-header` that is not text, is empty, is not an HTTP field-name token, is on a parameter that is not a string, integer or boolean, is not reached through `properties` alone from the root, or repeats another header of the tool without regard to case |

`Admission.rulesOf` returns the `Extension.ParamRule`s of a catalog: the header name, the
kind of the parameter and its path in the arguments (a nested annotation is reached
through its chain of `properties`). The fixture's `ProviderInfo.paramRules` is
`rulesOf(fixtureTools)`, so the `Mcp-Param-*` headers the HTTP layer enforces are exactly
the headers the catalog annotates, and nothing is supplied by hand.

## Output sanitization

The specification says servers MUST sanitize tool outputs and does not say what that
means. Here it is the following, applied to every string of a `tools/call` result (text,
URIs, names, structured content, member names) and of each progress notification, on the
whole `Json` tree just before it is written:

* a C0 control character other than tab (U+0009), line feed (U+000A) and carriage return
  (U+000D), the DEL character (U+007F), a C1 control character (U+0080 to U+009F), and the
  bidirectional embedding, override and isolate controls (U+202A to U+202E and U+2066 to
  U+2069) are each replaced by U+FFFD;
* every other character, including U+00A0, the line and paragraph separators, the other
  format characters and every character outside the Basic Multilingual Plane, is kept.

The choice is the one that stops a tool from moving a terminal cursor, writing an escape
sequence, or reordering the text a person reads, and changes nothing else. `Base64`
payloads are checked, not rewritten. Sanitization is not escaping: a JSON string is
escaped by the encoder as before.

## The rate limit

`ProviderInfo.toolCallLimit` is the parameter `rate.tool_calls`: the number of `tools/call`
requests a client may make in a window. The core cannot count, because it keeps no state,
so the count is an input: the request frame carries `tool_calls`, the number of calls the
host has already admitted for this client in the current window ([The boundary
encoding](LIFECYCLE.md#the-boundary-encoding)). The decision is a pure function,
`Tools.admits limit used = used < limit`, and a kernel-checked theorem states it at the
limit and below it. A call that is not admitted is refused as `rateLimited`: JSON-RPC code
429 and HTTP status 429 (RFC 6585 section 4), message `Rate limit exceeded: too many tool
calls`, `data.limit` the limit. 429 is outside the range the JSON-RPC specification
reserves (-32768 to -32000), so it does not collide with an MCP code (-32020 to -32099).
No `Retry-After` is sent, because the core does not know when the host's window ends; an
adapter that does may add it. Only `tools/call` is limited. The host owns the window, the
client identity and the increment; the fixture's limit is 100.

## Progress

A tool returns its progress steps with its result: a progress value, an optional total and
an optional message. If the request's `_meta` has a `progressToken` (text or number) the
core writes one JSON-RPC `notifications/progress` per step, with the token echoed, in
order; otherwise it writes none. The steps must be strictly increasing. The notifications
reach the client as the first events of an event stream when the client accepts
`text/event-stream`, with the result as the last event, and are not sent otherwise
([TRANSPORT.md](TRANSPORT.md)). The scenario `tools-call-with-progress` reads the
notifications from that stream; its vector asks for the stream and the tests read it back
with an independent reader of the WHATWG format.

## The fixture tools

| Tool | Behaviour | Source |
| --- | --- | --- |
| `echo`, `execute_sql`, `test_missing_capability` | the lifecycle fixture (#7) | `stateless.ts`, `http-standard-headers.ts` |
| `test_simple_text` | text `This is a simple text response for testing.` | `tools.ts` (description), `everything-server.ts` |
| `test_image_content` | a PNG | `TEST_IMAGE_BASE64` of `everything-server.ts` |
| `test_audio_content` | a WAV | `TEST_AUDIO_BASE64` of `everything-server.ts` |
| `test_embedded_resource` | `test://embedded-resource`, `text/plain` | `tools.ts` |
| `test_multiple_content_types` | text, image, embedded resource `test://mixed-content-resource` | `tools.ts` |
| `test_error_handling` | `isError: true`, the text the scenario names | `tools.ts` |
| `test_tool_with_progress` | steps 0, 50 and 100 of 100 | `tools.ts`, `everything-server.ts` |
| `json_schema_2020_12_tool` | the focal `inputSchema` | `JSON_SCHEMA_2020_12_FIXTURE` of `json-schema-2020-12.ts` |
| `test_arguments`, `test_nested_header`, `test_structured_output`, `test_bad_structured_output`, `test_bad_payload`, `test_unsafe_text`, `test_backward_progress` | tools of this repository that the register rows need: a schema with bounds, a nested header, structured output that conforms and that does not, a bad payload, text with controls, progress that goes back | none: not in the suite |
| `test_streaming_elicitation` | text `Streaming complete`, nothing else on the stream (#13) | `stateless.ts` line 887, `everything-server.ts` |
| `test_logging_tool` | text `Logging evaluated`; no `notifications/message`, with or without a `logLevel` (#13) | `stateless.ts` line 947, `everything-server.ts` |

The reference server writes a progress notification before the result of `test_streaming_elicitation`
and a `notifications/message` after a `logLevel` for `test_logging_tool`. The probes check neither: the
first fails on a frame that carries a method other than `notifications/*`, the second on a
`notifications/message` when no level was set. The core writes progress only for a request that carries a
`progressToken` (TL-09), declares no `logging` capability and so writes no log message, and asks nothing of
the client on the stream: a request for input is an input-required result in the answer (#10). The two
tools answer a text result in every case. `subscriptions/listen` stays unregistered; the five checks of
`server-stateless` that need it are skipped by the suite for a server that advertises no list-changed or
subscribe capability, which the specification schema makes optional (`ServerCapabilities`, `ToolListChangedNotification`:
"An optional notification").

## Kernel theorems

`ToolLaws.lex.tex` states, and `lexlean verify` checks, the order of `tools/list`, that an
unknown tool is `unknownTool` with code -32602, that an invalid argument is a tool result
with `isError` (and a missing, mistyped or undeclared one, and that a valid one is
executed), the rate-limit decision at, below and away from the limit, the typed result of
each failure of a tool's output, the verdict and the failing instance of every validator
keyword, and each fault of admission that does not depend on text encoding.

LexLean's proof backend cannot evaluate the primitives `utf8_encode`, `utf8_decode`, `join`,
`format_decimal`, `parse_decimal`, `split_exact` and `compare_bytes` (SPEC, the section on
`reflexivity`). A claim whose evaluation reaches one of them cannot be a theorem, and the
module states none. The code is split so that what can be stated is stated: `Admission`
checks a name through `nameProblemBytes` and a header through `headerTextStep` and
`loweredBytes`, all over lists of bytes, and `admitShape` is `admitTool` without the name
check. The theorems cover the byte-level functions and `admitShape`; the encoding step
(`bytesOf`), the rules derived for a whole catalog, the Base64 check, `$ref` through
`#/$defs/<name>` or an `#anchor`, `minLength` in code points and the JSON Pointer text are
covered by the tests of `tests/tools.rs` and `tests/stress.rs`, which run LexLean's export.

## Evidence

| ID | Evidence |
| --- | --- |
| TL-01 to TL-08, TL-13 | `tests/tools.rs` over the vectors of `vectors/tools.json` (the requests of `tools-list`, `tools-call-simple-text`, `tools-call-image`, `tools-call-audio`, `tools-call-embedded-resource`, `tools-call-mixed-content`, `tools-call-error` and `tools-call-with-progress`), and the probes beyond them; every response and every notification is validated against the specification's JSON Schema |
| TL-02 | the vector of `json-schema-2020-12`, and the focal schema's conditionals run by `tools/call` |
| TL-09 to TL-12, TL-14, TL-15 | `tests/tools.rs` |
| TL-16, TL-17 | `ToolLaws.lex.tex` (76 theorems) and `Tools.lex.tex` (2, the rate-limit decision), checked by `lexlean verify`, the limits of which are stated in [Kernel theorems](#kernel-theorems), and the tests that name them |
| TL-18 | `tests/stress.rs`, `tests/provenance.rs` |
| TL-19 | `tests/tools.rs` over the vectors TS-10 and TS-11 and the probes beyond them |

## Planted defects

Each was planted alone in the sources of a tree on which every gate passed, run, and
reverted with `git checkout`. The first three are stopped by the kernel, the fourth by the
oracle after the root was exported again.

| Defect | Planted in | Gate | Command | Diagnostic |
| --- | --- | --- | --- | --- |
| an unknown tool is answered as a tool result with `isError` | `Tools.answerCall`: the `Option.none` branch returns `textResult "Unknown tool" true` | kernel | `lexlean verify src/ToolLaws.lex.tex` | `LLV7002`, `Tactic rfl failed` in `an_unknown_tool_is_invalid_params` (`ToolLaws.lex.tex:87`, now :94 after the theorem below was added): `callFixture "no_such_tool" none` is not definitionally equal to `Except.error (Refusal.ProviderRefusal.unknownTool "no_such_tool")` |
| `tools/list` is out of order | `Tools.answerList`: the first tool moved to the end | kernel | same | `LLV7002`, `Tactic rfl failed` in `tools_list_answers_with_the_catalog_in_its_declared_order` (`ToolLaws.lex.tex:87`): `Tools.answerList Fixture.fixtureCache Fixture.fixtureTools { meta := probeMeta, cursor := none }` is not definitionally equal to `Except.ok (Methods.ResultBody.listTools { cache := Fixture.fixtureCache, nextCursor := none, tools := Fixture.fixtureTools })`. The theorem `the_catalog_lists_its_tools_in_the_declared_order` did not see it: it states the order of the catalog, not of the answer, and the planted defect passed `lexlean verify` until the second theorem was added |
| the rate limit is off by one | `Tools.admits`: `used <=? limit` for `used <? limit` | kernel | same | `LLV7002`, `Tactic decide proved that the proposition admits 3 3 = false ∧ admits 3 4 = false ∧ admits 0 0 = false is false` in `a_call_at_or_above_the_limit_is_refused` (`Tools.lex.tex:300`) |
| an image or audio block is written without `mimeType` | `ResultJson.dataBlockJson`: the `mimeType` member left out | oracle | `cargo test -p lifecycle-oracle --no-fail-fast` | passes `lexlean verify` (no theorem covers the encoder); 3 tests fail: `the_image_is_the_reference_fixtures_image_tl_04` (`TS-03: /result/content/0/mimeType is absent ... scenario "tools-call-image", line 302`), `the_audio_is_the_reference_fixtures_audio_tl_05` (`TS-07 ... "tools-call-audio", line 1057`) and `mixed_content_keeps_the_order_the_tool_produced_tl_07` (`TS-04: /result/content/1/mimeType is absent`) |

Restored result: every defect was reverted with `git checkout`, and on the final tree
`lexlean verify` reports 41 modules verified, the three packages equal a fresh export, and
`cargo test --workspace`, `cargo fmt --all -- --check` and `cargo clippy --workspace
--all-targets -- -D warnings` pass.
