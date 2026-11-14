#!/usr/bin/env bash
# rust-mcp-preflight on server.json: will the MCP Registry accept this listing?
#
#   bash scripts/preflight.sh           # CI: skips while the version is not on crates.io yet
#   bash scripts/preflight.sh --strict  # release gate (registry-publish.sh): every check, no skip
set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v rust-mcp-preflight >/dev/null; then
  if [ "${1:-}" = "--strict" ]; then
    echo "preflight: rust-mcp-preflight is required here (cargo binstall rust-mcp-preflight)" >&2
    exit 1
  fi
  echo "preflight: skipped, rust-mcp-preflight not installed (cargo binstall rust-mcp-preflight)"
  exit 0
fi

VER=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
if [ "${1:-}" != "--strict" ]; then
  CODE=$(curl -s -o /dev/null -w '%{http_code}' -A "mcp-better-ci (github.com/Wolfe-Jam/mcp-better)" \
    "https://crates.io/api/v1/crates/mcp-better/$VER")
  if [ "$CODE" = "404" ]; then
    echo "preflight: mcp-better $VER is not on crates.io yet (a release in progress)."
    echo "           The full check runs in scripts/registry-publish.sh after cargo publish."
    exit 0
  fi
fi

rust-mcp-preflight server.json
