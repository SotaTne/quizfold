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
        Build::Wasm => build_wasm(),
        Build::Cli { selections } => build_cli(&selections),
    }
}

fn build_wasm() -> Result<()> {
    let workspace_root = workspace_root();
    let parser_package = workspace_root.join("packages/parser");
    let dist = parser_package.join("dist");
    let wasm_dist = dist.join("wasm");

    if dist.exists() {
        fs::remove_dir_all(&dist).context("remove parser dist")?;
    }
    fs::create_dir_all(&wasm_dist).context("create parser wasm dist")?;

    for (target, output_name) in [
        ("web", "browser"),
        ("bundler", "bundler"),
        ("nodejs", "node"),
    ] {
        let output_dir = wasm_dist.join(output_name);
        tools::wasm_pack::build(
            &workspace_root,
            &workspace_root.join("crates/parser-wasm"),
            target,
            &output_dir,
            "parser",
        )?;
        let gitignore = output_dir.join(".gitignore");
        if gitignore.exists() {
            fs::remove_file(gitignore).context("remove wasm-pack gitignore")?;
        }
    }
    write_public_types(
        &dist.join("wasm/bundler/parser.d.ts"),
        &dist.join("index.d.ts"),
    )?;

    Ok(())
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

fn write_public_types(input: &Path, output: &Path) -> Result<()> {
    let generated_types = fs::read_to_string(input).context("read generated parser types")?;
    let public_types = generated_types
        .replace(
            "export function parseQuizFold(input: string): ParseResult;",
            "export function parseQuizFold(input: string): Promise<ParseResult>;",
        )
        .replace(
            "export function validateQuizFold(input: string): Diagnostic[];",
            "export function validateQuizFold(input: string): Promise<Diagnostic[]>;",
        );

    fs::write(output, public_types).context("write public parser types")?;
    Ok(())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is inside workspace")
        .to_path_buf()
}
