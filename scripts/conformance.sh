#!/usr/bin/env bash
# Official MCP conformance suite against an mcp-better binary over Streamable HTTP.
#
#   bash scripts/conformance.sh                                       # textbook binary + baseline
#   MCP_BETTER_BIN=conformance/target/debug/mcp-better-conformance \
#     MCP_CONFORMANCE_BASELINE='' bash scripts/conformance.sh         # conformance build, strict
#
# Two parts:
#   1. --requirements 2026-07-28 (50 scenarios: 37 scored, 13 run but not scored).
#      With a baseline, the suite's own exit code decides; it covers scored scenarios only.
#      With no baseline (MCP_CONFORMANCE_BASELINE=''), every check of all 50 scenarios must
#      pass: any FAILURE or WARNING fails the run, scored or not.
#   2. The Skills scenarios (SEP-2640, an extension, not in the requirement set):
#      sep-2640-skills-enumeration and sep-2640-skills-manifest, strict for both binaries.
#
# Results are kept in MCP_CONFORMANCE_OUT (default: a temp dir, removed on success).
# Needs a built binary and Node/npx (the suite is fetched at the pinned version, so: network).
set -euo pipefail
cd "$(dirname "$0")/.."

SUITE="@modelcontextprotocol/conformance@0.2.0-alpha.12"
PORT="${MCP_CONFORMANCE_PORT:-8799}"
ADDR="127.0.0.1:$PORT"
BIN="${MCP_BETTER_BIN:-$(pwd)/target/debug/mcp-better}"
BASELINE="${MCP_CONFORMANCE_BASELINE-conformance-baseline.yml}"
OUT="${MCP_CONFORMANCE_OUT:-}"
KEEP_OUT=1
if [ -z "$OUT" ]; then
  OUT="$(mktemp -d)"
  KEEP_OUT=0
fi
mkdir -p "$OUT"

port_open() { (exec 3<>"/dev/tcp/127.0.0.1/$PORT") 2>/dev/null; }

# Refuse to test whatever already listens on the port (a leftover server would be tested instead).
if port_open; then
  echo "conformance: port $PORT is already in use; stop that process or set MCP_CONFORMANCE_PORT" >&2
  exit 1
fi

MCP_BETTER_HTTP_ADDR="$ADDR" "$BIN" --http >"$OUT/server.log" 2>&1 &
SERVER_PID=$!
STATUS=1
cleanup() {
  kill "$SERVER_PID" 2>/dev/null || true
  wait "$SERVER_PID" 2>/dev/null || true
  if [ "$STATUS" -eq 0 ] && [ "$KEEP_OUT" -eq 0 ]; then
    rm -rf "$OUT"
  elif [ "$STATUS" -ne 0 ]; then
    echo "conformance: results kept in $OUT" >&2
  fi
}
trap cleanup EXIT

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

# Every check in every checks.json under a results folder must be SUCCESS, SKIPPED or INFO.
strict() {
  node - "$1" <<'JS'
const fs = require("fs"), path = require("path");
const bad = [];
const walk = (d) => fs.readdirSync(d, { withFileTypes: true }).forEach((e) => {
  const p = path.join(d, e.name);
  if (e.isDirectory()) return walk(p);
  if (e.name !== "checks.json") return;
  const scenario = path.basename(d).replace(/^server-/, "").replace(/-\d{4}-\d{2}-\d{2}T.*$/, "");
  const visit = (v) => {
    if (Array.isArray(v)) return v.forEach(visit);
    if (v && typeof v === "object") {
      if (v.status === "FAILURE" || v.status === "WARNING")
        bad.push(`${v.status} ${scenario}:${v.id} ${String(v.errorMessage || "").slice(0, 120)}`);
      Object.values(v).forEach(visit);
    }
  };
  visit(JSON.parse(fs.readFileSync(p, "utf8")));
});
walk(process.argv[2]);
if (bad.length) { console.error(`strict: ${bad.length} check(s) not passing:\n` + bad.join("\n")); process.exit(1); }
console.log("strict: every check passed (no FAILURE, no WARNING)");
JS
}

echo "== requirements 2026-07-28"
ARGS=(server --url "http://$ADDR/mcp" --requirements 2026-07-28 -o "$OUT/requirements")
if [ -n "$BASELINE" ]; then
  npx --yes "$SUITE" "${ARGS[@]}" --expected-failures "$BASELINE"
else
  npx --yes "$SUITE" "${ARGS[@]}"
  strict "$OUT/requirements"
fi

echo "== Skills (SEP-2640): enumeration and manifest"
for scenario in sep-2640-skills-enumeration sep-2640-skills-manifest; do
  npx --yes "$SUITE" server --url "http://$ADDR/mcp" --scenario "$scenario" -o "$OUT/skills"
done
strict "$OUT/skills"

STATUS=0
