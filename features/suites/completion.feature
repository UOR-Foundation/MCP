Feature: Completion

  The LexLean core completes the arguments of the prompts and resource templates of a provider's catalog, at most 100 values at a time, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @CO-01 @build
  Scenario: completion/complete of an argument of a prompt (ref/prompt) or of a resource template (ref/resource) answers with HTTP status 200 and a result of resultType complete whose completion holds values, total and hasMore, where values are the provider's candidates that begin with the argument's current value, in the provider's order.
    Given the completion-complete request and a ref/resource request
    When the root answers each
    Then completion.values is an array of the candidates that begin with the typed value, in order, and total and hasMore are present

  @CO-02 @build
  Scenario: A completion holds at most 100 values; total is the number of candidates that match, and hasMore is true exactly when more candidates match than values are returned; kernel-checked theorems state the bound, the total and hasMore.
    Given a provider with more than 100 matching candidates
    When the root completes a prefix that they all share
    Then 100 values are returned, total is the number of matches and hasMore is true

  @CO-03 @build
  Scenario: The arguments already resolved, which a completion/complete request carries in context.arguments, reach the provider, so that its candidates may depend on them.
    Given a completion request with and without context.arguments
    When the root answers each
    Then the candidates differ with the resolved argument

  @CO-04 @build
  Scenario: completion/complete whose reference names a prompt the catalog does not list or a template the catalog does not list, or whose argument the prompt or the template does not declare, is refused as invalidParams (-32602) with HTTP status 400 and the provider is not asked; kernel-checked theorems state each.
    Given references to an unlisted prompt and an unlisted template, and arguments the prompt and the template do not declare
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request

  @CO-05 @build
  Scenario: The completion layer states in kernel-checked theorems the bound of 100 values, the total and hasMore of a page, the prefix rule and the refusals of an unknown reference and of an undeclared argument, and the source declares each of them.
    Given the theorems of the completion layer
    When the source is read
    Then each named theorem is declared
