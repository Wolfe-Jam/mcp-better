# mcp-better — built for 7/28

**NONE | GOOD | [BETTER] | BEST**

*(protocol **2026-07-28** — the modern MCP release)*

AAIF-verified modern MCP textbook. Rust · `rmcp` 3.0 · Discover · stamped list cache.

> **BEST** (persistent AI project context) lives at **[faf.one/agents](https://faf.one/agents)** — one hop up from this textbook.

## What is 7/28?

| Name | What it is |
|------|------------|
| **7/28** | The **era name** — speakable, brandable. “Built for 7/28.” |
| **2026-07-28** | The **protocol version** — the date string on the wire / in SDKs. |

**7/28 is a great name. 2026-07-28 is a date.**  
Humans say **7/28**. Machines negotiate **`2026-07-28`**.

## What to expect

1. **Built for 7/28** — not bolted onto a legacy server (official `rmcp` 3.0).
2. **Honest surface** — transport and capabilities match docs and CI.
3. **Roadmap expands the era** — versions add road; they do not “become” 7/28 later.

| Version | What you get |
|---------|----------------|
| **v0.1** | **7/28 over stdio** — foundation: tools, Discover, stamped `ttlMs` / `cacheScope` |
| **v0.2** | Same 7/28 era + **Streamable HTTP** + routing headers (`Mcp-Method` / `Mcp-Name`) |

## What BETTER means

1. **Protocol honesty** — claim 7/28 / `2026-07-28` only for surfaces you implement and test.
2. **Discover-compatible** — clients should use `ClientLifecycleMode::Discover` (or Auto → 7/28), not only legacy initialize.
3. **List cache stamps** — `tools/list` returns positive `ttlMs` and `cacheScope` (static catalog → `public`). SDK defaults are unstamped.
4. **Stable tool order** — same process, same order across N list calls.
5. **v0.1 transport** — **stdio** (7/28-coherent). HTTP is **v0.2** road in the **same era**, not “when we become modern.”

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

## Protocol claims (v0.1 — 7/28 over stdio)

| Surface | Status |
|---------|--------|
| Era / protocol | **7/28** · negotiated **`2026-07-28`** (Discover preferred) |
| Transport | **stdio** (HTTP → v0.2, same era) |
| Capabilities | **tools** only |
| List cache | **`ttlMs=60000`**, **`cacheScope=public`** |
| Streamable HTTP / `Mcp-Method` · `Mcp-Name` | **v0.2** |
| Resources / prompts / OAuth / tasks | out of v0.1 |

## Non-goals (GOOD-era habits we refuse)

- Shipping unstamped list results while claiming 7/28 modernity  
- Requiring `project.faf` or any BEST tooling on this repo’s main branch  
- Treating stdio as “not real 7/28” — **stdio is a first-class 7/28 transport**  
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
