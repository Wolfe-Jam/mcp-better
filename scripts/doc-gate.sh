#!/usr/bin/env bash
# mcp-better Doc Gate — three-file lockstep + docs identity
# Usage: bash scripts/doc-gate.sh
#
# Prefer skill copy when present (local wolfejam machine); otherwise run
# the inlined gate so GitHub Actions / cold clones still pass.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SKILL="$HOME/.claude/skills/pubbetter/scripts/doc-gate.sh"
if [[ -f "$SKILL" ]]; then
  exec bash "$SKILL" "$ROOT"
fi

# ── Inlined gate (CI / no skill) ─────────────────────────────────
cd "$ROOT"

if [[ ! -f Cargo.toml ]]; then
  echo "❌ No Cargo.toml in $ROOT"
  exit 1
fi

pkg=$(grep -E '^version\s*=' Cargo.toml | head -1 | sed -E 's/.*"([^"]+)".*/\1/')
if [[ -z "$pkg" ]]; then
  echo "❌ Cargo.toml version unreadable"
  exit 1
fi

drift=0

echo "— Three-file lockstep (Cargo.toml · package.json · server.json)"

if [[ ! -f package.json ]]; then
  echo "❌ package.json missing (required for dual-package lockstep)"
  drift=1
else
  npmv=$(python3 -c "import json; print(json.load(open('package.json')).get('version',''))" 2>/dev/null || true)
  if [[ -z "$npmv" ]]; then
    echo "❌ package.json version unreadable"
    drift=1
  elif [[ "$npmv" != "$pkg" ]]; then
    echo "❌ package.json version = $npmv (expected $pkg)"
    drift=1
  else
    echo "  ✅ package.json = $npmv"
  fi
fi

if [[ -f server.json ]]; then
  sj=$(python3 -c "import json; d=json.load(open('server.json')); print(d.get('version',''))" 2>/dev/null || true)
  if [[ -n "$sj" && "$sj" != "$pkg" ]]; then
    echo "❌ server.json version = $sj (expected $pkg)"
    drift=1
  else
    echo "  ✅ server.json = $sj"
  fi
  pkg_drift=$(python3 -c "
import json, sys
d=json.load(open('server.json'))
want=sys.argv[1]
ps=d.get('packages') or []
if not ps:
    print('no-packages')
    sys.exit(0)
bad=[]
for i,p in enumerate(ps):
    v=p.get('version','')
    if v != want:
        bad.append(f\"packages[{i}].version={v}\")
print(','.join(bad) if bad else 'ok')
" "$pkg" 2>/dev/null || echo "parse-error")
  if [[ "$pkg_drift" == "ok" ]]; then
    echo "  ✅ server.json packages[*].version lockstep"
  elif [[ "$pkg_drift" == "no-packages" ]]; then
    echo "❌ server.json has no packages[]"
    drift=1
  else
    echo "❌ server.json package version drift: $pkg_drift (expected $pkg)"
    drift=1
  fi
  types=$(python3 -c "
import json
ps=json.load(open('server.json')).get('packages') or []
print(' '.join(sorted({p.get('registryType','') for p in ps})))
" 2>/dev/null || true)
  echo "  · registryTypes: $types"
  if ! echo " $types " | grep -q ' cargo '; then
    echo "❌ server.json missing cargo package"
    drift=1
  fi
  if ! echo " $types " | grep -q ' npm '; then
    echo "❌ server.json missing npm package"
    drift=1
  fi
  name=$(python3 -c "import json; print(json.load(open('server.json')).get('name',''))" 2>/dev/null || true)
  if [[ "$name" != "io.github.Wolfe-Jam/mcp-better" ]]; then
    echo "❌ server.json name = $name (expected io.github.Wolfe-Jam/mcp-better)"
    drift=1
  fi
else
  echo "❌ server.json missing"
  drift=1
fi

echo "  ✅ Cargo.toml = $pkg"

if [[ -f CHANGELOG.md ]]; then
  top=$(grep -oE '^## \[?[0-9]+\.[0-9]+\.[0-9]+\]?' CHANGELOG.md | head -1 | tr -d '[]# ' || true)
  if [[ -n "$top" && "$top" != "$pkg" ]]; then
    echo "❌ CHANGELOG top entry = $top (expected $pkg)"
    drift=1
  fi
else
  echo "❌ CHANGELOG.md missing"
  drift=1
fi

if [[ -f README.md ]]; then
  readme=$(grep -oE 'mcp-better v[0-9]+\.[0-9]+\.[0-9]+' README.md | head -1 | sed 's/mcp-better v//' || true)
  if [[ -n "$readme" && "$readme" != "$pkg" ]]; then
    echo "❌ README version stamp = $readme (expected $pkg)"
    drift=1
  fi
  if ! grep -q 'mcp-name: io.github.Wolfe-Jam/mcp-better' README.md; then
    echo "❌ README missing visible mcp-name: io.github.Wolfe-Jam/mcp-better"
    drift=1
  fi
fi

if [[ ! -f dist-workspace.toml ]]; then
  echo "❌ dist-workspace.toml missing (cargo-dist durable path required)"
  drift=1
fi

if [[ -f project.faf || -f .faf ]]; then
  echo "❌ project.faf/.faf present — BETTER main must stay pure"
  drift=1
fi

if [[ "$drift" -eq 1 ]]; then
  echo ""
  echo "🚫 pubbetter Doc Gate REFUSED — fix mismatches, re-run."
  exit 1
fi

echo "✅ pubbetter Doc Gate: three-file lockstep + docs agree on v$pkg"
exit 0
