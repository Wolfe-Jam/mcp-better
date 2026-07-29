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

## Wire as an MCP server (host config)

Example (Claude Desktop / generic MCP host — shape varies by host):

```json
{
  "mcpServers": {
    "mcp-better": {
      "command": "/absolute/path/to/mcp-better",
      "args": []
    }
  }
}
```

Logs go to **stderr**; protocol is **stdio** JSON-RPC on stdin/stdout.

## Cold clone target

From empty machine with Rust installed: clone → `cargo test` → `cargo run --example stdio-client` in under ~10 minutes on a normal network (first `rmcp` download dominates).
