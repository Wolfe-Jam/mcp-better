# Dual-package for Rust MCP servers

**Cargo stays first.** An optional npm package is only an on-ramp so more hosts can start the **same native binary**.

This page is the **why** (positioning + FAQ). Step-by-step publish: [Full guide — DUAL-PACKAGE-RUST-MCP.md](./DUAL-PACKAGE-RUST-MCP.md).

---

## Problem

Rust MCP servers on **crates.io** are real binaries: fast, native, first-class.

Many MCP clients and docs still start servers with **npx**. Developers without a Rust toolchain—or who never use crates.io—never reach that binary. That is a **launch** gap, not a failure of cargo or of the server.

## What dual-package is

| Piece | Role |
|-------|------|
| **crates.io** | Source of truth — the Rust server |
| **npm (optional)** | Thin package that **downloads and runs** that binary |
| **Registry `server.json`** | May list **both** at the **same version** |

The process that runs is still **Rust**. npm is not a second implementation.

## Why it exists

So Rust MCP servers are **easier to try** on paths hosts already document, without replacing cargo.

| Who | What they get |
|-----|----------------|
| Cargo users | Unchanged — `cargo install`, crates.io, native binary |
| npx-oriented users / hosts | Same binary, familiar start command |
| Authors | Optional publish path — not required |

Live examples (same pattern, different identity):

- **mcp-better** — textbook · `io.github.Wolfe-Jam/mcp-better`
- **rust-faf-mcp** — product · `one.faf/rust-faf-mcp` (DNS auth)

Both ship cargo + npm + registry dual at one version.

## How to do it

Follow the guide: [Dual-package Rust MCP (full how-to)](./DUAL-PACKAGE-RUST-MCP.md)

- Lockstep version on crate, npm, and `server.json`
- `mcpName` on the npm package
- npm = downloader only; binary from GitHub Release (or equivalent)
- Registry dual entry optional but recommended if you dual-publish

## FAQ

### Do I need both?

**No.** Cargo-only is valid. Dual-package is for authors who want npx-style launch as well.

### Why add npm?

So clients and docs that only show `npx -y …` can start **your Rust binary** without a Rust toolchain. Discoverability on a **known path** — not a rewrite in JavaScript.

### Can I publish only npm?

**Not as a Rust MCP server.** If there is no crate binary behind it, you are not shipping a Rust server. npm alone without a native artifact is a different product. This path assumes **cargo is authoritative**.

### Recommended setup

1. **Always:** solid crate on crates.io; stdio NDJSON; binary you can run and score.
2. **If you care about npx hosts:** dual-package at the same version + dual `server.json`.
3. **Publish:** when you automate releases, prefer OIDC / Trusted Publishing (not required for every author).
4. **Quality:** certify the **native** binary (`wjttc` or equivalent), not only the npx wrapper.

Default for Rust-first authors: **cargo required · npm optional · same binary either way.**

## Also

| Topic | Doc |
|-------|-----|
| Full dual-package steps (how) | [DUAL-PACKAGE-RUST-MCP.md](./DUAL-PACKAGE-RUST-MCP.md) |
| Score the native binary | [§11](./DUAL-PACKAGE-RUST-MCP.md#11-score-the-native-binary-optional) |
| Stdio NDJSON / stdout | [§12](./DUAL-PACKAGE-RUST-MCP.md#12-stdio-wire-hygiene) |
| Mid-call input (MRTR) example — optional early-mover | [MRTR-CONFIRM-ECHO.md](./MRTR-CONFIRM-ECHO.md) |

## Not this

- Not “prefer npm over cargo”
- Not a registry rule that cargo servers must ship npm
- Not two different servers under one name

---

Questions and suggestions welcome.
