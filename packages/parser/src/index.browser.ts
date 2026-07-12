import initWasm, {
  astToDocumentModel as astToDocumentModelWasm,
  documentModelToAst as documentModelToAstWasm,
  printQuizFold as printQuizFoldWasm,
  parseQuizFold as parseQuizFoldWasm,
  validateQuizFold as validateQuizFoldWasm,
} from "@quizfold/parser-wasm/browser";

import type {
  Diagnostic,
  ModelDocument,
  ParseResult,
  QuizFoldDocument,
} from "@quizfold/parser-wasm/browser";

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
} from "@quizfold/parser-wasm/browser";

let initialization: Promise<unknown> | undefined;

function initialize(): Promise<unknown> {
  initialization ??= initWasm().catch((error) => {
    initialization = undefined;
    throw error;
  });

  return initialization;
}

export async function parseQuizFold(input: string): Promise<ParseResult> {
  await initialize();
  return parseQuizFoldWasm(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  await initialize();
  return validateQuizFoldWasm(input);
}

export async function printQuizFold(
  document: QuizFoldDocument,
): Promise<string> {
  await initialize();
  return printQuizFoldWasm(document);
}

export async function astToDocumentModel(
  document: QuizFoldDocument,
): Promise<ModelDocument> {
  await initialize();
  return astToDocumentModelWasm(document);
}

export async function documentModelToAst(
  document: ModelDocument,
): Promise<QuizFoldDocument> {
  await initialize();
  return documentModelToAstWasm(document);
}
