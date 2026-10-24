# Resources, prompts, completion and caching

The core lists and reads the resources and resource templates of a provider's catalog,
lists and gets its prompts, completes the arguments of both, and tells a client how long
each cacheable result may be kept, for revision 2026-07-28. It is LexLean source in
language 1.2 (`src/*.lex.tex`); the host adapter (PrismPM issue 216) moves bytes and
nothing else. Issue #9; register rows RP-01 to RP-19.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Resources.lex.tex` | the answers that do not depend on the catalog: one-page lists, the outcome of a read, the lookup of a prompt and the check of its required arguments, and the completion object (the cut at 100, `total`, `hasMore`) |
| `src/ResourceFixture.lex.tex` | the catalogs of the pinned suite's reference fixture (`everything-server.ts`): four resources, one template, four prompts, the candidates of a completion, and the six answer functions `Fixture` routes to |
| `src/ResourceLaws.lex.tex` | kernel-checked theorems about the above; hand-maintained source like the other modules, with no generator behind it |
| `src/ResultJson.lex.tex` | extended: the result bodies of the six methods as JSON, with `ttlMs` and `cacheScope` on the four cacheable ones |
| `src/Refusal.lex.tex`, `src/Status.lex.tex` | extended: `data.uri` of the refusal of an unknown resource, `data.reason` of an invalid prompt argument, and theorems for the codes and statuses |
| `src/Fixture.lex.tex`, `src/Crowd.lex.tex` | extended: the fixture declares the `resources`, `prompts` and `completions` capabilities and routes the six methods; the crowd root keeps the tools-only capabilities it had |
| `crates/lifecycle-oracle/generated/prism_mcp`, `mcp_crowd` | LexLean's exports of the two roots, unedited |
| `crates/lifecycle-oracle/vectors/resources.json` | the requests of the twelve scenarios, with file and line at the pinned suite commit |
| `crates/lifecycle-oracle/tests/resources.rs` | RP-01 to RP-19; `provenance.rs` accounts for every request the four new scenario files send |
| `crates/lifecycle-oracle/suite/conformance-232a9014457eaf4c59559916e3d616d2f7f28f05` | now twelve vendored suite files: also `resources.ts`, `prompts.ts`, `caching.ts` and `utils.ts`, which holds `completion-complete` |

Regenerate the packages after any change to the sources, as [LIFECYCLE.md](LIFECYCLE.md)
describes; `generated/request.json` names both roots.

## What a call does

Every method below runs after the request has been read (`_meta`, headers, version, route
and params, as in LIFECYCLE.md), and only when the provider declares its capability:
`resources` for the first three, `prompts` for the next two and `completions` for the
last. A provider that does not declare it answers the method as method not found (-32601,
HTTP 404). `resources/read` and `prompts/get` need an `Mcp-Name` header, the uri and the
prompt name.

* **`resources/list`, `resources/templates/list`, `prompts/list`.** The catalog is one page.
  A `cursor` is `invalidCursor`: JSON-RPC -32602, HTTP 400. Otherwise the answer is the
  catalog in its declared order, with no `nextCursor`, and the cache hint of the list.
  `resources/list` holds the direct resources only; a template is listed by
  `resources/templates/list`.
* **`resources/read`.** The provider's lookup maps a uri to the contents it reads as.
  No contents is `resourceNotFound`: -32602, HTTP 400, message `Invalid params: resource not
  found`, and `data.uri` the uri that was asked for. The core never answers with an empty
  `contents` array: a listed resource that reads as no contents is an internal error (-32603,
  HTTP 500), because the specification makes an empty array ambiguous. Otherwise the answer
  is the contents as they are stored, text or Base64 blob, and the cache hint of a read.
* **`prompts/get`.** A name the catalog does not list is `unknownPrompt`. A prompt whose
  declaration marks an argument required (`required: true`) and whose call does not give it
  is `invalidArguments`, and `data.reason` is `missing required argument: <name>`. Both are
  -32602, HTTP 400. An argument the prompt does not declare is ignored. Otherwise the
  prompt's function answers its messages, and the text of every message is sanitized
  ([Sanitization](#sanitization)).
* **`completion/complete`.** The reference must name a prompt of the catalog or the resource
  template of the catalog, and the argument must be one that prompt or template declares;
  otherwise it is `unknownPrompt`, `resourceNotFound` or `invalidArguments` (-32602, HTTP
  400). The values are the candidates of that argument that begin with the text typed so
  far, compared as UTF-8 bytes, in the order of the candidates. At most 100 are returned
  (`Resources.completionLimit`), `total` is the number of matching candidates and `hasMore`
  is true exactly when there are more than 100. The `context` of the request is read and
  does not change the answer. A completion is not cacheable and carries no hint.
* **A malformed request.** A `ref` or `argument` that is missing or of the wrong type, a
  `uri`, `name` or `cursor` that is not text, and `arguments` that are not an object of
  texts are -32602, HTTP 400, as for any method.

The provider is data and functions, as for tools: `Fixture.fixtureServe` routes the six
requests to `ResourceFixture`, which applies the answers of `Resources` to the catalogs. A
provider gives the lists as `List`, so it lists its resources by listing them, and a theorem
states the order for the fixture for each of the three lists.

## The fixture

| Entry | Content | Source |
| --- | --- | --- |
| resources | `test://static-text` (`text/plain`), `test://static-binary` (`image/png`, the fixture's 1x1 PNG), `test://watched-resource` (`text/plain`, its initial content) and `test://stateless-static-text` (`text/plain`), in that order | `everything-server.ts`: the three `registerResource` calls with a uri and the stateless-only resource its draft handler appends |
| template | `test://template/{id}/data`, `application/json`, read as the canonical JSON text of `{"id": <id>, "templateTest": true, "data": "Data for ID: <id>"}` | `everything-server.ts`; the scenario `resources-templates-read` |
| prompts | `test_simple_prompt`; `test_prompt_with_arguments` (`arg1`, `arg2`, both required); `test_prompt_with_embedded_resource` (`resourceUri`, required); `test_prompt_with_image` | `everything-server.ts`; `prompts.ts` |
| completion | the candidates of `resourceUri` of the embedded-resource prompt (the uris of the four resources) and of `id` of the template (`123`, `124`, `456`); every other argument has none | the candidates are this repository's: the reference fixture answers every completion with no values |
| cache hints | lists: `ttlMs` 300000, `public`; reads: `ttlMs` 300000, `private` | `everything-server.ts` |

The reference fixture's stateless handler also lists `test_input_required_result_prompt`, a
prompt that answers `input_required`. That is the multi round-trip mechanism of issue #10,
and the prompt is registered there.

`watched-resource` changes every three seconds in the reference fixture and is read here
with its initial content: the core keeps no state, and `resources/subscribe` is removed
from 2026-07-28.

## Decisions

| Question | Choice |
| --- | --- |
| The code of an unknown resource | -32602 (specification MUST; the scenario `sep-2164-resource-not-found` calls it SHOULD and warns on -32002). `data.uri` is the requested uri, as the scenario and the specification's example show |
| A uri the template nearly matches | the template is one variable between a fixed start and a fixed end; the variable is not empty and holds no solidus, as in a path segment; matching is by bytes and case-sensitive. `test://template//data`, `test://template/a/b/data` and a different scheme are not found |
| Pagination | every catalog is one page, so a cursor is refused; the specification's `nextCursor` is never written |
| Cache hints | the lists are `public` (they do not depend on the caller), the reads are `private`; `tools/list` keeps the hint of #8. The values are the reference fixture's |
| `completion/complete` for an argument without candidates | an empty list with `total` 0 and `hasMore` false, which is the reference fixture's answer to every request |
| An undeclared prompt argument | ignored; only a required argument that is absent is refused |
| `inputResponses` and `requestState` on a read or a get | read and ignored here; the interim results are issue #10 |
| `resources` capability | `{}`: `subscribe` is removed from 2026-07-28 and the list-changed notifications are issue #11 |

## Sanitization

The text of a `prompts/get` result is sanitized as the text of a `tools/call` result is
([TOOLS.md](TOOLS.md#output-sanitization)): a prompt echoes the arguments a client sent, so
the characters that move a terminal cursor or reorder text are replaced by U+FFFD before the
messages are written. The contents of `resources/read` are written as the catalog stores
them, because they are the resource and not a message about it: a template read echoes the
uri it was asked for, and that is the uri's own bytes.

## Kernel theorems

`ResourceLaws.lex.tex` states, and `lexlean verify` checks, the order of the three lists
for the fixture, the refusal of an unknown resource as `resourceNotFound` with code -32602
for every cache hint and uri, that a resource with no contents is never answered with an empty
array, the refusal of an unknown prompt and of a prompt missing a required argument, the
first missing argument, the messages of the simple and the image prompt, the cache hints
and that each of the cacheable results is cacheable, that each encoder writes `ttlMs` and
`cacheScope`, the cut of a completion at 100 values with the count of what it cut and
`hasMore`, the refusals of a completion, and the capabilities the fixture declares and the
crowd root does not. `Refusal` and `Status` state the code, the status and the data of the
three new refusals.

LexLean's proof backend cannot evaluate `utf8_encode` and `utf8_decode` (TOOLS.md says why).
The matching of a template uri, the substitution into its text and the prefix test of a
completion all read a string as bytes, so none of them is a theorem; the tests of
`tests/resources.rs`, which run LexLean's export, cover them. The read of a uri is split so
that what can be stated is stated: `Resources.readOutcome` is a function of the lookup's
result, and `ResourceFixture.lookupStatic` is the lookup of the direct resources by string
equality.

## Evidence

| ID | Evidence |
| --- | --- |
| RP-01 to RP-11, RP-13, RP-15 | `tests/resources.rs` over the vectors of `vectors/resources.json` (the requests of `resources-list`, `resources-read-text`, `resources-read-binary`, `resources-templates-read`, `sep-2164-resource-not-found`, `prompts-list`, `prompts-get-simple`, `prompts-get-with-args`, `prompts-get-embedded-resource`, `prompts-get-with-image`, `completion-complete` and the five requests of `caching`), and the probes beyond them; every response is validated against the specification's JSON Schema |
| RP-04 | the request `resources/templates/list` of the caching scenario, and the probes |
| RP-12, RP-14, RP-16 | `tests/resources.rs`: a negative test for each refusal |
| RP-17 | `ResourceLaws.lex.tex`, and the named theorems of `Refusal` and `Status`, checked by `lexlean verify`; the test that names them |
| RP-18 | `tests/resources.rs`: the discover result of the fixture, the six methods it serves, and the crowd root that offers tools only |
| RP-19 | `tests/resources.rs` |

## The vectors are falsifying evidence

`vectors/resources.json` holds 16 vectors, one for each request the twelve scenarios send
to a server that implements the scope of #9, built as the stateless connection builds it
(`_meta` with the client info and the default client capabilities, and the standard headers,
with the uri or the name as `Mcp-Name`). Each cites the scenario file, the suite commit, the
line of the call and the check identifiers it stands for. `tests/provenance.rs` checks every
citation against the vendored files and that every call site in `resources.ts`,
`prompts.ts`, `caching.ts` and `utils.ts` is a vector or a listed request that is not run.
In `caching.ts` the call sites are the calls of `queryEndpoint`, the one function that
sends. A vector the root fails is a defect of the root; one it passes shows that this
request meets these checks in process. It is not the verdict of the suite: the suite run
against the conformance build (issue #13) is, and it has not been done.

Not run, with the reason (listed in the file as `not_run`): `resources/subscribe`,
`resources/unsubscribe`, `logging/setLevel` and `ping`, which the scenarios send and which
2026-07-28 removed; the core answers them as method not found and none of the four
scenarios is in the required list. The check `sep-2549-caching-connection` is emitted
only when the connection fails, which a request to the root cannot.

## Planted defects

Each was planted alone in the sources of a tree on which every gate passed, run, and
reverted with `git checkout`. The first six are stopped by the kernel before an export is
possible. The next two pass `lexlean verify` because no theorem covers the encoder or the
byte-level matching, and are stopped by the oracle after `cargo xtask export` exported the
root again. The last five are stopped by the gates that read the vendored files, the vectors,
the tests and the register.

| Defect | Planted in | Gate | Command | Diagnostic |
| --- | --- | --- | --- | --- |
| an unknown resource is refused as an internal error | `Resources.readOutcome`: the `none` branch returns `internal` | kernel | `lexlean verify src/ResourceLaws.lex.tex` | `LLV7002`, `Tactic rfl failed` in `an_unknown_resource_is_refused_as_not_found`: `Resources.readOutcome cache uri none` is not definitionally equal to `Result.error (resourceNotFound uri)`; `an_unknown_resource_has_the_invalid_params_code` fails too |
| `resources/list` is out of order | `ResourceFixture.resourcesOf`: the closing reversal left out | kernel | same | `LLV7002` in `the_catalog_lists_its_resources_in_the_declared_order`: `ResourceFixture.resourceUris` is not definitionally equal to the declared order |
| a completion is cut at 101 values | `Resources.completionLimit` 101 for 100 | kernel | same | `LLV7002` in `a_completion_is_cut_at_one_hundred_values`: `Resources.countOnto (Resources.completionOf (many 150)).completion.values 0` is not definitionally equal to `100` |
| a required prompt argument is not enforced | `Resources.isRequired`: `some flag => false` | kernel | same | `LLV7002` in `a_prompt_called_without_its_arguments_is_invalid_params` and `a_prompt_called_with_some_of_its_arguments_is_invalid_params`: `codeOfAnswer ...` is not definitionally equal to `some (-32602)` |
| a missing resource is answered with 404 | `Status.providerStatus`: `resourceNotFound` maps to `notFoundStatus` | kernel | `lexlean verify src/Status.lex.tex` | `LLV7002` in `a_missing_resource_is_a_bad_request`: `statusOf (provider (resourceNotFound uri))` is not definitionally equal to `400`; `the_status_follows_the_code` fails too |
| the refusal of a missing resource drops `data.uri` | `Refusal.dataOf`: `resourceNotFound` returns no data | kernel | `lexlean verify src/Refusal.lex.tex` | `LLV7002` in `a_missing_resource_names_its_uri`: `dataOf (provider (resourceNotFound uri))` is not definitionally equal to `some {"uri": uri}` |
| a template id may hold a solidus | `ResourceFixture.withoutSlash`: the solidus test left out | oracle | `cargo xtask export`, then `cargo test -p lifecycle-oracle --test resources` | 1 test fails: `a_uri_the_template_does_not_match_is_not_found_rp_05` (`test://template/a/b/data` is answered with contents, not -32602) |
| a prompt is written unsanitized | `ResultJson.bodyMembers`: the `getPrompt` arm without `sanitizedMembers` | oracle | same | 1 test fails: `control_and_bidirectional_characters_of_a_prompt_are_replaced_rp_19`: the text still holds `\u{7}`, `\u{1b}`, `\u{202e}` and `\u{85}` where U+FFFD is expected |
| a vendored suite file is altered | `suite/.../resources.ts` | pin and checksum | `cargo test -p lifecycle-oracle --test provenance` | `the_vendored_suite_is_the_pinned_commit_and_matches_its_recorded_checksum_lc_12`: `the vendored files are not the files the authority records` |
| a vector cites the wrong line | `vectors/resources.json`, RS-05 line 515 made 516 | coverage | same | `every_request_a_scenario_sends_is_a_vector_or_a_listed_probe_that_is_not_run_lc_14`: `"RS-05": line 516 of src/scenarios/server/resources.ts does not contain "result = await conn.request<ReadResourceResult>('resources/read', {"` |
| a listed request that is not run cites the wrong line | `vectors/resources.json`, the `ping` entry line 142 made 143 | coverage | same | same test: `line 143 of src/scenarios/server/utils.ts does not contain "const result = await conn.request<EmptyResult>('ping');"` |
| the tests of RP-12 are renamed so none ends in `rp_12` | `tests/resources.rs` | meta-gate | `cargo test -p repo-conformance` | `CM-02: RP-12 is registered but no test name ends in rp_12. A claim with no test is an assertion.` |
| the register changes and `CONFORMANCE.md` is not regenerated | `model/ids.toml`, the statement of RP-19 | model | `cargo xtask check-model` | `gate failed: CONFORMANCE.md is stale: it disagrees with model/ids.toml.` |

Restored result: every defect was reverted with `git checkout`, and on the final tree
`lexlean verify` reports the project verified, the committed packages equal a fresh export
(`cargo xtask export`), and `cargo fmt --all -- --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, the audits of `xtask` and `cargo test --workspace` pass.

## Not met

* The acceptance of #9 is that the twelve scenarios pass against the conformance build
  (#13). That build does not exist yet and the scenarios have not been run. Not met.
* The suite validates each response against its own wire schema before the scenario sees
  it. Here every response is validated against the specification's JSON Schema (LC-15) by
  another validator, which is falsifying evidence; the suite's validator has not been run
  on the answers.
* The candidates of a completion for `resourceUri` and for the id of the template are this
  repository's. The reference fixture has none, so no scenario exercises the matching,
  the order or the cut; the tests and the theorems do.
* The cut of a completion at 100 values is a theorem and not a test: no catalog of the
  fixture has more than four candidates.
* The reference fixture's prompt that answers `input_required` is not in the catalog (#10),
  and the notifications that a changed list sends are not written (#11).
* `cargo deny` and the PrismPM steps cannot run without the SDK (`prismpm.lock` is absent,
  #4).
