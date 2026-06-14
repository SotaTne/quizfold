import { StreamableHTTPTransport } from "@hono/mcp";
import { Hono } from "hono";

import { createMcpServer } from "../src/index.js";

const app = new Hono();
const mcpServer = createMcpServer();
const transport = new StreamableHTTPTransport({
  sessionIdGenerator: undefined,
});

await mcpServer.connect(transport);

app.all("/mcp", (context) => transport.handleRequest(context));

export default app;
