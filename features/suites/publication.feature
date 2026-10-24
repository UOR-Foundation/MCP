Feature: The protocol core crate

  The protocol core is packaged as a crate from the committed export of its production root, the packaged bytes are bound to the core digest the conformance build records, and the upload to crates.io is the last step of the release.

  @PB-01 @build
  Scenario: The protocol core is published as one crate that is named once.
    Given the export request of the lifecycle oracle, the publication record, the committed export and the workspace manifest
    When the name and the version are read from each
    Then the root UorMcp.Fixture.fixtureRun is exported as prism_mcp at the workspace version, and the publication record, the export's manifest and provenance, and the package name all say the same

  @PB-02 @build
  Scenario: The packaging script makes the crate from the committed export alone, offline.
    Given a checkout and the script with the release version and with a development version
    When the script packages the export twice
    Then both archives are the same bytes, the program files are the export's byte for byte, the manifest differs from the export's only by the publication metadata, and the version is the workspace version or the workspace version with dev.N

  @PB-03 @build
  Scenario: The packaged crate is bound to the core digest the conformance build records.
    Given the packaged crate and conformance/core-identity.json
    When the program digest and the source digest are read from the crate and from the record
    Then they are equal, and the SHA-256 of the archive is written beside it

  @PB-04 @build
  Scenario: A package that does not match is refused with a typed diagnostic and no crate is written.
    Given a tree with a tampered export file, a wrong crate name, a wrong version, and a digest the identity record does not hold
    When the script is run on each
    Then each is refused with its own code and no archive is written

  @PB-05 @build
  Scenario: An existing archive is checked against the tree.
    Given the crate the script made, and copies with a changed file, an extra file, a changed manifest, and another version
    When the script verifies each
    Then the crate it made is accepted and each copy is refused with its own code

  @PB-06 @build
  Scenario: The upload is the last step of the release and nothing else uploads.
    Given the crate workflow and the lexlean workflow
    When their steps are read
    Then the upload step follows the packaging, the identity check and the dry run, runs only for a version tag with the secret present, and the pull request job packages and dry-runs without a credential
