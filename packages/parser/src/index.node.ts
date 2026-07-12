import * as nodeWasm from "@quizfold/parser-wasm/node";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/node";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/node";

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return nodeWasm.parseQuizFold(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return nodeWasm.validateQuizFold(input);
}

export async function printQuizFold(
  document: QuizFoldDocument,
): Promise<string> {
  return nodeWasm.printQuizFold(document);
}
