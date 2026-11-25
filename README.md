# mcp-better — built for 7/28

**NONE | GOOD | [BETTER] | BEST**

Textbook for **AGENTS.md** on that scale — honest modern MCP at the **BETTER** step.  
*(protocol **2026-07-28** — the modern MCP release)*

```text
the book is the app is the book
```

Modern MCP textbook **that runs**, reviewed and approved as an AAIF Ambassador contribution. Rust · `rmcp` **3.10** · Discover · stamped cache hints · the official MCP conformance suite in CI: [100% for the conformance build](#conformance), and for the textbook binary every scored check except those that need a suite fixture or an undeclared method.  
**Book:** [`textbook/`](./textbook/) — what · why · how · [doctrine](./textbook/DOCTRINE-book-is-app.md).  
**App:** this binary + smokes. Lesson after lesson, version after version — knowledge compounds.

> **BEST** (BETTER **plus** `project.faf` that is resident **and used**: persistent project context that authors the agent files) lives at **[faf.one/agents](https://faf.one/agents)**, one hop up from this textbook.

## Dual-package (optional)

**We are Cargo.** Install is `cargo install`. `npx` is a try path. We do **not** offer `npm install` as an option.

→ [Why dual-package](./docs/DUAL-PACKAGE-FOR-RUST-MCP.md) — positioning + FAQ  
→ [Full guide (how)](./docs/DUAL-PACKAGE-RUST-MCP.md) — lockstep, publish order, OIDC, score + wire

## Skills over MCP (optional · textbook)

One Agent Skill (`mcp-better-lab`) on the same process as tools:

→ **Tutorial: [Ship a skill with your MCP server](./docs/SKILLS-OVER-MCP.md)** — each step linked to the code, and the four checks that prove a host loads the skill (verified in Claude Code 2.1.332: `mcp-better:mcp-better-lab` listed and loaded).

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
| **v0.6** | Same era + **the spec's own tests** — the official MCP conformance suite in CI ([Conformance](#conformance)) · cache hints on every list and read · declare what you answer · `rmcp` 3.10 |
| **v0.7** | Same era + **tools say what they do** — every tool carries all four annotation hints (read-only, not destructive, idempotent, closed world) · `confirm_echo` asks only clients that declare elicitation (else `-32021`) · `server/discover` stamped like the lists · a strict command line (`--version`, unknown arguments exit 2) |

## What BETTER means

1. **Protocol honesty** — claim 7/28 / `2026-07-28` only for surfaces you implement and test.
2. **Discover-compatible** — clients should use `ClientLifecycleMode::Discover` (or Auto → 7/28), not only legacy initialize.
3. **Cache hints by choice** — `server/discover`, `tools/list`, `resources/list`, `resources/templates/list`, `resources/read`, `skills/list` and `skills/get` return a positive `ttlMs` and `cacheScope` (static catalog → `public`). The SDK default is `ttlMs: 0` + `private`: valid, but it tells every client not to cache.
4. **Stable tool order** — same process, same order across N list calls.
5. **Annotated tools** — every tool sets `readOnlyHint`, `destructiveHint`, `idempotentHint` and `openWorldHint`. A tool with no hints gets the spec's defaults: it may write, may destroy, and reaches the outside world.
6. **Transports** — **stdio** (default) and **Streamable HTTP** (`--http`) in the **same 7/28 era**.

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
cargo install mcp-better --version 0.7.0 --locked
mcp-better --version   # mcp-better 0.7.0
mcp-better --help
# optional lying companion (teaching only — not for hosts):
# cargo install mcp-better --version 0.7.0 --locked --bin mcp-worse
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
| CLI | `mcp-better` (stdio) · `mcp-better --http` · `mcp-better --stdio` · `--version` / `--help` win wherever they appear |
| Bare args | `http` / `stdio` (same meaning as flags) |
| Env | `MCP_TRANSPORT` or `MCP_BETTER_TRANSPORT` → `stdio` \| `http` (**`MCP_TRANSPORT` first** if both set) |
| Anything else | An unknown argument or transport value prints the error and exits **2**; nothing falls back to stdio silently |
| HTTP bind | `MCP_BETTER_HTTP_ADDR` — default **`127.0.0.1:8787`**. Do **not** use `0.0.0.0` unless you accept an unauthenticated open endpoint. |

## Tools

| Tool | Purpose |
|------|---------|
| `health` | Liveness — status, version, protocol. No side effects. Not a k8s probe contract. |
| `echo` | Pure demo — returns `message` unchanged. |
| `confirm_echo` | Textbook **MRTR** (SEP-2322) — echo after mid-call confirm · sealed `requestState`. |

All three are annotated `readOnlyHint: true`, `destructiveHint: false`, `idempotentHint: true`, `openWorldHint: false`: they read only, change nothing and stay in the process.

## Protocol claims (v0.7.0 — same 7/28 era · the official conformance suite in CI)

| Surface | Status |
|---------|--------|
| Era / protocol | **7/28** · negotiated **`2026-07-28`** (Discover preferred) |
| Transport | **stdio** (default) · **Streamable HTTP** (`--http`) |
| HTTP mode | Stateless for 7/28 · `json_response` · local **Host** guards |
| Routing headers | Streamable HTTP POSTs use **`Mcp-Method`** and **`Mcp-Name`** when naming a tool (SEP-2243); `http-smoke` asserts this happy path |
| Capabilities | **tools** · **resources** (skill docs) · **skills** extension (SEP-2640, rmcp native) · prompts and completions **not declared** (`prompts/*`, `completion/complete` → method not found) |
| Cache hints | discover, tools, resources and skills: **`ttlMs=60000`**, **`cacheScope=public`** · tool order **`health`→`echo`→`confirm_echo`** (restart-stable) |
| Tool annotations | all four hints on every tool: read-only · not destructive · idempotent · closed world (`tests/handlers.rs`) |
| MRTR (optional) | **`confirm_echo`** — mid-call confirm · sealed `requestState` · asked only of clients that declare elicitation (else `-32021`) · matching client **`mrtr-client`** · [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md)
| Skills (optional) | **`mcp-better-lab`** · `skills/list` · digests · [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md) |
| Lying companion | **`mcp-worse`** — reversed tool order (the lie) + the SDK defaults left in place: no cache hints of its own (the wire shows `ttlMs: 0` + `private`, valid but uninformative), no tool annotations, and `prompts/list` / `completion/complete` answered without being declared · stdio only · same `--version` · contrast-smoke only |
| OAuth / tasks | out of hero |

## Conformance

CI runs the official MCP conformance suite ([`@modelcontextprotocol/conformance`](https://github.com/modelcontextprotocol/conformance), pinned) against two binaries. Each run covers `--requirements 2026-07-28` (50 scenarios: 37 the suite scores, 13 it runs but does not score) and the two Skills scenarios (`sep-2640-skills-enumeration`, `sep-2640-skills-manifest`, an extension outside that set).

| Run | Binary | 2026-07-28 requirements | Skills |
|-----|--------|-------------------------|--------|
| Conformance build | [`conformance/`](./conformance/): `mcp-better-conformance` | **100%, no baseline:** 196 checks pass across the 50 scenarios, none fail or warn; the suite itself skips one scenario (`tasks-status-notifications`, 0 checks) | 38 of 38 |
| Textbook | `mcp-better`, what you install | the 37 scored scenarios: 74 checks pass; 33 fail or warn, each listed in [`conformance-baseline.yml`](./conformance-baseline.yml) as `fixture` or `not declared` | 38 of 38 |

**What each run measures.** Most scenarios call tools, prompts and resources that exist only in the suite's reference server (`test_*`, `test://…`), and the 13 unscored ones are the Tasks extension and three pending scenarios. The official SDKs pass those with a separate server that carries the fixtures; `mcp-better-conformance` is that server for mcp-better. Its fixtures, capabilities and Tasks are adapted from the Rust MCP SDK's conformance server (Apache-2.0, [`conformance/NOTICE`](./conformance/NOTICE)); `BetterServer` serves mcp-better's three tools, the skill resource and Skills. So the 100% mostly measures the fixtures and the shared transport (`mcp_better::http_config`). The textbook run and the Skills scenarios measure mcp-better's own code.

**Reading the textbook run.** `caching` is 7 of 8: the eighth check calls `prompts/list`, and mcp-better doesn't declare prompts, so it answers method-not-found (`not declared` in the baseline). `prompts-list` and `completion-complete` fail for the same reason. Every other failure is a missing suite fixture. The suite's MRTR scenarios use its own fixture tools, so `confirm_echo` is checked by the matching client (`examples/mrtr-client`), which declares elicitation, and by a handler test that a client without it gets `-32021` (missing required client capability).

```bash
bash scripts/conformance.sh                                             # textbook + baseline
(cd conformance && cargo build) && \
  MCP_BETTER_BIN="$(pwd)/conformance/target/debug/mcp-better-conformance" \
  MCP_CONFORMANCE_BASELINE='' bash scripts/conformance.sh                # conformance build, strict
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

MIT — see [LICENSE](./LICENSE). Except [`conformance/`](./conformance/) (the unpublished conformance build), which is Apache-2.0: see [`conformance/NOTICE`](./conformance/NOTICE).

---

[![Business card · Get yours →](https://wolfe-jam.github.io/agent-business-card/badge.svg)](https://wolfe-jam.github.io/agent-business-card/?repo=Wolfe-Jam/mcp-better)
