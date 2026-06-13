import {
  initSync,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "./wasm/browser/parser.js";
import parserWasm from "./wasm/browser/parser_bg.wasm?module";

import type {
  Diagnostic,
  ParseResult,
} from "./wasm/browser/parser.js";

export type { Diagnostic, ErrorCode, ParseResult } from "./wasm/browser/parser.js";

initSync({ module: parserWasm });

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return validateQuizFoldWasm(input);
}
