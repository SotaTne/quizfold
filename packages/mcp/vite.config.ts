import { fileURLToPath } from "node:url";

import { defineConfig } from "vitest/config";

const e2eTests = "tests/e2e/**/*.test.ts";

function fromPackage(path: string): string {
  return fileURLToPath(new URL(path, import.meta.url));
}

export default defineConfig({
  test: {
    projects: [
      {
        resolve: {
          alias: {
            "#mcp-test-server": fromPackage("./tests/server.node.ts"),
          },
        },
        test: {
          environment: "node",
          include: [e2eTests],
          name: "node",
        },
      },
      {
        resolve: {
          alias: {
            "#mcp-test-server": fromPackage("./tests/server.worker.ts"),
          },
        },
        test: {
          environment: "node",
          include: [e2eTests],
          name: "worker",
        },
      },
    ],
  },
});
