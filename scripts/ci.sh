#!/usr/bin/env bash
# mcp-better local ship bar — the gates of .github/workflows/ci.yml's `check` job.
# The `msrv` job (cargo check on Rust 1.88, the declared rust-version) runs in CI only.
# Usage: bash scripts/ci.sh
set -euo pipefail

export PATH="$HOME/.cargo/bin:${PATH:-}"
cd "$(dirname "$0")/.."

echo "==> PATH cargo: $(command -v cargo)"
echo "==> rustc: $(rustc --version)"

echo "==> three-file lockstep (doc-gate)"
bash scripts/doc-gate.sh

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

echo "==> MRTR matching client (confirm_echo R1→R2)"
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example mrtr-client

echo "==> Order restart smoke (two processes)"
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example order-restart-smoke

echo "==> Contrast smoke (better passes · worse fails)"
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" \
  MCP_WORSE_BIN="$(pwd)/target/debug/mcp-worse" \
  cargo run --example contrast-smoke

echo "==> Streamable HTTP smoke"
# http-smoke spawns the bin — must exist; example alone does not build [[bin]]
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example http-smoke

echo "==> Official MCP conformance suite: textbook binary (baseline: conformance-baseline.yml)"
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" bash scripts/conformance.sh

echo "==> Official MCP conformance suite: conformance build, no baseline (must be 100%)"
(cd conformance && cargo fmt -- --check && cargo clippy --locked --all-targets -- -D warnings \
  && cargo build --locked && cargo test --locked)
MCP_BETTER_BIN="$(pwd)/conformance/target/debug/mcp-better-conformance" \
  MCP_CONFORMANCE_BASELINE='' bash scripts/conformance.sh

echo "==> Registry listing (rust-mcp-preflight server.json)"
bash scripts/preflight.sh

echo "✅ scripts/ci.sh green"
