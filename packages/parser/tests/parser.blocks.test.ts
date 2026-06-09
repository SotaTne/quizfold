import { describe, expect, it } from "vitest";

import { parseQuizFold } from "#parser-under-test";

describe("QuizFold blocks", () => {
  it("parses mermaid fences", async () => {
    const source = "```mmd\nflowchart LR\nA --> B\n```\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items).toHaveLength(1);
    expect(result.document.items[0]?.kind).toMatchObject({
      Block: {
        kind: {
          MermaidBlock: {
            source: "flowchart LR\nA --> B",
          },
        },
      },
    });
  });

  it("parses code fences", async () => {
    const source = "```rust\nfn main() {}\n```\n";
    const result = await parseQuizFold(source);

    expect(result.diagnostics).toEqual([]);
    expect(result.document.items).toHaveLength(1);
    expect(result.document.items[0]?.kind).toMatchObject({
      Block: {
        kind: {
          CodeBlock: {
            language: "rust",
            source: "fn main() {}",
          },
        },
      },
    });
  });
});
