//! Builds this crate's wasm-bindgen targets (browser/bundler/node) into
//! ts-gen/. Not part of the production wasm build (see `build-support`
//! feature / `required-features` on this bin) — it runs on the host, driving
//! wasm-pack. Self-contained: packages/parser derives its own public
//! `dist/index.d.ts` from its real source via `tsc`, so this crate doesn't
//! need to know anything about packages/parser.
//!
//! ```bash
//! cargo run -p quizfold-parser-wasm --bin build-wasm --features build-support
//! ```
use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use wasm_build_support::{normalize_flattened_interfaces, wasm_pack_build};

fn main() -> Result<()> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .parent()
        .and_then(std::path::Path::parent)
        .context("locate workspace root")?
        .to_path_buf();
    let ts_gen = crate_dir.join("ts-gen");

    if ts_gen.exists() {
        fs::remove_dir_all(&ts_gen).context("remove existing ts-gen output")?;
    }
    fs::create_dir_all(&ts_gen).context("create ts-gen output dir")?;

    for (target, output_name) in [
        ("web", "browser"),
        ("bundler", "bundler"),
        ("nodejs", "node"),
    ] {
        let output_dir = ts_gen.join(output_name);
        wasm_pack_build(&workspace_root, &crate_dir, target, &output_dir, "parser")?;
        normalize_flattened_interfaces(&output_dir.join("parser.d.ts"))?;

        let gitignore = output_dir.join(".gitignore");
        if gitignore.exists() {
            fs::remove_file(gitignore).context("remove wasm-pack gitignore")?;
        }
    }

    Ok(())
}
