Feature: Input-required results

  The LexLean core answers tools/call, resources/read and prompts/get with an InputRequiredResult when a tool needs input from the client, protects the requestState it hands out with an HMAC, and refuses a retry whose state, responses or capabilities do not hold, against the MCP specification revision 2026-07-28 (SEP-2322) and the pinned conformance scenarios.

  @IR-01 @build
  Scenario: A tools/call of test_input_required_result_elicitation without inputResponses is answered with HTTP status 200 and an InputRequiredResult whose resultType is input_required and whose inputRequests holds one request, user_name, of method elicitation/create with a message and a requestedSchema; the retry with an accepted ElicitResult for user_name is answered with a complete result whose content is a text block.
    Given the test_input_required_result_elicitation tool of the input-required fixture
    When the tool is called without inputResponses and again with an accepted user_name
    Then the first answer is input_required with the user_name elicitation request and the second is a complete text result

  @IR-02 @build
  Scenario: A tools/call of test_input_required_result_sampling without inputResponses is answered with an InputRequiredResult whose one inputRequest is of method sampling/createMessage with a user message and maxTokens; the retry carrying the sampling result for that key is answered with a complete result that quotes the sampled text.
    Given the test_input_required_result_sampling tool
    When the tool is called without inputResponses and again with a CreateMessageResult for the key it asked
    Then the first answer is input_required with a sampling request and the second is a complete result with the sampled text

  @IR-03 @build
  Scenario: A tools/call of test_input_required_result_list_roots without inputResponses is answered with an InputRequiredResult whose one inputRequest is of method roots/list with empty params; the retry carrying a ListRootsResult for that key is answered with a complete result that counts the roots.
    Given the test_input_required_result_list_roots tool
    When the tool is called without inputResponses and again with a ListRootsResult for the key it asked
    Then the first answer is input_required with a roots/list request and the second is a complete result counting the roots

  @IR-04 @build
  Scenario: A tools/call of test_input_required_result_request_state is answered with an InputRequiredResult that carries inputRequests and a requestState text; the retry that echoes the requestState exactly and answers the request is answered with a complete result whose text contains state-ok, and a retry without the requestState is asked again rather than completed.
    Given the test_input_required_result_request_state tool
    When the tool is called, then retried with the echoed requestState, then retried without it
    Then the retry with the state is complete with state-ok and the retry without it is input_required again

  @IR-05 @build
  Scenario: A tools/call of test_input_required_result_multiple_inputs is answered with one InputRequiredResult whose inputRequests hold an elicitation, a sampling and a roots/list request under three distinct keys, with a requestState; the retry that answers all three and echoes the requestState is answered with a complete result.
    Given the test_input_required_result_multiple_inputs tool
    When the tool is called and then retried with an answer for every key and the echoed requestState
    Then the first answer holds three requests of three methods with a state and the retry is complete

  @IR-06 @build
  Scenario: A tools/call of test_input_required_result_multi_round is answered over three rounds: the first and the second are InputRequiredResults, each with inputRequests and a requestState, the requestState of the second differs from the first, and the third, retried with the answer to the second round and its requestState, is a complete result.
    Given the test_input_required_result_multi_round tool
    When the tool is called, retried with the first answer and state, and retried with the second answer and state
    Then rounds one and two are input_required with different states and round three is complete

  @IR-07 @build
  Scenario: A retry whose inputResponses lack a key the server needs, or carry only keys it did not ask for, is answered with HTTP status 200 and a new InputRequiredResult asking again, and only for the missing keys, never with a JSON-RPC error.
    Given the elicitation tool and the multiple-inputs tool
    When each is retried with inputResponses that name a wrong key or answer only some keys
    Then each answer is a 200 input_required that asks again for exactly the missing keys

  @IR-08 @build
  Scenario: A prompts/get of test_input_required_result_prompt without inputResponses is answered with an InputRequiredResult asking user_context by elicitation/create, and the retry carrying the answer is answered with a complete GetPromptResult that has messages, because InputRequiredResult is not specific to tools/call.
    Given a provider that offers prompts and the test_input_required_result_prompt prompt
    When the prompt is requested without inputResponses and again with the answer
    Then the first answer is input_required and the second is a complete GetPromptResult

  @IR-09 @build
  Scenario: Every InputRequiredResult carries the member resultType with the value input_required, the serverInfo of the serving provider under _meta as every result does, and conforms to the specification schema; a result that is complete carries the resultType complete.
    Given the answers of the input-required tools
    When each answer is read as JSON and checked against the specification schema
    Then an input_required answer names resultType and serverInfo and a kernel-checked theorem fixes the result type

  @IR-10 @build
  Scenario: InputRequiredResult is sent on tools/call, resources/read and prompts/get only: tools/list and prompts/list are never answered with resultType input_required, and a provider result of that type on any other method is refused as misplacedInputRequired, JSON-RPC code -32603, HTTP status 500, message Internal error and no data; kernel-checked theorems state both.
    Given tools/list, prompts/list and a provider that answers input_required on a method outside the three
    When each is answered by the root or by the guard
    Then no answer is input_required on an unsupported method and the stray result is a typed internal error

  @IR-11 @build
  Scenario: A requestState that fails verification, whether it was altered, truncated, extended, malformed, sealed for another method or tool, or sealed under another key, is refused as invalidRequestState with JSON-RPC code -32602, HTTP status 400, message Invalid params: requestState failed verification and no data, before the tool runs; a kernel-checked theorem states that a refused request never reaches the answer function.
    Given the test_input_required_result_tampered_state tool and its signed requestState
    When the state is altered, extended with text, shortened, replaced by the state of another tool and sent under no key
    Then each retry is a 400 with -32602 and the tool is not run

  @IR-12 @build
  Scenario: A tool that answers with inputRequests for a capability the client did not declare in _meta is refused as missingCapability, JSON-RPC code -32021, HTTP status 400, with the undeclared capabilities in data.requiredCapabilities, and a tool that reads the client capabilities asks only for the declared ones: a client that declares only sampling is asked to sample and not to elicit.
    Given the test_input_required_result_capabilities tool and the multiple-inputs tool
    When each is called by a client that declares only sampling, only elicitation, both and neither
    Then the capabilities tool asks only for what is declared and the multiple-inputs tool is refused with -32021

  @IR-13 @build
  Scenario: Keys of inputResponses that the server did not ask for are ignored when they are objects: a retry that answers user_name and adds unknown keys is answered with the complete result.
    Given the elicitation tool
    When the tool is retried with user_name and two extra object entries
    Then the answer is the complete result as if the extra entries were absent

  @IR-14 @build
  Scenario: inputResponses that are not a JSON object, an entry that is not an object, and an entry for an expected key that does not fit its result type (an ElicitResult without a valid action, a CreateMessageResult without a model, a ListRootsResult whose roots are not an array) are refused as invalidInputResponses or as invalid params, JSON-RPC code -32602, HTTP status 400, and never completed.
    Given the elicitation, sampling and roots tools
    When each is retried with a null inputResponses, a number for an entry or an object of the wrong shape
    Then each answer is a 400 with -32602 and none is a complete result

  @IR-15 @build
  Scenario: HMAC-SHA-256 (RFC 2104 over SHA-256 of FIPS 180-4) is computed in the model: the digests of the RFC 4231 test cases 1, 2, 3, 4, 6 and 7, including the two with a key longer than the 64-byte block, are kernel-checked theorems, and the tag in a requestState equals the HMAC-SHA-256 that an independent implementation computes over the length-prefixed method and name and the payload.
    Given the RFC 4231 test cases and a sealed requestState
    When the model computes the tags and an independent HMAC computes the same inputs
    Then the digests equal the published ones and the tag in the state equals the independent tag

  @IR-16 @build
  Scenario: The requestState key arrives in the request frame and a key of fewer than 32 bytes is no key: a result that needs a requestState is refused as requestStateKeyMissing, JSON-RPC code -32603, HTTP status 500 and no data, instead of being sent unprotected, a state presented without a key is refused as invalid, and a request that needs no state is served without a key.
    Given the tools that keep state, called with an empty key and with a key of 31 bytes
    When each is called with the same request and a different key
    Then the state-bearing results are 500, the state presented without a key is 400 and the stateless tool is served

  @IR-17 @build
  Scenario: An InputRequiredResult with neither inputRequests nor requestState is refused as emptyInputRequired, JSON-RPC code -32603, HTTP status 500 and no data, because the specification requires at least one; a kernel-checked theorem states it.
    Given a provider result of type input_required with an empty request list and no state
    When the guard settles it
    Then the result is refused as emptyInputRequired and nothing is sent

  @IR-18 @build
  Scenario: The request frame carries the requestState key after the count of tool calls as a length-prefixed byte string, and a frame that ends inside it or has bytes after it is a frame the core cannot read: HTTP status 500 and the fixed internal-error body.
    Given frames with a key, without the key chunk and with trailing bytes
    When the root reads each
    Then the first is served and the other two are the fixed internal error

  @IR-19 @build
  Scenario: The result bodies of elicitation (form and URL), sampling (messages with text, image, audio, tool_use and tool_result content, model preferences, system prompt, include context, temperature, stop sequences, metadata, tools and tool choice) and roots requests are written as the JSON that the specification schema names, and every string of them is sanitized as a tool result is.
    Given input requests of every kind the model can state
    When they are encoded and the answers of the fixture tools are checked against the specification schema
    Then each encoded request has the members the schema names
