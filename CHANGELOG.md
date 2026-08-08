# Changelog

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
