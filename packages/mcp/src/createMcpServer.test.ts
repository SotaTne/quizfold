import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { describe, expect, it } from "bun:test";

import { createMcpServer } from "./index.js";

describe("createMcpServer", () => {
  it("creates an MCP server instance", () => {
    expect(createMcpServer()).toBeInstanceOf(McpServer);
  });
});
