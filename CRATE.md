# The protocol core crate

Issue #14. The protocol core is packaged as the crate `prism_mcp`, from the committed export of
its production root, and the package is bound to the core digest that `conformance/core-identity.json`
records (#13). The IDs are PB-01 to PB-06 in `CONFORMANCE.md`. The upload to crates.io is the last
step of the release, at the version tag, and is the owner's: nothing here requests, stores or
uses a token, and nothing uploads before the tag.

## Which crate

The export of the root `UorMcp.Fixture.fixtureRun`, the root whose program digest the conformance
build records (`5da88084…`). Of the three roots this repository exports, it is the only one that is
the whole core: `codecRun` is the JSON codec, `crowdRun` is a 60-tool stress catalog, and the pipeline
(`Serve`) has no provider-free root, because a record of functions cannot be an exported root's
parameter in LexLean 1.2 ([LIFECYCLE.md](LIFECYCLE.md)). lexlean-mcp expects `core` to be "a
LexLean production root exported as `run(Bytes)`", linked unchanged and identified by digest
(README, lexlean-mcp `EFFECTS.md`), and #14 ties the published crate to the digest #13 records.

**What this does not settle.** The crate serves the tool and resource catalogs of the conformance
suite's fixture provider. lexlean-mcp's tools cannot be registered on it. A core that reaches a
tool layer through a host effect, or a root owned by lexlean-mcp, is a modelling change, and it
changes the digest this crate is bound to. It is the lead's decision and a separate issue.

## Name and version

* **Name: `prism_mcp`**, written in the `package` of the export request
  (`crates/lifecycle-oracle/generated/request.json`). PrismPM names crates `prism-<application>`
  (`prism-stdlib`, `prism-calculator`). A LexLean production request admits a crate name only as
  `[a-z][a-z0-9_]*`, so `prism-mcp` cannot be written there; one spelling is then the name in the
  request, `Cargo.toml`, `package.json`, `provenance.json` and the Rust path. Both spellings were free on
  crates.io on 2026-10-24.
* **Release: the version of the workspace** (`Cargo.toml`, `[workspace.package]`), written in the request, so
  it is in the bytes of the export. A release bumps the workspace version, the request and the export
  together (`cargo xtask export`); the packaging script refuses a request that disagrees.
* **Pre-release: `<release>-dev.<n>`**, `n` being `git rev-list --first-parent --count <commit>`, for a
  commit imported during development: LexLean#302's scheme, PrismPM#319's proposal. It sorts below the release,
  is the same in every clone, and is pinned with `=`. Build metadata would not pin, because cargo ignores it
  when matching a requirement. LexLean's request grammar admits no pre-release, so `-dev.n` is stamped into
  `Cargo.toml` only; `provenance.json` and `package.json` keep the release's version, which names the release
  the pre-release precedes.

## What is packaged

`crates/lifecycle-oracle/generated/prism_mcp` is LexLean's export, byte for byte (the `lexlean / export`
job of CI compares it with a fresh export). The crate is that directory with:

* **`Cargo.toml`**: LexLean writes `publish = false` into every package (SPEC §17.16, the `RustPackage`
  scaffold), which `cargo publish` refuses (`prism_mcp cannot be published`). That line is replaced by
  `description`, `license`, `repository`, `homepage`, `readme` and `include`; for a pre-release the
  `version` line changes. Nothing else changes. The description is in
  `crates/lifecycle-oracle/publication/publication.toml`, and the licence, repository and homepage are the
  workspace's.
* `README.md` (`crates/lifecycle-oracle/publication/README.md`), `LICENSE-MIT` and `LICENSE-APACHE`.

`package.json`, `provenance.json` and `src/lib.rs` are the export's. The crate has no dependencies, so
nothing has to be on crates.io before it (`prism-stdlib` and `lexlean` are not involved).

## The script

```sh
scripts/package-core-crate.sh --dev auto --out "$OUT"    # 0.1.0-dev.<first-parent count of HEAD>
scripts/package-core-crate.sh --dev 123 --out "$OUT"     # the count given: a shallow clone, an archive without git
scripts/package-core-crate.sh --release --out "$OUT"     # 0.1.0, the crate the tag publishes
scripts/package-core-crate.sh --release --out "$OUT" --dry-run   # and `cargo publish --dry-run` (network, no credential)
scripts/package-core-crate.sh --release --verify FILE.crate      # check an archive against the tree
```

It needs `sh`, `cargo`, `tar`, `sha256sum`, `cmp`, `diff`, `sed`, `find` and `sort`, and no network (but
`--dry-run`), no node and no LexLean. It reads only files of the tree. It writes `$OUT/prism_mcp-<version>.crate`
and `$OUT/prism_mcp-<version>.crate.sha256` (in `sha256sum`'s form) and nothing when a check refuses. The package
is built from a copy outside git, so the same tree gives the same crate at any commit. The contents are the same on
every machine; the archive's SHA-256 is also the same for the same cargo, so the script uses the
toolchain `rust-toolchain.toml` pins when rustup has it and prints the cargo it ran. `--root DIR` reads
another tree (the tests plant defects in a copy).

### The checks, and the code each refusal has

`refused <code>: <message>` on standard error, exit status 1.

| Code | The tree or archive that is refused |
| --- | --- |
| `input-missing`, `publication-record` | a file the script reads is absent, or the publication record is incomplete |
| `root-mismatch` | the core is not `UorMcp.Fixture.fixtureRun` in the request, the publication record and the identity record |
| `name-mismatch` | the request, the publication record, the export's manifest and provenance, or an archive disagree on the crate name |
| `version-mismatch` | the request, the workspace, the export's manifest and provenance, or an archive disagree on the version asked for |
| `export-missing`, `export-tampered` | the export is absent, or its `Cargo.toml` or `src/lib.rs` is not the file its `provenance.json` records |
| `program-mismatch` | the program digest in the export's provenance is not `program_sha256` of `conformance/core-identity.json` |
| `source-mismatch` | the SHA-256 of `src/lib.rs` is not the one that record lists for the core of the conformance build |
| `shallow-clone` | `--dev auto` without the history that makes the count the same in every clone |
| `not-reproducible` | two packagings of the same files are different bytes |
| `file-set`, `file-tampered`, `manifest-differs` | an archive does not hold exactly the files of the package; a program, README or licence file differs; `Cargo.toml`, `Cargo.toml.orig` or `Cargo.lock` differ from the export's manifest with the publication metadata |
| `dry-run-differs` | `cargo publish --dry-run` failed, or verified other files than the packaged ones |

### How the bytes are bound to the core digest

1. The export's `provenance.json` records the SHA-256 of `src/lib.rs` and `Cargo.toml`; a hand edit is
   `export-tampered`. CI's export job proves the committed export is a fresh export of the sources.
2. Its `program` is `program_sha256` of `core-identity.json`, and the SHA-256 of `src/lib.rs` is the one that
   record lists for `mcp_conformance_core`. `check_core_identity` (the conformance job) proves that record is
   what a rebuild of the conformance build produces, so the crate is, by program digest and by the digest of
   its generated source, the core that the conformance suite ran against.
3. An archive is the package re-derived from the tree, file by file. The script prints the SHA-256 of the
   archive; the upload job checks that crates.io serves the same bytes.

`package.json` and `provenance.json` are not those of the conformance build: they carry another name and version,
and `provenance.json` the semantic ID of the sources of the other project (`sources`). The program and the
generated source are the same.

## The release

`.github/workflows/crate.yml` is the last step of the release. For a version tag `v<workspace version>` it
packages with `--release`, checks the identity, dry-runs, and then `upload` runs in the `crates.io` environment.
That job uploads only when the `CARGO_REGISTRY_TOKEN` secret exists, repeats the checks, uploads the files they
packaged, and checks that crates.io serves the crate with the SHA-256 the checks agreed on. With no secret it
succeeds and says nothing was uploaded. Dispatched by hand, or called from the release workflow of #16, it
checks a development pre-release and never uploads. Pull requests run the same script with `--dry-run`
(`lexlean.yml`, step 10), so a tag never meets a check for the first time.

### The owner's step

The first upload of a name is the owner's, as the last step of the release, after the tag's other gates. There is
no dependency order: the crate has none.

```sh
gh secret set CARGO_REGISTRY_TOKEN --env crates.io --repo UOR-Foundation/MCP   # the owner enters the token
git tag v0.1.0 <reviewed commit> && git push origin v0.1.0                    # runs crate.yml
```

or from a clean checkout of the tag, without GitHub Actions:

```sh
scripts/package-core-crate.sh --release --out out --dry-run --keep files
(cd files && CARGO_REGISTRY_TOKEN=<token> cargo publish --locked --allow-dirty)
```

## Planted defects

Each was planted alone in a copy of the files the script reads (`--root`) and the script run for the release;
the clean copy packages and exits 0. The same plants are the tests of PB-04 and PB-05.

| Defect | Command | Diagnostic |
| --- | --- | --- |
| a line appended to the export's `src/lib.rs` | `package-core-crate.sh --release --out OUT` | exit 1, `refused export-tampered: Cargo.toml or src/lib.rs of crates/lifecycle-oracle/generated/prism_mcp is not the file its provenance.json records: regenerate the export, never edit it`; no `OUT` |
| the publication record names `prism_mcp_core` | the same | `refused name-mismatch: the export request names the crate prism_mcp and the publication record prism_mcp_core` |
| the request names `prism_mcp_other` | the same | `refused name-mismatch` |
| the workspace version is `9.9.9` | the same | `refused version-mismatch: the export request versions the crate 0.1.0 and the workspace 9.9.9` |
| the identity record's `program_sha256` changed in its last digit | the same | `refused program-mismatch: the export's program is 5da88084… and conformance/core-identity.json records 5da88084…9` |
| the identity record's digest of `src/lib.rs` changed in its last digit | the same | `refused source-mismatch: src/lib.rs is 789c4901… and conformance/core-identity.json records 789c4901…9 for the core of the conformance build` |
| a byte appended to `src/lib.rs` inside the made archive | `--release --verify tampered.crate` | `refused file-tampered: src/lib.rs of the archive is not the one the tree gives` |
| a file `build.rs` added to the archive | `--verify` | `refused file-set` |
| a `[dependencies]` table added to `Cargo.toml.orig` | `--verify` | `refused manifest-differs` |
| the archive of `0.1.0` verified as `--dev 5` | `--dev 5 --verify good.crate` | `refused version-mismatch: the archive is prism_mcp-0.1.0 and the version asked for is 0.1.0-dev.5` |

## Not met

* **lexlean-mcp resolving `prism_mcp` from crates.io** (the Definition of done): the upload is the last step
  of the release and has not been made. Until then lexlean-mcp builds a local registry from this script.
* **"Byte-identical to the one the release's build manifest lists"**: the program files are the export's bytes
  and `src/lib.rs` is the conformance build's, by SHA-256. `Cargo.toml` is not LexLean's generated manifest, because
  that manifest cannot be published (`publish = false`); the difference is the publication metadata and is proved
  to be nothing else. It would disappear if LexLean's package scaffold took the metadata (a change of the
  language identity upstream). The release's own build manifest does not exist before a release's `prismpm build`.
* A core that lexlean-mcp's tools can be registered on (above).
