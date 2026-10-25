Feature: A provider requests a host effect as a typed outcome

  A server application built by PrismPM runs one core root that a generated driver replays: a turn is the request and the effects completed so far, and a step is done with the response frame or perform with an effect tag and its payload. The provider of the LexLean core acts through host effects, so it must be able to ask for one as a typed outcome, and the core must return it as a typed step, with every check the core runs before the provider still run first.

  @PE-01 @build
  Scenario: A provider asks for a host effect with a typed outcome, and no result carries the request.
    Given a call, a read and a prompt outcome that perform an effect with a tag and a payload
    When Extension.answerOf turns each into the answer of the provider
    Then each is the provider refusal effectRequested with the same request, and no result body is built from it

  @PE-02 @build
  Scenario: The step serve reads the turn and writes the step in PrismPM's encoding, and a turn it cannot read never performs.
    Given a turn that holds a request frame and no completed effect, one that holds two, every proper prefix of a turn and a turn with a byte after its last outcome
    When the step root answers each
    Then a request that performs is the word 1, the four-byte tag and the payload, a response is the word 0 and the frame, and every unreadable turn is done with the internal error response

  @PE-03 @build
  Scenario: A provider's effect request reaches the root as a typed step and the replayed outcome gives the final response.
    Given a tools/call, a resources/read and a prompts/get whose provider requests an effect, and a tools/call that needs two effects in turn
    When a driver performs each requested effect and replays the turn with the outcomes it completed
    Then each request is performed once, in order, with the payload the provider chose, and the last turn is done with a response that holds the outcome and no marker

  @PE-04 @build
  Scenario: A request that fails a check of the core performs no effect.
    Given requests of a tool that requests an effect, each with one defect: a foreign Origin, a GET, a body that is not JSON, an unsupported version, a missing header, a missing _meta, an unknown method, params that do not decode, a call past the rate limit and arguments that fail the input schema
    When a driver replays each and records every effect it is asked to perform
    Then no effect is performed and each answer is done with the refusal or the result the byte serve gives

  @PE-05 @build
  Scenario: The step serve and the byte serve agree whenever no effect is requested.
    Given every request of the lifecycle, tools, resources and transport vectors
    When the byte root answers the request and the step root answers the turn of the request with no completed effect
    Then the step is done with exactly the response frame of the byte root

  @PE-06 @build
  Scenario: The byte serve never performs and answers a provider that requests an effect with a typed internal error.
    Given a provider that requests an effect and the byte serve over it
    When a client calls it
    Then the answer is HTTP 500 with the JSON-RPC code -32603 and no data member

  @PE-07 @build
  Scenario: A tag that does not fit thirty-two bits is never written as a step.
    Given a provider that requests the tag 4294967296
    When the step root answers the call
    Then the step is done with the internal error response and not a perform of tag 0

  @PE-08 @build
  Scenario: The kernel checks the laws of the step.
    Given the theorems of the step laws module
    When the module is checked
    Then the step encodings, the turn round trip, the refusal cases and the done-or-perform split are stated and proved

  @PE-09 @build
  Scenario: The conformance fixture requests no effect.
    Given the fixture module, the fixture root and the conformance declaration
    When their imports and the answers of the step serve are read
    Then the fixture imports neither the effect module nor the step serve, every vector is answered done, and the declaration still names fixtureRun
