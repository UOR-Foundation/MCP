# MCP

The [Model Context Protocol](https://modelcontextprotocol.io) revision
`2026-07-28`, implemented as a server protocol core in
[LexLean](https://github.com/UOR-Foundation/LexLean).

Conformance to MCP is established only by MCP's own oracles: the
`2026-07-28` schema and the `modelcontextprotocol/conformance` suite, run
against a conformance build of this core with the suite's fixture tools. The
same core artifact, by digest, is linked unchanged by
[UOR-Foundation/lexlean-mcp](https://github.com/UOR-Foundation/lexlean-mcp)
and by any other LexLean-authored MCP server.

## Status

The protocol core is modeled in LexLean language 1.2 under `src/` (project
`lexlean.toml`, module prefix `UorMcp`, locked by `lexlean.lock`). It
type-checks, builds and verifies under Lean 4.32.1 with an exact axiom policy
on every declaration and no `sorry`. Nothing in it is yet checked against
MCP's own oracles, so this repository makes no conformance claim.

| Module | Source of |
| --- | --- |
| `Json` | the closed JSON value type, object member lookup and the typed accessors every decoder uses |
| `JsonRpc` | the JSON-RPC 2.0 envelope: request, notification, result and error responses, ids, and the error codes `-32700`, `-32600` to `-32603`, `-32020` to `-32022` |
| `Common` | shared schema types: roles, logging levels, icons, `Implementation`, annotations, progress tokens, and the `ttlMs` and `cacheScope` cache hint |
| `Capabilities` | client and server capabilities and the `io.modelcontextprotocol/tasks` extension identifier |
| `Content` | content blocks, resources, resource templates, prompts and tools |
| `Input` | multi-round-trip input: elicitation, sampling and roots requests and responses, `InputRequiredResult`, and the capability each request needs |
| `Tasks` | the tasks extension state machine, `DetailedTask` and `CreateTaskResult` |
| `Meta` | the request `_meta` keys (`protocolVersion`, `clientCapabilities`, `clientInfo`, `logLevel`, `progressToken`), `resultType` and the result `_meta` with `serverInfo` |
| `Methods` | the server method names, typed params and results for `server/discover`, `tools/list`, `tools/call`, `resources/list`, `resources/read`, `resources/templates/list`, `prompts/list`, `prompts/get`, `completion/complete`, `tasks/get`, `tasks/update` and `tasks/cancel`, and which methods may answer `input_required` or `task` |
| `Refusal` | the typed, closed refusals (`-32602`, `-32601`, `-32020`, `-32022`, `-32021`, `-32603`) and their JSON-RPC error objects |
| `Decode` | decoding a validated `params` object into the typed params of each method |
| `Validation` | per-request validation: the required `_meta` keys, header and body agreement, version support, and the order in which refusals are chosen |
| `Extension` | the interface through which a tool layer contributes tools, resources, resource templates and prompts (the `Provider` record) |
| `Dispatch` | dispatch from a validated request to a reply or a typed refusal, parameterized over the `Provider` |
| `Envelope` | the result envelope: `resultType` and `_meta` `io.modelcontextprotocol/serverInfo` |

What is not modeled: the JSON text codec (the LexLean stdlib has no JSON
parser or printer), the JSON encoding of result bodies beyond the envelope,
`subscriptions/listen`, and host effects.

- The claim registers in `model/` are empty, so `CONFORMANCE.md` lists no
  capability and this repository claims none.
- The repository is not bound to a PrismPM SDK release (#4): `prismpm.lock` and
  `template.lock` are absent, so `just vv` and the devcontainer cannot run.
- v0.1.0 is tracked in #16.

v0.1 is scoped to Streamable HTTP. stdio servers, MCP Apps and resource-server
authorization have no executable oracle at the pinned revisions, so no
conformance claim is made for them (#15).

## Repository

This is an instance of the [UOR template](https://github.com/UOR-Foundation/template).

- [AGENTS.md](AGENTS.md) sets the rules every change follows (R1–R6).
- [TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) defines inherited policy and the
  SDK and template update process.
- [VERIFICATION.md](VERIFICATION.md) defines the acceptance boundary; `just vv`
  is the complete gate.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
