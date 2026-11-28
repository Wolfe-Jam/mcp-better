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

## v0.3.0 — deeper correctness (same 7/28 era) — **2026-07-31**

**Lesson:** louder proof of the list contract · lying companion for contrast. Not a new era.

| # | Surface | Required? | v0.3.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (Cargo/server.json/CHANGELOG/README = **0.3.0**) | **Yes** | ✅ | |
| M2 | BETTER purity | **Yes** | ✅ | |
| M3–M4 | Identity | **Yes** | ✅ | |
| M5 | `scripts/ci.sh` (fmt·clippy·test·**all smokes**) | **Yes** | ✅ | |
| M6 | Discover smoke | **Yes** | ✅ | multi-list ×6 |
| M7 | Stamped list | **Yes** | ✅ | |
| M8 | Tools health+echo order | **Yes** | ✅ | restart-stable smoke |
| **M22** | **order-restart-smoke** (two processes) | **Yes** | ✅ | |
| **M23** | **contrast-smoke** + **mcp-worse** | **Yes** | ✅ | worse not Registry package |
| M17–M18 | HTTP + headers | **Yes** | ✅ | retained |
| M9 | GH CI green ship SHA | **Yes** | ✅ | `@8a74697` |
| M10–M11 | crates.io 0.3.0 | **Yes** | ✅ | published 2026-07-31 |
| M13–M14 | tag + GH Release | **Yes** | ✅ | [v0.3.0](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.3.0) |
| M15 | MCP Registry `@0.3.0` | **Yes** | ✅ | HTTP 200 · device re-login 2026-07-31 |
| M16 | Truth-table | **Yes** | ✅ | ✪ RELEASE COHERENT |
| M24 | Product release note | Soft | ✅ | GH Release + `docs/RELEASE-NOTES/v0.3.0.md` |

**Verdict v0.3.0:** **FULL LIVE** — crates · Registry · GH tag/release · truth-table ✪.

---

## v0.4.0–v0.4.3 — dual package then MRTR + Skills — **lived**

Same 7/28 era. Matrix not rewritten per patch; receipts live in CHANGELOG + GH Releases.

| Cut | Lesson | Registries | Notes |
|-----|--------|------------|-------|
| **0.4.0** | Dual package cargo + npm | ✅ | `npx` zero-toolchain |
| **0.4.1** | npm `mcpName` | ✅ | Registry dual-package metadata |
| **0.4.2** | OIDC Trusted Publishing | ✅ | tag = cargo + npm |
| **0.4.3** | **`confirm_echo` MRTR** + Agent Skills | ✅ crates + npm + GH · Registry optional | Tool order **`health` → `echo` → `confirm_echo`** · [v0.4.3](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.4.3) |

**M8 from 0.4.3:** three tools, restart-stable. Not “health + echo only.”

---

## v0.4.4 — book matches 0.4.3 wire (honesty / prep) — **2026-08-18**

**Lesson:** no new tool. Book + GETTING-STARTED name the catalog 0.4.3 already shipped.  
**Publish:** **not tagged.** crates.io / npm / GH Release stay **0.4.3** until explicit GO tag.

| # | Surface | Required? | v0.4.4 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (Cargo/package.json/server.json/CHANGELOG = **0.4.4**) | **Yes** | ✅ | `scripts/doc-gate.sh` 2026-08-18 |
| M2 | BETTER purity (`! project.faf` on main) | **Yes** | ✅ | |
| M3–M4 | Identity `io.github.Wolfe-Jam/mcp-better` · not `one.faf/*` | **Yes** | ✅ | |
| M5 | `scripts/ci.sh` | **Yes** | ✅ | fmt · clippy · 18 tests · all smokes |
| M6–M7 | Discover + stamped list | **Yes** | ✅ | same as 0.4.3 |
| **M8** | Tools = **`health` → `echo` → `confirm_echo`**, stable order | **Yes** | ✅ | wire unchanged; book now matches |
| M9 | GH CI green on ship SHA | Soft this cut | — | land commit; no tag |
| M10–M15 | dry-run · crates · npm · tag · Registry | **No this cut** | — | **do not publish** |
| M17–M18 | HTTP + headers | **Yes** | ✅ | http-smoke catalog includes `confirm_echo` |

**Next lesson cut is 0.5.0 — matching client completes MRTR.**

---

## v0.5.0 — matching client completes MRTR — **2026-08-18**

**Lesson:** client resumes `confirm_echo` (R1 `input_required` → R2 complete). No new tool.  
**Publish:** **v0.5.0** tagged · crates + npm + GH Release live. MCP Registry separate.

| # | Surface | Required? | v0.5.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (lockstep **0.5.0**) | **Yes** | ✅ | 2026-08-18 |
| M2 | BETTER purity | **Yes** | ✅ | |
| M5 | `scripts/ci.sh` (incl. **mrtr-client**) | **Yes** | ✅ | |
| **M8** | Tools `health` → `echo` → `confirm_echo` | **Yes** | ✅ | no fourth tool |
| **M25** | **mrtr-client** R1→R2 + reject paths | **Yes** | ✅ | `call_tool_once` |
| M11 | crates.io | **Yes** | ✅ | **0.5.0** |
| npm | npm `mcp-better` | **Yes** | ✅ | **0.5.0** · mcpName present |
| M13–M14 | tag + GH Release | **Yes** | ✅ | [v0.5.0](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.5.0) |
| M15 | MCP Registry | Soft | — | separate human step |
| M17–M18 | HTTP + headers | **Yes** | ✅ | retained |

## v0.6.0 — the official MCP conformance suite in CI — **2026-11-15**

**Lesson:** a textbook that claims a spec should pass that spec's own tests.  
**Publish:** **v0.6.0** tagged · crates + npm (OIDC, from the Release workflow) · GH Release with 9 assets · MCP Registry **0.6.0** active.

| # | Surface | Required? | v0.6.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (lockstep **0.6.0**) | **Yes** | ✅ | |
| M5 | `scripts/ci.sh` (smokes + conformance + preflight) | **Yes** | ✅ | CI on the tag commit |
| C1 | Conformance build, no baseline | **Yes** | ✅ | 196 checks pass, none fail or warn; suite skips `tasks-status-notifications` |
| C2 | Textbook binary vs `conformance-baseline.yml` | **Yes** | ✅ | 74 scored checks pass; 33 baselined (`fixture` / `not declared`); Skills 38 of 38 |
| msrv | Rust 1.88 | **Yes** | ✅ | CI |
| M11 | crates.io | **Yes** | ✅ | **0.6.0**, matches the tag |
| npm | npm `mcp-better` | **Yes** | ✅ | **0.6.0**, provenance |
| M13–M14 | tag + GH Release | **Yes** | ✅ | [v0.6.0](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.6.0) |
| M15 | MCP Registry | **Yes** | ✅ | **0.6.0** active |

## v0.7.0 — tools say what they do — **released 2026-11-18**

**Lesson:** a tool with no annotations gets the spec's defaults; a server should ask only for what the client declared.

| # | Surface | Required? | v0.7.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (lockstep **0.7.0**, every README / guide pin) | **Yes** | ✅ | |
| M5 | `scripts/ci.sh` | **Yes** | ✅ | |
| A1 | Every tool annotated (4 hints) | **Yes** | ✅ | handler test |
| A2 | `confirm_echo` without declared elicitation → `-32021` | **Yes** | ✅ | unit + handler test |
| A3 | `--version` / unknown arguments | **Yes** | ✅ | `tests/cli.rs` |
| M11 · npm · M13–M15 | crates.io · npm · tag + GH Release · MCP Registry | **Yes** | ✅ | **0.7.0** on all four |

## v0.8.0 — confirmations that hold — **released 2026-11-27**

**Lesson:** a confirmation should hold only for what the user confirmed, and only for a while; a skill should list only what the host reads the same way.

| # | Surface | Required? | v0.8.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (lockstep **0.8.0**, every README / guide pin) | **Yes** | ✅ | `doc-gate.sh` in CI on the tag commit |
| M5 | `scripts/ci.sh` | **Yes** | ✅ | CI on the tag commit |
| R1 | Expired `requestState` → `-32602` | **Yes** | ✅ | `confirm_echo_rejects_expired_state` |
| R2 | Changed round-2 `message` → `-32602` | **Yes** | ✅ | `confirm_echo_rejects_changed_message` |
| S1 | Strict frontmatter refuses what YAML (1.2 or 1.1) reads differently; no panic on non-ASCII | **Yes** | ✅ | `frontmatter_refuses_what_it_cannot_read_exactly`, `frontmatter_refuses_review_cases` |
| S2 | One file per skill | **Yes** | ✅ | `every_skill_is_a_single_skill_md` |
| M11 · npm · M13–M15 | crates.io · npm · tag + GH Release · MCP Registry | **Yes** | ✅ | **0.8.0** on all four; npm with provenance; [v0.8.0](https://github.com/Wolfe-Jam/mcp-better/releases/tag/v0.8.0), 9 assets |

## v0.9.0 — a plugin from the registry

**Lesson:** A plugin is packaging: `mcp-better plugin` turns an MCP Registry entry into an Agent Plugins folder, and `--check` says when the registry has moved on.

| # | Surface | Required? | v0.9.0 | Notes |
|---|---------|:---------:|:------:|-------|
| M1 | Doc Gate (lockstep **0.9.0**, every README / guide pin) | **Yes** | ⏳ | `doc-gate.sh` in CI on the release commit |
| M5 | `scripts/ci.sh` | **Yes** | ⏳ | CI on the release commit |
| P1 | Plugin files follow Agent Plugins 1.0.0 (names, key order) | **Yes** | ✅ | `plugin_names_follow_the_spec`, `json_keys_in_spec_order` |
| P2 | Secrets, `{variable}` and required values with no default listed, never written | **Yes** | ✅ | `secrets_and_missing_values_are_listed_never_written` |
| P3 | `--check` exits 1 on drift only, hand-written plugins included | **Yes** | ✅ | `check_finds_drift_and_only_drift`, `check_reads_hand_written_plugins` |
| P4 | npm shim maps only the release's five targets (Windows ARM → `cargo install`) | **Yes** | ✅ | `lib/platform.js` |
| M11 · npm · M13–M15 | crates.io · npm · tag + GH Release · MCP Registry | **Yes** | ⏳ | **0.9.0** on all four |

---

## Version roadmap × matrix (advance here)

| Gate | v0.1 stdio foundation | v0.2 HTTP + headers | v0.3 deeper correctness |
|------|:---------------------:|:-------------------:|:-----------------------:|
| 7/28 era claim | **Yes** | **Yes** (same era) | **Yes** (same era) |
| stdio | **Required** | Keep | Keep |
| Discover + stamped lists | **Required** | **Required** | **Required** + multi-list |
| Streamable HTTP | — | **Required ✅** | Keep |
| Routing headers | — | **Required ✅** | Keep |
| Restart-order smoke | — | — | **Required** |
| Lying contrast | — | — | **Required** (mcp-worse) |
| crates.io | **Required** | **Required ✅** | **Required** at ship |
| MCP Registry | **Required** | **Required ✅** | **Required** at ship |
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
cd mcp-better   # your clone

bash scripts/doc-gate.sh
bash scripts/ci.sh

# crates install path
cargo install mcp-better --version 0.2.0
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example stdio-client
MCP_BETTER_BIN="$(command -v mcp-better)" cargo run --example http-smoke

bash scripts/verify-release.sh <version>   # e.g. 0.9.0; --local also runs scripts/ci.sh here

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
