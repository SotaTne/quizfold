import { describe, expect, it } from "vitest";

import {
  astToDocumentModel,
  documentModelToAst,
  parseQuizFold,
  printQuizFold,
} from "#parser-under-test";
import type { ModelDocument } from "#parser-under-test";

describe("document model", () => {
  it("converts AST and document model in both directions", async () => {
    const parsed = await parseQuizFold(
      "? Capital of Japan?\n---\n${Tokyo} and ${東京}\n",
    );
    const model = await astToDocumentModel(parsed.document);

    expect(model.items[0]?.kind).toBe("QaFold");
    if (model.items[0]?.kind !== "QaFold") {
      throw new Error("expected QaFold model item");
    }
    expect(model.items[0].value.blanks).toHaveLength(2);
    expect(model.items[0].value.blanks[0]).toEqual({
      answer: [{ kind: "Raw", value: "Tokyo" }],
    });
    expect(model.items[0].value.question.blocks[0]).toMatchObject({
      kind: "Paragraph",
      value: {
        inlines: [{ kind: "Raw", value: "Capital of Japan?" }],
      },
    });

    const restored = await documentModelToAst(model);
    expect(await printQuizFold(restored)).toBe(
      "? Capital of Japan?\n---\n${Tokyo} and ${東京}\n",
    );
  });

  it("rejects an invalid blank reference", async () => {
    const invalid: ModelDocument = {
      items: [
        {
          kind: "Fold",
          value: {
            content: [{ kind: "Blank", value: 1 }],
            blanks: [
              {
                answer: [{ kind: "Raw", value: "Tokyo" }],
              },
            ],
          },
        },
      ],
    };

    await expect(documentModelToAst(invalid)).rejects.toMatchObject({
      code: "QFM004",
      error: "BlankOutOfOrder",
      severity: "Error",
    });
  });
});
