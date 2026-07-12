// Build-time support for wasm crates (`crates/*-wasm`).
// Each wasm crate drives its own `bin/build.rs` and calls into this library;
// this crate holds only the parts that are the same across all of them
// (invoking wasm-pack, fixing up wasm-bindgen's generated type output).
use std::{fs, path::Path, process::Command};

use anyhow::{bail, Context, Result};

pub fn wasm_pack_build(
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

/// wasm-bindgen (via tsify/serde-wasm-bindgen) sometimes emits
/// `export interface Foo extends Bar { ... }` for what is actually a flattened
/// type alias, which is invalid TypeScript for a type that isn't an object
/// literal. Rewrite those as intersection type aliases instead.
pub fn normalize_flattened_interfaces(path: &Path) -> Result<()> {
    let generated = fs::read_to_string(path)
        .with_context(|| format!("read generated types from {}", path.display()))?;
    let normalized = replace_flattened_wrapper_type_aliases(generated);

    fs::write(path, normalized)
        .with_context(|| format!("write normalized types to {}", path.display()))?;
    Ok(())
}

fn replace_flattened_wrapper_type_aliases(generated_types: String) -> String {
    let lines = generated_types.lines().collect::<Vec<_>>();
    let mut output = String::with_capacity(generated_types.len());
    let mut index = 0;

    while index < lines.len() {
        if let Some((name, kind)) = parse_extends_interface_header(lines[index]) {
            output.push_str(&format!("export type {name} = {kind} & {{\n"));
            index += 1;
            let mut brace_depth = 1;

            while index < lines.len() {
                let line = lines[index];
                output.push_str(line);
                brace_depth += count_char(line, '{');
                brace_depth -= count_char(line, '}');

                if brace_depth == 0 {
                    output.push(';');
                    output.push('\n');
                    index += 1;
                    break;
                }

                output.push('\n');
                index += 1;
            }

            continue;
        }

        output.push_str(lines[index]);
        output.push('\n');
        index += 1;
    }

    output
}

fn parse_extends_interface_header(line: &str) -> Option<(&str, &str)> {
    let line = line.strip_prefix("export interface ")?;
    let (name, rest) = line.split_once(" extends ")?;
    let kind = rest.strip_suffix(" {")?;

    Some((name, kind))
}

fn count_char(value: &str, needle: char) -> usize {
    value
        .chars()
        .filter(|character| *character == needle)
        .count()
}
