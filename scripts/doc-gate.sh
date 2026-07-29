#!/usr/bin/env bash
# Vendor entrypoint — delegates to skill script when present, else inline.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SKILL="$HOME/.claude/skills/pubbetter/scripts/doc-gate.sh"
if [[ -x "$SKILL" || -f "$SKILL" ]]; then
  exec bash "$SKILL" "$ROOT"
fi
echo "❌ pubbetter doc-gate skill missing at $SKILL"
exit 1
