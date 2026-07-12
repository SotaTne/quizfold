// Build orchestration for native binaries. wasm packaging lives with each
// wasm crate itself (crates/*-wasm/bin/build.rs), backed by the local
// wasm-build-support crate, so xtask stays agnostic of wasm targets.
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

use crate::{
    cli::{BuildCommand as Build, CliBuildSelection, CliBuildTarget},
    tools,
};

pub fn run(command: Build) -> Result<()> {
    match command {
        Build::Cli { selections } => build_cli(&selections),
    }
}

fn build_cli(selections: &[CliBuildSelection]) -> Result<()> {
    let workspace_root = workspace_root();
    let selected_targets = resolve_cli_targets(selections)?;

    for target in selected_targets {
        build_cli_binary(&workspace_root, target)?;
        copy_cli_binary(&workspace_root, target)?;
    }

    Ok(())
}

fn build_cli_binary(workspace_root: &Path, target: CliBuildTarget) -> Result<()> {
    let target_triple = target.target_triple();

    if target.is_windows() {
        tools::cargo_xwin::build(workspace_root, "quizfold-cli", "quizfold", target_triple)
    } else {
        tools::cargo_zigbuild::build(workspace_root, "quizfold-cli", "quizfold", target_triple)
    }
}

fn copy_cli_binary(workspace_root: &Path, target: CliBuildTarget) -> Result<()> {
    let target_triple = target.target_triple();
    let binary_name = target.binary_name();
    let built_binary = workspace_root
        .join("target")
        .join(target_triple)
        .join("release")
        .join(binary_name);
    let package_dir = workspace_root.join(format!("packages/cli-{target_triple}"));
    let output_binary = package_dir.join("bin").join(binary_name);

    if !package_dir.is_dir() {
        bail!("cli package does not exist: {}", package_dir.display());
    }

    fs::create_dir_all(package_dir.join("bin")).context("create cli bin dir")?;
    fs::copy(&built_binary, &output_binary).with_context(|| {
        format!(
            "copy built cli binary from {} to {}",
            built_binary.display(),
            output_binary.display()
        )
    })?;
    set_executable_if_unix(&output_binary)?;
    println!("{} copied to {}", target_triple, output_binary.display());

    Ok(())
}

fn resolve_cli_targets(selections: &[CliBuildSelection]) -> Result<Vec<CliBuildTarget>> {
    if selections.is_empty() {
        bail!("at least one cli target is required");
    }

    if selections.contains(&CliBuildSelection::All) {
        if selections.len() != 1 {
            bail!("all cannot be combined with specific cli targets");
        }

        return Ok(CliBuildSelection::All.expand());
    }

    Ok(selections
        .iter()
        .flat_map(|selection| selection.expand())
        .collect())
}

#[cfg(unix)]
fn set_executable_if_unix(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .with_context(|| format!("read permissions for {}", path.display()))?
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
        .with_context(|| format!("set executable permissions for {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_executable_if_unix(_path: &Path) -> Result<()> {
    Ok(())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is inside workspace")
        .to_path_buf()
}
