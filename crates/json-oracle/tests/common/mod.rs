//! Shared by the oracle tests: where the pinned corpus lives and how to read it.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// The commit of nst/JSONTestSuite the corpus is vendored from.
pub const PIN: &str = "1ef36fa01286573e846ac449e8683f8833c5b26a";

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/json-oracle is two below the root")
        .to_path_buf()
}

/// The vendored directory of the pinned corpus; its name carries the pin.
pub fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("corpus")
        .join(format!("JSONTestSuite-{PIN}"))
}

/// Every file of `test_parsing/`, as (name, bytes), in byte order of the names.
pub fn corpus() -> Vec<(String, Vec<u8>)> {
    let dir = corpus_root().join("test_parsing");
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("the vendored corpus is missing at {}: {e}", dir.display()))
        .map(|entry| {
            let path = entry.expect("a directory entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("corpus names are ASCII")
                .to_owned();
            let bytes = std::fs::read(&path).expect("a corpus file reads");
            (name, bytes)
        })
        .collect();
    out.sort();
    out
}

/// The corpus files whose names start with `prefix` (`y_`, `n_`, `i_`).
pub fn kind(prefix: &str) -> Vec<(String, Vec<u8>)> {
    corpus()
        .into_iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect()
}

/// SHA-256 over the manifest: one `<sha256 of file>  <name>` line per file.
pub fn manifest_sha256() -> String {
    let mut manifest = String::new();
    for (name, bytes) in corpus() {
        manifest.push_str(&format!("{}  {name}\n", hex(&Sha256::digest(&bytes))));
    }
    hex(&Sha256::digest(manifest.as_bytes()))
}

/// Lower-case hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The rows of a recorded table: `name<TAB>value`, comments start with `#`.
pub fn table(file: &str) -> Vec<(String, String)> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tables")
        .join(file);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the recorded table {} is missing: {e}", path.display()));
    text.lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (name, value) = l.split_once('\t').expect("a table row has a tab");
            (name.to_owned(), value.to_owned())
        })
        .collect()
}
