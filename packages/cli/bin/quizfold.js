#!/usr/bin/env node

import { spawn } from "node:child_process";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

const require = createRequire(import.meta.url);
const targets = {
  "darwin-arm64": {
    packageName: "@quizfold/cli-aarch64-apple-darwin",
    binaryName: "quizfold",
  },
  "darwin-x64": {
    packageName: "@quizfold/cli-x86_64-apple-darwin",
    binaryName: "quizfold",
  },
  "linux-arm64": {
    packageName: "@quizfold/cli-aarch64-unknown-linux-gnu",
    binaryName: "quizfold",
  },
  "linux-x64": {
    packageName: "@quizfold/cli-x86_64-unknown-linux-gnu",
    binaryName: "quizfold",
  },
  "win32-arm64": {
    packageName: "@quizfold/cli-aarch64-pc-windows-msvc",
    binaryName: "quizfold.exe",
  },
  "win32-x64": {
    packageName: "@quizfold/cli-x86_64-pc-windows-msvc",
    binaryName: "quizfold.exe",
  },
};

function main() {
  const platformKey = `${process.platform}-${process.arch}`;
  const target = targets[platformKey];

  if (!target || (process.platform === "linux" && !isGlibc())) {
    console.error(`QuizFold CLI does not support this platform: ${platformKey}`);
    return 1;
  }

  let launcherPath;
  try {
    launcherPath = require.resolve(`${target.packageName}/bin/quizfold.js`);
  } catch {
    console.error(`QuizFold native package is not installed: ${target.packageName}`);
    console.error("Reinstall @quizfold/cli with optional dependencies enabled.");
    return 1;
  }

  const binaryPath = join(dirname(launcherPath), target.binaryName);
  const child = spawn(binaryPath, process.argv.slice(2), {
    shell: false,
    stdio: "inherit",
    windowsHide: false,
  });

  child.once("error", (error) => {
    console.error(`Failed to start QuizFold CLI: ${error.message}`);
    process.exitCode = 1;
  });

  child.once("exit", (code, signal) => {
    if (signal) {
      process.kill(process.pid, signal);
      return;
    }

    process.exitCode = code ?? 1;
  });

  return 0;
}

function isGlibc() {
  return Boolean(process.report?.getReport?.().header?.glibcVersionRuntime);
}

process.exitCode = main();
