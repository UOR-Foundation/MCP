Feature: Streamable HTTP transport

  The LexLean core decides the bytes, the Content-Type and the status of every response, including the event stream and the refusal of a forbidden Origin or Host, against the MCP specification revision 2026-07-28, the WHATWG event stream format, and the pinned conformance scenarios server-sse-multiple-streams and dns-rebinding-protection. The host adapter moves bytes and nothing else.

  @SH-01 @build
  Scenario: A request that accepts text/event-stream and is answered with HTTP status 200 and progress notifications is answered as an event stream whose events are the notifications in order and then the result, each one data line ending in a blank line, with no id, event or retry field.
    Given a tools/call of test_tool_with_progress with a progress token and an Accept header that lists text/event-stream
    When the root answers it and a client reads the body as the WHATWG event stream format
    Then the Content-Type is text/event-stream, Cache-Control is no-cache, X-Accel-Buffering is no, and the events are three progress notifications that echo the token and the result last

  @SH-02 @build
  Scenario: An answer that is not an event stream is the single JSON object with Content-Type application/json, when the request does not accept the stream, when there is no notification to deliver, and for every status other than 200.
    Given a request with no Accept header, with Accept application/json, with text/event-stream at weight zero, a request that needs no notification, and a refused request that accepts the stream
    When the root answers each
    Then each answer is application/json, none is an event stream, and a client that does not accept the stream gets the result without the progress notifications

  @SH-03 @build
  Scenario: Every POST is its own exchange and its own response stream, so concurrent POSTs are independent.
    Given the three concurrent tools/list requests of server-sse-multiple-streams and three concurrent progress calls with different tokens and ids
    When the root answers them on separate threads
    Then each answer is HTTP 200, equals the answer to the same frame alone, and carries only its own request id and progress token

  @SH-04 @build
  Scenario: A request whose Origin is present and outside the allow-list is refused as forbiddenOrigin with HTTP status 403 and a JSON-RPC error with no id before anything else is read, and a request whose Host and Origin are allowed is served.
    Given the requests of dns-rebinding-protection with an attacker host and with the server's own loopback address
    When the root answers each
    Then the attacker request is HTTP 403 with error code -32000 and no id, and the loopback request is HTTP 200 with a result

  @SH-05 @build
  Scenario: A request whose Host is present and outside the allow-list is refused as forbiddenHost with HTTP status 403, the Origin is judged first, and a missing Origin or Host is not refused.
    Given a foreign Host with a loopback Origin, a foreign Host with a foreign Origin, and requests with neither field
    When the root answers each
    Then the first is a forbiddenHost 403, the second a forbiddenOrigin 403, and the others are served

  @SH-06 @build
  Scenario: The streaming probes of server-stateless that issue 7 left to this issue: subscriptions/listen is answered as methodNotFound while server/discover advertises no subscription capability.
    Given the server/discover request and the two subscriptions/listen requests the scenario sends
    When the root answers each
    Then the discover result advertises no listChanged or subscribe capability and each listen request is HTTP 404 with -32601

  @SH-07 @build
  Scenario: An Origin or Host the core cannot judge is refused like a foreign one, and a name that only starts like an allowed one is refused.
    Given the opaque origin null, an Origin with a path, with user information, with an empty port, without a scheme, an empty Host, an unclosed bracket, a repeated field, a control byte, and localhost.evil.example
    When the root answers each
    Then each is HTTP 403

  @SH-08 @build
  Scenario: The allow-list the host puts in the frame after the tool-call count adds names, and a frame cut inside it or followed by bytes is a frame the core cannot read.
    Given a request from a configured host name with and without that name in the frame, and frames cut inside the list and extended after it
    When the root answers each
    Then the name is allowed only when the frame lists it, and the broken frames are HTTP 500 with the fixed internal-error body

  @SH-09 @build
  Scenario: No function of the transport recurses except as a loop, so a very large Accept field and a very large allow-list are judged on the default stack.
    Given an Accept field of two hundred thousand media ranges and an allow-list of one hundred thousand names
    When the root answers on a 2 MiB stack
    Then the answers are correct

  @SH-10 @build
  Scenario: The transport scenarios and the streaming probes are accounted for.
    Given the vendored sse-multiple-streams.ts, dns-rebinding.ts and stateless.ts
    When every request site, check id and probe is compared with the vectors
    Then each is a vector or is listed with a reason and an owner
