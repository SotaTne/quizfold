import {
  formatQuizFold as formatQuizFoldWasm,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "./wasm/bundler/parser.js";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/bundler/parser.js";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/bundler/parser.js";

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
