import * as nodeWasm from "./wasm/node/parser.js";

import type {
  Diagnostic,
  ParseResult,
} from "./wasm/node/parser.js";

export async function parseQuizFold(input: string): Promise<ParseResult> {
  return nodeWasm.parseQuizFold(input);
}

export async function validateQuizFold(input: string): Promise<Diagnostic[]> {
  return nodeWasm.validateQuizFold(input);
}
