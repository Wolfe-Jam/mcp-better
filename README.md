# mcp-better — built for 7/28

**NONE | GOOD | [BETTER] | BEST**

Textbook for **AGENTS.md** on that scale — honest modern MCP at the **BETTER** step.  
*(protocol **2026-07-28** — the modern MCP release)*

```text
the book is the app is the book
```

AAIF-verified modern MCP textbook **that runs**. Rust · `rmcp` **3.0.x** (lock **3.0.1**, Tier 1 assessed) · Discover · stamped list cache.  
**Book:** [`textbook/`](./textbook/) — what · why · how · [doctrine](./textbook/DOCTRINE-book-is-app.md).  
**App:** this binary + smokes. Lesson after lesson, version after version — knowledge compounds.

> **BEST** (persistent project DNA for agents — **AGENTS.md** / FAF at scale) lives at **[faf.one/agents](https://faf.one/agents)** — one hop up from this textbook.

## Dual-package (optional)

**Cargo stays first.** Optional npm is only an on-ramp to the same native binary for npx-style hosts.

→ [Why dual-package](./docs/DUAL-PACKAGE-FOR-RUST-MCP.md) — positioning + FAQ  
→ [Full guide (how)](./docs/DUAL-PACKAGE-RUST-MCP.md) — lockstep, publish order, OIDC, score + wire

## Skills over MCP (optional · textbook)

One Agent Skill (`mcp-better-lab`) on the same process as tools:

→ [docs/SKILLS-OVER-MCP.md](./docs/SKILLS-OVER-MCP.md) — extension · `skills/list` · `skills/get` · digests

## What is 7/28?

| Name | What it is |
|------|------------|
| **7/28** | The **era name** — speakable, brandable. “Built for 7/28.” |
| **2026-07-28** | The **protocol version** — the date string on the wire / in SDKs. |

**7/28 is a great name. 2026-07-28 is a date.**  
Humans say **7/28**. Machines negotiate **`2026-07-28`**.

## What to expect

1. **Built for 7/28** — not bolted onto a legacy server (official `rmcp` 3.0.x / Tier-1 assessed cut).
2. **Honest surface** — transport and capabilities match docs and CI.
3. **Roadmap expands the era** — versions add road; they do not “become” 7/28 later.

| Version | Lesson (the version *is* the lesson) |
|---------|--------------------------------------|
| **v0.1** | **7/28 over stdio** — Discover, stamped `ttlMs` / `cacheScope`, stable order, `health` + `echo` |
| **v0.2** | Same 7/28 era + **Streamable HTTP** road + routing headers (`Mcp-Method` / `Mcp-Name`) |
| **v0.3** | Same era + **deeper correctness** — multi-list + restart-order smokes · `mcp-worse` contrast |
| **v0.4** | Same era + **dual package** — cargo + npm shim · `npx mcp-better` with **no Rust toolchain** |
| **v0.4.3** | Same era + **`confirm_echo` MRTR** (SEP-2322) + **Agent Skills** (`mcp-better-lab`) — see [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md) · [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md) |

## What BETTER means

1. **Protocol honesty** — claim 7/28 / `2026-07-28` only for surfaces you implement and test.
2. **Discover-compatible** — clients should use `ClientLifecycleMode::Discover` (or Auto → 7/28), not only legacy initialize.
3. **List cache stamps** — `tools/list` returns positive `ttlMs` and `cacheScope` (static catalog → `public`). SDK defaults are unstamped.
4. **Stable tool order** — same process, same order across N list calls.
5. **Transports** — **stdio** (default) and **Streamable HTTP** (`--http`) in the **same 7/28 era**.

## Quickstart (≤10 min)

### Zero Rust — npm / npx (v0.4+)

```bash
# No Rust toolchain required. Downloads the native binary from GitHub Releases.
npx mcp-better --help
npx mcp-better
# hosts: point stdio command at `npx` / `mcp-better` from the npm package
```

### From source / crates.io (Rust 1.85+)

```bash
git clone https://github.com/Wolfe-Jam/mcp-better.git
cd mcp-better
cargo build --bins
cargo test
cargo run --example stdio-client
# louder 0.3 smokes (build --bins first; or: bash scripts/ci.sh)
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example order-restart-smoke
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" \
  MCP_WORSE_BIN="$(pwd)/target/debug/mcp-worse" \
  cargo run --example contrast-smoke
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example http-smoke
```

**stdio** (default — Cursor / Claude Desktop):

```bash
cargo run --release
# or from crates.io:
cargo install mcp-better --version 0.4.3
mcp-better --help
# optional lying companion (teaching only — not for hosts):
# cargo install mcp-better --version 0.4.3 --bin mcp-worse
```

> First cargo install compiles the ecosystem once (not 100+ of our tools — just Rust deps). One-time wait; then you’re done.  
> Prefer **no compile**? Use `npx mcp-better` (npm shim).

**Streamable HTTP** (local demo only — see [SECURITY.md](./SECURITY.md)):

```bash
cargo run --release -- --http
# http://127.0.0.1:8787/mcp
# MCP_BETTER_HTTP_ADDR=127.0.0.1:9000 mcp-better --http
```

**Transport selection** (CLI wins over env):

| How | Value |
|-----|--------|
| CLI | `mcp-better` (stdio) · `mcp-better --http` · `mcp-better --stdio` |
| Bare args | `http` / `stdio` (same meaning as flags) |
| Env | `MCP_TRANSPORT` or `MCP_BETTER_TRANSPORT` → `stdio` \| `http` (**`MCP_TRANSPORT` first** if both set) |
| HTTP bind | `MCP_BETTER_HTTP_ADDR` — default **`127.0.0.1:8787`**. Do **not** use `0.0.0.0` unless you accept an unauthenticated open endpoint. |

## Tools

| Tool | Purpose |
|------|---------|
| `health` | Liveness — status, version, protocol. No side effects. Not a k8s probe contract. |
| `echo` | Pure demo — returns `message` unchanged. |
| `confirm_echo` | Textbook **MRTR** (SEP-2322) — echo after mid-call confirm · sealed `requestState`. |

## Protocol claims (v0.4.3 — same 7/28 era, dual package)

| Surface | Status |
|---------|--------|
| Era / protocol | **7/28** · negotiated **`2026-07-28`** (Discover preferred) |
| Transport | **stdio** (default) · **Streamable HTTP** (`--http`) |
| HTTP mode | Stateless for 7/28 · `json_response` · local **Host** guards |
| Routing headers | Streamable HTTP POSTs use **`Mcp-Method`** and **`Mcp-Name`** when naming a tool (SEP-2243); `http-smoke` asserts this happy path |
| Capabilities | **tools** · **resources** (skill docs) · **experimental** skills extension |
| List cache | **`ttlMs=60000`**, **`cacheScope=public`**, order **`health`→`echo`→`confirm_echo`** (restart-stable) |
| MRTR (optional) | **`confirm_echo`** — mid-call confirm · sealed `requestState` · [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md) |
| Skills (optional) | **`mcp-better-lab`** · `skills/list` · digests · [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md) |
| Lying companion | **`mcp-worse`** — unstamped + reversed order (contrast-smoke only) |
| OAuth / tasks | out of hero |

## Textbook

The book is the app is the book — [`textbook/`](./textbook/) (Season 1 · [doctrine](./textbook/DOCTRINE-book-is-app.md)).

Start: [textbook/README.md](./textbook/README.md) → lab [Ch 09](./textbook/09-run-the-textbook.md).

## Non-goals (GOOD-era habits we refuse)

- Shipping unstamped list results while claiming 7/28 modernity  
- Requiring `project.faf` or any BEST tooling on this repo’s main branch  
- Treating stdio as “not real 7/28” — **stdio is a first-class 7/28 transport**  
- FAF install tax in the AAIF lede — this repo is protocol textbook, not a FAF product

## Registry identity

- MCP Registry name: `mcp-name: io.github.Wolfe-Jam/mcp-better`
- **Dual packages** (same version, both **stdio**):
  - `registryType`: **cargo** · `identifier`: `mcp-better` · crates.io
  - `registryType`: **npm** · `identifier`: `mcp-better` · registry.npmjs.org  
    (Node shim downloads the native binary from GitHub Releases — no Rust on the host)
- **Package transport in `server.json` is stdio only — by design.**  
  Hosts spawn via `cargo install` / `npx mcp-better` on **stdio**.  
  Streamable HTTP (`--http`) is an **opt-in local demo** in the same binary and the same 7/28 era; it is **not** a Registry remote package. Discover it in this README and `--help`.

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
- [docs/DUAL-PACKAGE-FOR-RUST-MCP.md](./docs/DUAL-PACKAGE-FOR-RUST-MCP.md) — why dual-package (cargo first · FAQ)  
- [docs/DUAL-PACKAGE-RUST-MCP.md](./docs/DUAL-PACKAGE-RUST-MCP.md) — how: full dual cargo+npm guide  
- [docs/MCP-DIST-POST.md](./docs/MCP-DIST-POST.md) — lockstep post-step (does not publish)  
- [SECURITY.md](./SECURITY.md) · [CONTRIBUTING.md](./CONTRIBUTING.md)

## License

MIT — see [LICENSE](./LICENSE).
