import { describe, expect, it } from "vitest";

import { printQuizFold, parseQuizFold } from "#parser-under-test";

describe("QuizFold printer", () => {
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
    const printed = await printQuizFold(first.document);
    const second = await parseQuizFold(printed);

    expect(first.diagnostics).toEqual([]);
    expect(second.diagnostics).toEqual([]);
    expect(printed).toBe(source);
    expect(await printQuizFold(second.document)).toBe(printed);
  });
});
