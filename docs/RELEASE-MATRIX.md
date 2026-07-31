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
| M15 | MCP Registry `io.github.Wolfe-Jam/mcp-better@0.1.0` | **Yes** | ✅ | Published 2026-07-29 · HTTP 200 |
| M16 | Truth-table exit 0 | **Yes** | ✅ | Registry warned only |
| M17 | Streamable HTTP | — | — | **v0.2** |
| M18 | `Mcp-Method` / `Mcp-Name` | — | — | **v0.2** |
| M19 | Host smoke (Claude Desktop / Cursor) | Soft | ✅ | Cursor Home MCP: health+echo · stdio |
| M20 | AAIF project_contribution filed | Soft | ✅ | [#310](https://github.com/aaif/ambassadors/issues/310) · **scored 15** (2026-07-30) |
| M21 | Multi-OS CI matrix | Soft | ○ | ubuntu-only |

### v0.1.0 proof log (automated pass 2026-07-29)

```text
local ci.sh          ✅ green
Discover (workspace) ✅ 7/28 · ttl 60000 · Public · health+echo
cargo install 0.1.0  ✅ from crates.io
Discover (installed) ✅ same stamps against crates binary
GH CI main           ✅ success @ 9486831
crates.io API        ✅ max_version=0.1.0
GH release           ✅ v0.1.0 published
MCP Registry         ✅ 0.1.0 HTTP 200
truth-table          ✅ coherent
```

**Verdict v0.1.0:** **FULL LIVE** — crates + Registry + GH · protocol claims **proved**.

---

## v0.2.0 — same 7/28 era + Streamable HTTP — **2026-07-29/30**

**Rule:** more **road**, not a new era. Registry package transport remains **stdio only by design**; HTTP is opt-in local demo in the same binary.

| # | Surface | Required? | v0.2.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (Cargo/server.json/CHANGELOG/README = **0.2.0**) | **Yes** | ✅ | |
| M2 | BETTER purity (`! project.faf` on main) | **Yes** | ✅ | |
| M3 | Identity `io.github.Wolfe-Jam/mcp-better` | **Yes** | ✅ | |
| M4 | Identity not `one.faf/*` | **Yes** | ✅ | |
| M5 | Local `scripts/ci.sh` (fmt·clippy·test·**stdio + HTTP smokes**) | **Yes** | ✅ | includes `http-smoke` |
| M6 | Discover smoke — protocol `2026-07-28` | **Yes** | ✅ | stdio-client |
| M7 | Stamped list `ttlMs>0` · `cacheScope=Public` | **Yes** | ✅ | both transports |
| M8 | Tools = `health` + `echo` only, stable order | **Yes** | ✅ | |
| M9 | GitHub Actions green on ship SHA | **Yes** | ✅ | release `@9881f34` · follow-ups green |
| M10 | `cargo publish --dry-run` clean | **Yes** | ✅ | pre-GO |
| M11 | crates.io `max_version` == ship | **Yes** | ✅ | **0.2.0** (published 2026-07-30) |
| M12 | `cargo install mcp-better@0.2.0` path | **Yes** | ✅ | install + smokes via CI/examples |
| M13 | git tag `v0.2.0` | **Yes** | ✅ | |
| M14 | GitHub Release published | **Yes** | ✅ | [v0.2.0](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.2.0) |
| M15 | MCP Registry `@0.2.0` | **Yes** | ✅ | HTTP 200 · cargo/stdio package |
| M16 | Truth-table / release-verify coherent | **Yes** | ✅ | |
| **M17** | **Streamable HTTP** (`--http` · loopback · SECURITY.md) | **Yes** | ✅ | default `127.0.0.1:8787/mcp` · no auth/TLS (local demo) |
| **M18** | **`Mcp-Method` / `Mcp-Name`** on HTTP smoke | **Yes** | ✅ | `examples/http-smoke` · list + health + **echo** |
| M19 | Host smoke (stdio) | Soft | ✅ | Cursor path retained from v0.1 |
| M20 | AAIF project_contribution | Soft | ✅ | [#310](https://github.com/aaif/ambassadors/issues/310) · **15 pts** · status:approved |
| M21 | Multi-OS CI matrix | Soft | ○ | ubuntu-only |

### v0.2.0 proof log (honesty pass 2026-07-31)

```text
product version       ✅ Cargo.toml / server.json / CHANGELOG = 0.2.0
era claim             ✅ same 7/28 — HTTP is more road, not a new protocol date
local ci.sh           ✅ dual smokes (stdio Discover + HTTP)
GH CI release ship    ✅ success @ 9881f34 (release: v0.2.0)
GH CI post-ship docs  ✅ success through @ 64f9570
crates.io             ✅ max_version=0.2.0
GH release            ✅ https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.2.0
MCP Registry          ✅ io.github.Wolfe-Jam/mcp-better@0.2.0 (stdio package)
M17 Streamable HTTP   ✅ --http · SECURITY.md · loopback Host guards
M18 routing headers   ✅ http-smoke: Mcp-Method / Mcp-Name · echo round-trip
AAIF #310             ✅ scored 15 project_contribution (Goose/@angiejones 2026-07-30)
Registry transport    ✅ stdio only by design (HTTP not a second Registry package)
```

**Verdict v0.2.0:** **FULL LIVE** — same 7/28 era · dual-transport claim surface **proved** · AAIF-verified contribution scored.

---

## Version roadmap × matrix (advance here)

| Gate | v0.1 stdio foundation | v0.2 HTTP + headers | v0.3+ polish |
|------|:---------------------:|:-------------------:|:------------:|
| 7/28 era claim | **Yes** | **Yes** (same era) | **Yes** |
| stdio | **Required** | Keep | Keep |
| Discover + stamped lists | **Required** | **Required** | **Required** |
| Streamable HTTP | — | **Required ✅** | Required |
| Routing headers | — | **Required ✅** | Required |
| crates.io | **Required** | **Required ✅** | **Required** |
| MCP Registry | **Required** | **Required ✅** | **Required** |
| Host smoke | Soft | Soft | Soft→Required |
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
[ ] M17–M18 HTTP + headers CI (required since 0.2)
```

---

## How to re-run this matrix

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cd ~/FAF/mcp-better

bash scripts/doc-gate.sh
bash scripts/ci.sh

# crates install path
cargo install mcp-better --version 0.2.0
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example http-smoke

bash scripts/verify-release.sh .   # or pubbetter release-verify

gh run list --repo Wolfe-Jam/mcp-better --branch main --limit 3
```

---

## Next (after honesty close)

| Priority | Action |
|----------|--------|
| **P1** | **MCP Part I InterOp** — GOOD→BETTER content (no FAF lede) |
| **Soft** | Optional quiet amplify (if still open) |
| **P2** | ≤1 modern-depth teach story · TS twin · MCP-BEST after soak |
| **Hold** | OAuth/prod HTTP · resources/prompts/tasks hero · SEP zoo |
