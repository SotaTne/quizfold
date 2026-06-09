import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const packageDirectory = dirname(dirname(fileURLToPath(import.meta.url)));
const workspaceDirectory = join(packageDirectory, "../..");
const crateDirectory = join(workspaceDirectory, "crates/parser-wasm");
const sourceDirectory = join(packageDirectory, "src");
const distributionDirectory = join(packageDirectory, "dist");
const wasmDirectory = join(distributionDirectory, "wasm");

if (import.meta.main) {
  await build();
}

async function build(): Promise<void> {
  await rm(distributionDirectory, { force: true, recursive: true });
  await mkdir(wasmDirectory, { recursive: true });

  await buildWasm("web", "browser");
  await buildWasm("bundler", "bundler");
  await buildWasm("nodejs", "node");

  await Promise.all([buildEntries(), writeTypes()]);
}

async function buildWasm(target: string, outputName: string): Promise<void> {
  const outputDirectory = join(wasmDirectory, outputName);
  const process = Bun.spawn(
    [
      "wasm-pack",
      "build",
      crateDirectory,
      "--target",
      target,
      "--out-dir",
      outputDirectory,
      "--out-name",
      "parser",
      "--release",
    ],
    {
      cwd: workspaceDirectory,
      stderr: "inherit",
      stdout: "inherit",
    },
  );

  const exitCode = await process.exited;
  if (exitCode !== 0) {
    throw new Error(`wasm-pack failed for target ${target}`);
  }

  await rm(join(outputDirectory, ".gitignore"), { force: true });
}

async function buildEntries(): Promise<void> {
  const result = await Bun.build({
    entrypoints: [
      join(sourceDirectory, "index.ts"),
      join(sourceDirectory, "index.browser.ts"),
      join(sourceDirectory, "index.worker.ts"),
      join(sourceDirectory, "index.node.ts"),
    ],
    external: ["./wasm/*"],
    format: "esm",
    minify: false,
    outdir: distributionDirectory,
    target: "browser",
  });

  if (!result.success) {
    throw new AggregateError(result.logs, "Failed to build parser entries");
  }
}

async function writeTypes(): Promise<void> {
  const generatedTypes = await readFile(
    join(wasmDirectory, "bundler/parser.d.ts"),
    "utf8",
  );
  const publicTypes = generatedTypes
    .replace(
      "export function parseQuizFold(input: string): ParseResult;",
      "export function parseQuizFold(input: string): Promise<ParseResult>;",
    )
    .replace(
      "export function validateQuizFold(input: string): Diagnostic[];",
      "export function validateQuizFold(input: string): Promise<Diagnostic[]>;",
    );

  await writeFile(join(distributionDirectory, "index.d.ts"), publicTypes);
}
