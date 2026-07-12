declare module "#parser-under-test" {
  export type ErrorCode = import("../src/index").ErrorCode;
  export type QuizFoldDocument = import("../src/index").QuizFoldDocument;
  export type ModelDocument = import("../src/index").ModelDocument;
  export function parseQuizFold(
    input: string,
  ): Promise<import("../src/index").ParseResult>;
  export function validateQuizFold(
    input: string,
  ): Promise<import("../src/index").Diagnostic[]>;
  export function printQuizFold(
    document: QuizFoldDocument,
  ): Promise<string>;
  export function astToDocumentModel(
    document: QuizFoldDocument,
  ): Promise<ModelDocument>;
  export function documentModelToAst(
    document: ModelDocument,
  ): Promise<QuizFoldDocument>;
}
