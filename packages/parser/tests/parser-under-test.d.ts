declare module "#parser-under-test" {
  export type ErrorCode = import("../src/index").ErrorCode;
  export type QuizFoldDocument = import("../src/index").QuizFoldDocument;
  export function parseQuizFold(
    input: string,
  ): Promise<import("../src/index").ParseResult>;
  export function validateQuizFold(
    input: string,
  ): Promise<import("../src/index").Diagnostic[]>;
  export function formatQuizFold(
    document: QuizFoldDocument,
  ): Promise<string>;
}
