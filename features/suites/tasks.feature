Feature: Tasks

  The LexLean core registers the tasks extension io.modelcontextprotocol/tasks (SEP-2663): a tools/call may answer a task handle, and tasks/get, tasks/update and tasks/cancel serve it, against the extension text and schema at the pinned ext-tasks commit and the ten tasks scenarios of the pinned conformance suite, which the pinned requirements run and do not score.

  @TK-01 @build
  Scenario: The server declares the extension io.modelcontextprotocol/tasks under capabilities.extensions of its server/discover result, with the empty object that means support without settings, and declares no capabilities.tasks member; the declaration is the server's and does not depend on what the client declared.
    Given the server/discover request of the tasks-capability-negotiation scenario
    When the root answers it, from a client that declared the extension and from one that declared nothing
    Then capabilities.extensions holds the extension as an empty object and capabilities has no tasks member

  @TK-02 @build
  Scenario: tasks/get, tasks/update and tasks/cancel from a client whose request does not declare the extension, in the session capabilities or in the request's own _meta, are refused as missingRequiredClientCapability (-32021, HTTP 400) with data.requiredCapabilities.extensions naming the extension, before the task is looked up and without changing the task table.
    Given a task in the table and clients that declare the extension nowhere, only in the session or only in the request _meta
    When each task method is sent
    Then the client without the extension is refused with -32021 and the table is untouched, and an unknown task is -32021 before it is -32602

  @TK-03 @build
  Scenario: Whether a tools/call becomes a task is the server's decision and no request member asks for it: a tool that supports tasks answers a client that declared the extension with a CreateTaskResult and any other client with the result it would have returned at once; a tool that does not support tasks never answers a task, whatever a legacy task member of the request says; the arguments are validated and the rate limit applies before a task is created.
    Given the tools greet and slow_compute and clients with and without the extension
    When tools/call is sent with and without a legacy task member, with invalid arguments and over the rate limit
    Then only slow_compute for a declaring client answers a task, the synchronous answer equals the task's result, and nothing is created for invalid arguments or past the limit

  @TK-04 @build
  Scenario: A tools/call of a tool that can only be a task, from a client that did not declare the extension, is refused as missingRequiredClientCapability (-32021, HTTP 400) with data.requiredCapabilities.extensions naming the extension, and creates no task.
    Given the tool failing_job, which is only ever a task, and clients that did not declare the extension
    When tools/call is sent
    Then the answer is -32021 naming the extension and the response returns no table

  @TK-05 @build
  Scenario: A created task is answered as a flat CreateTaskResult: resultType task and the task's taskId, status, createdAt, lastUpdatedAt and ttlMs at the top level, with no nested task member, no result, error, inputRequests or requestState, and no notification; the identifier and the instants are the host's, and a tasks/get for the identifier resolves as soon as the result is returned, because the task is in the table the response returns.
    Given a client that declared the extension and a host that supplies a clock and an identifier
    When slow_compute is called and tasks/get follows at once
    Then the result is the flat CreateTaskResult with the host's identifier and instants, and the task answers

  @TK-06 @build
  Scenario: ttlMs and pollIntervalMs are integers in milliseconds (ttlMs null when unlimited) and the legacy ttl and pollInterval members are not written; a task is kept for its ttlMs and is dropped from the table, and so answered as unknown, only once that has elapsed; the inlined result carries no related-task _meta.
    Given a created task and a host clock that advances
    When tasks/get is sent just before and just at the end of the ttl
    Then the task answers before and is unknown at the end, and no legacy member or related-task _meta appears

  @TK-07 @build
  Scenario: tasks/get answers the DetailedTask of the instant it is asked: working without a result, input_required with inputRequests, completed with the tool's result inlined (a tool error is completed with isError true), failed with the JSON-RPC error and no result, or cancelled; resultType is complete and no requestState is written; a working task whose work has finished by the host's clock is answered as finished.
    Given tasks that are working, waiting, finished, failed and cancelled
    When tasks/get is sent at several instants
    Then each answer is the DetailedTask of its status and an old terminal answer does not change

  @TK-08 @build
  Scenario: tasks/get, tasks/update and tasks/cancel for a task the table does not hold are refused as invalidParams (-32602, HTTP 400), and tasks/result and tasks/list, which the extension removed, are not offered: methodNotFound (-32601, HTTP 404).
    Given an empty table and the removed methods
    When each task method is sent for an unknown task, and tasks/result and tasks/list are sent
    Then unknown tasks are -32602 and the removed methods are -32601

  @TK-09 @build
  Scenario: tasks/cancel is acknowledged with an empty result whose resultType is complete; a working or waiting task becomes cancelled; a task that is already terminal, or that finished by the host's clock before the cancel arrived, is acknowledged and stays as it was.
    Given working, waiting, finished and cancelled tasks
    When tasks/cancel is sent for each
    Then each is acknowledged, only the working and waiting ones become cancelled, and a second cancel changes nothing

  @TK-10 @build
  Scenario: A task that waits for input is answered input_required with inputRequests keyed by identifiers that are unique over the task; tasks/update with inputResponses is acknowledged with an empty result and removes the keys it answers; answers to keys that are not outstanding are ignored; the task stays input_required while any key remains and resumes when none does; a task that is not waiting for input is not changed by an update.
    Given the tools confirm_delete and multi_input
    When their tasks are read, answered in part, answered with unknown keys and answered in full
    Then the requests are surfaced once, the answered keys leave, strangers are ignored and the task resumes after the last answer

  @TK-11 @build
  Scenario: tasks/get, tasks/update and tasks/cancel need an Mcp-Name header equal to params.taskId, and every request an Mcp-Method equal to its method; a missing or different value is refused as headerMismatch (-32020, HTTP 400) without changing the table.
    Given a task and requests with matching, missing and different routing headers
    When each task method is sent
    Then matching headers dispatch and every other value is -32020 with the table untouched

  @TK-12 @build
  Scenario: A change the core makes to a task while it answers a request (the task settles, parks for input, is answered or is cancelled) is written, beside the body, as a notifications/tasks notification that carries the complete task exactly as tasks/get would have answered it at that moment; creating a task and a request that changes nothing write none, and the encoding of the stream is #11's. The pinned scenario tasks-status-notifications sends no request, so no vector stands for this row.
    Given a task that finishes, parks and is answered and cancelled
    When each request that changes it is sent, and a request that changes nothing
    Then one notification equal to the task precedes each change and none follows a request that changes nothing

  @TK-13 @build
  Scenario: A tool that asks for input before it becomes a task answers its first round with an input_required result that carries inputRequests and no taskId, and its last round with a CreateTaskResult that carries no requestState or inputRequests, whose task finishes with a result that reflects the answer given; the last round is refused to a client that did not declare the extension.
    Given the tool test_tool_with_task and a client that declared elicitation and the extension
    When its two rounds are sent and the task is read
    Then the first round asks, the second creates a task and the task's text names the answer

  @TK-14 @build
  Scenario: The task lifecycle is stated in kernel-checked theorems: a task in a terminal state is not changed by settling, answering or cancelling; an update changes a task only while it is input_required; cancelling a working or waiting task yields a cancelled task; only a completed task carries a result; every step keeps the task's identifier and creation instant; a task is expired only once its ttl has elapsed.
    Given the Tasks and TaskLaws modules
    When lexlean verify checks them
    Then each theorem is accepted and the laws source states every transition and every refusal the tasks layer uses

  @TK-15 @build
  Scenario: The core keeps no task state: the host supplies a clock, the identifier of a task the request may create and the task table with each request, and stores the table the response returns; a table that cannot be read, a host that supplies no identifier and a frame whose host section is cut or is not text are server errors (-32603 or the fixed internal body, HTTP 500) that leave the table as it was; the table is returned only when a task was created or touched.
    Given a host and tables that are valid, unreadable and large
    When requests are sent
    Then the table is returned exactly when a task was created or touched and an unreadable table is never rewritten

  @TK-16 @build
  Scenario: The ten tasks scenarios at the pinned suite commit and the pinned ext-tasks commit are accounted for: every request they send is a step of vectors/tasks.json with its file and line, every check id they emit is covered or listed with its reason, the vendored files match the checksums the authorities record, and the requirements file runs the scenarios as extension scenarios that are not scored.
    Given the vendored scenarios, the extension schema and text and the vectors
    When the provenance tests read them
    Then every call site and check id is accounted for and every checksum equals its record
