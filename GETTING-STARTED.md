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

Expected last line:

```text
stdio-client: OK (Discover + list tools + health + echo)
```

Optional: `MCP_BETTER_BIN=/path/to/mcp-better cargo run --example stdio-client`

## Wire as an MCP server (host smoke)

Install from crates (or use a local `cargo build --release` binary):

```bash
cargo install mcp-better
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
2. Tools list shows **`health`** then **`echo`** (stable order).
3. Call **`health`** → JSON with `"protocol":"2026-07-28"`, `"tier":"BETTER"`.
4. Call **`echo`** with `{"message":"hello"}` → `hello`.

Automated Discover smoke (no host UI):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client
```

## Cold clone target

From empty machine with Rust installed: clone → `cargo test` → `cargo run --example stdio-client` in under ~10 minutes on a normal network (first `rmcp` download dominates).
