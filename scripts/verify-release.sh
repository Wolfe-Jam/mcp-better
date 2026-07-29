#!/usr/bin/env bash
# Post-ship / pre-matrix verification for mcp-better
# Usage: bash scripts/verify-release.sh [version]
set -euo pipefail

export PATH="$HOME/.cargo/bin:${PATH:-}"
cd "$(dirname "$0")/.."
VER="${1:-$(grep -E '^version\s*=' Cargo.toml | head -1 | sed -E 's/.*"([^"]+)".*/\1/')}"
REPO=Wolfe-Jam/mcp-better
fail=0

pass() { echo "  ✅ $1"; }
warn() { echo "  ⚠️  $1"; }
die()  { echo "  ❌ $1"; fail=$((fail+1)); }

echo "═══ mcp-better release verify v$VER ═══"
echo ""

echo "— Docs / purity"
bash scripts/doc-gate.sh >/dev/null && pass "doc-gate" || die "doc-gate"
test ! -f project.faf && test ! -f .faf && pass "purity" || die "purity"

echo "— Local CI"
if bash scripts/ci.sh >/tmp/mcp-better-verify-ci.log 2>&1; then
  pass "scripts/ci.sh"
else
  die "scripts/ci.sh (see /tmp/mcp-better-verify-ci.log)"
fi

echo "— GitHub"
if command -v gh >/dev/null; then
  CONC=$(gh run list --repo "$REPO" --branch main --limit 1 --json conclusion -q '.[0].conclusion' 2>/dev/null || echo "")
  [ "$CONC" = "success" ] && pass "GH CI latest main=$CONC" || die "GH CI latest=$CONC"
  if gh release view "v$VER" --repo "$REPO" >/dev/null 2>&1; then
    pass "GH release v$VER"
  else
    die "GH release v$VER missing"
  fi
else
  warn "gh not installed — skip GH checks"
fi

echo "— crates.io"
CRATES=$(curl -sL "https://crates.io/api/v1/crates/mcp-better" -H "User-Agent: pubbetter-verify" \
  | python3 -c "import json,sys
try: print(json.load(sys.stdin)['crate']['max_version'])
except Exception: print('')" 2>/dev/null || echo "")
[ "$CRATES" = "$VER" ] && pass "crates.io max_version=$CRATES" || die "crates.io got=$CRATES want=$VER"

echo "— MCP Registry"
ENC=$(python3 -c "import urllib.parse; print(urllib.parse.quote('io.github.Wolfe-Jam/mcp-better', safe=''))")
HTTP=$(curl -sL -o /tmp/mcp-reg.json -w "%{http_code}" \
  "https://registry.modelcontextprotocol.io/v0.1/servers/${ENC}/versions/${VER}" || echo "000")
if [ "$HTTP" = "200" ]; then
  pass "MCP Registry HTTP 200"
else
  warn "MCP Registry HTTP $HTTP (publish pending?)"
fi

echo "— Truth-table"
if bash "$HOME/.claude/skills/pubbetter/scripts/release-verify.sh" . >/tmp/mcp-better-truth.log 2>&1; then
  pass "truth-table"
else
  die "truth-table"
fi

echo ""
if [ "$fail" -eq 0 ]; then
  echo "✪ MATRIX READY — v$VER gates clean (check ⚠️ for soft opens)"
  exit 0
else
  echo "🚫 $fail hard fail(s) — see above"
  exit 1
fi
