Feature: Stateless lifecycle

  The LexLean core serves server/discover and the per-request lifecycle checks over HTTP, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @LC-01 @build
  Scenario: A request whose params lack _meta, or whose _meta lacks the protocol version or the client capabilities, is refused as invalidParams (-32602) with HTTP status 400 and the request's own id.
    Given the requests of the server-stateless scenario that omit _meta, omit the protocol version, and omit the client capabilities
    When the root answers each
    Then each answer is HTTP 400 with JSON-RPC error -32602 and the id of its request

  @LC-02 @build
  Scenario: server/discover answers with the supported protocol versions, the declared capabilities and the server identity, and tools/list answers when the tools capability is declared.
    Given a fixture that declares the tools capability
    When server/discover and tools/list are requested
    Then the discover result lists supportedVersions and capabilities, and the tool list is a non-empty array

  @LC-03 @build
  Scenario: A request whose _meta omits clientInfo is served, because the client identity is optional.
    Given a server/discover request whose _meta has the protocol version and the client capabilities only
    When the root answers it
    Then the answer is HTTP 200 with a result and no error

  @LC-04 @build
  Scenario: A request for a protocol version the server does not support is refused as unsupportedProtocolVersion (-32022) with HTTP status 400, listing the supported versions in data.supported and echoing the requested version in data.requested.
    Given a request whose _meta and MCP-Protocol-Version header both name v999.0.0
    When the root answers it
    Then the answer is HTTP 400 with -32022, data.supported within the discovered versions and data.requested equal to v999.0.0

  @LC-05 @build
  Scenario: Every successful result carries resultType and the server identity under _meta io.modelcontextprotocol/serverInfo, and is sent with HTTP status 200.
    Given every request of the pinned scenarios that is accepted
    When the root answers each
    Then each answer is HTTP 200 whose result has resultType complete and a serverInfo with a name and a version

  @LC-06 @build
  Scenario: A method the server does not offer, whether removed from the protocol, unknown, or gated on a capability the server did not declare, is refused as methodNotFound (-32601) with HTTP status 404.
    Given initialize, ping, logging/setLevel, resources/subscribe, resources/unsubscribe, unknown/method and prompts/list against a server that declares tools only
    When the root answers each
    Then each answer is HTTP 404 with JSON-RPC error -32601 and the id of its request

  @LC-07 @build
  Scenario: A request that needs a client capability the client did not declare is refused as missingRequiredClientCapability (-32021) with HTTP status 400, naming the requirement as a ClientCapabilities object in data.requiredCapabilities.
    Given a tools/call of test_missing_capability whose client capabilities are empty
    When the root answers it
    Then the answer is HTTP 400 with -32021 and data.requiredCapabilities.sampling is an object

  @LC-08 @build
  Scenario: Every refusal is a JSON-RPC error response that echoes the request id, or carries a null id when the text is not JSON (-32700) or is not a single request (-32600: batch, notification, response, bad id, bad version, missing or non-text method, non-object params).
    Given refused requests with string, number and absent ids, and texts that are not requests
    When the root answers each
    Then each error carries the id of its request, or null when none can be read

  @LC-09 @build
  Scenario: The core decides the HTTP status of every outcome: 200 for a result, 400 for -32700, -32600, -32602, -32020, -32021 and -32022, 404 for -32601, 405 with Allow POST for any request method other than POST, and 500 for a frame it cannot read; a kernel-checked theorem states that the status follows the code.
    Given requests that provoke each outcome, request methods other than POST, and frames that are truncated or have trailing bytes
    When the root answers each
    Then the status is the one the mapping states and a 500 carries the fixed internal error body

  @LC-10 @build
  Scenario: The checks run in a fixed order, _meta then headers then protocol version then route and capability then params, and an earlier fault is reported whatever later faults the same request carries.
    Given requests that carry two or more faults
    When the root answers each
    Then the error code is that of the earliest check in the order

  @LC-11 @build
  Scenario: The response depends only on the request method, the header fields the core reads and the body; the request target and every other header field leave it unchanged.
    Given one request sent with different targets and with unrelated header fields added
    When the root answers each
    Then the answers are byte for byte equal

  @LC-12 @build
  Scenario: The vectors are taken from the pinned conformance suite, whose requirements file for 2026-07-28 has the recorded checksum, and the vendored files match the recorded manifest checksum; the vectors are falsifying evidence and not the verdict of the suite run.
    Given the vendored suite files and the pin the register records
    When the oracle hashes them
    Then the manifest checksum, the requirements checksum and the pinned commit equal the recorded authority

  @LC-13 @build
  Scenario: The oracle executes the package generated from the LexLean lifecycle root, whose recorded digest and provenance match the committed files.
    Given the committed generated package and its provenance
    When the oracle compares them with the file it compiled
    Then the digests are equal and a discover request reaches the fixture identity

  @LC-14 @build
  Scenario: Every request that the pinned scenarios send is a vector or is listed as a probe that is not run with its owning issue, and every check identifier the scenarios emit is covered, listed as not run, or listed as unreached by the fixture.
    Given the request sites and check identifiers of the pinned scenario files
    When the oracle scans the vendored files
    Then each site and identifier is accounted for and each cited line contains its quoted text
