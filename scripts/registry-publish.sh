#!/usr/bin/env bash
# Publish mcp-better to the official MCP Registry (cargo package).
# Identity: io.github.Wolfe-Jam/mcp-better  — NOT one.faf/*
#
# YOU must complete the GitHub device login in a browser when prompted.
set -euo pipefail

export PATH="/usr/local/bin:$HOME/.cargo/bin:${PATH:-}"
cd "$(dirname "$0")/.."

echo "==> server.json"
cat server.json
echo ""

# Clear stale JWT if present (expired tokens cause 401)
if [[ -f "$HOME/.mcp_publisher_token" ]]; then
  echo "==> removing stale ~/.mcp_publisher_token (re-login required)"
  rm -f "$HOME/.mcp_publisher_token"
fi
# optional registry token cache
rm -f "$HOME/.mcpregistry_registry_token" 2>/dev/null || true

echo "==> mcp-publisher login github"
echo "    Open https://github.com/login/device and enter the code shown."
mcp-publisher login github

echo "==> mcp-publisher publish"
mcp-publisher publish server.json

echo "==> verify"
ENC=$(python3 -c "import urllib.parse; print(urllib.parse.quote('io.github.Wolfe-Jam/mcp-better', safe=''))")
VER=$(python3 -c "import json; print(json.load(open('server.json'))['version'])")
sleep 2
HTTP=$(curl -sL -o /tmp/mcp-reg-verify.json -w "%{http_code}" \
  "https://registry.modelcontextprotocol.io/v0.1/servers/${ENC}/versions/${VER}")
echo "HTTP $HTTP"
head -c 400 /tmp/mcp-reg-verify.json; echo
[[ "$HTTP" == "200" ]] && echo "✅ Registry live" || echo "⚠️  not 200 yet — retry curl in ~30s"

echo "==> truth-table"
bash "$HOME/.claude/skills/pubbetter/scripts/release-verify.sh" .
