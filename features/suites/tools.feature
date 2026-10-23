Feature: Tools

  The LexLean core lists and calls the tools of a provider's catalog, validates arguments and results against their JSON Schema, reports progress, limits the rate of calls and sanitizes what it writes, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @TL-01 @build
  Scenario: tools/list answers with the tool catalog in its fixed declaration order, every tool with a name of 1 to 64 characters from A-Z, a-z, 0-9, underscore, dot and hyphen (the intersection of the specification and of the suite check), a description and an inputSchema, and an outputSchema when it declares one; the catalog is one page, so a request that carries a cursor is refused as invalidParams (-32602); a kernel-checked theorem states the order.
    Given the fixture catalog of twenty-five tools
    When tools/list is requested, with and without a cursor
    Then the names are the catalog's in its declared order, every tool has the members and the name format, and a cursor is refused as -32602

  @TL-02 @build
  Scenario: The inputSchema and outputSchema of every listed tool are JSON Schema 2020-12 objects, and tools/list preserves every keyword the catalog declares, among them $schema, $defs, $anchor, $ref, allOf, anyOf, if, then, else and additionalProperties, as the scenario json-schema-2020-12 requires.
    Given the json-schema-2020-12 scenario's tools/list request
    When the root answers it
    Then the tool json_schema_2020_12_tool carries the focal inputSchema member for member

  @TL-03 @build
  Scenario: tools/call of a tool that returns text answers with HTTP status 200 and a result of resultType complete whose content is the text block the tool returned and which carries no isError member.
    Given the tools-call-simple-text request
    When the root answers it
    Then the result has the text block and no isError

  @TL-04 @build
  Scenario: tools/call of a tool that returns an image answers with an image content block carrying the Base64 data and its mimeType.
    Given the tools-call-image request
    When the root answers it
    Then the result has an image block with data and the mimeType image/png

  @TL-05 @build
  Scenario: tools/call of a tool that returns audio answers with an audio content block carrying the Base64 data and its mimeType.
    Given the tools-call-audio request
    When the root answers it
    Then the result has an audio block with data and the mimeType audio/wav

  @TL-06 @build
  Scenario: tools/call of a tool that returns an embedded resource answers with a resource content block whose resource carries a uri, a mimeType and either text or a Base64 blob.
    Given the tools-call-embedded-resource request
    When the root answers it
    Then the result has a resource block with its uri, mimeType and text

  @TL-07 @build
  Scenario: tools/call of a tool that returns several content types answers with its blocks in the order the tool produced them, text, image and embedded resource among them.
    Given the tools-call-mixed-content request
    When the root answers it
    Then the result has a text, an image and a resource block in that order

  @TL-08 @build
  Scenario: A tool whose execution fails answers with HTTP status 200 and a result with isError true and its message as content, and never with a JSON-RPC error.
    Given the tools-call-error request
    When the root answers it
    Then the answer is HTTP 200 whose result has isError true and a text message

  @TL-09 @build
  Scenario: tools/call of a tool the catalog does not list is refused as invalidParams (-32602) with HTTP status 400, and so is a tools/call whose params lack a text name or whose arguments are not an object; the refusal is a JSON-RPC error that echoes the request id, and a kernel-checked theorem states that an unknown tool is refused as unknownTool.
    Given a tools/call of an unlisted tool, one without a name and one whose arguments are not an object
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request

  @TL-10 @build
  Scenario: A tools/call whose arguments do not satisfy the tool's inputSchema (a missing required member, a member of the wrong type, an undeclared member where additionalProperties is false, a value outside a bound, an enumeration or a length) is answered with HTTP status 200 and a result with isError true whose text names the JSON Pointer of the failing instance and the failing keyword, and the tool does not run; a kernel-checked theorem states this.
    Given tools/call requests whose arguments miss a required member, have a member of the wrong type, add an undeclared member or leave a bound
    When the root answers each
    Then each answer is HTTP 200 with isError true and a text naming the pointer and the keyword, and the tool did not run

  @TL-11 @build
  Scenario: A tool that declares an outputSchema returns structuredContent that conforms to it, and a result whose structuredContent is missing or does not conform, whose Base64 payload is not canonical Base64, or whose progress does not increase is refused as an internal error (-32603) with HTTP status 500 and a fixed message that says nothing of the tool's output.
    Given tools whose structuredContent, Base64 payload or progress is bad
    When the root answers each call
    Then each answer is HTTP 500 with -32603 and a fixed message, and a conforming structuredContent is returned

  @TL-12 @build
  Scenario: The Mcp-Param rules the HTTP layer enforces are derived from the x-mcp-header annotations of the catalog, including an annotation on a nested property, and no rule is supplied by hand: the rules of the fixture catalog are exactly the headers its schemas annotate, and a catalog whose annotation is unreachable, duplicated or of an unsupported type is refused at admission.
    Given the catalog of the fixture
    When the parameter rules are derived and the headers of execute_sql and test_nested_header are checked
    Then the rules are the annotated headers, including the nested one, and a mismatching header is refused as headerMismatch

  @TL-13 @build
  Scenario: A tools/call whose _meta carries a progressToken is answered with the progress notifications the tool produced, in order, each a JSON-RPC notifications/progress whose progressToken is the request's token and whose progress is strictly greater than the one before; a request without a token gets none; the notifications travel beside the body in the response frame, and their SSE encoding is issue 11.
    Given a tools/call of test_tool_with_progress with and without a progressToken
    When the root answers each
    Then the notifications carry the token with increasing progress, or there are none

  @TL-14 @build
  Scenario: The core admits a tools/call if and only if the host's count of tool calls for the client in the current window, which the request frame carries, is below the limit the provider declares in rate.tool_calls, and refuses a call at or over the limit as rateLimited with JSON-RPC code 429 and HTTP status 429 (RFC 6585 section 4), with the limit in data.limit; no other method is limited; a kernel-checked theorem states the decision.
    Given tools/call requests sent with a count of calls below, at and above the limit of 100
    When the root answers each
    Then a count below the limit is served, and a count at or above it is HTTP 429 with code 429 and data.limit 100; tools/list is never limited

  @TL-15 @build
  Scenario: Every text of a tools/call result and of a progress notification is sanitized before it is written: C0 control characters other than tab, line feed and carriage return, DEL, C1 control characters and the Unicode bidirectional embedding, override and isolate controls (U+202A to U+202E, U+2066 to U+2069) are replaced by U+FFFD, and the other characters are kept.
    Given a tool that returns control and bidirectional characters
    When the root answers it
    Then each such character is U+FFFD in the answer and the rest of the text is unchanged

  @TL-16 @build
  Scenario: Every fault in a tool catalog (an invalid or duplicate name, an unsupported schema dialect, a root schema that is not an object, an unsupported or malformed keyword, an unresolved reference, an x-mcp-header that is not text, empty, not a token, unreachable, of an unsupported type or duplicated) is a typed AdmitFault, and a kernel-checked negative theorem states the refusal of each.
    Given the faults a catalog can have
    When the catalog is admitted
    Then each fault is a typed AdmitFault and a negative theorem states it

  @TL-17 @build
  Scenario: The tools layer states in kernel-checked theorems the order of tools/list, the refusal of an unknown tool as invalidParams, the isError result for an invalid argument and the rate-limit decision, and the source declares each of them.
    Given the theorems of the tools layer
    When the source is read
    Then each named theorem is declared

  @TL-18 @build
  Scenario: Tool calls with a very large arguments object, a very large text argument and a catalog of many tools are answered on the default 2 MiB thread stack, because no function of the generated package recurses except as a loop.
    Given a very large arguments object, a very large text argument and a catalog of many tools
    When the root answers each on the default stack
    Then each is answered and none overflows the stack
