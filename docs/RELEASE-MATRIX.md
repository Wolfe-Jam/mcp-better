# Release matrix — mcp-better

**Product:** built for **7/28** (protocol **2026-07-28**)  
**Process:** `/pubbetter` · `scripts/ci.sh` · truth-table  

Every row is a **gate surface**. A version is **REAL** only when required cells are ✅.

---

## Legend

| Mark | Meaning |
|------|---------|
| ✅ | Proved this release |
| ⏳ | Required but open |
| ○ | Optional / N/A this version |
| — | Not in scope for this version |

---

## v0.1.0 — 7/28 over stdio (foundation) — **2026-07-29**

| # | Surface | Required? | v0.1.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (version sync Cargo/server.json/CHANGELOG/README) | **Yes** | ✅ | `scripts/doc-gate.sh` |
| M2 | BETTER purity (`! project.faf`) | **Yes** | ✅ | |
| M3 | Visible `mcp-name: io.github.Wolfe-Jam/mcp-better` | **Yes** | ✅ | crates.io README bake |
| M4 | Identity not `one.faf/*` | **Yes** | ✅ | |
| M5 | Local `scripts/ci.sh` (fmt·clippy·test·smoke) | **Yes** | ✅ | |
| M6 | Discover smoke — protocol `2026-07-28` | **Yes** | ✅ | |
| M7 | Stamped list `ttlMs>0` · `cacheScope=Public` | **Yes** | ✅ | 60000 / Public |
| M8 | Tools = `health` + `echo` only, stable order | **Yes** | ✅ | |
| M9 | GitHub Actions green on ship SHA | **Yes** | ✅ | `@9486831` |
| M10 | `cargo publish --dry-run` clean | **Yes** | ✅ | pre-GO |
| M11 | crates.io `max_version` == ship | **Yes** | ✅ | **0.1.0** |
| M12 | `cargo install mcp-better@0.1.0` + Discover smoke | **Yes** | ✅ | installed binary via `MCP_BETTER_BIN` |
| M13 | git tag `vX.Y.Z` | **Yes** | ✅ | `v0.1.0` |
| M14 | GitHub Release published | **Yes** | ✅ | [release](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.1.0) |
| M15 | MCP Registry `io.github.Wolfe-Jam/mcp-better@0.1.0` | **Yes** (LIVE complete) | ✅ | Published 2026-07-29 · HTTP 200 |
| M16 | Truth-table exit 0 | **Yes** | ✅ | Registry warned only |
| M17 | Streamable HTTP | — | — | **v0.2** |
| M18 | `Mcp-Method` / `Mcp-Name` | — | — | **v0.2** |
| M19 | Host smoke (Claude Desktop / Cursor) | Soft | ✅ | Cursor Home MCP: mcp-better green · health+echo enabled · stdio-client proved same binary |
| M20 | AAIF project_contribution filed | Soft | ⏳ | after Registry preferred |
| M21 | Multi-OS CI matrix | Soft | ○ | ubuntu-only pre-ship today |

### v0.1.0 proof log (automated pass 2026-07-29)

```text
local ci.sh          ✅ green
Discover (workspace) ✅ 7/28 · ttl 60000 · Public · health+echo
cargo install 0.1.0  ✅ from crates.io (~8 min cold)
Discover (installed) ✅ same stamps against crates binary
GH CI main           ✅ success @ 9486831
crates.io API        ✅ max_version=0.1.0
GH release           ✅ v0.1.0 published
MCP Registry         ✅ 0.1.0 HTTP 200 (2026-07-29)
truth-table          ✅ coherent (all hard surfaces)
```

**Verdict v0.1.0:** **FULL LIVE** — crates + Registry + GH · protocol claims **proved**.

---

## Version roadmap × matrix (advance here)

| Gate | v0.1 stdio foundation | v0.2 HTTP + headers | v0.3+ polish |
|------|:---------------------:|:-------------------:|:------------:|
| 7/28 era claim | **Yes** | **Yes** (same era) | **Yes** |
| stdio | **Required** | Keep | Keep |
| Discover + stamped lists | **Required** | **Required** | **Required** |
| Streamable HTTP | — | **Required** | Required |
| Routing headers | — | **Required** | Required |
| crates.io | **Required** | **Required** | **Required** |
| MCP Registry | **Required** | **Required** | **Required** |
| Host smoke | Soft | Soft→Required | Required |
| TS twin | — | Soft | Soft |
| MRTR / subscriptions demo | — | Soft (pick one) | Soft |

**Rule:** later versions **add rows**, they do not redefine 7/28.  
v0.2 is **same era, more road** — not “when we become 7/28.”

---

## Per-release checklist (copy for next ship)

```text
[ ] M1–M4  docs + purity + identity
[ ] M5–M8  local protocol ship bar
[ ] M9     GH CI green on ship SHA
[ ] M10    cargo publish --dry-run
[ ] GO!
[ ] M11–M12 crates publish + install smoke
[ ] M13–M14 tag + GH release
[ ] M15     mcp-publisher publish server.json
[ ] M16     release-verify.sh exit 0 (no Registry warn)
[ ] (v0.2+) M17–M18 HTTP + headers CI
```

---

## How to re-run this matrix

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cd ~/FAF/mcp-better

bash scripts/doc-gate.sh
bash scripts/ci.sh

# crates install path (optional temp CARGO_HOME)
cargo install mcp-better --version 0.1.0
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client

bash ~/.claude/skills/pubbetter/scripts/release-verify.sh .

gh run list --repo Wolfe-Jam/mcp-better --branch main --limit 3
```

---

## Next actions to close v0.1 matrix → advance

| Priority | Action |
|----------|--------|
| **P0** | Registry: `mcp-publisher login github` → `publish server.json` → re-verify |
| **P1** | Optional: one host config smoke (Claude Desktop / Cursor) |
| **P2** | AAIF filing |
| **Road** | v0.2 Streamable HTTP + routing headers (new matrix columns M17–M18) |
