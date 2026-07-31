# Changelog

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
