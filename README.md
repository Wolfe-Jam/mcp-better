# mcp-better — built for 7/28

[![Business card · Get yours →](https://wolfe-jam.github.io/agent-business-card/badge.svg)](https://wolfe-jam.github.io/agent-business-card/?repo=Wolfe-Jam/mcp-better)

**NONE | GOOD | [BETTER] | BEST**

Textbook for **AGENTS.md** on that scale — honest modern MCP at the **BETTER** step.  
*(protocol **2026-07-28** — the modern MCP release)*

```text
the book is the app is the book
```

Modern MCP textbook **that runs**, reviewed and approved as an AAIF Ambassador contribution. Rust · `rmcp` **3.10** · Discover · stamped cache hints · passes the official MCP conformance suite, 100% of the 2026-07-28 requirements, as the [conformance build](#conformance).  
**Book:** [`textbook/`](./textbook/) — what · why · how · [doctrine](./textbook/DOCTRINE-book-is-app.md).  
**App:** this binary + smokes. Lesson after lesson, version after version — knowledge compounds.

> **BEST** (BETTER **plus** `project.faf` that is resident **and used**: persistent project context that authors the agent files) lives at **[faf.one/agents](https://faf.one/agents)**, one hop up from this textbook.

## Dual-package (optional)

**We are Cargo.** Install is `cargo install`. `npx` is a try path. We do **not** offer `npm install` as an option.

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

1. **Built for 7/28** — not bolted onto a legacy server (official `rmcp` 3.10).
2. **Honest surface** — transport and capabilities match docs and CI.
3. **Roadmap expands the era** — versions add road; they do not “become” 7/28 later.

| Version | Lesson (the version *is* the lesson) |
|---------|--------------------------------------|
| **v0.1** | **7/28 over stdio** — Discover, stamped `ttlMs` / `cacheScope`, stable order, `health` + `echo` |
| **v0.2** | Same 7/28 era + **Streamable HTTP** road + routing headers (`Mcp-Method` / `Mcp-Name`) |
| **v0.3** | Same era + **deeper correctness** — multi-list + restart-order smokes · `mcp-worse` contrast |
| **v0.4** | Same era + **dual package** — cargo + npm shim · `npx mcp-better` with **no Rust toolchain** |
| **v0.4.3** | Same era + **`confirm_echo` MRTR** (SEP-2322) + **Agent Skills** (`mcp-better-lab`) — see [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md) · [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md) |
| **v0.4.4** | Same era + **book matches 0.4.3 wire** — no new tool · catalogs named `health` → `echo` → `confirm_echo` |
| **v0.5** | Same era + **matching client completes MRTR** — `mrtr-client` finishes `confirm_echo` |

## What BETTER means

1. **Protocol honesty** — claim 7/28 / `2026-07-28` only for surfaces you implement and test.
2. **Discover-compatible** — clients should use `ClientLifecycleMode::Discover` (or Auto → 7/28), not only legacy initialize.
3. **Cache hints by choice** — `tools/list`, `resources/list`, `resources/templates/list`, `resources/read`, `prompts/list`, `skills/list` and `skills/get` return a positive `ttlMs` and `cacheScope` (static catalog → `public`). The SDK default is `ttlMs: 0` + `private`: valid, but it tells every client not to cache.
4. **Stable tool order** — same process, same order across N list calls.
5. **Transports** — **stdio** (default) and **Streamable HTTP** (`--http`) in the **same 7/28 era**.

## Quickstart (≤10 min)

| Path | What it is |
|------|------------|
| **`cargo install mcp-better`** | **The install** — we are Cargo. crates.io, native binary on PATH. |
| **`npx mcp-better`** | **Suggested try** — no Rust, no compile. Not the install. |
| **npm** | A published **shim** so `npx` / some hosts can start the same binary. We do **not** offer `npm install` / `npm i -g`. |

### Try (suggested) — `npx`

```bash
# No Rust toolchain. Downloads the native binary from GitHub Releases.
npx mcp-better --help
npx mcp-better
```

### From source / crates.io (Rust 1.88+)

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
# 0.5 matching client (completes confirm_echo)
MCP_BETTER_BIN="$(pwd)/target/debug/mcp-better" cargo run --example mrtr-client
```

**Install** (crates.io) — stdio default (Cursor / Claude Desktop):

**First `cargo install` compiles Rust deps once** (often 100+ units — `rmcp` / `tokio` / …, not 100 tools of ours). One-time; then the binary is instant. To skip that compile, **try** with `npx mcp-better` (above) — that is not the install.

```bash
cargo run --release
# install from crates.io — first hit compiles deps once (see above):
cargo install mcp-better --version 0.5.0 --locked
mcp-better --help
# optional lying companion (teaching only — not for hosts):
# cargo install mcp-better --version 0.5.0 --locked --bin mcp-worse
```

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

## Protocol claims (v0.5.0 — same 7/28 era · matching client completes MRTR)

| Surface | Status |
|---------|--------|
| Era / protocol | **7/28** · negotiated **`2026-07-28`** (Discover preferred) |
| Transport | **stdio** (default) · **Streamable HTTP** (`--http`) |
| HTTP mode | Stateless for 7/28 · `json_response` · local **Host** guards |
| Routing headers | Streamable HTTP POSTs use **`Mcp-Method`** and **`Mcp-Name`** when naming a tool (SEP-2243); `http-smoke` asserts this happy path |
| Capabilities | **tools** · **resources** (skill docs) · **skills** extension (SEP-2640, rmcp native) · **prompts** (declared, none defined: empty stamped list, `prompts/get` → unknown prompt) · completions not declared (`completion/complete` → method not found) |
| Cache hints | tools, resources and skills: **`ttlMs=60000`**, **`cacheScope=public`** · tool order **`health`→`echo`→`confirm_echo`** (restart-stable) |
| MRTR (optional) | **`confirm_echo`** — mid-call confirm · sealed `requestState` · matching client **`mrtr-client`** · [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md)
| Skills (optional) | **`mcp-better-lab`** · `skills/list` · digests · [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md) |
| Lying companion | **`mcp-worse`** — reversed tool order (the lie) + no cache hints of its own (the wire shows the SDK default `ttlMs: 0` + `private`, valid but uninformative) · contrast-smoke only |
| OAuth / tasks | out of hero |

## Conformance

CI runs the official MCP conformance suite ([`@modelcontextprotocol/conformance`](https://github.com/modelcontextprotocol/conformance), pinned, `--requirements 2026-07-28`) twice:

| Run | Binary | Result |
|-----|--------|--------|
| Conformance build | [`conformance/`](./conformance/): `mcp-better-conformance`, mcp-better's own server plus the suite's test fixtures | **100%**, no baseline (63 scenarios, 196 checks) |
| Textbook | `mcp-better`, what you install | every scenario that doesn't need a fixture; [`conformance-baseline.yml`](./conformance-baseline.yml) lists the rest, each labelled |

Many scenarios call tools, prompts and resources that only exist in the suite's reference server (`test_*`, `test://…`), and several need the Tasks extension. The official SDKs pass those the same way: a separate server that carries the fixtures. In `mcp-better-conformance`, mcp-better's tools, skill resources, Skills and cache hints are served by `BetterServer` itself; the fixtures and Tasks are adapted from the Rust MCP SDK's conformance server (Apache-2.0, [`conformance/NOTICE`](./conformance/NOTICE)). The textbook binary stays clean.

The suite's MRTR scenarios use its own fixture tools, so `confirm_echo` is checked by the matching client (`examples/mrtr-client`) in CI instead.

```bash
bash scripts/conformance.sh                                             # textbook + baseline
(cd conformance && cargo build) && \
  MCP_BETTER_BIN="$(pwd)/conformance/target/debug/mcp-better-conformance" \
  MCP_CONFORMANCE_BASELINE='' bash scripts/conformance.sh                # conformance build, 100%
```

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

The ship bar needs Node 18+ and network: it fetches the official MCP conformance suite with `npx` (pinned version) and, if installed, runs [`rust-mcp-preflight`](https://github.com/Wolfe-Jam/rust-mcp-preflight) on `server.json` (it reads crates.io and the MCP Registry).

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
