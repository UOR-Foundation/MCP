Feature: HTTP header capabilities

  The LexLean core validates the standard and parameter headers of a request against its body, against RFC 9110, RFC 4648, the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @HH-01 @build
  Scenario: An MCP-Protocol-Version header that is absent, repeated, malformed or different from the protocolVersion in _meta is refused as headerMismatch (-32020) with HTTP status 400.
    Given requests whose header is absent, repeated, padded, empty or different from the body version
    When the root answers each
    Then a difference or fault is HTTP 400 with -32020 and optional whitespace alone is accepted

  @HH-02 @build
  Scenario: An Mcp-Method header that differs from the body method is refused as headerMismatch (-32020) with HTTP status 400.
    Given a tools/list request whose Mcp-Method header says prompts/list
    When the root answers it
    Then the answer is HTTP 400 with -32020

  @HH-03 @build
  Scenario: A request without an Mcp-Method header, or with a repeated, empty or malformed one, is refused as headerMismatch (-32020) with HTTP status 400.
    Given requests with the Mcp-Method header missing, repeated, empty, with a control byte and with a byte above the ASCII range
    When the root answers each
    Then each answer is HTTP 400 with -32020

  @HH-04 @build
  Scenario: An Mcp-Name header that differs from params.name, params.uri for resources/read, is refused as headerMismatch (-32020) with HTTP status 400; a Base64-wrapped Mcp-Name is decoded before the comparison.
    Given tools/call requests whose Mcp-Name differs, or is wrapped in the Base64 sentinel
    When the root answers each
    Then a difference is HTTP 400 with -32020, and a wrapped name that decodes to params.name is accepted

  @HH-05 @build
  Scenario: Optional whitespace around a header value is not part of the value, so a padded Mcp-Name equal to params.name is accepted.
    Given a tools/call request whose Mcp-Name is padded with spaces and tabs
    When the root answers it
    Then the answer is HTTP 200 with a result

  @HH-06 @build
  Scenario: Mcp-Name is required on tools/call, resources/read and prompts/get, and a request without it is refused as headerMismatch before the method is routed; it is not required on other methods.
    Given tools/call, resources/read and prompts/get requests without Mcp-Name, and server/discover and tools/list requests without it
    When the root answers each
    Then the three are HTTP 400 with -32020 and the two are accepted

  @HH-07 @build
  Scenario: Header names are compared without regard to case.
    Given requests whose header names are lower case, upper case and mixed case
    When the root answers each
    Then each answer is HTTP 200 with a result

  @HH-08 @build
  Scenario: Header values are compared exactly, so a method value that differs from the body method only in case is refused as headerMismatch (-32020) with HTTP status 400.
    Given a tools/list request whose Mcp-Method value is TOOLS/LIST
    When the root answers it
    Then the answer is HTTP 400 with -32020

  @HH-09 @build
  Scenario: A parameter header wrapped as =?base64?...?= is decoded before it is compared with the body value, and an unwrapped value is compared as it is.
    Given tools/call requests whose Mcp-Param-Region header is plain text, or a Base64 sentinel for ASCII, non-ASCII or line-feed text
    When the root answers each
    Then a header that decodes to the body value is accepted and any other is HTTP 400 with -32020

  @HH-10 @build
  Scenario: A Base64 parameter header with missing padding, characters outside the alphabet, padding inside the text or non-canonical trailing bits is refused as headerMismatch (-32020) with HTTP status 400.
    Given tools/call requests whose sentinel payload is malformed in each of those ways
    When the root answers each
    Then each answer is HTTP 400 with -32020

  @HH-11 @build
  Scenario: A value that only resembles the sentinel, with a missing closing mark, a missing opening mark or the wrong case, is a literal and is compared as written.
    Given tools/call requests whose Mcp-Param-Region header is such a value and whose body carries the same text
    When the root answers each
    Then each answer is HTTP 200 with a result, and the wrong-case value differs from a decoded one

  @HH-12 @build
  Scenario: A parameter header that is omitted while the body carries a value, that differs from it, or that is repeated or malformed is refused as headerMismatch (-32020) with HTTP status 400; when the body value is null or absent the header is not expected; an unrecognized parameter header is ignored.
    Given tools/call requests of execute_sql with the Mcp-Param-Region header omitted, wrong, repeated, malformed or not expected
    When the root answers each
    Then a violation is HTTP 400 with -32020 and a header that is not expected does not cause a refusal

  @HH-13 @build
  Scenario: An integer parameter header is compared numerically with the body number within the safe integer range, and a boolean parameter header is exactly true or false.
    Given tools/call requests of execute_sql with Mcp-Param-Limit and Mcp-Param-Dry-Run headers
    When the root answers each
    Then an equal number or the exact boolean is accepted and any other header is HTTP 400 with -32020
