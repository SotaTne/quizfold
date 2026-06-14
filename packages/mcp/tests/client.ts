import type { Client } from "@modelcontextprotocol/sdk/client/index.js";

import type { McpDeps } from "../src/index.js";

export type ConnectedMcpClient = {
  client: Client;
  close(): Promise<void>;
};

export type ConnectMcpClient = (
  deps?: McpDeps,
) => Promise<ConnectedMcpClient>;
