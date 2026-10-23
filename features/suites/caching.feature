Feature: Caching

  The LexLean core writes the ttlMs and cacheScope of every cacheable result, and none on any other, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @CH-01 @build
  Scenario: Every cacheable result, that of server/discover, tools/list, resources/list, resources/templates/list, resources/read and prompts/list, carries a ttlMs that is a non-negative integer and a cacheScope that is public or private, as the scenario caching requires of five of them; a kernel-checked theorem states that a result is cacheable if and only if it has a cache hint.
    Given the caching scenario's requests for tools/list, prompts/list, resources/list, resources/templates/list and resources/read, and server/discover
    When the root answers each
    Then each result has an integer ttlMs of at least 0 and a cacheScope of public or private

  @CH-02 @build
  Scenario: The ttlMs and cacheScope of a result are the provider's own for that result: two reads of different resources may carry different hints, and the core writes them as given.
    Given the reads of the text resource and of the binary resource
    When the root answers each
    Then the hints are those the fixture declares for each, and they differ

  @CH-03 @build
  Scenario: A result that is not cacheable, that of tools/call, prompts/get, completion/complete, a task or an input-required result, carries neither ttlMs nor cacheScope.
    Given a tools/call, a prompts/get and a completion/complete
    When the root answers each
    Then no result has ttlMs or cacheScope

  @CH-04 @build
  Scenario: The caching layer states in kernel-checked theorems that cacheability and the cache hint agree for every result and that the members written for a hint are ttlMs and cacheScope, and the source declares each of them.
    Given the theorems of the caching layer
    When the source is read
    Then each named theorem is declared
