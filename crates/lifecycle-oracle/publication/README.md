# prism_mcp

The server protocol core of the [Model Context Protocol](https://modelcontextprotocol.io)
revision `2026-07-28`, generated from its LexLean model in
[UOR-Foundation/MCP](https://github.com/UOR-Foundation/MCP) by `lexlean production build`.
The program is not handwritten: `src/lib.rs` is the rendering of the production root
`UorMcp.Fixture.fixtureRun`, and `provenance.json` and `package.json` record the program it
renders and the sources and compiler it was exported from.

The crate exports one function, `run`, from an encoded request frame to an encoded response
frame (`LIFECYCLE.md` in the repository describes both). It has no dependencies and no
`unsafe` code.

This version serves the tool and resource catalogs of the conformance suite's fixture
provider; it is the core that the repository's conformance build runs the suite against.
The program digest in `provenance.json` is the core digest the repository records in
`conformance/core-identity.json`, and the repository's `scripts/package-core-crate.sh`
checks that this crate is that core, byte for byte, before it is uploaded.

Licensed under MIT or Apache-2.0.
