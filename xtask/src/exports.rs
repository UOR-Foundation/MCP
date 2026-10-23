//! `cargo xtask export`: the one command that regenerates what a source change
//! invalidates in the committed LexLean exports (#21).
//!
//! Every `crates/*/generated/request.json` pins, in its `expect`, the source
//! ID, the semantic ID and the SHA-256 of `lexlean.lock` that its packages were
//! exported from (SPEC section 24.7), and names each target by the digest of
//! its descriptor. `lexlean production build` refuses a request whose pins the
//! project no longer matches (`LLC0204`) and one whose target digest is not the
//! compiler's (`LLC0203`). So a commit that changes a `src/*.lex.tex`, the lock
//! or the pinned compiler must also change the pins and the packages; this
//! task writes both from the tree and from the compiler, and nothing else:
//!
//! 1. `lexlean check` states the source ID and the semantic ID of the tree;
//! 2. the lock's SHA-256 is read from `lexlean.lock`;
//! 3. `lexlean production targets` states the descriptor digest of each target;
//! 4. each request gets those values, and is built with
//!    `lexlean production build --request`, which verifies the project;
//! 5. each exported `rust-std` package replaces the one committed beside the
//!    request, so the committed packages equal the export byte for byte.
//!
//! The compiler is the `lexlean` on `PATH`, or the one `LEXLEAN` names; it must
//! be the one `.github/lexlean.rev` pins.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::Fail;

/// The compiler binary: `LEXLEAN`, else `lexlean` on `PATH`.
fn compiler() -> String {
    std::env::var("LEXLEAN").unwrap_or_else(|_| "lexlean".to_string())
}

/// Run `lexlean` with `args` in `root`, returning its standard output.
fn capture(root: &Path, args: &[&str]) -> Result<Vec<u8>, Fail> {
    let out = Command::new(compiler())
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run `{} {}`: {e}", compiler(), args.join(" ")))?;
    if !out.status.success() {
        return Err(format!(
            "`lexlean {}` failed ({}):\n{}{}",
            args.join(" "),
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )
        .into());
    }
    Ok(out.stdout)
}

/// Every `crates/*/generated/request.json`, in path order. Found, never listed:
/// a request added under a new crate is regenerated without editing this file.
fn requests(root: &Path) -> Result<Vec<PathBuf>, Fail> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(root.join("crates"))? {
        let candidate = entry?.path().join("generated").join("request.json");
        if candidate.is_file() {
            found.push(candidate);
        }
    }
    found.sort();
    Ok(found)
}

/// The values a request pins, as the tree and the compiler state them.
struct Pins {
    source_id: String,
    semantic_id: String,
    lock_sha256: String,
    /// The descriptor digest of each registered target, by target ID.
    targets: Vec<(String, String)>,
}

fn text_field(value: &Value, key: &str, what: &str) -> Result<String, Fail> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("{what} has no `{key}`").into())
}

fn read_pins(root: &Path) -> Result<Pins, Fail> {
    let checked: Value = serde_json::from_slice(&capture(
        root,
        &["--color", "never", "--diagnostic-format", "json", "check"],
    )?)?;
    let lock = std::fs::read(root.join("lexlean.lock"))
        .map_err(|e| format!("lexlean.lock is required: {e}"))?;
    let descriptors: Value = serde_json::from_slice(&capture(root, &["production", "targets"])?)?;
    let mut targets = Vec::new();
    for target in descriptors
        .get("targets")
        .and_then(Value::as_array)
        .ok_or("`lexlean production targets` lists no targets")?
    {
        targets.push((
            text_field(target, "id", "a target descriptor")?,
            text_field(target, "digest", "a target descriptor")?,
        ));
    }
    Ok(Pins {
        source_id: text_field(&checked, "source_id", "the result of `lexlean check`")?,
        semantic_id: text_field(&checked, "semantic_id", "the result of `lexlean check`")?,
        lock_sha256: Sha256::digest(&lock)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
        targets,
    })
}

/// Write the pins into one request, keeping everything else it says.
fn pin_request(path: &Path, pins: &Pins) -> Result<(), Fail> {
    let mut request: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let roots = request
        .get_mut("roots")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("{} has no `roots`", path.display()))?;
    for root in roots {
        let targets = root
            .get_mut("targets")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| format!("{} has a root with no `targets`", path.display()))?;
        for target in targets {
            let id = text_field(target, "id", "a requested target")?;
            let (_, digest) = pins
                .targets
                .iter()
                .find(|(known, _)| *known == id)
                .ok_or_else(|| format!("{}: `{id}` is not a registered target", path.display()))?;
            target["digest"] = Value::String(digest.clone());
        }
    }
    request["expect"] = json!({
        "lock_sha256": pins.lock_sha256,
        "semantic_id": pins.semantic_id,
        "source_id": pins.source_id,
    });
    let mut text = serde_json::to_string(&request)?;
    text.push('\n');
    std::fs::write(path, text)?;
    Ok(())
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), Fail> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

/// The `rust-std` packages one build published, as directories in name order.
fn exported_packages(root: &Path) -> Result<Vec<PathBuf>, Fail> {
    let mut found = Vec::new();
    let production = root.join(".lexlean").join("production");
    if production.is_dir() {
        for bundle in std::fs::read_dir(&production)? {
            let packages = bundle?.path().join("packages").join("rust-std");
            if packages.is_dir() {
                for package in std::fs::read_dir(&packages)? {
                    found.push(package?.path());
                }
            }
        }
    }
    found.sort();
    Ok(found)
}

/// Replace the packages committed beside `request` with the ones its build
/// published.
fn export_request(root: &Path, request: &Path) -> Result<(), Fail> {
    let relative = request.strip_prefix(root).unwrap_or(request);
    let relative = relative.to_str().ok_or("a request path is not UTF-8")?;
    let production = root.join(".lexlean").join("production");
    if production.exists() {
        std::fs::remove_dir_all(&production)?;
    }
    let status = Command::new(compiler())
        .current_dir(root)
        .args([
            "--color",
            "never",
            "production",
            "build",
            "--request",
            relative,
        ])
        .status()
        .map_err(|e| format!("cannot run `{} production build`: {e}", compiler()))?;
    if !status.success() {
        return Err(
            format!("`lexlean production build --request {relative}` failed ({status})").into(),
        );
    }
    let exported = exported_packages(root)?;
    if exported.is_empty() {
        return Err(format!("{relative}: the build published no rust-std package").into());
    }
    let dir = request.parent().ok_or("a request has no directory")?;
    let names: Vec<_> = exported.iter().filter_map(|p| p.file_name()).collect();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() && path.file_name().is_some_and(|n| !names.contains(&n)) {
            println!(
                "export: removing {}, which {relative} no longer exports",
                path.display()
            );
            std::fs::remove_dir_all(&path)?;
        }
    }
    for package in &exported {
        let name = package.file_name().ok_or("a package has no name")?;
        let committed = dir.join(name);
        if committed.exists() {
            std::fs::remove_dir_all(&committed)?;
        }
        copy_tree(package, &committed)?;
        println!("export: {} <- {relative}", committed.display());
    }
    Ok(())
}

/// Regenerate the pins of every request and the packages beside it.
pub fn export(root: &Path) -> Result<(), Fail> {
    let pins = read_pins(root)?;
    println!(
        "export: source {} semantic {} lock {}",
        pins.source_id, pins.semantic_id, pins.lock_sha256
    );
    for request in requests(root)? {
        pin_request(&request, &pins)?;
        export_request(root, &request)?;
    }
    Ok(())
}
