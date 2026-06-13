import {
  formatQuizFold as formatQuizFoldWasm,
  initSync,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "./wasm/browser/parser.js";
import parserWasm from "./wasm/browser/parser_bg.wasm?module";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/browser/parser.js";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/browser/parser.js";

initSync({ module: parserWasm });

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return validateQuizFoldWasm(input);
}

export async function formatQuizFold(
  document: QuizFoldDocument,
): Promise<string> {
  return formatQuizFoldWasm(document);
}
