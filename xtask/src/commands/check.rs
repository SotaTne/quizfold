// Environment validation commands for repository tooling.
// These checks report missing external tools before build commands need them.
use anyhow::Result;

use crate::{cli::CheckCommand as Check, tools};

pub fn run(command: Check) -> Result<()> {
    match command {
        Check::Tool => run_tool_checks(),
    }

    Ok(())
}

fn run_tool_checks() {
    let mut ok = true;

    ok &= tools::check::ToolCheck {
        command: "wasm-pack",
        args: &["--version"],
        install_hint: "cargo install --locked --force wasm-pack",
        display_name: "wasm-pack",
    }
    .verify();
    ok &= tools::check::ToolCheck {
        command: "cargo",
        args: &["zigbuild", "--help"],
        install_hint: "cargo install --locked cargo-zigbuild",
        display_name: "cargo-zigbuild",
    }
    .verify();
    ok &= tools::check::ToolCheck {
        command: "cargo",
        args: &["xwin", "--help"],
        install_hint: "cargo install --locked cargo-xwin",
        display_name: "cargo-xwin",
    }
    .verify();
    ok &= tools::check::ToolCheck {
        command: "clang",
        args: &["--version"],
        install_hint: "please install LLVM/Clang",
        display_name: "clang",
    }
    .verify();
    ok &= tools::check::ToolCheck {
        command: "zig",
        args: &["version"],
        install_hint: "please install zig",
        display_name: "zig",
    }
    .verify();

    if !ok {
        std::process::exit(1);
    }
}
