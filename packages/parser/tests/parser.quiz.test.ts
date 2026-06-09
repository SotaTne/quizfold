import { describe, expect, it } from "vitest";

import { parseQuizFold } from "#parser-under-test";

describe("QuizFold quizzes", () => {
  it("parses fold quizzes with embedded blanks", async () => {
    const source = "! Japan is ${Tokyo}.\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]?.kind).toMatchObject({
      Quiz: {
        kind: {
          Fold: {
            content: {
              blocks: [
                {
                  kind: {
                    Paragraph: {
                      inlines: [
                        {
                          kind: {
                            Raw: {
                              value: "Japan is ",
                            },
                          },
                        },
                        {
                          kind: {
                            FoldBlank: {
                              answer: {
                                inlines: [
                                  {
                                    kind: {
                                      Raw: {
                                        value: "Tokyo",
                                      },
                                    },
                                  },
                                ],
                              },
                            },
                          },
                        },
                        {
                          kind: {
                            Raw: {
                              value: ".",
                            },
                          },
                        },
                      ],
                    },
                  },
                },
              ],
            },
          },
        },
      },
    });
  });

  it("parses qa quizzes with answers", async () => {
    const source = "? Capital of Japan?\n---\nTokyo\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items[0]?.kind).toMatchObject({
      Quiz: {
        kind: {
          Qa: {
            question: {
              blocks: [
                {
                  kind: {
                    Paragraph: {
                      inlines: [
                        {
                          kind: {
                            Raw: {
                              value: "Capital of Japan?",
                            },
                          },
                        },
                      ],
                    },
                  },
                },
              ],
            },
            answer: {
              blocks: [
                {
                  kind: {
                    Paragraph: {
                      inlines: [
                        {
                          kind: {
                            Raw: {
                              value: "Tokyo",
                            },
                          },
                        },
                      ],
                    },
                  },
                },
              ],
            },
          },
        },
      },
    });
  });
});
