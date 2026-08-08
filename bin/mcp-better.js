#!/usr/bin/env node
"use strict";

/**
 * npm shim for mcp-better — zero Rust toolchain required.
 *
 * Detects platform/arch, downloads the matching binary from GitHub Releases
 * (cargo-dist asset layout), then execs it with inherited stdio.
 */

const { spawn } = require("child_process");
const path = require("path");
const { ensureBinary } = require("../lib/download");

const pkg = require("../package.json");
const version = pkg.version;

async function main() {
  // Escape hatch: use a local/dev binary without download
  const override = process.env.MCP_BETTER_BIN;
  let bin;
  if (override) {
    bin = override;
  } else {
    bin = await ensureBinary(version);
  }

  const child = spawn(bin, process.argv.slice(2), {
    stdio: "inherit",
    windowsHide: true,
  });

  child.on("error", (err) => {
    console.error(`mcp-better: failed to spawn ${bin}: ${err.message}`);
    process.exit(1);
  });

  child.on("exit", (code, signal) => {
    if (signal) {
      process.kill(process.pid, signal);
      return;
    }
    process.exit(code == null ? 1 : code);
  });
}

main().catch((err) => {
  console.error(err.message || err);
  process.exit(1);
});
