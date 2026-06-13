import { describe, expect, it } from "vitest";

import { formatQuizFold, parseQuizFold } from "#parser-under-test";

describe("QuizFold formatter", () => {
  it("round-trips parser AST as canonical Markdown", async () => {
    const source = [
      "? Energy equation?",
      "@memo",
      "Use mass and velocity.",
      "@end",
      "---",
      "$E = mc^2$",
      "",
    ].join("\n");

    const first = await parseQuizFold(source);
    const formatted = await formatQuizFold(first.document);
    const second = await parseQuizFold(formatted);

    expect(first.diagnostics).toEqual([]);
    expect(second.diagnostics).toEqual([]);
    expect(formatted).toBe(source);
    expect(await formatQuizFold(second.document)).toBe(formatted);
  });
});
