#!/usr/bin/env node

import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const binaryName = "quizfold";
const binaryPath = join(dirname(fileURLToPath(import.meta.url)), binaryName);
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
