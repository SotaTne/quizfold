import { describe, expect, it } from "vitest";

import { parseQuizFold, validateQuizFold } from "#parser-under-test";
import type { ErrorCode } from "#parser-under-test";

const UNCLOSED_MEMO_CODE: ErrorCode = "QF008";

describe("QuizFold memo blocks", () => {
  it("parses a memo with structured content", async () => {
    const source = "@memo\nRemember this.\n\n$$\nE = mc^2\n$$\n@end\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]).toMatchObject({
      kind: "Block",
      value: {
        kind: "Memo",
        value: {
          blocks: [
            { kind: "Paragraph", value: expect.any(Object) },
            { kind: "MathBlock", value: { source: "E = mc^2" } },
          ],
        },
      },
    });
  });

  it("returns a typed diagnostic for an unclosed memo", async () => {
    const diagnostics = await validateQuizFold("@memo\nRemember this.\n");

    expect(diagnostics).toMatchObject([
      {
        code: UNCLOSED_MEMO_CODE,
        error: "UnclosedMemo",
        severity: "Error",
      },
    ]);
  });

  it("treats quiz markers inside memo as text", async () => {
    const source = "@memo\n? note\n! ${still text}\n---\n@end\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items).toHaveLength(1);
    expect(result.document.items[0]).toMatchObject({
      kind: "Block",
      value: {
        kind: "Memo",
        value: expect.any(Object),
      },
    });
  });
});
