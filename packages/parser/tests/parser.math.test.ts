import { describe, expect, it } from "vitest";

import { parseQuizFold } from "#parser-under-test";

describe("QuizFold math", () => {
  it("parses math blocks", async () => {
    const source = "$$\nE = mc^2\n$$\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]).toMatchObject({
      kind: "Block",
      value: {
        kind: "MathBlock",
        value: {
          source: "E = mc^2",
        },
      },
    });
  });

  it("parses inline math", async () => {
    const source = "Energy is $E = mc^2$.\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]).toMatchObject({
      kind: "Block",
      value: {
        kind: "Paragraph",
        value: {
          inlines: [
            {
              kind: "Raw",
              value: {
                value: "Energy is ",
              },
            },
            {
              kind: "MathInline",
              value: {
                source: "E = mc^2",
              },
            },
            {
              kind: "Raw",
              value: {
                value: ".",
              },
            },
          ],
        },
      },
    });
  });
});
