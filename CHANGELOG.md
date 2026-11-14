# Changelog

## [Unreleased]

**Lesson:** a textbook that claims a spec should pass that spec's own tests. CI runs the official MCP conformance suite against two binaries: the conformance build passes every check, and the textbook binary passes every scored check that needs no suite fixture.

### Fixed

- **Cache hints on every list and read** (SEP-2549): `resources/list`, `resources/templates/list`, `resources/read`, `skills/list` and `skills/get` now stamp `ttlMs=60000` + `cacheScope=public`, like `tools/list`. Before, only `tools/list` did.
- **Declare what you answer:** the server answered `prompts/list` and `completion/complete` without declaring either (rmcp's defaults). Neither is declared, and both now answer method-not-found.
- **Skills manifest has `size`** (SEP-2640): each file in `skills/list` / `skills/get` now carries `size` next to `uri` and `digest`.
- **Not-found errors name the URI** (SEP-2164): `resources/read` and `skills/get` carry `data.uri`.
- **The skill document lists and reads as the same thing** (SEP-2640): `resources/read` returns `text/markdown`, as `resources/list` says, and the listed name and description are the skill's frontmatter.

### Changed

- **rmcp 3.0.1 → 3.10.0.** Deprecated names updated (`ServerConfig`, `ClientConfig`, `RequestStateCodec::try_new`). rustls 0.23.46 in the lock.
- **Skills on rmcp's native API:** `list_skills` / `get_skill` (rmcp ≥ 3.9) replace the custom-request handler.
- **Official MCP conformance suite in CI** (`scripts/conformance.sh`, `--requirements 2026-07-28`: 50 scenarios, 37 scored and 13 run but not scored; plus the two Skills scenarios). `conformance/` builds `mcp-better-conformance`: mcp-better's tools, skill resource and Skills, with the suite's fixtures, capabilities and Tasks adapted from the Rust MCP SDK's conformance server (Apache-2.0). It passes every check with no baseline: 196 checks, no failure, no warning, enforced by the script over all 50 scenarios, plus Skills 38 of 38. The textbook binary passes the 37 scored scenarios except 33 checks listed in `conformance-baseline.yml` (`fixture` or `not declared`), and Skills 38 of 38. Results are kept as a CI artifact when a run fails.
- **rust-mcp-preflight** checks the Registry listing (`scripts/preflight.sh`): in CI, and strict in `scripts/registry-publish.sh` before the Registry publish.
- **GitHub CI:** the MRTR matching client, Preflight, and an msrv job. `rust-version` is now 1.88 (1.85 was false: the dependencies need 1.88).
- **Install lines use `--locked`**, so `cargo install` builds with the tested lockfile.
- **Wording:** "AAIF-verified" → "reviewed and approved as an AAIF Ambassador contribution" (README) / "AAIF Ambassador contribution" (package descriptions).
- **Docs:** two internal release briefs left the public `docs/` folder; `docs/PUBBETTER.md` no longer points at a local skill path.
- **`mcp-better-lab` carries a FAF Block** (also called a Context Block): its facts as YAML at the top of the body: what it needs, what it reads, that it writes nothing and publishes nothing, its network use (none on stdio; localhost over Streamable HTTP), and when it's done. The description now says when to use it. The skill is served over MCP, so the facts travel with it; the digest still matches `resources/read`.

## [0.5.0] — 2026-08-18 — matching client completes MRTR

**Lesson:** the peer is two-sided. 0.4.3 proved the server can pause; this cut is the client that resumes.

### Added

- **`mrtr-client` example** — Discover `2026-07-28`, then `confirm_echo` R1 `input_required` → echo sealed `requestState` + `CONFIRM` → complete. Rejects wrong confirm text and tampered state. Uses `call_tool_once` so the retry stays visible.
- CI: `scripts/ci.sh` runs the matching-client smoke after stdio-client.

### Unchanged

- Tools: **`health` → `echo` → `confirm_echo`** (no fourth tool)
- Dual package cargo + npm · identity **`io.github.Wolfe-Jam/mcp-better`**
- Era **7/28** · wire **`2026-07-28`**
- BETTER purity — no `project.faf` on main
- `stdio-client` Discover baseline (OK line unchanged)

### Note

- 0.4.4 honesty (book = 0.4.3 wire) rides in this tree. **Published** `v0.5.0` (crates + npm + GH Release).

---

## [0.4.4] — 2026-08-18 — book matches the 0.4.3 wire

**Lesson:** honesty cut — no new tool. The book names the catalog that already shipped.

### Fixed / honesty

- Textbook + GETTING-STARTED: catalog is **`health` → `echo` → `confirm_echo`** (matches `BETTER_TOOL_ORDER` + stdio-client OK line)
- Ch 06: `confirm_echo` is one sealed MRTR retry — not a session, not a handle store
- Ch 15: MRTR + Agent Skills are **on the wire since 0.4.3** — do not re-propose
- README claims header + lesson table name **0.4.4**; crates.io / npm install pins stay **0.4.3** until this cut is tagged

### Unchanged

- Tools: **`health` → `echo` → `confirm_echo`** (no fourth tool)
- Dual package cargo + npm · identity **`io.github.Wolfe-Jam/mcp-better`** (not `one.faf/*`)
- Era **7/28** · wire **`2026-07-28`** · stdio default · Streamable HTTP `--http` local demo
- BETTER purity — no `project.faf` on main

### Prep

- Surfaces cleared for the next **lesson** cut (**0.5.0** — unnamed). This version is not that lesson.

---

## [0.4.3] — 2026-08-11 — MRTR + Agent Skills over MCP

**Lesson:** optional modern depth stays textbook — claim = wire.

### Added

- **`confirm_echo`** — SEP-2322 **MRTR** textbook tool (mid-call confirm · sealed `requestState` · `input_required` → retry). See [`docs/MRTR-CONFIRM-ECHO.md`](./docs/MRTR-CONFIRM-ECHO.md).
- **Agent Skills over MCP (J1)** — extension `io.modelcontextprotocol/skills` · `skills/list` · `skills/get` · skill `mcp-better-lab` with digests. See [`docs/SKILLS-OVER-MCP.md`](./docs/SKILLS-OVER-MCP.md).
- Tool order: **`health` → `echo` → `confirm_echo`** (restart-stable · stamped list).

### Unchanged

- Dual package cargo + npm · identity **`io.github.Wolfe-Jam/mcp-better`** (not `one.faf/*`)
- Era **7/28** · wire **`2026-07-28`** · stdio default · Streamable HTTP `--http` local demo
- BETTER purity — no `project.faf` on main

---

## [0.4.2] — 2026-08-08 — OIDC Trusted Publishing proof

- **OIDC ship path:** `v*` tag triggers crates.io (crates-io-auth-action) + npm Trusted Publishing via Environment `release` — zero long-lived registry tokens after bootstrap.
- Pure patch for proof of the dual-package OIDC release workflow. No tool/protocol behaviour change.

---

## [0.4.1] — 2026-08-08 — Registry dual-package metadata

- **npm `mcpName`:** `package.json` now declares `"mcpName": "io.github.Wolfe-Jam/mcp-better"` so the MCP Registry accepts the npm package alongside cargo (validator requirement; no behaviour change).

---

## [0.4.0] — 2026-08-07 — dual package (cargo + npm, zero-toolchain npx)

**Lesson:** ship the textbook where the host already is.  
Rust stays the source of truth; npm is a **downloader shim** so `npx mcp-better` starts a real stdio session on a machine with **no Rust toolchain**.

### Added

- **npm package `mcp-better`** — `bin/mcp-better.js` detects platform/arch, downloads the matching binary from GitHub Releases, execs it on stdio
- **`server.json` dual packages** — `cargo` (`crates.io`) + `npm` (`registry.npmjs.org`), both stdio, same version
- **cargo-dist durable path** — `dist-workspace.toml` + `.github/workflows/release.yml` (native multi-target archives on `v*` tags)
- **Three-file lockstep gate** — `Cargo.toml` · `package.json` · `server.json` versions must agree (doc-gate fails on drift)

### Unchanged

- Tools on **mcp-better:** `health` + `echo` only
- Era **7/28** / wire **`2026-07-28`**
- stdio default · Streamable HTTP `--http` local demo
- **cargo path** — `cargo install mcp-better` still works; no breaking change
- **mcp-worse** teaching companion only

---

## [0.3.0] — 2026-07-31 — deeper correctness (same 7/28 era)

**Lesson:** make the BETTER list contract harder to lie about and easier to verify.  
More road inside 7/28 — not a new protocol era. Still two tools on `mcp-better`.

### Added

- **Louder smokes:** `stdio-client` asserts stamps + order on **6** list calls (1 + 5)
- **`order-restart-smoke`** — two sequential processes, same order + stamps
- **`mcp-worse`** binary — lying companion (unstamped list, reversed order)
- **`contrast-smoke`** — better passes BETTER contract · worse fails it
- Unit tests: 20× same-process lists · multi-instance order · worse contract
- Public `BETTER_TOOL_ORDER` constant
- **`textbook/`** Season 1 + doctrine book=app (from unreleased) shipped with app

### Fixed / honesty (pre-0.3 doc pass)

- `GETTING-STARTED` exact OK lines · env precedence · header claim wording

### Unchanged

- Tools on **mcp-better:** `health` + `echo` only
- Era **7/28** / wire **`2026-07-28`**
- stdio default · Streamable HTTP `--http` local demo
- **mcp-worse** is teaching-only · not Registry product surface

---

## [0.2.0] — 2026-07-29 — same 7/28 era + Streamable HTTP

**mcp-better — built for 7/28** · more road, not a new era.

### Added

- **`--http`** Streamable HTTP on `MCP_BETTER_HTTP_ADDR` (default `127.0.0.1:8787/mcp`)
- Stateless 7/28 path · `json_response` · loopback **Host** guards
- Example `http-smoke` — list stamps + `health` with `Mcp-Method` / `Mcp-Name`
- CLI `--help` · keep **stdio** as default (Cursor / Desktop)
- **SECURITY.md** v0.2 HTTP threat posture (no auth/TLS · loopback bind · Host ≠ ACL)
- Docs: transport env (`MCP_BETTER_TRANSPORT` / `MCP_TRANSPORT`) · Registry lists **stdio only by design**

### Fixed / honesty

- Server `instructions` are transport-neutral (no longer claim “over stdio” only)
- HTTP smoke: `echo` round-trip over Streamable HTTP (parity with stdio-client)
- Lib/test comments: dual-transport entry; integration tests labeled as crate-boundary only

### Unchanged

- Tools `health` + `echo` · Discover-compatible stamped `tools/list`
- Identity `io.github.Wolfe-Jam/mcp-better`

---

## [0.1.0] — 2026-07-29 — 7/28 over stdio (foundation)

**mcp-better — built for 7/28** (protocol `2026-07-28`).

### Added

- Greenfield **mcp-better** Rust server on **rmcp 3.0**
- Tools: `health`, `echo`
- Stamped `tools/list`: `ttlMs=60000`, `cacheScope=public`, stable order
- Example `stdio-client` with `ClientLifecycleMode::Discover` → `2026-07-28`
- CI: fmt, clippy, test, Discover smoke · `scripts/ci.sh`
- Docs: 7/28 definition · README · BETTER.md · roadmap v0.1 stdio / v0.2 HTTP
- `server.json` identity: `io.github.Wolfe-Jam/mcp-better` (cargo)
- **`/pubbetter`** publish protocol (dry-run · CI · tag · crates · Registry)
