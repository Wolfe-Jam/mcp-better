#!/usr/bin/env bash
# mcp-better local ship bar — same gates as .github/workflows/ci.yml
# Usage: bash scripts/ci.sh
set -euo pipefail

export PATH="$HOME/.cargo/bin:${PATH:-}"
cd "$(dirname "$0")/.."

echo "==> PATH cargo: $(command -v cargo)"
echo "==> rustc: $(rustc --version)"

echo "==> fmt"
cargo fmt --all -- --check

echo "==> clippy"
cargo clippy --all-targets -- -D warnings

echo "==> test"
cargo test --all-targets

echo "==> release build"
cargo build --release

echo "==> BETTER purity"
test ! -f project.faf
test ! -f .faf

echo "==> Discover smoke (stdio)"
cargo build --bins
cargo run --example stdio-client

echo "==> Streamable HTTP smoke"
# http-smoke spawns the bin — must exist; example alone does not build [[bin]]
cargo build --bins
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example http-smoke

echo "✅ scripts/ci.sh green"
