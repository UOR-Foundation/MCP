Feature: Prompts

  The LexLean core lists and gets the prompts of a provider's catalog, binds their arguments, and refuses a request or a result it must not answer, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @PM-01 @build
  Scenario: prompts/list answers with the prompts of the provider's catalog in their fixed declaration order, each with a name, a description and, where it declares arguments, the name, description and required flag of each; the catalog is one page, so a request that carries a cursor is refused as invalidParams (-32602); a kernel-checked theorem states the order.
    Given the fixture catalog of five prompts
    When prompts/list is requested, with and without a cursor
    Then the names are the catalog's in its declared order, every prompt has a name and a description, and a cursor is refused as -32602

  @PM-02 @build
  Scenario: prompts/get of a prompt without arguments answers with HTTP status 200 and a result of resultType complete whose messages are the prompt's, each with a role and a content block.
    Given the prompts-get-simple request
    When the root answers it
    Then the result has one user message whose content is the text block of the prompt

  @PM-03 @build
  Scenario: prompts/get of a prompt with arguments binds the supplied arguments in the order the prompt declares them and hands every one of them to the prompt, so that each appears in the messages the prompt builds; a kernel-checked theorem states that the binding keeps every supplied argument.
    Given the prompts-get-with-args request
    When the root answers it
    Then the message text carries both supplied values, arg1 and arg2 in that order

  @PM-04 @build
  Scenario: prompts/get answers a message whose content is an embedded resource carrying a uri, a mimeType and its text or a Base64 blob, in the order the prompt builds its messages.
    Given the prompts-get-embedded-resource request
    When the root answers it
    Then the first message is a resource block with the requested uri, the mimeType text/plain and its text, followed by the text message

  @PM-05 @build
  Scenario: prompts/get answers a message whose content is an image block carrying Base64 data and a mimeType, in the order the prompt builds its messages.
    Given the prompts-get-with-image request
    When the root answers it
    Then the first message is an image block with data and the mimeType image/png, followed by the text message

  @PM-06 @build
  Scenario: prompts/get of a prompt the catalog does not list, with a required argument missing, or with an argument the prompt does not declare is refused as invalidParams (-32602) with HTTP status 400 and the prompt does not run; the refusal echoes the request id, and kernel-checked theorems state each.
    Given an unlisted prompt, a call without a required argument and a call with an undeclared argument
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request

  @PM-07 @build
  Scenario: A provider result for prompts/get with a message whose image, audio or embedded blob payload is not canonical Base64 is refused as badPromptOutput: JSON-RPC code -32603 with HTTP status 500 and a fixed message that says nothing of the content.
    Given a prompt result with a payload that is not canonical Base64
    When the core settles it
    Then it is refused as badPromptOutput with -32603 and HTTP 500, and the message does not repeat the content

  @PM-08 @build
  Scenario: Every fault in a prompt catalog (an empty or duplicate prompt name, an empty or duplicate argument name) is a typed CatalogFault, a kernel-checked negative theorem states the refusal of each, and a provider with a faulty catalog answers every request with an internal error (-32603).
    Given the faults a prompt catalog can have
    When the catalog is admitted
    Then each fault is a typed CatalogFault and a negative theorem states it

  @PM-09 @build
  Scenario: The prompts layer states in kernel-checked theorems the order of prompts/list, the refusal of an unknown prompt, of a missing required argument and of an undeclared argument as invalidParams, and that the argument binding keeps every supplied argument, and the source declares each of them.
    Given the theorems of the prompts layer
    When the source is read
    Then each named theorem is declared
