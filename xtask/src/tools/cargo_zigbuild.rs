// Thin process wrapper around cargo-zigbuild.
// It builds Unix-like CLI binaries for cross-platform npm packages.
use std::{path::Path, process::Command};

use anyhow::{bail, Context, Result};

pub fn build(
    workspace_root: &Path,
    package_name: &str,
    binary_name: &str,
    target: &str,
) -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "zigbuild",
            "-p",
            package_name,
            "--bin",
            binary_name,
            "--release",
            "--target",
            target,
        ])
        .status()
        .context("run cargo zigbuild")?;

    if !status.success() {
        bail!("cargo zigbuild failed");
    }

    Ok(())
}
