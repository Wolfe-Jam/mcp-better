# mcp-better

**NONE | GOOD | [BETTER] | BEST**

AAIF-verified modern MCP setup optimised to the **2026-07-28** model.  
Rust · `rmcp` 3.0 · stdio · tools · stamped list cache.

> **BEST** (persistent AI project context) lives at **[faf.one/agents](https://faf.one/agents)** — one hop up from this textbook.

## What BETTER means

1. **Protocol honesty** — claim `2026-07-28` only for surfaces you implement and test.
2. **Discover-compatible** — clients should use `ClientLifecycleMode::Discover` (or Auto → 7/28), not only legacy initialize.
3. **List cache stamps** — `tools/list` returns positive `ttlMs` and `cacheScope` (static catalog → `public`). SDK defaults are unstamped.
4. **Stable tool order** — same process, same order across N list calls.
5. **Small honest surface** — v0.1 is **stdio + tools** only. No “full 7/28” claim for HTTP/headers/resources.

## Quickstart (≤10 min)

```bash
# Requires Rust stable (1.85+)
git clone https://github.com/Wolfe-Jam/mcp-better.git
cd mcp-better
cargo build
cargo test
cargo run --example stdio-client
```

Run the server alone (stdio JSON-RPC on stdin/stdout; logs on stderr):

```bash
cargo run --release
```

## Tools

| Tool | Purpose |
|------|---------|
| `health` | Liveness — status, version, protocol. No side effects. Not a k8s probe contract. |
| `echo` | Pure demo — returns `message` unchanged. |

## Protocol claims (v0.1)

| Surface | Status |
|---------|--------|
| Transport | **stdio only** |
| Protocol | **2026-07-28** (negotiated; Discover preferred) |
| Capabilities | **tools** only |
| List cache | **`ttlMs=60000`**, **`cacheScope=public`** |
| Streamable HTTP / Mcp-Method headers | **v0.2** |
| Resources / prompts / OAuth / tasks | **out of v0.1** |

## Non-goals (GOOD-era habits we refuse)

- Shipping unstamped list results while claiming 7/28 modernity  
- Requiring `project.faf` or any BEST tooling on this repo’s main branch  
- Pretending stdio samples equal full multi-transport conformance  
- FAF install tax in the AAIF lede — this repo is protocol textbook, not a FAF product

## Registry identity

- MCP Registry name: `mcp-name: io.github.Wolfe-Jam/mcp-better`
- `registryType`: cargo · `identifier`: `mcp-better`

See [`server.json`](./server.json). **Not** `one.faf/*`.

## Publish

Ship process: **`/pubbetter`** (skill) · short form [`docs/PUBBETTER.md`](./docs/PUBBETTER.md) · local ship bar:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/ci.sh
```

## BEST

For persistent, versionable AI project context beyond a protocol textbook:

**https://faf.one/agents**

## Docs

- [BETTER.md](./BETTER.md) — ladder + claim surface  
- [docs/BETTER-BEST.md](./docs/BETTER-BEST.md) — BETTER vs BEST  
- [GETTING-STARTED.md](./GETTING-STARTED.md)  
- [docs/SDK-NOTES.md](./docs/SDK-NOTES.md) — `serve` vs Discover honesty  
- [SECURITY.md](./SECURITY.md) · [CONTRIBUTING.md](./CONTRIBUTING.md)

## License

MIT — see [LICENSE](./LICENSE).
