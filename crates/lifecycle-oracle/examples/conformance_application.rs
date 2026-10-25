//! Writes or checks `conformance/src/Application.lex.tex`, the declaration of the
//! conformance build, from the vectors of the pinned scenarios and the committed core
//! package (`tests/common/application.rs`).
//!
//! `cargo run -p lifecycle-oracle --example conformance_application -- --write`
//! rewrites the file; without `--write` it fails when the file is not the rendering.

#[path = "../tests/common/mod.rs"]
mod common;

fn main() {
    let rendered = common::application::render();
    let path = common::root().join("conformance/src/Application.lex.tex");
    if std::env::args().any(|argument| argument == "--write") {
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the directory");
        std::fs::write(&path, rendered).expect("the file writes");
        println!("wrote {}", path.display());
    } else if std::fs::read_to_string(&path).ok().as_deref() != Some(rendered.as_str()) {
        eprintln!("{} is not the rendering of the vectors", path.display());
        std::process::exit(1);
    }
}
