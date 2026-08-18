# Getting started

## Prerequisites

- Rust **stable** ≥ 1.85 (`rustup default stable`)
- Prefer **rustup** toolchain (`~/.cargo/bin` before Homebrew `/usr/local/bin` if both exist)

## Build & test

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Discover client smoke

```bash
cargo build
cargo run --example stdio-client
```

Expected last line (exact wording from the example):

```text
stdio-client: OK (Discover + stamped list ×6 + health + echo; catalog includes confirm_echo)
```

Optional: `MCP_BETTER_BIN=/path/to/mcp-better cargo run --example stdio-client`

### Louder smokes (v0.3)

```bash
cargo build --bins
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example order-restart-smoke
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" \
  MCP_WORSE_BIN="$(pwd)/target/debug/mcp-worse" \
  cargo run --example contrast-smoke
```

```text
order-restart-smoke: OK (two processes · same order · same stamps)
contrast-smoke: OK (mcp-better passes BETTER list contract · mcp-worse fails it)
```

HTTP happy-path (spawns the binary itself — build bins first):

```bash
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example http-smoke
```

```text
http-smoke: OK (Streamable HTTP · tools/list stamped · health · echo · confirm_echo · Mcp-Method/Mcp-Name)
```

### Matching client (v0.5) — complete `confirm_echo`

```bash
cargo build --bins
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example mrtr-client
```

```text
mrtr-client: OK (Discover + confirm_echo R1 input_required → R2 complete · reject wrong confirm · reject tampered state)
```

Uses `call_tool_once` so the mid-call retry stays visible. `stdio-client` still does not invoke MRTR.

## Wire as an MCP server (host smoke)

**Install** is `cargo install` (or a local `cargo build --release` binary).  
**Try** without compiling: `npx mcp-better` (npm package runs the same Release binary). npm is that package — not the install.

**First `cargo install` compiles Rust deps once** (often 100+ units). One-time.

```bash
# first hit: cargo compiles the crate graph once — not 100 of our tools
cargo install mcp-better --version 0.5.0
which mcp-better   # use this absolute path below
```

### Claude Desktop / Cursor-style `mcpServers`

```json
{
  "mcpServers": {
    "mcp-better": {
      "command": "/absolute/path/from/which/mcp-better",
      "args": []
    }
  }
}
```

### Claude Code (example)

```bash
# shape varies by version — binary stdio server:
# point your MCP server entry at: mcp-better  (no args)
```

### Host smoke checklist

1. Host starts the process (stdio JSON-RPC; logs on **stderr**).
2. Tools list shows **`health`** → **`echo`** → **`confirm_echo`** (stable order).
3. Call **`health`** → JSON with `"protocol":"2026-07-28"`, `"tier":"BETTER"`.
4. Call **`echo`** with `{"message":"hello"}` → `hello`.
5. Optional: **`confirm_echo`** is the MRTR textbook — not required for the Discover smoke.

Automated Discover smoke (no host UI):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client
```

## Cold clone target

From empty machine with Rust installed: clone → `cargo test` → `cargo run --example stdio-client` in under ~10 minutes on a normal network (first `rmcp` download dominates).
