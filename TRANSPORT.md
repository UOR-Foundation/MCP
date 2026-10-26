# Streamable HTTP transport

The core decides the bytes, the `Content-Type` and the status of every response of the
Streamable HTTP transport of revision 2026-07-28: the event stream that carries a request's
progress notifications and its result, the single JSON object otherwise, and the refusal of a
request whose `Origin` or `Host` is not one the server allows (DNS rebinding). It is LexLean
source in language 1.2 (`src/*.lex.tex`); the host adapter (PrismPM issue 216) moves bytes
and nothing else. Issue #11; register rows SH-01 to SH-10.

Authorities: the Streamable HTTP transport page and the security best practices page of the
specification at commit `80041ab0a139a8f4fb3adbc99f7ae9113ea59671`; RFC 9110 (sections 8.3.1,
12.4.2, 12.5.1 and 15.5.4); the event stream format of the WHATWG HTML Standard, section 9.2
(`https://html.spec.whatwg.org/multipage/server-sent-events.html`); and the pinned conformance
scenarios `server-sse-multiple-streams` and `dns-rebinding-protection` of
`modelcontextprotocol/conformance` at `cf022cbb541f9d939068d7f84758b9728be7fc23`.

## Where it lives

| Path | What it is |
| --- | --- |
| `src/Transport.lex.tex` | the `Origin` and `Host` gate (`TransportRefusal`, `gateOf`), the reading of the `Accept` field (`acceptsStream`), and the event stream (`eventOf`, `streamOf`, `shouldStream`, `streamResponse`) |
| `src/TransportLaws.lex.tex` | kernel-checked theorems about the above and about the gate in front of dispatch; hand-maintained source like `HeaderLaws` |
| `src/Serve.lex.tex` | `serveRequest` is `serveGated` over `Transport.gateOf`: a refused request is answered by `forbiddenResponse` and the earlier pipeline is `serveOpen`, unchanged; `respond` turns a result with notifications into a stream |
| `src/Http.lex.tex`, `src/Frame.lex.tex`, `src/Status.lex.tex` | extended: `HttpRequest.allowedHosts`, the optional allow-list after the tool-call count, and the status 403 |
| `crates/lifecycle-oracle/generated/mcp_lifecycle`, `mcp_crowd` | LexLean's exports of the two roots, unedited |
| `crates/lifecycle-oracle/vectors/transport.json` | the requests of the two scenarios and of the streaming probes, with file and line at the pinned suite commit |
| `crates/lifecycle-oracle/tests/transport.rs` | SH-01 to SH-10 |
| `crates/lifecycle-oracle/src/lib.rs` | `encode_request_allowing`, `exchange_allowing`, and `parse_event_stream`, an independent reader of the WHATWG format that the tests use to read every stream |
| `crates/lifecycle-oracle/suite/conformance-cf022cbb541f9d939068d7f84758b9728be7fc23` | now ten vendored suite files: also `sse-multiple-streams.ts` and `dns-rebinding.ts` |

Regenerate the packages after any change to the sources, as [LIFECYCLE.md](LIFECYCLE.md)
describes; `generated/request.json` names both roots.

## What the core decides and what the host does

| The core (this repository) | The host adapter (PrismPM issue 216) |
| --- | --- |
| whether the answer is a single JSON object or an event stream, from `Accept`, the status and the notifications | binds the loopback interface only (specification: servers SHOULD bind to 127.0.0.1) |
| the bytes of the stream: every event, in order, and the result last | accepts a connection, runs one root call per POST and writes the response body as it is |
| `Content-Type`, `Cache-Control` and `X-Accel-Buffering` of a stream, and the status of every response | closes the stream after its last event; treats the client closing it as cancellation |
| whether `Origin` and `Host` are allowed, and the 403 and its body | puts its configured host names in the frame and the `Host` it received in the fields; terminates TLS and authenticates |
| that the answer to a request depends on its frame alone | one stream per connection, concurrency, back-pressure and timeouts |

"Multiple streams" means this: every POST is its own exchange and its own response stream
(specification, "Sending Messages" and "Receiving Messages"), and the core keeps nothing
between two exchanges, so the answer to a frame is a function of that frame (SH-03). Concurrent
POSTs, as the three of `server-sse-multiple-streams`, are therefore independent by
construction. The sockets that carry them are the host's.

## The response stream

A response is an event stream when all of these hold, and the single JSON object otherwise:

1. the request accepts `text/event-stream` (the next section);
2. the status is 200;
3. the answer has at least one notification to deliver before its result.

Decisions:

| Question | Choice |
| --- | --- |
| Why not stream every answer that accepts the stream | the specification lets the server choose per request ("either a single JSON object or an SSE stream"), the scenario `server-sse-multiple-streams` accepts JSON for every request ("SSE streaming is optional"), and a stream with one event adds nothing to the client; the suite's own requests always send `Accept: application/json, text/event-stream`, so streaming every answer would turn each existing vector into a stream |
| Why only 200 | a stream starts with its status line before the first event, so a refusal that the core decides before any work (all of them are) cannot follow a stream; every refusal keeps its status and its JSON body |
| The notifications of a client that does not accept the stream | not sent: progress is optional ("The server MAY send JSON-RPC notifications"), and the answer is the result alone as JSON |
| Events | one per message, in this order: the notifications as the tool produced them, then the result; the result is the last event and the host closes the stream after it ("The final JSON-RPC response SHOULD terminate the stream") |
| Event format | `data: ` + the compact JSON-RPC message + a line feed + a blank line (WHATWG 9.2.5 and 9.2.6); no `event`, `id` or `retry` field, because "Resumable SSE streams via `Last-Event-ID` are not supported"; no carriage return, no comment line |
| One data line | a compact JSON message holds no raw line feed or carriage return (the writer escapes controls), so one `data:` line per event is exact; `tests/transport.rs` checks every line of every stream |
| Headers | `Content-Type: text/event-stream`, `Cache-Control: no-cache` (WHATWG authoring notes) and `X-Accel-Buffering: no` (specification: servers SHOULD include it) |
| The `notifications` of the response frame | reserved and always empty; progress travels inside the body. The field stays so that a frame written before #11 is still read |
| Keep-alive comments | none: the core's streams end with the result. They belong to a stream the host holds open, such as `subscriptions/listen`, which is not registered |

## The Accept field

`Transport.acceptsStream` reads every `Accept` field (repeated fields combine, RFC 9110
section 5.3) as a list of comma-separated entries. An entry accepts the stream when its media
type is `text/event-stream`, compared without regard to case after optional whitespace
(section 8.3.1), and it has no `q` parameter of zero (`q=0`, `q=0.0`, `q=0.000`; section 12.4.2).

| `Accept` | Stream |
| --- | --- |
| `text/event-stream`, `application/json, text/event-stream`, `TEXT/EVENT-STREAM`, `text/event-stream ; q=1`, `text/event-stream;q=0.5` | yes |
| none, `application/json`, `*/*`, `text/*`, `text/event-streams`, `xtext/event-stream`, `text/event-stream;q=0`, empty | no |

Decisions: a wildcard does not select the stream, because the specification requires a client
to list both types and a client that lists neither gets the answer every client can read; a
missing or unacceptable `Accept` is not refused (no 406): the specification states no server
refusal for it, and the lifecycle and tool vectors of #7 and #8 send none. The `q` weight only
excludes (`q=0`); it does not rank the two types, because the server chooses.

## DNS rebinding

The specification: "Servers MUST validate the `Origin` header on all incoming connections to
prevent DNS rebinding attacks. If the `Origin` header is present and invalid, servers MUST
respond with HTTP 403 Forbidden. The HTTP response body MAY comprise a JSON-RPC error response
that has no `id`." The scenario `dns-rebinding-protection` sends `Host` and `Origin` naming
the same attacker (`evil.example.com`) and expects a 4xx, then `Host` and `Origin` naming the
server's own `host:port` and expects a 2xx.

`Transport.gateOf` judges the fields of the request before the method, the body or the
provider is read, so the refusal is the same for a GET, for text that is not JSON and for a
call that would be rate limited:

| Field | Absent | Present and allowed | Anything else |
| --- | --- | --- | --- |
| `Origin` | not refused | not refused | `forbiddenOrigin` |
| `Host` | not refused | not refused | `forbiddenHost` |

The `Origin` is judged first. A value is allowed when its host name is in the allow-list:
`localhost`, `127.0.0.1` and `[::1]` with any port, and the names the host adds to the frame.
`Origin` is `scheme://host[:port]` and `Host` is `host[:port]` (RFC 6454 and RFC 9110 section
7.2); the host is read up to the first colon, or to the closing bracket of an IPv6 literal, and
what follows must be empty or a colon and digits. Names compare without regard to case and the
optional whitespace around a value is not part of it. Refused as unjudgeable, like a foreign
host: the opaque origin `null`, a value with a path or user information (`http://localhost@evil`),
a port that is not digits or is empty, an `Origin` without `scheme://`, an unclosed bracket, an
empty value, a field repeated (an attacker may not choose which copy counts), and a value with
a control byte. A name that only starts like an allowed one (`localhost.evil.example`,
`127.0.0.1.evil.example`) is a different name and is refused.

The refusal is `TransportRefusal` (`forbiddenOrigin`, `forbiddenHost`), status 403, and a
JSON-RPC error with code -32000, the message `Forbidden: invalid Origin` or `Forbidden: invalid
Host`, no `data` (the allow-list is not disclosed) and no `id` member (the request was not read).
-32000 is in the range JSON-RPC leaves to the implementation (`-32019` to `-32000`; the MCP
codes are `-32099` to `-32020`); the schema types no error with that code, so tests check the
body as a `JSONRPCErrorResponse`.

| Decision | Choice |
| --- | --- |
| A missing `Origin` | accepted: the specification refuses one that "is present and invalid", and a client that is not a browser sends none (the scenario `server-sse-multiple-streams` sends none) |
| A missing `Host` | accepted: HTTP/1.1 requires one, so a request the adapter passes has it; the in-process vectors of #7 and #8 send none, and a browser attacker always sends one |
| Why `Host` too | the scenario sends both, the security best practices call for validating either, and a rebinding request carries the attacker's name in both |
| Port | not compared: the attacker's name is what differs under rebinding, and the port the host listens on is not the core's to know |
| Binding to loopback | the host's (PrismPM#216): no socket is the core's. The core refuses what reaches an interface the host bound by mistake only to the extent of the names it is given |

## The allow-list

The host's configured names ride in the request frame after the tool-call count: a four-byte
count, then each name as a four-byte length and the bytes, all big-endian, and nothing after
them. A name is a host name or an IPv6 literal in brackets with no port, as the host would
write it in a `Host` field; the loopback names need not be listed. A frame that ends right
after the tool-call count carries none, which is the frame of #7 and #8, so those frames and
their tests are unchanged; a count of zero is the same. A frame cut inside the list, with more
bytes after it, or with a count the bytes do not satisfy is a frame the core cannot read:
HTTP 500 with the fixed internal-error body (SH-08). The oracle writes the list with
`encode_request_allowing`.

## The streaming probes of server-stateless

LIFECYCLE.md listed ten requests of `server-stateless` as not run because they belong to the
transport. Their standing now:

| Lines | Probe | Standing |
| --- | --- | --- |
| 1037, 1144 | `subscriptions/listen` | vectors PL-02 and PL-03: the core does not register the method and answers 404 with -32601; the scenario's `listenRejected` (line 1007) reads that as a feature that is absent because PL-01 shows `server/discover` advertises no `listChanged` or `subscribe` capability |
| 1043, 1150 | the `tools/call` that mutates the tool list once the stream delivers its first frame | not reached: the stream is refused, so no frame arrives (`not_run`) |
| 1209, 1279 and 1215, 1285 | list-changed streams and their triggers | not reached: the scenario returns `skipped` at lines 1193 and 1263 because the capability is not declared (PL-01) (`not_run`) |
| 887 | `test_streaming_elicitation` | vector TS-10: the fixture lists the tool (TL-19) and answers a text result; nothing but the result is on the stream |
| 947 | `test_logging_tool` | vector TS-11: the fixture lists the tool (TL-19), declares no logging capability and writes no `notifications/message` |

This is the suite's own reading of a server that offers no subscription: PL-01 to PL-03 show the
core gives it the inputs that make the probes skip. Registering `subscriptions/listen`, a stream
the host holds open and writes to over time, is not part of #11's two capabilities and needs
state the core does not have.

## Kernel theorems

`TransportLaws.lex.tex` states, and `lexlean verify` checks:

* the gate, as examples by evaluation: the loopback names with and without ports and in any
  case, a configured name, a missing field, and every refusal of the list above;
* the `Accept` field: each row of the table above, and that a second `Accept` field is read;
* the event stream: `eventOf` is `data: `, the message and a blank line; `streamOf` of the
  messages is their events in order and of none is empty; a stream response carries its status,
  an empty frame notification list, `text/event-stream`, `no-cache` and `no`;
* `shouldStream` is false with no messages, false for a client that does not accept the stream,
  false for a status other than 200, and true for a 200 with messages that the client accepts;
* `serveGated` with a refusal is `forbiddenResponse` of that refusal for every provider and every
  request (a refused request never reaches dispatch, and its answer depends on the refusal
  alone), and with no refusal it is `serveOpen`, the pipeline of #7 and #8 (additivity);
* `streamed` leaves a response unchanged when there is no notification;
* the status of each refusal is 403 (`Status`), and -32000 is in the implementation range.

LexLean's proof backend cannot evaluate `utf8_encode` and `utf8_decode`, so these statements
are over lists of bytes. The reading of `Accept` and of the gate over text is covered by
`tests/transport.rs`, which runs LexLean's export.

## Evidence

| ID | Evidence |
| --- | --- |
| SH-01 | `tests/transport.rs`: the vectors SP-01 and TS-06 (the progress call of `tools-call-with-progress` asking for the stream), the exact bytes of SP-01 against the JSON answer of the same request, the headers, the framing read back by `parse_event_stream` and a line-by-line check; `TransportLaws` |
| SH-02 | the vectors SS-01 to SS-03; every `Accept` of the table; every status other than 200 with a stream accepted (400, 404, 405, 429, 500); `TransportLaws` |
| SH-03 | the vectors SS-01 to SS-03; four request frames run repeatedly and interleaved on eight threads, each answer equal to its answer alone |
| SH-04, SH-05 | the vectors DR-01 and DR-02; the loopback names; foreign origins and hosts; the order of the two checks; absent fields; a GET, text that is not JSON and a rate-limited call with a foreign `Origin` |
| SH-06 | the vectors PL-01 to PL-03 |
| SH-07 | the unjudgeable values of the list above, a repeated field, and field names in any case |
| SH-08 | a configured name allowed and refused, every proper prefix of a frame with an allow-list, trailing bytes, and an explicit count of zero |
| SH-09 | an `Accept` of 200 000 entries and an allow-list of 100 000 names on a 2 MiB stack; the structural checks of LC-16 cover the new modules |
| SH-10 | `tests/transport.rs` and `tests/provenance.rs`: every request site of the two scenarios, every check id, and every probe of `server-stateless` is a vector or is listed |

Every response any test obtains from the root, including each event of a stream, is validated
against the specification's JSON Schema as LC-15 states: a stream is checked event by event, the
notifications as `JSONRPCNotification` and `ProgressNotification` and the last event as the
response of its method.

## Planted defects

Each was planted alone in the sources of the tree of commit `c02af88`, run, and reverted with
`git checkout`. The first group is stopped by the kernel before an export is possible. The
second passes `lexlean verify` because no theorem covers it, and is stopped by the oracle after
`lexlean production build` exported the packages again (the request's `expect` pins were left
out for the plant, so the build did not stop at `LLC0204`). The third is stopped by the
provenance and register gates.

| Defect | Planted in | Gate | Command | Diagnostic |
| --- | --- | --- | --- | --- |
| `localhost` is not an allowed name | `Transport.hostAllowed`: the `localhost` condition replaced by `false` | kernel | `lexlean verify src/TransportLaws.lex.tex` | `LLV7002`, `Tactic rfl failed` in `localhost_is_allowed_without_a_port` (`TransportLaws.lex.tex:21`), `a_host_name_is_compared_without_regard_to_case` (:35) and `optional_whitespace_around_the_origin_is_not_part_of_it` (:56) |
| an event ends with one line feed, so a client never sees it dispatched | `Transport.eventEnd`: `[10, 10]` made `[10]` | kernel | same | `LLV7002`, `Tactic rfl failed` in `an_event_is_a_data_line_and_a_blank_line` (:308) and `the_events_of_a_stream_are_its_messages_in_order` (:322) |
| the `Host` is judged before the `Origin` | `Transport.gateOf`: the two problems swapped | kernel | same | `Tactic rfl failed` in `the_origin_is_judged_before_the_host` (:84) |
| a refused request is served anyway | `Serve.serveGated`: `some refusal` answered by `serveOpen` | kernel | same | `Tactic rfl failed` in `a_refused_gate_is_answered_by_the_refusal_alone` (:441) and `a_refusal_does_not_depend_on_the_provider_or_the_request` (:466) |
| a forbidden `Origin` is answered 400 | `Status.transportStatus`: `forbiddenOrigin` maps to `badRequestStatus` | kernel | `lexlean verify src/Status.lex.tex` | `Tactic rfl failed` in `a_forbidden_origin_is_forbidden` (`Status.lex.tex:258`) |
| the result comes before the notifications in a stream | `Serve.streamed`: the result appended ahead of the notes | oracle | `cargo test -p lifecycle-oracle --no-fail-fast` | passes `lexlean verify` (no theorem states the order of a stream with notifications); fails `a_request_that_accepts_the_stream_and_has_notifications_is_answered_as_a_stream_sh_01`, `a_stream_has_no_resumption_fields_and_every_line_is_a_data_line_sh_01`, `the_notifications_and_the_result_are_one_event_stream_for_a_client_that_accepts_it_tl_13`, `a_token_gets_increasing_progress_notifications_that_echo_it_tl_13`, `every_vector_response_validates_against_the_schema_lc_15` and two TL-15 tests: the last event is read as the response and is a notification |
| bytes after the allow-list are accepted | `Frame.afterCalls`: the end-of-frame comparison replaced by `true` | oracle | same | passes `lexlean verify`; `a_frame_cut_inside_the_allow_list_or_followed_by_bytes_is_unreadable_sh_08` (`transport.rs:660`): left 200, right 500 |
| a vendored scenario file is altered | `suite/.../dns-rebinding.ts`: one check id misspelled | pin and checksum | `cargo test -p lifecycle-oracle --test provenance` | `the_vendored_suite_is_the_pinned_commit_and_matches_its_recorded_checksum_lc_12`: `the vendored files are not the files the authority records`, left `b4a65fc1...b5db15`, right `74d3c617...e3d6` |
| a vector cites the wrong line | `vectors/transport.json`: DR-01 line 206 made 207 | coverage | `cargo test -p lifecycle-oracle --test transport` | `every_request_the_transport_scenarios_send_is_a_vector_sh_10` and `every_vector_and_listed_probe_of_the_transport_cites_the_pin_and_a_real_line_sh_10`: `"DR-01": line 207 of src/scenarios/server/dns-rebinding.ts does not contain "const response = await sendRequestWithHostAndOrigin("` |
| tests are renamed so none ends in `sh_06` | `tests/transport.rs` | meta-gate | `cargo test -p repo-conformance` | `CM-02: SH-06 is registered but no test name ends in `sh_06`. A claim with no test is an assertion.` |

Restored result: every defect was reverted with `git checkout`; on the final tree `lexlean
verify` (inside `lexlean production build`) reports the project verified, the packages equal a
fresh export, and `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `cargo xtask check-model`, `audit-limits` and
`audit-deferral` pass.

## Not met

* The acceptance of #11 is that the scenarios `server-sse-multiple-streams` and
  `dns-rebinding-protection` pass against the conformance build (#13). That build does not
  exist yet (it needs the generated host adapter of PrismPM#216) and the scenarios have not
  been run. Not met. What exists is the replay of the scenarios' requests against LexLean's
  export of `fixtureRun`, the checks the scenarios make on the answers, a read of every stream
  by an independent implementation of the WHATWG format, and a validation of every message
  against the specification's JSON Schema. That is falsifying evidence, not the suite's verdict:
  the suite reads streams with `eventsource-parser` and over real sockets.
* The host-side parts are not built here, and the core cannot decide them: binding to the
  loopback interface only, one stream per connection, closing a stream after its last event,
  treating the client closing a stream as cancellation, the host's configured names and the
  `Host` it received being passed in the frame, TLS and authentication (PrismPM#216).
* `subscriptions/listen`, list-changed notifications and `notifications/message` are not
  registered. The streaming probes of `server-stateless` that need them are not reached by the
  fixture (it advertises none). See [the probes](#the-streaming-probes-of-server-stateless).
* Whether the suite's `server-sse-multiple-streams` reports `SUCCESS` or `INFO` for its second
  check depends on the answer: with `tools/list` the core answers JSON, which the scenario
  reports as `INFO` ("SSE streaming is optional"); only an answer with notifications is a
  stream. A different reading of the specification ("either a JSON object or a stream") would
  stream every answer; this one does not, for the reasons in [The response stream](#the-response-stream).
* The universal statement that every request with a forbidden `Origin` is refused is a theorem
  about the structure of the pipeline (`serveGated`); the judgement of a name over text is
  checked by examples (`TransportLaws`) and by the tests, not proved for every byte string.
* `cargo deny` is not installed in this environment, and the prismpm steps cannot run without
  the SDK (`prismpm.lock` is absent); the `bootstrap` workflow fails on every pull request until
  #4 lands.

## Findings

* A byte-list literal of 60 or more elements still trips the nesting limit of the semantic
  module reader (LIFECYCLE.md, Findings; `LLT4001`, "recursion limit exceeded"). The expected
  stream of the event-stream theorem is written as appended chunks of 30 bytes for that reason.
* `linear_arithmetic` cannot split a list or a boolean: two laws about `shouldStream` that were
  written with it failed (`omega could not prove the goal`), and are examples by evaluation
  with the status or the client's acceptance fixed. A definition whose first condition is the
  fixed variable evaluates for every value of the others; the order of the conditions of
  `shouldStream` (acceptance, then messages, then status) is chosen for that.
* The frame's allow-list is optional at its end so that no frame of #7 or #8 changes. The
  ambiguity this admits is a frame cut exactly after the tool-call count, which is the frame of
  #7: it is read as carrying no allow-list. The cut anywhere inside the list is refused.
* The existing tests of TL-13 read the notifications from the frame. They read them from the
  stream now (`common::notes_of`), and the test `the_content_type_stays_json_and_the_body_is_the_result_tl_13`
  became two tests that state both answers. TL-13's register row changes its last clause from
  "their SSE encoding is issue 11" to the stream.
* `tests/boundary.rs` (LC-11) listed `Origin: http://evil.example` among the header fields the
  core must ignore. The core reads `Origin` and `Host` since this issue, so the test sends
  `Referer` instead; the other unrelated fields are unchanged.
