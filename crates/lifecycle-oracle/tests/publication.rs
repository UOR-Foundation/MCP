//! PB-01 to PB-06: the protocol core is packaged as a crate from the committed export,
//! bound to the core digest the conformance build records, and uploaded only as the last
//! step of the release.
//!
//! The tests run `scripts/package-core-crate.sh` on copies of the files it reads (`--root`),
//! so a planted defect never touches the tree. They need `sh`, `tar`, `cmp`, `diff`, `sed`,
//! `sha256sum` and `cargo`, the tools the SDK image's `just vv` already runs on, and no
//! network: `cargo package` of a crate without dependencies is offline.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use sha2::{Digest, Sha256};

const ROOT_NAME: &str = "UorMcp.Fixture.fixtureRun";
const CRATE: &str = "prism_mcp";
const EXPORT: &str = "crates/lifecycle-oracle/generated";
const PUBLICATION: &str = "crates/lifecycle-oracle/publication";

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/lifecycle-oracle is two below the root")
        .to_path_buf()
}

fn hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn json(path: &Path) -> Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The version in `[workspace.package]` of the manifest at `root`.
fn workspace_version(root: &Path) -> String {
    let text = read(&root.join("Cargo.toml"));
    let section = text
        .split("[workspace.package]")
        .nth(1)
        .expect("a [workspace.package] table");
    section
        .lines()
        .find_map(|line| {
            line.strip_prefix("version = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .expect("a workspace version")
        .to_string()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create the copy");
    for entry in fs::read_dir(from).unwrap_or_else(|e| panic!("{}: {e}", from.display())) {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy a file");
        }
    }
}

/// A scratch directory, removed when it is dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("publication-{name}"));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create the scratch directory");
        Scratch(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The files the script reads, copied from the repository into `scratch/tree`.
fn scratch_tree(scratch: &Scratch) -> PathBuf {
    let source = repository();
    let tree = scratch.path().join("tree");
    copy_tree(
        &source.join(EXPORT).join(CRATE),
        &tree.join(EXPORT).join(CRATE),
    );
    copy_tree(&source.join(PUBLICATION), &tree.join(PUBLICATION));
    for file in [
        format!("{EXPORT}/request.json"),
        "conformance/core-identity.json".to_string(),
        "Cargo.toml".to_string(),
        "LICENSE-MIT".to_string(),
        "LICENSE-APACHE".to_string(),
    ] {
        let to = tree.join(&file);
        fs::create_dir_all(to.parent().expect("a parent")).expect("create the parent");
        fs::copy(source.join(&file), to).unwrap_or_else(|e| panic!("{file}: {e}"));
    }
    tree
}

fn script(root: &Path, arguments: &[&str]) -> Output {
    Command::new("sh")
        .arg(repository().join("scripts/package-core-crate.sh"))
        .arg("--root")
        .arg(root)
        .args(arguments)
        .output()
        .expect("run sh")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// The script succeeded; stdout and stderr for the failure message.
fn succeeded(output: &Output) {
    assert!(
        output.status.success(),
        "the script failed ({}):\n{}{}",
        output.status,
        text(&output.stdout),
        text(&output.stderr)
    );
}

/// The script refused with `code`, exit status 1, and wrote no crate to `out`.
fn refused(output: &Output, code: &str, out: &Path) {
    let stderr = text(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected a refusal, got {}:\n{}{}",
        output.status,
        text(&output.stdout),
        stderr
    );
    assert!(
        stderr.contains(&format!("refused {code}: ")),
        "expected `refused {code}: ` in:\n{stderr}"
    );
    let written = fs::read_dir(out)
        .map(|entries| entries.flatten().count())
        .unwrap_or(0);
    assert_eq!(
        written,
        0,
        "a refused package left files in {}",
        out.display()
    );
}

fn extract(archive: &Path, into: &Path) -> PathBuf {
    fs::create_dir_all(into).expect("create the extraction directory");
    let status = Command::new("tar")
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(into)
        .status()
        .expect("run tar");
    assert!(status.success(), "tar could not read {}", archive.display());
    let mut roots: Vec<_> = fs::read_dir(into)
        .expect("read the extraction")
        .flatten()
        .collect();
    assert_eq!(roots.len(), 1, "an archive has one top directory");
    roots.remove(0).path()
}

fn repack(directory: &Path, archive: &Path) {
    let name = directory.file_name().expect("a directory name");
    let status = Command::new("tar")
        .arg("-czf")
        .arg(archive)
        .arg("-C")
        .arg(directory.parent().expect("a parent"))
        .arg(name)
        .status()
        .expect("run tar");
    assert!(
        status.success(),
        "tar could not write {}",
        archive.display()
    );
}

fn identity(root: &Path) -> Value {
    json(&root.join("conformance/core-identity.json"))
}

/// A change planted in an extracted archive.
type Plant<'a> = &'a dyn Fn(&Path);

fn crate_file(out: &Path, version: &str) -> PathBuf {
    out.join(format!("{CRATE}-{version}.crate"))
}

#[test]
fn the_core_is_one_crate_named_once_pb_01() {
    let root = repository();
    let request = json(&root.join(EXPORT).join("request.json"));
    let exported = request["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .find(|entry| entry["root"] == ROOT_NAME)
        .unwrap_or_else(|| panic!("the request exports {ROOT_NAME}"));
    let version = workspace_version(&root);
    assert_eq!(exported["package"]["name"], CRATE);
    assert_eq!(
        exported["package"]["version"], version,
        "the crate is released at the version of the workspace"
    );

    let publication = read(&root.join(PUBLICATION).join("publication.toml"));
    assert!(publication.contains(&format!("crate = \"{CRATE}\"")));
    assert!(publication.contains(&format!("root = \"{ROOT_NAME}\"")));
    assert!(root.join(PUBLICATION).join("README.md").is_file());

    let directory = root.join(EXPORT).join(CRATE);
    let manifest = read(&directory.join("Cargo.toml"));
    assert!(manifest.contains(&format!("name = \"{CRATE}\"")));
    assert!(manifest.contains(&format!("version = \"{version}\"")));
    assert!(
        manifest.lines().any(|line| line == "publish = false"),
        "LexLean writes publish = false into every package, which is why the manifest is overlaid"
    );
    let provenance = json(&directory.join("provenance.json"));
    assert_eq!(provenance["name"], CRATE);
    assert_eq!(provenance["version"], version);
    assert_eq!(json(&directory.join("package.json"))["name"], CRATE);
    assert!(
        !root.join(EXPORT).join("mcp_lifecycle").exists(),
        "the export is under its published name only"
    );
}

#[test]
fn the_script_packages_the_export_offline_and_reproducibly_pb_02() {
    let scratch = Scratch::new("pb-02");
    let tree = scratch_tree(&scratch);
    let version = workspace_version(&tree);
    let export = tree.join(EXPORT).join(CRATE);

    let release_a = scratch.path().join("release-a");
    let release_b = scratch.path().join("release-b");
    for out in [&release_a, &release_b] {
        succeeded(&script(
            &tree,
            &["--release", "--out", out.to_str().expect("utf-8")],
        ));
    }
    let archive = crate_file(&release_a, &version);
    assert_eq!(
        fs::read(&archive).expect("the crate"),
        fs::read(crate_file(&release_b, &version)).expect("the second crate"),
        "two packagings are the same bytes"
    );

    let unpacked = extract(&archive, &scratch.path().join("unpacked"));
    let mut files: Vec<String> = fs::read_dir(&unpacked)
        .expect("read the crate")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    files.sort();
    assert_eq!(
        files,
        [
            "Cargo.lock",
            "Cargo.toml",
            "Cargo.toml.orig",
            "LICENSE-APACHE",
            "LICENSE-MIT",
            "README.md",
            "package.json",
            "provenance.json",
            "src"
        ]
    );
    for file in ["package.json", "provenance.json", "src/lib.rs"] {
        assert_eq!(
            fs::read(unpacked.join(file)).expect("a packaged file"),
            fs::read(export.join(file)).expect("an exported file"),
            "{file} is the committed export's, byte for byte"
        );
    }
    let original = read(&unpacked.join("Cargo.toml.orig"));
    let exported = read(&export.join("Cargo.toml"));
    assert!(!original.lines().any(|line| line == "publish = false"));
    let added: Vec<&str> = original
        .lines()
        .filter(|line| !exported.lines().any(|e| e == *line))
        .map(|line| line.split(" = ").next().expect("a key"))
        .collect();
    assert_eq!(
        added,
        [
            "description",
            "license",
            "repository",
            "homepage",
            "readme",
            "include"
        ],
        "the manifest differs from the export's by the publication metadata only"
    );
    let removed: Vec<&str> = exported
        .lines()
        .filter(|line| !original.lines().any(|o| o == *line))
        .collect();
    assert_eq!(removed, ["publish = false"]);
    assert!(read(&unpacked.join("Cargo.toml")).contains(&format!("version = \"{version}\"")));

    // A commit imported during development is the pre-release of the workspace version.
    let dev = scratch.path().join("dev");
    succeeded(&script(
        &tree,
        &["--dev", "123", "--out", dev.to_str().expect("utf-8")],
    ));
    let pre = format!("{version}-dev.123");
    let unpacked = extract(
        &crate_file(&dev, &pre),
        &scratch.path().join("unpacked-dev"),
    );
    assert!(read(&unpacked.join("Cargo.toml.orig")).contains(&format!("version = \"{pre}\"")));
    assert_eq!(
        fs::read(unpacked.join("provenance.json")).expect("provenance"),
        fs::read(export.join("provenance.json")).expect("the export's provenance"),
        "a pre-release does not change the program files: their version names the release it precedes"
    );
    assert_eq!(
        json(&unpacked.join("provenance.json"))["version"],
        version,
        "provenance.json still names the release"
    );
}

#[test]
fn the_crate_is_bound_to_the_recorded_core_digest_pb_03() {
    let scratch = Scratch::new("pb-03");
    let tree = scratch_tree(&scratch);
    let version = workspace_version(&tree);
    let out = scratch.path().join("out");
    let output = script(&tree, &["--release", "--out", out.to_str().expect("utf-8")]);
    succeeded(&output);
    let stdout = text(&output.stdout);

    let record = identity(&tree);
    let program = record["program_sha256"].as_str().expect("a program digest");
    let unpacked = extract(
        &crate_file(&out, &version),
        &scratch.path().join("unpacked"),
    );
    assert_eq!(
        json(&unpacked.join("provenance.json"))["program"],
        program,
        "the crate's program is the core digest #13 records"
    );
    let recorded = record["package"]["files"]
        .as_array()
        .expect("files")
        .iter()
        .find(|file| file["path"] == "src/lib.rs")
        .expect("the record lists src/lib.rs")["sha256"]
        .as_str()
        .expect("a digest")
        .to_string();
    let lib = fs::read(unpacked.join("src/lib.rs")).expect("src/lib.rs");
    assert_eq!(hex(&lib), recorded);
    assert_eq!(
        json(&unpacked.join("provenance.json"))["files"]["src/lib.rs"],
        recorded
    );

    let archive = fs::read(crate_file(&out, &version)).expect("the crate");
    let sha = read(&out.join(format!("{CRATE}-{version}.crate.sha256")));
    assert_eq!(
        sha,
        format!("{}  {CRATE}-{version}.crate\n", hex(&archive)),
        "the SHA-256 of the archive is written beside it, in sha256sum's form"
    );
    for line in [
        format!("crate {CRATE} {version}"),
        format!("program {program}"),
        format!("src/lib.rs {recorded}"),
        format!("sha256 {}", hex(&archive)),
    ] {
        assert!(
            stdout.lines().any(|l| l.starts_with(&line)),
            "the script reports `{line}`:\n{stdout}"
        );
    }
}

/// Run the script on `tree` for the release and require the refusal `code`.
fn refuses_tree(scratch: &Scratch, tree: &Path, code: &str) {
    let out = scratch.path().join("out");
    let output = script(tree, &["--release", "--out", out.to_str().expect("utf-8")]);
    refused(&output, code, &out);
}

#[test]
fn a_tampered_export_file_is_refused_pb_04() {
    let scratch = Scratch::new("pb-04-tamper");
    let tree = scratch_tree(&scratch);
    let lib = tree.join(EXPORT).join(CRATE).join("src/lib.rs");
    let mut bytes = fs::read(&lib).expect("src/lib.rs");
    bytes.extend_from_slice(b"// hand edit\n");
    fs::write(&lib, bytes).expect("plant the edit");
    refuses_tree(&scratch, &tree, "export-tampered");
}

#[test]
fn a_wrong_crate_name_or_version_is_refused_pb_04() {
    let scratch = Scratch::new("pb-04-name");
    let tree = scratch_tree(&scratch);
    let record = tree.join(PUBLICATION).join("publication.toml");
    let text = read(&record).replace("crate = \"prism_mcp\"", "crate = \"prism_mcp_core\"");
    fs::write(&record, text).expect("plant a wrong name");
    refuses_tree(&scratch, &tree, "name-mismatch");

    let scratch = Scratch::new("pb-04-version");
    let tree = scratch_tree(&scratch);
    let manifest = tree.join("Cargo.toml");
    let version = workspace_version(&tree);
    let text = read(&manifest).replacen(
        &format!("version = \"{version}\""),
        "version = \"9.9.9\"",
        1,
    );
    fs::write(&manifest, text).expect("plant a wrong version");
    refuses_tree(&scratch, &tree, "version-mismatch");

    let scratch = Scratch::new("pb-04-request");
    let tree = scratch_tree(&scratch);
    let request = tree.join(EXPORT).join("request.json");
    let text = read(&request).replace(
        &format!("\"name\":\"{CRATE}\""),
        "\"name\":\"prism_mcp_other\"",
    );
    fs::write(&request, text).expect("plant a wrong name in the request");
    refuses_tree(&scratch, &tree, "name-mismatch");
}

#[test]
fn a_digest_the_identity_record_does_not_hold_is_refused_pb_04() {
    let scratch = Scratch::new("pb-04-program");
    let tree = scratch_tree(&scratch);
    let record = tree.join("conformance/core-identity.json");
    let program = identity(&tree)["program_sha256"]
        .as_str()
        .expect("a digest")
        .to_string();
    let changed = format!("{}0", &program[..63]);
    // Both places the record holds it, so that only the disagreement with the crate remains.
    fs::write(&record, read(&record).replace(&program, &changed)).expect("plant a digest");
    refuses_tree(&scratch, &tree, "program-mismatch");

    let scratch = Scratch::new("pb-04-source");
    let tree = scratch_tree(&scratch);
    let record = tree.join("conformance/core-identity.json");
    let lib = identity(&tree)["package"]["files"]
        .as_array()
        .expect("files")
        .iter()
        .find(|file| file["path"] == "src/lib.rs")
        .expect("src/lib.rs")["sha256"]
        .as_str()
        .expect("a digest")
        .to_string();
    let changed = format!("{}0", &lib[..63]);
    fs::write(&record, read(&record).replace(&lib, &changed)).expect("plant a digest");
    refuses_tree(&scratch, &tree, "source-mismatch");
}

#[test]
fn an_existing_archive_is_checked_against_the_tree_pb_05() {
    let scratch = Scratch::new("pb-05");
    let tree = scratch_tree(&scratch);
    let version = workspace_version(&tree);
    let out = scratch.path().join("out");
    succeeded(&script(
        &tree,
        &["--release", "--out", out.to_str().expect("utf-8")],
    ));
    let archive = crate_file(&out, &version);
    let archive_text = archive.to_str().expect("utf-8");
    succeeded(&script(&tree, &["--release", "--verify", archive_text]));

    // Each copy is the made crate with one change, and the made crate is refused for it.
    let cases: [(&str, Plant); 3] = [
        ("file-tampered", &|dir| {
            let lib = dir.join("src/lib.rs");
            let mut bytes = fs::read(&lib).expect("src/lib.rs");
            bytes[0] ^= 1;
            fs::write(lib, bytes).expect("plant");
        }),
        ("file-set", &|dir| {
            fs::write(dir.join("build.rs"), "fn main() {}\n").expect("plant");
        }),
        ("manifest-differs", &|dir| {
            let manifest = dir.join("Cargo.toml.orig");
            let text = read(&manifest) + "\n[dependencies]\nrand = \"0.9\"\n";
            fs::write(manifest, text).expect("plant");
        }),
    ];
    for (code, plant) in cases {
        let work = scratch.path().join(format!("copy-{code}"));
        let dir = extract(&archive, &work);
        plant(&dir);
        let copy = scratch.path().join(format!("{code}.crate"));
        repack(&dir, &copy);
        let output = script(
            &tree,
            &["--release", "--verify", copy.to_str().expect("utf-8")],
        );
        assert_eq!(
            output.status.code(),
            Some(1),
            "{code}: {}",
            text(&output.stderr)
        );
        assert!(
            text(&output.stderr).contains(&format!("refused {code}: ")),
            "{code}: {}",
            text(&output.stderr)
        );
    }

    // The same archive is not the crate of another version.
    let output = script(&tree, &["--dev", "5", "--verify", archive_text]);
    assert_eq!(output.status.code(), Some(1));
    assert!(text(&output.stderr).contains("refused version-mismatch: "));
}

fn workflow(name: &str) -> String {
    read(&repository().join(".github/workflows").join(name))
}

/// The text of the step of `workflow` whose `name:` line contains `title`.
fn step<'a>(workflow: &'a str, title: &str) -> &'a str {
    let start = workflow
        .find(&format!("- name: {title}"))
        .unwrap_or_else(|| panic!("a step named `{title}`"));
    let rest = &workflow[start + 2..];
    let end = rest.find("\n      - ").unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn the_upload_is_the_last_step_of_the_release_pb_06() {
    let release = workflow("crate.yml");
    assert!(
        release.contains("tags:") && release.contains("'v*'"),
        "the crate workflow runs for a version tag"
    );
    assert!(release.contains("workflow_dispatch:"), "and by rehearsal");
    let package = release
        .find("scripts/package-core-crate.sh")
        .expect("packaging");
    let dry = release.find("--dry-run").expect("a dry run");
    let upload = release.find("cargo publish --locked").expect("the upload");
    assert!(
        package < dry && dry < upload,
        "the upload follows the dry run"
    );
    assert_eq!(
        release.matches("cargo publish").count(),
        1,
        "the script's dry run is the only other publish, and it is the script's"
    );
    let uploading = step(&release, "Upload the checked crate");
    assert!(uploading.contains("if: ${{ env.HAS_CARGO_REGISTRY_TOKEN == 'true' }}"));
    assert!(uploading.contains("cargo publish --locked"));
    assert!(
        release.contains("HAS_CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN != '' }}"),
        "the presence of the secret decides the upload"
    );
    assert_eq!(
        release
            .matches("CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}")
            .count(),
        1,
        "the credential is read once"
    );
    assert!(
        uploading.contains("CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}"),
        "and only by the step that uploads"
    );
    let upload_job = release.find("\n  upload:").expect("an upload job");
    assert!(
        release[upload_job..].contains("if: ${{ startsWith(github.ref, 'refs/tags/v') }}")
            && release[upload_job..].contains("needs: readiness")
            && release[upload_job..].contains("environment: crates.io"),
        "the upload job needs the readiness job and runs for a version tag"
    );
    assert!(
        !release[..upload_job].contains("environment:")
            && !release[..upload_job].contains("secrets."),
        "the readiness job sees no credential"
    );
    for line in release.lines().filter(|l| l.contains("secrets.")) {
        assert!(
            line.contains("CARGO_REGISTRY_TOKEN"),
            "no other secret is read: {line}"
        );
    }

    let pull_request = workflow("lexlean.yml");
    let packaging = step(&pull_request, "10. the core crate is packaged and dry-run");
    assert!(packaging.contains("scripts/package-core-crate.sh"));
    assert!(packaging.contains("--dry-run"));
    assert!(
        !pull_request.contains("CARGO_REGISTRY_TOKEN"),
        "a pull request never sees the credential"
    );
}
