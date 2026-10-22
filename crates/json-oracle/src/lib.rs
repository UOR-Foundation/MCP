//! Runs the package generated from the LexLean JSON codec against the pinned
//! JSONTestSuite corpus (issue #18).
//!
//! Dev and CI only. Nothing here is a dependency of a shipped crate: the codec
//! is LexLean source in `src/`, and this crate is where its generated Rust
//! package is executed by `cargo test`.
