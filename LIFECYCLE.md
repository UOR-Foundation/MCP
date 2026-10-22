# Stateless lifecycle and HTTP header capabilities

The core serves `server/discover` and checks every request against the stateless
lifecycle of revision 2026-07-28, over an HTTP exchange that is modelled as data. It
is LexLean source in language 1.2. The protocol and tool semantics live in
`src/*.lex.tex` only; the host adapter (PrismPM issue 216) moves bytes and nothing
else. Issue #7; register rows LC-01 to LC-16 and HH-01 to HH-13.

`Validation` and `Dispatch` are extended, not duplicated, and the order of the checks
is unchanged: `_meta`, then headers, then protocol version, then route and capability,
then params.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Base64.lex.tex` | strict RFC 4648 decoding of a list of bytes |
| `src/Param.lex.tex` | the `Mcp-Param` comparison: text, integer and boolean parameters |
| `src/Http.lex.tex` | the exchange as data (`Field`, `HttpRequest`, `HttpResponse`), header fields read without regard to case, the Base64 sentinel, the headers of a request |
| `src/RpcWire.lex.tex` | JSON-RPC faults before a request exists, and the error and result messages as `Json` |
| `src/ResultJson.lex.tex` | the result bodies of `server/discover`, `tools/list` and `tools/call` as `Json` |
| `src/Status.lex.tex` | the HTTP status of every outcome, and its theorems |
| `src/Frame.lex.tex` | the boundary encoding: the request frame and the response frame |
| `src/Serve.lex.tex` | the pipeline from an encoded request to an encoded response |
| `src/Fixture.lex.tex` | the production root `fixtureRun` over a tools-only provider |
| `src/HeaderLaws.lex.tex` | kernel-checked laws for header names, values, sentinels and Base64 |
| `src/Validation.lex.tex`, `src/Dispatch.lex.tex`, `src/Extension.lex.tex`, `src/Refusal.lex.tex` | extended: `Headers` carries a header fault and the parameter headers, `Dispatch.handleWith` takes the provider's data and answer function, `ProviderInfo` and `ParamRule` |
| `crates/lifecycle-oracle/generated/mcp_lifecycle` | LexLean's export of `fixtureRun`, unedited |
| `crates/lifecycle-oracle/vectors/suite.json` | the vectors taken from the pinned scenarios |
| `crates/lifecycle-oracle/tests/schema.rs`, `tests/common/mod.rs` | LC-15: every response a test produces is validated against the specification's JSON Schema |
| `crates/lifecycle-oracle/tests/stress.rs` | LC-16: the largest frames the tests build, on a 2 MiB stack |
| `crates/lifecycle-oracle/suite/mcp-spec-80041ab0a139a8f4fb3adbc99f7ae9113ea59671` | the vendored `schema/2026-07-28/schema.json` and the licence |
| `crates/lifecycle-oracle/suite/conformance-232a9014457eaf4c59559916e3d616d2f7f28f05` | the five vendored suite files |

Regenerate the package after any change to the sources (it verifies the project first,
which takes about ten minutes on two cores):

```
lexlean production build --request crates/lifecycle-oracle/generated/request.json
```

with that request naming root `UorMcp.Fixture.fixtureRun`, package `mcp_lifecycle`
1.0.0, target `rust-std`. Copy `Cargo.toml`, `package.json`, `provenance.json` and
`src/lib.rs` from `.lexlean/production/<id>/packages/rust-std/mcp_lifecycle` to
`crates/lifecycle-oracle/generated/mcp_lifecycle`. A re-export of the committed sources
reproduces the committed package byte for byte: the build of the final sources (the
codec of #23 included) reported export `9c92f312995db81415b1fb650080a51f69c00cd5a54448fea048ee2a9546249a`
and `diff -r .lexlean/production/<id>/packages/rust-std/mcp_lifecycle
crates/lifecycle-oracle/generated/mcp_lifecycle` was empty. The package records the semantic ID of the sources it states
(`aad8fdc8e49153c7fceed7c181eae54878d0df7cd76db57119523b713c5a339a`). The lock was made
with LexLean `release/v0.4` at `9796067`.

## The boundary encoding

The root `fixtureRun(input : Bytes) : List Nat` maps an encoded request to an encoded
response. Both are first-order data: a sequence of bytes, every word of the answer below
256. This is the interface PrismPM issue 216's adapter satisfies; the oracle crate
(`crates/lifecycle-oracle/src/lib.rs`) does for a test what that adapter does for a socket.

A request frame, every length four bytes big-endian:

```
u32 method_length     method            the request method, for example POST
u32 target_length     target            the request target; the core never reads it
u32 field_count
  u32 name_length     name              repeated field_count times, in the order received
  u32 value_length    value
u32 body_length       body
```

and nothing after the body. A response frame:

```
u32 status
u32 field_count
  u32 name_length     name              repeated field_count times
  u32 value_length    value
u32 body_length       body
```

The core decides every status and every field of the response. The adapter:

* passes the request method, target, header fields (names and values as bytes, in the
  order received, repeated fields as repeated entries) and body unchanged, and writes the
  response status, fields and body unchanged;
* does not parse, trim, fold, join or reorder a header field;
* answers 403 for a bad `Origin` and checks `Accept` and `Content-Type` itself (issue
  #11); the core reads neither;
* bounds the header count and the body size, for memory (the heap the root uses is linear
  in the frame), and gives the root a stack of 2 MiB, the default of a spawned Rust thread.
  The stack the root needs does not depend on the frame ([The stack](#the-stack)): the
  export states no `recursion` effect and no function of the package calls itself except
  as a loop;
* treats a response with status 500 and the fixed internal-error body as the core's
  answer to a frame it could not read (a truncated frame or trailing bytes).

Header names are case-insensitive (RFC 9110 section 5.1). Repeated fields: for each
header the core reads (`MCP-Protocol-Version`, `Mcp-Method`, `Mcp-Name`, and each
recognized `Mcp-Param-*`) more than one field of that name is `headerMismatch`; other
repeated fields are ignored. A value is trimmed of leading and trailing space and
horizontal tab (RFC 9110 section 5.5) and must then be visible ASCII, space or
horizontal tab; any other byte makes the field malformed, which is `headerMismatch`.

## What the core decides

| Outcome | JSON-RPC code | HTTP status |
| --- | --- | --- |
| result | | 200 |
| text that is not JSON | -32700, no `id` member | 400 |
| a batch, notification, response, bad id, bad version, missing or non-text method, non-object params | -32600, no `id` member, or the id when it can be read | 400 |
| `_meta` missing, or lacking the protocol version or the client capabilities; params invalid | -32602 | 400 |
| a header absent, repeated, malformed or different from the body | -32020 | 400 |
| a protocol version the server does not support | -32022, `data.supported`, `data.requested` | 400 |
| a client capability the request needs and the client did not declare | -32021, `data.requiredCapabilities` | 400 |
| a method the server does not offer | -32601 | 404 |
| a request method other than POST | none; `Allow: POST` | 405 |
| a frame the core cannot read, or a message it cannot encode | -32603, no `id` member | 500 |

A kernel-checked theorem in `Status` states that the status follows the code
(`the_status_follows_the_code`), and one states each row (`a_header_mismatch_is_a_bad_request`,
`an_unknown_method_is_not_found` and the others). Their axiom policies are exact: seven
are axiom-free and seven, the ones about `statusForCode`, use `propext` only.

## Decisions

| Question | Choice |
| --- | --- |
| Notification over HTTP | the core defines none, so a notification POST is refused: 400, -32600, no `id` member |
| Non-POST | 405 with `Allow: POST`; the method is case-sensitive |
| Missing `MCP-Protocol-Version` | `headerMismatch`; the header is compared with the protocol version in `_meta` before the version is gated, so a header and body that agree on an unsupported version reach -32022 |
| `Mcp-Method` | required on every POST; compared exactly (values are case-sensitive) |
| `Mcp-Name` | required on `tools/call`, `resources/read` (the uri), `prompts/get` and the tasks methods (the task id) before the method is routed, so a missing header is -32020 even for a method the server does not offer; compared with the body value after the Base64 sentinel is decoded |
| Repeated standard or parameter header | `headerMismatch` (the issue does not say; a repeated field has no single value) |
| Malformed value (a control byte, DEL, a byte above 0x7E) | `headerMismatch`; non-ASCII must travel as the Base64 sentinel |
| Base64 sentinel | `=?base64?` + payload + `?=`, case-sensitive; a value without both marks is a literal |
| Base64 decoding | strict RFC 4648: padding required, the alphabet only, padding only in the last group, canonical trailing bits; the decoded bytes must be UTF-8 |
| `Mcp-Param-*` | only the headers a `ParamRule` names are recognized; others are ignored. A body value that is present and not null must be matched by the header; null or absent means the header is not expected (and one sent anyway is not compared, but is still refused when malformed) |
| Integer parameters | the header is a decimal integer with an optional minus sign; the body number must be a whole number within 2^53 - 1; the comparison is numeric, so `10.0` and `1e1` in the body match `10` |
| Boolean parameters | the header is exactly `true` or `false` |
| `paramRules` | first-order data supplied by the provider; deriving them from `x-mcp-header` in tool schemas is for #8 |
| Result bodies encoded | `server/discover`, `tools/list`, `tools/call` (text content blocks only); a body the encoder does not cover is a 500, so #8 to #12 each add theirs |
| Provider | a record of functions cannot be an exported root's parameter in LexLean 1.2, so `Dispatch.handleWith` takes `ProviderInfo` (data) and one answer function; `Dispatch.handle` is `handleWith` over a `Provider` and keeps every earlier theorem |

## Evidence

| ID | Evidence |
| --- | --- |
| LC-01 to LC-08 | `tests/vectors.rs` over the vectors of `server-stateless`; `tests/boundary.rs` for the JSON-RPC faults, the ids and the per-method cases |
| LC-09 | `Status` theorems; `tests/boundary.rs`: the 405 cases, the status of each refusal, the frames the core cannot read |
| LC-10 | `Validation` theorems on the order; `tests/boundary.rs`: one request with several faults at a time |
| LC-11 | `tests/boundary.rs`: targets and unrelated header fields leave the answer unchanged |
| LC-12 to LC-14 | `tests/provenance.rs` |
| LC-15 | `tests/schema.rs`; every response any test produces goes through `common::exchange_checked`, and a test fails if a file reaches the root another way |
| LC-16 | `tests/provenance.rs`: no call cycle in the package and no `recursion` effect; `tests/stress.rs`: the large frames on a 2 MiB stack |
| HH-01 to HH-08 | the vectors of `http-header-validation`; `HeaderLaws`; `tests/boundary.rs` |
| HH-09 to HH-13 | the vectors of `http-custom-header-server-validation`; `HeaderLaws`; `tests/boundary.rs` |

Commands:

```
lexlean fmt --check && lexlean lock --check && lexlean check && lexlean build && lexlean verify
RUSTUP_TOOLCHAIN=stable cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p xtask -- validate
```

## The vectors are falsifying evidence

`crates/lifecycle-oracle/vectors/suite.json` holds 32 vectors, one for each request that
`server-stateless`, `http-header-validation` and `http-custom-header-server-validation`
send to a server that implements the scope of #7, built as the scenario builds it, with
the checks the scenario makes on the answer. Each cites the scenario file, the suite
commit `232a9014457eaf4c59559916e3d616d2f7f28f05`, the line of the call, a quoted
fragment of that line and the check identifiers it stands for. `tests/provenance.rs`
checks every citation against the vendored files and checks that every call site and
every check identifier in those two files is a vector, a listed probe that is not run,
or a check the fixture does not reach.

A vector the root fails is a defect of the root. A vector it passes shows that this
request meets these checks in process. It is not the verdict of the suite: the suite
run against the conformance build (issue #13) is, and it has not been done.

The pin: the suite at commit `232a9014457eaf4c59559916e3d616d2f7f28f05` (2026-08-07),
whose `requirements/2026-07-28.yaml` has SHA-256
`ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57`. That file was
introduced at this commit and is unchanged at the suite's head (`c37eec8`). It is
absent at the commit that bumped the package to 0.2.0-alpha.10 (`468e535`); the file's
own comment names alpha.10 as its anchor, and between those two commits only lines
889 to 900 of `stateless.ts` changed, inside a streaming probe that is not run here.
The authority row `A-MCP-CONFORMANCE` records the commit and a manifest checksum over
the five vendored files.

Not run, with the owning issue (listed in the vectors file as `not_run`): the six
`listenToStream` probes of `server-stateless` (lines 887, 947, 1037, 1144, 1209 and
1279) and the four `tools/call` requests that trigger them (lines 1043, 1150, 1215 and
1285): streams, `subscriptions/listen` and list-changed notifications belong to the
transport (#11), and their triggers to #8 and #9. In the pinned requirements,
`server-stateless` is a required scenario; `http-header-validation` and
`http-custom-header-server-validation` are `not_scored`, reason `pending` (SEP-2243).

## The schema check

Every response that a test of `crates/lifecycle-oracle` obtains from the root is
validated, before the test reads it, against `schema/2026-07-28/schema.json` of the
specification at commit `80041ab0a139a8f4fb3adbc99f7ae9113ea59671` (vendored, with the
licence, under `crates/lifecycle-oracle/suite/mcp-spec-80041ab0a139a8f4fb3adbc99f7ae9113ea59671`;
authority row `A-MCP-SCHEMA-2026-07-28` records the commit, the git blob and a manifest
checksum, and `tests/schema.rs` recomputes it). The validator is the crate `boon` 0.6.1
(JSON Schema 2020-12), a dev-dependency of the oracle crate only, pinned in `Cargo.lock`;
its one licence beyond those `deny.toml` already allowed is `MIT-0`, added there with the
reason.

A response is checked as `JSONRPCResponse`; a success is also checked as the response the
schema names for the request's method (`DiscoverResultResponse`, `ListToolsResultResponse`,
`CallToolResultResponse`); an error is also checked as the typed error of its code
(`ParseError`, `InvalidRequestError`, `MethodNotFoundError`, `InvalidParamsError`,
`InternalError`, `HeaderMismatchError`, `MissingRequiredClientCapabilityError`,
`UnsupportedProtocolVersionError`). A response the schema refuses fails the test that
produced it, and a test fails if a test file calls `exchange`, `exchange_frame` or
`decode_response` without the check. `tests/schema.rs` also shows the check refuses what
it should: an error with `"id": null`, a discover result without `serverInfo` and
`resultType`, an unsupported-version error without `data.supported`, and a header
mismatch with another code.

This is falsifying evidence and nothing more. A validator can show that a response is not
the shape the specification names; it cannot show that the core is right, and it is not
the oracle for the protocol: MCP#6 binds that. The earlier by-hand check against the
suite's own wire schema (a node script, not run in CI) is removed. It had passed all 32
answers and did not catch the `"id": null` below, which the specification's schema
refuses: `JSONRPCErrorResponse.id` is optional and a `RequestId`, a string or a number, and
`null` is neither.

The id of an error response is therefore omitted when it is unknown. `RpcWire.idMember`
returns no member for `none`, and three kernel-checked theorems in `RpcWire` state it
(`an_error_without_an_id_has_no_id_member`, `an_error_with_an_id_carries_that_id`,
`an_error_response_is_an_object_of_its_members`; axiom-free). The 500 body for a frame the
core cannot read is a fixed byte string and is written without the member too.

## The stack

A stack overflow is an untyped failure, so the stack the root needs must be stated, and
it must not be a number nobody declared. The first export stated the `recursion` effect
and the oracle gave it 512 MiB. The recursions that grew with a structure were these
eight functions, found by reading the call graph of the generated package (a function
that calls itself, or a cycle of calls, is not a loop):

| Definition | Recursed over | Input or provider data |
| --- | --- | --- |
| `Decode.textMembers` | the members of the `arguments` of `prompts/get` and of the context of `completion/complete` | **input**, but not reachable through the fixture, which offers tools only (the method is refused as not found before its params are decoded) |
| `Http.paramHeaders` | the provider's `ParamRule`s | provider |
| `Frame.fieldsList` | the fields of the response | the core's (at most two) |
| `Refusal.textList` | the supported versions, the icon sizes, the required names | provider |
| `Envelope.iconsJson` | the icons of a server or tool | provider |
| `ResultJson.rolesJson`, `toolsJson`, `blocksJson` | annotation audiences, the tools, the content blocks | provider |

Every one is now an accumulating function over its argument whose self calls are all tail
calls (`textMembersOnto`, `paramHeadersOnto`, `fieldsListOnto`, `textListOnto`,
`iconsOnto`, `rolesOnto`, `toolsOnto`, `blocksOnto`), followed by one `reverseOnto`, itself a
loop. LexLean renders such a function as a loop (`RB-14`), so it uses no stack for the call.
No loop tail-calls another fallible function (the calls that can fail are scrutinees),
so the workaround of LexLean#279 was not needed. `Base64.groups` had the same shape with a worse problem: it
appended each decoded group to the end of an accumulating list, which is quadratic in the
header; a 768 KiB `Mcp-Param` header did not finish in ten minutes. It accumulates in
reverse now, like the others. The JSON codec reads and writes with an explicit stack in the
heap (JC-08), so it never recursed.

No declared bound was needed, so there is no 413 or 431 and no bound in the provider data:
the preferred way out, making every recursion a loop, was available for all eight.

What the adapter must give the root, as a function of the declared bounds, is therefore
a constant: there are no declared bounds that the stack depends on. The export states
`allocation` and `overflow`, and no `recursion`; `lexlean production build` refuses a root
whose declared effects are fewer than its analysis requires, so the declaration is a
checked claim and the first thing that would break if a recursion came back (the build
that exported this package declared them without `recursion` and was accepted).
`tests/provenance.rs` also searches the generated package for a call cycle, through
function values too, and fails on one. The only depth left is the nesting of a JSON value,
which the codec bounds at 512 levels (JC-08); reading it uses the heap, and dropping or
comparing such a value descends at most that far.

The structural checks are the evidence; the stress frames are not. The one recursion that
grew with the input, `textMembers`, is not reachable through the fixture, so
`tests/stress.rs` passed on the package of 4d8dacb too (a 7.8 MB frame with 400 000
arguments on a 64 KiB stack, and the other frames on 2 MiB). What the stress frames do is
guard the reachable paths against a recursion that arrives later: header scanning,
`Mcp-Param` decoding, the JSON read and drop at the deepest nesting, and the encoders.
`tests/provenance.rs` failed on that package, as it should: the effects were stated and
`f112` called itself.

How much stack that is, measured: the probe `the_stack_floor_probe` in `tests/stress.rs`
runs the same frames on a stack of a chosen size. The frames, scaled to a hundredth and a
tenth of the sizes in the tests, complete on 64 KiB and overflow on 48 KiB (the same
floor at both scales, which is the point; the oracle crate is compiled at opt-level 1, see
the workspace `Cargo.toml`). The 2 MiB the oracle and the adapter give is 32 times that.
The floor comes from the nesting of the JSON value, which the codec bounds at 512 levels,
and nothing else grows with the frame.

The heap is linear in the frame, and the adapter bounds the frame for that reason (header
count, body size), as the adapter of PrismPM#216 does for any HTTP server.

## Planted defects

Each was planted alone in the sources of a clean tree, run, and reverted. The first
group is stopped by `lexlean verify` before an export is possible. The second group
passes `verify` because no theorem covers it (the JSON encoders over `Json` and the
frame have none) or because the theorems that would catch it were left out of the
project, and is stopped by the oracle after `lexlean production build` re-exported the
package.

| Defect | Planted in | Gate | Command | Diagnostic |
| --- | --- | --- | --- | --- |
| a `tools/call` without `Mcp-Name` is accepted | `Validation.nameHeaderAgainst`: the absent-header branch returns no problem | kernel | `lexlean verify src/Validation.lex.tex` | `Tactic rfl failed` in `tools_call_needs_its_name_header` (`Validation.lex.tex:889`): `headerCheck (headersFor "2026-07-28" "tools/call" none) ...` is not definitionally equal to `some HeaderProblem.missingNameHeader`; the same in `tasks_update_needs_its_name_header` |
| a header name is compared case-sensitively | `Http.sameName`: `x =? y` instead of the lower-cased bytes | kernel | `lexlean verify src/HeaderLaws.lex.tex` | eight `rfl` failures, the first in `a_header_name_matches_in_lower_case` (`HeaderLaws.lex.tex:9`): `Http.sameName [77, 99, 112, ...] [109, 99, 112, ...]` is not definitionally equal to `true` |
| the -32022 error lacks the supported versions | `Refusal.dataOf`: `supported` dropped from the object | kernel | `lexlean verify src/Refusal.lex.tex` | `Tactic rfl failed` in `unsupported_version_error_data_lists_the_supported_versions` (`Refusal.lex.tex:267`): `dataOf (Refusal.unsupportedProtocolVersion requested supported)` is not definitionally equal to `some (Json.object [("supported", ...), ("requested", ...)])` |
| a result lacks `serverInfo` | `Dispatch.replyOf`: `serverInfo := none` | kernel | `lexlean verify src/Envelope.lex.tex` | `every_reply_names_the_server` (`Dispatch.lex.tex:552`) and `the_reply_names_the_serving_provider` (`:580`): `(replyOf (infoOf provider) body).meta.serverInfo` is not definitionally equal to `some provider.serverInfo` |
| -32020 is answered with 404 | `Status.statusOf`: `headerMismatch` maps to `notFoundStatus` | kernel | `lexlean verify src/Status.lex.tex` | `a_header_mismatch_is_a_bad_request` (`Status.lex.tex:69`): `statusOf (Refusal.headerMismatch problem)` is not definitionally equal to `400`; `the_status_follows_the_code` fails too |
| the same case-sensitive compare, with `HeaderLaws` left out of the project | `Http.sameName` | oracle | `cargo test -p lifecycle-oracle --no-fail-fast` | 11 of the 20 vector tests and 22 of the 48 boundary tests fail; `header_names_are_case_insensitive_hh_07`: `HV-06 (http-standard-headers.ts line 476): HTTP status` left 400, right 200, body `{"error":{"code":-32020,"message":"Header mismatch: MCP-Protocol-Version header is missing"},"id":105,...}` |
| the error object is written without `data` | `RpcWire.errorObjectJson` | oracle | same | 5 tests fail; `an_unsupported_version_is_refused_with_the_supported_list_and_the_echo_lc_04`: `ST-08: /error/data/supported is absent from {"error":{"code":-32022,"message":"Unsupported protocol version"},"id":301,"jsonrpc":"2.0"} (scenario "server-stateless", line 611)`; also `..._lc_07` (`ST-10`), `..._lc_08` and two boundary tests |
| a result is written without `resultType` and `serverInfo` | `ResultJson.resultObject`: the envelope members are left out | oracle | same | 9 tests fail; `every_result_carries_result_type_and_the_server_identity_lc_05`: `ST-04: /result/resultType is absent from {"id":105,"jsonrpc":"2.0","result":{"cacheScope":"private","capabilities":{"tools":{}},"supportedVersions":["2026-07-28"],"ttlMs":0}} (scenario "server-stateless", line 405)`, and `the_oracle_reaches_the_generated_package_lc_13` |
| an error response for an unknown id carries `"id": null` | `RpcWire.idMember`: `none` returns `[("id", null)]` | kernel | `lexlean verify src/RpcWire.lex.tex` | `LLV7002`, `Tactic rfl failed` in `an_error_without_an_id_has_no_id_member` (`RpcWire.lex.tex:274`): `Json.lookupIn "id" (errorMembers none error)` is not definitionally equal to `Json.Lookup.absent` |
| the same behaviour in the exported package (the package of 4d8dacb) | the committed package before this change | schema | `cargo test -p lifecycle-oracle --no-fail-fast` | 7 boundary tests fail, for example `a_batch_is_an_invalid_request_with_no_id_member_lc_08` and `a_frame_the_core_cannot_read_is_a_server_error_with_a_fixed_body_lc_09`: `the schema refuses the root's response (status 400): JSONRPCResponse: jsonschema validation failed with https://schema.invalid/mcp/2026-07-28/schema.json#/$defs/JSONRPCResponse`. The 20 vector tests passed, because the scenarios send ids, and so had the by-hand check against the suite's wire schema |
| a definition recurses again, and the root still declares no `recursion` | `Http.paramHeaders` restored to its non-tail form | effects | `lexlean check` (and so `lexlean production build`) | `LLT4005`: `production root UorMcp.Fixture.fixtureRun is not eligible for target rust-std: effect mismatch: construct declaration.definition.recursive realizes effect recursion, which the root does not admit` |
| a recursion in the exported package | the package of 4d8dacb | provenance | `cargo test -p lifecycle-oracle --test provenance` | `the_generated_package_is_the_recorded_export_of_the_lifecycle_root_lc_13`: the effects are `["allocation","overflow","recursion"]`, expected without `recursion`; `no_function_of_the_generated_package_is_on_a_call_cycle_lc_16`: `the package recurses: f112 -> f112` |
| the generated package is edited by hand | `generated/.../src/lib.rs` | provenance | `cargo test -p lifecycle-oracle --test provenance` | `src/lib.rs is not the file the export recorded: regenerate it, never edit it` |
| a vendored suite file is altered | `suite/.../stateless.ts` | pin and checksum | same | `the vendored files are not the files the authority records` |
| a vector cites the wrong line | `vectors/suite.json`, HV-04 line 411 made 412 | coverage | same | `"HV-04": line 412 of src/scenarios/server/http-standard-headers.ts does not contain "await this.testCase("` |
| tests are renamed so none ends in `lc_08` | `tests/*.rs` | meta-gate | `cargo test -p repo-conformance` | `CM-02: LC-08 is registered but no test name ends in lc_08. A claim with no test is an assertion.` |

Restored result: every defect was reverted with `git checkout`, and on the final tree
`lexlean verify` (inside `lexlean production build`) reports the project verified, the
re-export is byte-identical, and `cargo test --workspace`, `cargo fmt --all -- --check`
and `cargo clippy --workspace --all-targets -- -D warnings` pass.


## Not met

* The acceptance of #7 is that the scenarios `server-stateless`, `http-header-validation`
  and `http-custom-header-server-validation` pass against the conformance build (#13).
  That build does not exist yet and the scenarios have not been run. Not met.
* `server-stateless` cannot pass without the streaming probes: `subscriptions/listen`,
  notifications and logging are not registered (#11).
* The suite validates each response against its own wire schema before the scenario
  sees it. Here every response is validated against the specification's JSON Schema
  (LC-15) by another validator, which is falsifying evidence; the suite's validator has
  not been run on the answers since the by-hand check was removed.
* The tools in the fixture are a test double for the pinned scenarios. The tool layer is
  #8; `ParamRule`s are given, not derived from `x-mcp-header`.
* The universal statement "every request the root accepts has every check true" is not
  proved. The theorems are about the status mapping, the check order, the header and
  Base64 examples from the scenarios and the earlier model; the pipeline over bytes is
  checked by the vectors and the boundary tests.
* A successful `utf8_decode` is not kernel-evaluated (the primitive is opaque to the
  kernel), so no theorem states that a decoded header value is the expected text; the
  oracle covers it.
* `cargo deny` is not installed in this environment and the prismpm steps cannot run
  without the SDK (`prismpm.lock` is absent).

## Findings

* `lexlean.toml` `max_file_bytes` is 4 MiB; the certificate for the root is 5.0 MB, so
  `lexlean verify` refused it (`LLS8002`). The limit is now 8 MiB.
* The package manifest is read back through a JSON parser with a nesting limit of 128. A
  byte-list literal of 60 or more elements is a nested `cons` chain and tripped it
  (`LLI9001`, "recursion limit exceeded"); the 78-byte fixed internal-error body is three
  shorter lists appended.
* Reading a JSON string was quadratic in its length, and is not now (#23). `JsonText.listToBytes`
  built the bytes of a string with one `append` per byte, and an `append` of `Bytes` copies:
  in the stress run a frame with a 2 MiB string took 301 s and one with a 768 KiB string 41 s
  (0.07 s per 10^6 bytes squared). It now merges blocks the way a binary counter carries
  (`pushBlock`, `gatherBlocks`, `joinBlocks`): O(n log n) copying, and every self call is still
  a tail call. `crates/lifecycle-oracle/tests/linear.rs` (JC-13) sends a string of 256 KiB,
  1 MiB and 2 MiB, as an argument value and as an argument name, through this root and states
  the bound; `JSON-CODEC.md` gives the figures and the control.
  `Base64.groups` had the same shape in this issue's own code and was fixed earlier.
* `lexlean verify` peaks near 5.6 GB because it loads the JSON modules; it was run one
  module at a time, and an out-of-memory kill (exit 137) is the symptom of running two.
* `utf8_encode` and `utf8_decode` are opaque to the kernel, so header laws are stated
  over byte lists, and `asciiOf` and `textOf` are at the boundary only.
* The template's `audit-limits` rejects `Result<` in a shipped crate; the oracle crate is
  `publish = false`, like `json-oracle`.
