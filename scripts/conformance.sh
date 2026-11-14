#!/usr/bin/env bash
# Official MCP conformance suite against mcp-better (Streamable HTTP, 2026-07-28 requirements).
# Usage: bash scripts/conformance.sh   (needs a built target/debug/mcp-better and Node/npx)
set -euo pipefail
cd "$(dirname "$0")/.."

SUITE="@modelcontextprotocol/conformance@0.2.0-alpha.12"
ADDR="127.0.0.1:${MCP_CONFORMANCE_PORT:-8799}"
BIN="${MCP_BETTER_BIN:-$(pwd)/target/debug/mcp-better}"
OUT="$(mktemp -d)"

MCP_BETTER_HTTP_ADDR="$ADDR" "$BIN" --http >"$OUT/server.log" 2>&1 &
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true; wait "$SERVER_PID" 2>/dev/null || true; rm -rf "$OUT"' EXIT

for _ in $(seq 1 50); do
  (exec 3<>"/dev/tcp/${ADDR%:*}/${ADDR#*:}") 2>/dev/null && break
  sleep 0.1
done

npx --yes "$SUITE" server \
  --url "http://$ADDR/mcp" \
  --requirements 2026-07-28 \
  --expected-failures conformance-baseline.yml \
  -o "$OUT/results"
