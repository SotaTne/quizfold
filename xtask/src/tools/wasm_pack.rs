use std::{path::Path, process::Command};

use anyhow::{bail, Context, Result};

pub fn build(
    workspace_root: &Path,
    crate_dir: &Path,
    target: &str,
    output_dir: &Path,
    output_name: &str,
) -> Result<()> {
    let status = Command::new("wasm-pack")
        .current_dir(workspace_root)
        .args([
            "build",
            crate_dir
                .to_str()
                .context("crate path is not valid UTF-8")?,
            "--mode",
            "no-install",
            "--target",
            target,
            "--out-dir",
            output_dir
                .to_str()
                .context("output path is not valid UTF-8")?,
            "--out-name",
            output_name,
            "--no-opt",
            "--release",
        ])
        .status()
        .context("run wasm-pack build")?;

    if !status.success() {
        bail!("wasm-pack build failed");
    }

    Ok(())
}
