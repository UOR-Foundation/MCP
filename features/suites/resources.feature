Feature: Resources

  The LexLean core lists and reads resources and resource templates, lists and gets prompts, completes arguments and tells clients how long a result may be cached, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @RP-01 @build
  Scenario: resources/list answers the direct resources of the catalog in their declared order, each with a uri and a name and, when the catalog declares them, a title, a description and a mimeType; resource templates are not among them; the catalog is one page, so the answer carries no nextCursor; a kernel-checked theorem states the order.
    Given the resources-list request
    When the root answers it
    Then the result lists test://static-text, test://static-binary, test://watched-resource and test://stateless-static-text in that order, each with a uri and a name, and no template

  @RP-02 @build
  Scenario: resources/read of a text resource answers HTTP status 200 with a result of resultType complete whose contents hold one text entry carrying the uri that was read, its mimeType and its text.
    Given the resources-read-text request for test://static-text
    When the root answers it
    Then the contents are one text entry with the uri, the mimeType text/plain and the text of the reference fixture

  @RP-03 @build
  Scenario: resources/read of a binary resource answers one blob entry carrying the uri that was read, its mimeType and its content as canonical Base64, and the entry holds no text member.
    Given the resources-read-binary request for test://static-binary
    When the root answers it
    Then the contents are one blob entry with the uri, the mimeType image/png and the Base64 image of the reference fixture

  @RP-04 @build
  Scenario: resources/templates/list answers the resource templates of the catalog in their declared order, each with a name and a uriTemplate, and the catalog is one page.
    Given the resources/templates/list request of the caching scenario
    When the root answers it
    Then the result lists the template test://template/{id}/data with its name, title, description and mimeType

  @RP-05 @build
  Scenario: resources/read of a uri that a listed template matches substitutes the variable of the template into the uri and the text of the entry it answers, and a uri that matches the template with an empty variable or one that contains a solidus is not found.
    Given the resources-templates-read request for test://template/123/data
    When the root answers it
    Then the entry carries the uri that was read, the mimeType application/json and a text that holds the value 123, and the variants that do not match are refused as not found

  @RP-06 @build
  Scenario: resources/read of a uri that no resource and no template matches is refused as invalid params (-32602) with HTTP status 400, the message Invalid params: resource not found and the requested uri in data.uri, and it is never answered with a result whose contents array is empty; a kernel-checked theorem states the refusal.
    Given the sep-2164-resource-not-found request
    When the root answers it
    Then the answer is a JSON-RPC error -32602 with HTTP status 400 whose data.uri is the requested uri and which carries no result

  @RP-07 @build
  Scenario: prompts/list answers the prompts of the catalog in their declared order, each with a name, a description and, when it declares arguments, the arguments with their names, descriptions and required flags; the catalog is one page.
    Given the prompts-list request
    When the root answers it
    Then the result lists test_simple_prompt, test_prompt_with_arguments, test_prompt_with_embedded_resource and test_prompt_with_image in that order

  @RP-08 @build
  Scenario: prompts/get of a prompt without arguments answers its messages: one user message of text content for the simple prompt.
    Given the prompts-get-simple request
    When the root answers it
    Then the result has one user message whose content is the text This is a simple prompt for testing.

  @RP-09 @build
  Scenario: prompts/get of a prompt with arguments substitutes the arguments that were given into its messages.
    Given the prompts-get-with-args request
    When the root answers it
    Then the text of the message is the template of the reference fixture with arg1 and arg2 replaced by the arguments that were sent

  @RP-10 @build
  Scenario: prompts/get of a prompt with an embedded resource answers a user message of resource content whose resource carries the uri that was given as the argument, a mimeType and a text, followed by a user message of text content.
    Given the prompts-get-embedded-resource request
    When the root answers it
    Then the first message embeds a text resource with the uri sent as resourceUri and the second message is the instruction text

  @RP-11 @build
  Scenario: prompts/get of a prompt with an image answers a user message of image content carrying canonical Base64 data and a mimeType, followed by a user message of text content.
    Given the prompts-get-with-image request
    When the root answers it
    Then the first message is an image block of the reference fixture's image and the second message is the instruction text

  @RP-12 @build
  Scenario: prompts/get of a prompt the catalog does not list, and prompts/get of a prompt that is missing one of its required arguments, are each refused as invalid params (-32602) with HTTP status 400, the request id echoed and no result; the second refusal names the missing argument in data.reason; a kernel-checked theorem states both refusals.
    Given a prompts/get of an unlisted prompt, one without its arguments and one with only some of them
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request, and the missing argument is named

  @RP-13 @build
  Scenario: completion/complete answers a completion object whose values are the candidates of the referenced argument that begin with the text typed so far, in the order of the candidates and at most 100 of them, with total the number of matching candidates and hasMore true exactly when there are more than 100; a reference whose argument has no candidates gets an empty list; a kernel-checked theorem states the cut at 100.
    Given the completion-complete request and requests for the arguments that have candidates
    When the root answers each
    Then the values are an array, empty for test_prompt_with_arguments, and the matching candidates in order for the others, with total and hasMore

  @RP-14 @build
  Scenario: completion/complete is refused as invalid params (-32602) with HTTP status 400 when it references a prompt the catalog does not list, a resource template it does not list, or an argument the referenced prompt or template does not declare, and when its ref or argument is malformed.
    Given completion requests for an unlisted prompt, an unlisted template, an undeclared argument and with a malformed ref
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request

  @RP-15 @build
  Scenario: tools/list, prompts/list, resources/list, resources/templates/list and resources/read each answer with ttlMs, an integer that is not negative, and cacheScope, public or private, as the caching scenario requires; the lists are public and the reads are private; a refusal carries neither.
    Given the five requests of the caching scenario
    When the root answers each
    Then each result has ttlMs as a non-negative integer and cacheScope as public or private, and no refusal has either

  @RP-16 @build
  Scenario: A cursor on resources/list, resources/templates/list or prompts/list is refused as invalid params (-32602) with HTTP status 400, because each catalog is one page.
    Given the three list requests with a cursor
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request

  @RP-17 @build
  Scenario: The resources layer states in kernel-checked theorems the order of resources/list, resources/templates/list and prompts/list, the refusal of an unknown resource, of an unknown prompt and of a missing required argument as invalid params, the cache hints of the four new cacheable results, the cut of a completion at 100 values, and the capabilities the fixture declares, and the source declares each of them.
    Given the theorems of the resources layer
    When the source is read
    Then each named theorem is declared

  @RP-18 @build
  Scenario: server/discover declares the resources, prompts and completions capabilities exactly when the provider offers them, and a provider that does not declare one answers its methods as method not found (-32601) with HTTP status 404, so that the capabilities a discover result declares are the methods the root serves.
    Given the discover result of the fixture and the root over a catalog of tools only
    When the root answers each method
    Then the fixture declares the three capabilities and serves their six methods, and the tools-only root answers them as -32601

  @RP-19 @build
  Scenario: Every text of a prompts/get result is sanitized before it is written, as the text of a tools/call result is (TL-15), because a prompt echoes the arguments a client sent; the contents of resources/read are written as the catalog stores them.
    Given a prompts/get whose argument holds control and bidirectional characters
    When the root answers it
    Then each such character is U+FFFD in the message text and the contents of a resource are unchanged
