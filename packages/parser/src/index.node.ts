import * as nodeWasm from "@quizfold/parser-wasm/node";

import type {
  Diagnostic,
  ModelDocument,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/node";

export type {
  Diagnostic,
  ErrorCode,
  ModelDiagnostic,
  ModelError,
  ModelBlank,
  ModelBlankInline,
  ModelBlock,
  ModelContent,
  ModelDocument,
  ModelFold,
  ModelInline,
  ModelItem,
  ModelMemo,
  ModelNote,
  ModelParagraph,
  ModelQa,
  ModelQaFold,
  ModelErrorCode,
  ParseResult,
  ParseError,
  ParseErrorCode,
  Severity,
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

export async function astToDocumentModel(
  document: QuizFoldDocument,
): Promise<ModelDocument> {
  return nodeWasm.astToDocumentModel(document);
}

export async function documentModelToAst(
  document: ModelDocument,
): Promise<QuizFoldDocument> {
  return nodeWasm.documentModelToAst(document);
}
