import {
  printQuizFold as printQuizFoldWasm,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "@quizfold/parser-wasm/bundler";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/bundler";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/bundler";

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
