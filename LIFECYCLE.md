# Stateless lifecycle and HTTP header capabilities

The core serves `server/discover` and checks every request against the stateless
lifecycle of revision 2026-07-28, over an HTTP exchange that is modelled as data. It
is LexLean source in language 1.2. The protocol and tool semantics live in
`src/*.lex.tex` only; the host adapter (PrismPM issue 216) moves bytes and nothing
else. Issue #7; register rows LC-01 to LC-14 and HH-01 to HH-13.

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
| `crates/lifecycle-oracle/examples/dump_responses.rs`, `crates/lifecycle-oracle/scripts/wire_check.ts` | by hand: the answer to every vector, checked against the suite's own wire schema |
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
reproduces the committed package byte for byte: on the final tree, after every planted
defect was reverted, the build reported export `d8230f28a8f0de6dc63a4b48009a3a6703a5693adaaff25d5c443d38914a43cb`
and `diff -r .lexlean/production/<id>/packages/rust-std/mcp_lifecycle crates/lifecycle-oracle/generated/mcp_lifecycle`
was empty. The package records the semantic ID of the sources it states
(`6a7573437f1fe6eb99d44ffb2eee855f0c38eaf6bcc6fc5355eb9e052f5b1376`). The lock was made
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
* bounds the header count and the body size and gives the root a stack its input fits:
  the export states the `recursion` effect, so the depth is bounded only by the input
  (the codec bounds the nesting of the body at 512 levels), and the oracle runs the root
  on a 512 MiB stack;
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
| text that is not JSON | -32700, id null | 400 |
| a batch, notification, response, bad id, bad version, missing or non-text method, non-object params | -32600, id null, or the id when it can be read | 400 |
| `_meta` missing, or lacking the protocol version or the client capabilities; params invalid | -32602 | 400 |
| a header absent, repeated, malformed or different from the body | -32020 | 400 |
| a protocol version the server does not support | -32022, `data.supported`, `data.requested` | 400 |
| a client capability the request needs and the client did not declare | -32021, `data.requiredCapabilities` | 400 |
| a method the server does not offer | -32601 | 404 |
| a request method other than POST | none; `Allow: POST` | 405 |
| a frame the core cannot read, or a message it cannot encode | -32603, id null | 500 |

A kernel-checked theorem in `Status` states that the status follows the code
(`the_status_follows_the_code`), and one states each row (`a_header_mismatch_is_a_bad_request`,
`an_unknown_method_is_not_found` and the others). Their axiom policies are exact: seven
are axiom-free and seven, the ones about `statusForCode`, use `propext` only.

## Decisions

| Question | Choice |
| --- | --- |
| Notification over HTTP | the core defines none, so a notification POST is refused: 400, -32600, id null |
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

## Wire-schema check

`scripts/wire_check.ts` validates the answer to every vector against the wire schema of
the pinned suite (`src/validation/wire-schema.ts`, `wireSchemaErrors` for revision
2026-07-28), the check the suite makes on every response before a scenario reads it. It
is run by hand, against a checkout of the suite with its dependencies installed:

```
cargo run -q -p lifecycle-oracle --example dump_responses > responses.json
```

All 32 answers validate (0 violations). That is the same kind of evidence as the vectors
and no more: the suite run against the conformance build is the verdict.

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
* The suite validates each response against its wire schema before the scenario sees it.
  The vectors check the members the scenarios read, not the whole schema.
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
* `lexlean verify` peaks near 5.6 GB because it loads the JSON modules; it was run one
  module at a time, and an out-of-memory kill (exit 137) is the symptom of running two.
* `utf8_encode` and `utf8_decode` are opaque to the kernel, so header laws are stated
  over byte lists, and `asciiOf` and `textOf` are at the boundary only.
* The template's `audit-limits` rejects `Result<` in a shipped crate; the oracle crate is
  `publish = false`, like `json-oracle`.
