import { createServer } from "node:http";

import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StreamableHTTPClientTransport } from "@modelcontextprotocol/sdk/client/streamableHttp.js";
import { StreamableHTTPServerTransport } from "@modelcontextprotocol/sdk/server/streamableHttp.js";

import { createMcpServer } from "../src/index.js";

import type { ConnectMcpClient } from "./client.js";

export const connectMcpClient: ConnectMcpClient = async (deps = {}) => {
  const httpServer = createServer((request, response) => {
    if (request.url !== "/mcp") {
      response.writeHead(404).end();
      return;
    }

    const mcpServer = createMcpServer(deps);
    const serverTransport = new StreamableHTTPServerTransport({
      sessionIdGenerator: undefined,
    });

    void mcpServer
      .connect(serverTransport)
      .then(() => serverTransport.handleRequest(request, response))
      .catch((error: unknown) => {
        response.writeHead(500).end(
          error instanceof Error ? error.message : "Unknown MCP server error",
        );
      });

    response.once("close", () => {
      void serverTransport.close();
      void mcpServer.close();
    });
  });

  await new Promise<void>((resolve, reject) => {
    httpServer.once("error", reject);
    httpServer.listen(0, "127.0.0.1", resolve);
  });

  const address = httpServer.address();
  if (address === null || typeof address === "string") {
    throw new Error("Node MCP test server did not bind to a TCP port.");
  }

  const client = new Client({
    name: "quizfold-mcp-e2e",
    version: "0.1.0",
  });
  const clientTransport = new StreamableHTTPClientTransport(
    new URL(`http://127.0.0.1:${address.port}/mcp`),
  );

  try {
    await client.connect(clientTransport);
  } catch (error) {
    httpServer.close();
    throw error;
  }

  return {
    client,
    async close() {
      await client.close();
      await new Promise<void>((resolve, reject) => {
        httpServer.close((error) => {
          if (error) {
            reject(error);
            return;
          }
          resolve();
        });
      });
    },
  };
};
