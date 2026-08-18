# 09 — Lab: run the textbook

**Status:** SOLID  
**Read time:** ~15 minutes hands-on  
**Depends on:** [03](./03-interop.md)–[08](./08-claim-equals-wire.md) recommended  
**Pins:** published crates **mcp-better 0.4.3** · this tree **0.5.0** (matching client) · protocol **`2026-07-28`**  
**Repo:** https://github.com/Wolfe-Jam/mcp-better

---

## Goal

In ≤10 minutes on a machine with Rust, prove:

| Claim | Evidence you will see |
|-------|------------------------|
| Built for 7/28 | Discover path negotiates `2026-07-28` |
| Stamped list | Positive `ttlMs`, `cacheScope=public` |
| Stable order | `health` → `echo` → `confirm_echo` |
| Dual transport road | stdio default; HTTP optional local demo |
| Matching client | `mrtr-client` completes `confirm_echo` (v0.5) |

Copy the **operational contract**, not a megaserver.

---

## Prerequisites

- Rust stable ≥ 1.85 (`rustup`)  
- Network for first crate download  
- Optional: git for clone path  

---

## Path A — Install from crates.io

```bash
cargo install mcp-better --version 0.4.3
mcp-better --help
# this tree is 0.5.0 (matching client). Registries stay 0.4.3 until tagged.
```

First install compiles dependencies once. That wait is normal.

### Run stdio

```bash
mcp-better
```

This waits on stdio for a client. For automated proof, use Path B’s `stdio-client` against the installed binary:

```bash
git clone https://github.com/Wolfe-Jam/mcp-better.git
cd mcp-better
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client
```

Expect the success line (exact wording from the example binary):

```text
stdio-client: OK (Discover + stamped list ×6 + health + echo; catalog includes confirm_echo)
```

### Louder proof (v0.3)

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

`mcp-worse` is a **lying companion** (teaching only): same tool names, unstamped list, reversed order. Do not point hosts at it.

### Path C — matching client (v0.5)

`stdio-client` lists `confirm_echo` and does **not** invoke it. The matching client finishes the mid-call:

```bash
cargo build --bins
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example mrtr-client
```

Expect (exact wording from the example binary):

```text
mrtr-client: OK (Discover + confirm_echo R1 input_required → R2 complete · reject wrong confirm · reject tampered state)
```

What this smoke **proves:** Discover + stamped catalog, then `confirm_echo` round 1 `input_required` + sealed `requestState`, round 2 `CONFIRM` + echoed state → complete. Wrong confirm text and tampered state fail. It uses `call_tool_once` so the retry is visible (not hidden behind auto-MRTR).

---

## Path B — Clone and smoke from source

```bash
git clone https://github.com/Wolfe-Jam/mcp-better.git
cd mcp-better
cargo build --bins
cargo test
cargo run --example stdio-client
```

### Optional HTTP road (same era)

`http-smoke` **spawns** the binary itself on an ephemeral loopback port (does not require a long-lived server on 8787):

```bash
cargo build --bins
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example http-smoke
```

Expect:

```text
http-smoke: OK (Streamable HTTP · tools/list stamped · health · echo · confirm_echo · Mcp-Method/Mcp-Name)
```

What this smoke **proves:** stamped `tools/list` order `health` → `echo` → `confirm_echo`, plus `health` + `echo` calls over Streamable HTTP, with routing headers present on the requests it sends. It asserts `confirm_echo` in the catalog; it does not run the MRTR retry.

What it does **not** prove (yet): Discover lifecycle on HTTP (that is the stdio example’s job), or failure when headers are **missing** (classroom / 0.3 work).

Manual long-lived demo (separate from the smoke):

```bash
mcp-better --http
# http://127.0.0.1:8787/mcp
```

Local demo only — no auth/TLS. See repo `SECURITY.md`.

---

## Host attach (manual)

Install or build a binary; use the **absolute path** in host config.

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

### Host checklist

1. Host starts the process (stdio; logs on **stderr**).  
2. Tools list: **`health`**, then **`echo`**, then **`confirm_echo`**.  
3. Call **`health`** → tool result text is a JSON object string including `"protocol":"2026-07-28"` and `"tier":"BETTER"` (plus `status`, `server`, `version`).  
4. Call **`echo`** with `{"message":"hello"}` → the message echoed back (`hello`).  
5. Optional: **`confirm_echo`** is the MRTR textbook — not required for the Discover smoke.

Automated Discover smoke remains the source of truth if the host UI is unclear about lifecycle.

---

## What you just proved

| Layer | Proof |
|-------|--------|
| Lifecycle | Discover client path |
| List cache | Stamps present |
| Catalog | `health` → `echo` → `confirm_echo`, fixed order |
| Transports | stdio and/or HTTP as exercised |
| Honesty | Smokes exit non-zero when broken |

That is the textbook point.

---

## Registry identity

| Field | Value |
|-------|--------|
| Package name (Registry) | `io.github.Wolfe-Jam/mcp-better` |
| crates.io | `mcp-better` |
| Language | Rust · `rmcp` 3 |

---

## Troubleshooting

| Symptom | Check |
|---------|--------|
| Client uses initialize only | Use Discover example / modern client mode |
| No stamps in UI | Capture wire or run `stdio-client`; UI may hide fields |
| HTTP connection refused | Server running? loopback addr? firewall? |
| HTTP works without headers in a hand client | Your hand client is not the contract — use `http-smoke` |
| `0.0.0.0` bind | Read SECURITY.md; you accepted an open unauthenticated endpoint |

---

## After the lab

1. Re-read the InterOp card ([03](./03-interop.md)) with fresh evidence.  
2. Apply the same card to one other server you depend on.  
3. Read [10 — What we resist](./10-what-we-resist.md) before asking for more tools.

---

## Next

→ [10 — What we resist](./10-what-we-resist.md)
