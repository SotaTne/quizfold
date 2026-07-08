import { describe, expect, it } from "vitest";

import { parseQuizFold } from "#parser-under-test";

describe("QuizFold quizzes", () => {
  it("parses fold quizzes with embedded blanks", async () => {
    const source = "! Japan is ${Tokyo}.\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]).toMatchObject({
      kind: "Quiz",
      value: {
        kind: "Fold",
        value: {
          content: {
            blocks: [
              {
                kind: "Paragraph",
                value: {
                  inlines: [
                    {
                      kind: "Raw",
                      value: {
                        value: "Japan is ",
                      },
                    },
                    {
                      kind: "FoldBlank",
                      value: {
                        answer: {
                          inlines: [
                            {
                              kind: "Raw",
                              value: {
                                value: "Tokyo",
                              },
                            },
                          ],
                        },
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
            ],
          },
        },
      },
    });
  });

  it("parses qa quizzes with answers", async () => {
    const source = "? Capital of Japan?\n---\nTokyo\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]).toMatchObject({
      kind: "Quiz",
      value: {
        kind: "Qa",
        value: {
          question: {
            blocks: [
              {
                kind: "Paragraph",
                value: {
                  inlines: [
                    {
                      kind: "Raw",
                      value: {
                        value: "Capital of Japan?",
                      },
                    },
                  ],
                },
              },
            ],
          },
          answer: {
            blocks: [
              {
                kind: "Paragraph",
                value: {
                  inlines: [
                    {
                      kind: "Raw",
                      value: {
                        value: "Tokyo",
                      },
                    },
                  ],
                },
              },
            ],
          },
        },
      },
    });
  });
});
