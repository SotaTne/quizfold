import {
  printQuizFold as printQuizFoldWasm,
  initSync,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "@quizfold/parser-wasm/browser";
import parserWasm from "@quizfold/parser-wasm/browser/parser_bg.wasm?module";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/browser";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/browser";

initSync({ module: parserWasm });

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return validateQuizFoldWasm(input);
}

export async function printQuizFold(
  document: QuizFoldDocument,
): Promise<string> {
  return printQuizFoldWasm(document);
}
