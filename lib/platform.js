"use strict";

/**
 * Map Node's process.platform / process.arch → Rust target triple
 * used by cargo-dist / GitHub Release asset names.
 * One entry per target release.yml builds; a key with no release asset
 * would download a 404, so it stays out (Windows ARM: no asset yet).
 */

const TARGETS = {
  "darwin-arm64": "aarch64-apple-darwin",
  "darwin-x64": "x86_64-apple-darwin",
  "linux-arm64": "aarch64-unknown-linux-gnu",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "win32-x64": "x86_64-pc-windows-msvc",
};

/**
 * @returns {{ triple: string, isWindows: boolean, binaryName: string }}
 */
function resolveTarget() {
  const key = `${process.platform}-${process.arch}`;
  const triple = TARGETS[key];
  if (!triple) {
    const supported = Object.keys(TARGETS).join(", ");
    throw new Error(
      `mcp-better: no prebuilt binary for ${key}. Supported: ${supported}. ` +
        "Build from source instead: cargo install mcp-better"
    );
  }
  const isWindows = process.platform === "win32";
  return {
    triple,
    isWindows,
    binaryName: isWindows ? "mcp-better.exe" : "mcp-better",
  };
}

module.exports = { resolveTarget, TARGETS };
