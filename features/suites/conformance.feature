Feature: Conformance build and oracle run

  The conformance build serves the unchanged LexLean core, with the suite's fixture provider, over loopback HTTP through the generated PrismPM host adapter, and the pinned MCP conformance suite and the pinned 2026-07-28 schema decide whether it conforms. The verdict is the oracles', never this repository's.

  @CF-01 @build
  Scenario: The conformance build is a PrismPM server application whose core is the step serve over the fixture provider, whose host effects fill the request frame with the tool-call count, the clock, a task identifier and the task table and receive the table the core stores, and whose listener is loopback.
    Given the declaration conformance/src/Application.lex.tex, the conformance root in src/ConformanceFixture.lex.tex, the consumer workspace conformance/Cargo.toml and the binding crate
    When the declaration is read
    Then the core root is UorMcp.ConformanceFixture.conformanceRun with its production clause, the frame sections read the result fields of the effects tool.calls, clock.now, task.id and task.table, the address is 127.0.0.1, the workspace lists exactly the generated packages and the binding, and the binding answers the count before the request it serves

  @CF-02 @build
  Scenario: The vectors of the declaration are the committed core package's own answers.
    Given the requests the declaration names, taken from the pinned scenarios' vectors
    When the committed package answers each as the adapter frames it
    Then the acceptance and served vectors of the declaration equal the regenerated ones byte for byte and the transcripts hold the store of the task table

  @CF-03 @build
  Scenario: The suite's server runner is run against the served build and the verdict is read from its per-scenario checks.
    Given the frozen requirements for 2026-07-28 with 37 required and 13 not-scored server scenarios and a served conformance build
    When the verdict logic reads the checks of every scenario
    Then each scenario is passing only with a passing check and no failing check, each other scenario is named with the suite's diagnostic, and the job passes only when the failing scenarios are exactly those conformance/waiting.json lists with their issue or pull request, the listed ones still counted as failures

  @CF-04 @build
  Scenario: The recorded transcripts validate against the pinned schema.
    Given a transcript of the exchanges of the run, the pinned schema.json and the pinned ext-tasks schema.json
    When the schema oracle reads every message
    Then each message is valid for the definition it is held to, a message of the tasks extension is held to the extension's schema and is a finding without it, the inputResponses of a tasks/update under a key that is not outstanding are counted and not held to the schema of an input response, and an invalid message, an orphan response or a body that is not JSON is reported with its exchange

  @CF-05 @build
  Scenario: A planted defect fails the run with the suite's diagnostic.
    Given checks of a run in which tools-list reports a failing check and a scenario produced no checks
    When the verdict logic reads them
    Then the run is not valid, it names tools-list with the check id and message and the empty scenario, and exits 1, and so does a listed scenario that passes

  @CF-06 @build
  Scenario: The core artifact is recorded by digest and a rebuild reproduces the record.
    Given the production result of the conformance build
    When the identity of the core is derived from it
    Then it equals the committed core-identity.json

  @CF-07 @build
  Scenario: The pins of the oracle run agree with their authority records.
    Given .github/prismpm.rev, the suite pin of the fetch script, the oracle's frozen requirements digest and the vendored requirements file
    When each pin is compared with the authority records
    Then they agree and the scenario sets are 37 required and 13 not-scored server scenarios

  @CF-08 @build
  Scenario: The workspace test gate needs neither node nor the installed suite, and the conformance job runs the tests that do.
    Given the tests of the workspace, the ignore reasons of those that run a node oracle or read the installed suite, and the conformance job
    When the sources of the tests and of the job are read
    Then every such test is ignored with a reason that names the conformance job, no other test is ignored, the job runs them with --ignored and requires as many passes as there are ignored tests, and an unignored test that runs node or reads the suite is refused
