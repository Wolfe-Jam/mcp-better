#!/usr/bin/env bash
# Official MCP conformance suite against an mcp-better binary over Streamable HTTP
# (2026-07-28 requirements).
#
#   bash scripts/conformance.sh                 # textbook binary, baseline conformance-baseline.yml
#   MCP_CONFORMANCE_BASELINE= bash scripts/...  # no baseline: every failure fails the run
#
# Needs a built binary and Node/npx (the suite is fetched at the pinned version).
set -euo pipefail
cd "$(dirname "$0")/.."

SUITE="@modelcontextprotocol/conformance@0.2.0-alpha.12"
PORT="${MCP_CONFORMANCE_PORT:-8799}"
ADDR="127.0.0.1:$PORT"
BIN="${MCP_BETTER_BIN:-$(pwd)/target/debug/mcp-better}"
BASELINE="${MCP_CONFORMANCE_BASELINE-conformance-baseline.yml}"
OUT="$(mktemp -d)"

port_open() { (exec 3<>"/dev/tcp/127.0.0.1/$PORT") 2>/dev/null; }

# Refuse to test whatever already listens on the port (a leftover server would be tested instead).
if port_open; then
  echo "conformance: port $PORT is already in use; stop that process or set MCP_CONFORMANCE_PORT" >&2
  exit 1
fi

MCP_BETTER_HTTP_ADDR="$ADDR" "$BIN" --http >"$OUT/server.log" 2>&1 &
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true; wait "$SERVER_PID" 2>/dev/null || true; rm -rf "$OUT"' EXIT

for _ in $(seq 1 100); do
  port_open && break
  kill -0 "$SERVER_PID" 2>/dev/null || break
  sleep 0.1
done
if ! kill -0 "$SERVER_PID" 2>/dev/null || ! port_open; then
  echo "conformance: $BIN did not start on $ADDR" >&2
  cat "$OUT/server.log" >&2
  exit 1
fi

ARGS=(server --url "http://$ADDR/mcp" --requirements 2026-07-28 -o "$OUT/results")
[ -n "$BASELINE" ] && ARGS+=(--expected-failures "$BASELINE")
npx --yes "$SUITE" "${ARGS[@]}"
