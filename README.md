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

Nothing is built yet.

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
