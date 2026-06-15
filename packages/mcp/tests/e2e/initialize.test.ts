import { afterEach, beforeEach, expect, test } from "vitest";
import packageJson from "../../package.json" with { type: "json" };

import { connectMcpClient } from "#mcp-test-server";

import type { ConnectedMcpClient } from "../client.js";

let connection: ConnectedMcpClient | undefined;

beforeEach(async () => {
  connection = await connectMcpClient();
});

afterEach(async () => {
  await connection?.close();
  connection = undefined;
});

test("initializes the QuizFold MCP server", () => {
  expect(connection?.client.getServerVersion()).toEqual({
    name: "quizfold",
    version: packageJson.version,
  });
});
