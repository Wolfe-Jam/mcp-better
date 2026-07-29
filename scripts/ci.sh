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

echo "==> Discover smoke"
cargo build
cargo run --example stdio-client

echo "✅ scripts/ci.sh green"
