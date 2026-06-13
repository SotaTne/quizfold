import * as nodeWasm from "./wasm/node/parser.js";

import type {
  Diagnostic,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/node/parser.js";

export type {
  Diagnostic,
  ErrorCode,
  ParseResult,
  QuizFoldDocument,
} from "./wasm/node/parser.js";

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return nodeWasm.parseQuizFold(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return nodeWasm.validateQuizFold(input);
}

export async function formatQuizFold(
  document: QuizFoldDocument,
): Promise<string> {
  return nodeWasm.formatQuizFold(document);
}
