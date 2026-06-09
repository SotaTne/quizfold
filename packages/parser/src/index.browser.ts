import initWasm, {
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "./wasm/browser/parser.js";

import type {
  Diagnostic,
  ParseResult,
} from "./wasm/browser/parser.js";

let initialization: Promise<unknown> | undefined;

function initialize(): Promise<unknown> {
  initialization ??= initWasm().catch((error) => {
    initialization = undefined;
    throw error;
  });

  return initialization;
}

export async function parseQuizFold(input: string): Promise<ParseResult> {
  await initialize();
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  await initialize();
  return validateQuizFoldWasm(input);
}
