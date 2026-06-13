import type { AsyncOrSync } from "./types.js";

export type CheckStoredImage = (imageId: string) => AsyncOrSync<boolean>;

export type CheckRemoteUrl = (url: URL) => AsyncOrSync<boolean>;

export type McpDeps = Readonly<{
  checkStoredImage?: CheckStoredImage;
  checkRemoteUrl?: CheckRemoteUrl;
}>;

export const defaultMcpDeps: McpDeps = Object.freeze({});
