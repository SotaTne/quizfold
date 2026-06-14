import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StreamableHTTPClientTransport } from "@modelcontextprotocol/sdk/client/streamableHttp.js";
import { Miniflare } from "miniflare";

import type { ConnectMcpClient } from "./client.js";

const WORKER_SCRIPT = new URL("../.tmp/mcp-worker.js", import.meta.url);

export const connectMcpClient: ConnectMcpClient = async () => {
  const miniflare = new Miniflare({
    compatibilityDate: "2026-06-11",
    compatibilityFlags: ["nodejs_compat"],
    host: "127.0.0.1",
    modules: true,
    port: 0,
    scriptPath: WORKER_SCRIPT.pathname,
  });
  const workerUrl = await miniflare.ready;
  const client = new Client({
    name: "quizfold-mcp-e2e",
    version: "0.1.0",
  });
  const transport = new StreamableHTTPClientTransport(
    new URL("/mcp", workerUrl),
  );

  try {
    await client.connect(transport);
  } catch (error) {
    await miniflare.dispose();
    throw error;
  }

  return {
    client,
    async close() {
      await client.close();
      await miniflare.dispose();
    },
  };
};
