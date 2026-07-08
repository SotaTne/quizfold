// Thin process wrapper around cargo-xwin.
// It builds Windows CLI binaries from non-Windows hosts.
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
            "xwin",
            "build",
            "-p",
            package_name,
            "--bin",
            binary_name,
            "--release",
            "--target",
            target,
        ])
        .status()
        .context("run cargo xwin build")?;

    if !status.success() {
        bail!("cargo xwin build failed");
    }

    Ok(())
}
