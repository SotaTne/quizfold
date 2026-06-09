import { fileURLToPath } from "node:url";

import { cloudflareTest } from "@cloudflare/vitest-pool-workers";
import { playwright } from "@vitest/browser-playwright";
import { defineConfig } from "vitest/config";

const testFile = "tests/*.test.ts";

function fromPackage(path: string): string {
  return fileURLToPath(new URL(path, import.meta.url));
}

export default defineConfig({
  test: {
    projects: [
      {
        resolve: {
          alias: {
            "#parser-under-test": fromPackage("./dist/index.node.js"),
          },
        },
        test: {
          environment: "node",
          include: [testFile],
          name: "node",
        },
      },
      {
        resolve: {
          alias: {
            "#parser-under-test": fromPackage("./dist/index.browser.js"),
          },
        },
        test: {
          browser: {
            enabled: true,
            headless: true,
            instances: [{ browser: "chromium" }],
            provider: playwright(),
          },
          include: [testFile],
          name: "browser",
        },
      },
      {
        plugins: [
          cloudflareTest({
            miniflare: {
              compatibilityDate: "2026-06-09",
            },
            wrangler: {
              configPath: fromPackage("./tests/wrangler.jsonc"),
            },
          }),
        ],
        resolve: {
          alias: {
            "#parser-under-test": fromPackage("./dist/index.worker.js"),
          },
        },
        test: {
          include: [testFile],
          name: "worker",
        },
      },
    ],
  },
});
