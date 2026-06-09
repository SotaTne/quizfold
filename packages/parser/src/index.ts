import {
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "./wasm/bundler/parser.js";

import type { Diagnostic, ParseResult } from "./wasm/bundler/parser.js";

export type { Diagnostic, ParseResult } from "./wasm/bundler/parser.js";

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return validateQuizFoldWasm(input);
}
