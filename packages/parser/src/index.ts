import {
  astToDocumentModel as astToDocumentModelWasm,
  documentModelToAst as documentModelToAstWasm,
  printQuizFold as printQuizFoldWasm,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "@quizfold/parser-wasm/bundler";

import type {
  Diagnostic,
  ModelDocument,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/bundler";

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

export async function astToDocumentModel(
  document: QuizFoldDocument,
): Promise<ModelDocument> {
  return astToDocumentModelWasm(document);
}

export async function documentModelToAst(
  document: ModelDocument,
): Promise<QuizFoldDocument> {
  return documentModelToAstWasm(document);
}
