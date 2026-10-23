Feature: Resources

  The LexLean core lists and reads the resources and resource templates of a provider's catalog, answers a uri it does not know as invalid params, and refuses a result it must not write, against the MCP specification revision 2026-07-28 and the pinned conformance scenarios.

  @RS-01 @build
  Scenario: resources/list answers with the direct resources of the provider's catalog in their fixed declaration order, each with a uri and a name and, where the catalog declares them, a title, a description and a mimeType; the catalog is one page, so a request that carries a cursor is refused as invalidParams (-32602); a kernel-checked theorem states the order.
    Given the fixture catalog of four resources
    When resources/list is requested, with and without a cursor
    Then the uris are the catalog's in its declared order, every resource has a uri and a name, and a cursor is refused as -32602

  @RS-02 @build
  Scenario: resources/read of a text resource answers with HTTP status 200 and a result of resultType complete whose contents holds one entry with the requested uri, its mimeType and its text.
    Given the resources-read-text request
    When the root answers it
    Then the contents entry has the uri test://static-text, the mimeType text/plain and the text of the resource

  @RS-03 @build
  Scenario: resources/read of a binary resource answers with a contents entry that carries the uri, the mimeType and the Base64 data in a blob member and has no text member.
    Given the resources-read-binary request
    When the root answers it
    Then the contents entry has the uri test://static-binary, the mimeType image/png and a canonical Base64 blob, and no text

  @RS-04 @build
  Scenario: resources/templates/list answers with the resource templates of the provider's catalog in their fixed declaration order, each with a name and a uriTemplate and, where the catalog declares them, a title, a description and a mimeType; the catalog is one page, so a request that carries a cursor is refused as invalidParams (-32602); a kernel-checked theorem states the order.
    Given the fixture catalog of one resource template
    When resources/templates/list is requested, with and without a cursor
    Then the template test://template/{id}/data is listed with its name and a cursor is refused as -32602

  @RS-05 @build
  Scenario: resources/read of a uri that matches a resource template is answered by the provider with the variables the match bound, and the contents entry carries the requested uri; the template syntax is RFC 6570 level 1, a name between braces, and each variable takes the non-empty text between its neighbouring literals.
    Given the resources-templates-read request for test://template/123/data
    When the root answers it
    Then the contents entry has the requested uri and a text that carries the value 123 bound to id

  @RS-06 @build
  Scenario: resources/read of a uri that no resource of the catalog and no template of the catalog matches is refused as invalidParams (-32602) with HTTP status 400 and the requested uri in data.uri, never with a result and never with an empty contents array, and so is a resources/read whose params lack a text uri; the refusal echoes the request id, and a kernel-checked theorem states that an unknown resource is refused as resourceNotFound.
    Given the sep-2164-resource-not-found request and a resources/read without a uri
    When the root answers each
    Then each answer is HTTP 400 with -32602 and the id of its request, the first with data.uri equal to the requested uri and neither with a result

  @RS-07 @build
  Scenario: A uri is matched against a template in one pass without backtracking: an expression is an opening brace, a name of ASCII letters, digits and underscore, and a closing brace; each variable takes at least one byte, up to the first occurrence of the byte that follows it in the template, or the rest of the uri when it ends the template; the other bytes of the template must equal the bytes of the uri; kernel-checked theorems state the rule on byte lists.
    Given templates and uris written as byte lists
    When the matcher binds each uri to each template
    Then a uri that follows the template is bound variable by variable, and a uri with a missing literal, an empty variable, a trailing byte or a short tail is not matched

  @RS-08 @build
  Scenario: Every fault in a resource or template catalog (an empty uri or name, a duplicate uri, a duplicate template, an empty template, an unclosed or a stray brace, an empty or malformed variable name, or two variables with no literal between them) is a typed CatalogFault, a kernel-checked negative theorem states the refusal of each, and a provider with a faulty catalog answers every request with an internal error (-32603).
    Given the faults a resource catalog can have
    When the catalog is admitted
    Then each fault is a typed CatalogFault and a negative theorem states it

  @RS-09 @build
  Scenario: A provider result for resources/read with no contents entry, or with a blob that is not canonical Base64 (RFC 4648 section 4), is refused as badResourceOutput: JSON-RPC code -32603 with HTTP status 500 and a fixed message that says nothing of the content.
    Given a read result with an empty contents array and one with a blob that is not canonical Base64
    When the core settles each
    Then each is refused as badResourceOutput with -32603 and HTTP 500, and the message does not repeat the content

  @RS-10 @build
  Scenario: The resources layer states in kernel-checked theorems the order of resources/list and resources/templates/list, the refusal of an unknown resource as resourceNotFound with its uri in data, the refusal of an empty read result and the one-pass template rule, and the source declares each of them.
    Given the theorems of the resources layer
    When the source is read
    Then each named theorem is declared

  @RS-11 @build
  Scenario: server/discover declares the resources, prompts and completions capabilities beside tools when the provider offers them, and resources/list, resources/read, resources/templates/list, prompts/list, prompts/get and completion/complete are offered only to a provider that declares the capability.
    Given the fixture, which declares tools, resources, prompts and completions
    When server/discover is requested
    Then the capabilities name resources, prompts and completions, and the dispatch theorems state the gating

  @RS-12 @build
  Scenario: Reads of a very long uri, of a template with many variables and of a resource with a very large blob, and listings of a catalog of many resources, templates and prompts are answered on the default 2 MiB thread stack, because no function of the generated package recurses except as a loop.
    Given a very long uri, a template with many variables, a very large blob and a catalog of many resources, templates and prompts
    When the root answers each on the default stack
    Then each is answered and none overflows the stack
