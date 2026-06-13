import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";

import packageJson from "../package.json" with { type: "json" };

import type { McpDeps } from "./deps.js";

const MCP_SERVER_NAME = "quizfold";
const MCP_SERVER_VERSION = packageJson.version;

export function createMcpServer(_deps: McpDeps = {}): McpServer {
  return new McpServer({
    name: MCP_SERVER_NAME,
    version: MCP_SERVER_VERSION,
  });
}
