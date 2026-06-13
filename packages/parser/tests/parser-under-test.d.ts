declare module "#parser-under-test" {
  export type ErrorCode = import("../src/index").ErrorCode;
  export function parseQuizFold(
    input: string,
  ): Promise<import("../src/index").ParseResult>;
  export function validateQuizFold(
    input: string,
  ): Promise<import("../src/index").Diagnostic[]>;
}
