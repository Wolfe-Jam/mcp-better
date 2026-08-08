# Grok-Build Brief — Post 0.4.2 (OIDC proven)

**Status:** ACTIVE · 2026-08-08  
**Owner:** Wolfe-Jam  
**Context:** Rust-First plan · Phase 2A complete for the OIDC dual-package path on `mcp-better`

---

## 1. What just closed (do not re-open)

| Gate | Result |
|------|--------|
| `mcp-better@0.4.2` crates.io | ✅ live · OIDC Trusted Publishing (GitHub) |
| `mcp-better@0.4.2` npm | ✅ live · tokenless · `mcpName: io.github.Wolfe-Jam/mcp-better` |
| MCP Registry dual cargo+npm | ✅ `io.github.Wolfe-Jam/mcp-better` v0.4.2 published |
| OIDC workflow on main | ✅ `publish-registries` job · Environment `release` · tag `v*` only |
| Repo classic secrets scrub | ✅ not required (already clean) |
| Behaviour change | ❌ none — pure patch / publish-path only |

**Meaning:** Dual-package + OIDC steady-state is proven on a live server. Phase 1 pattern + Phase 2A OIDC path are both real.

---

## 2. What already exists (do not rebuild)

| Artifact | Location / note |
|----------|-----------------|
| `scripts/mcp-dist-post.sh` | Lockstep · mcpName gate · dual `server.json` emit |
| `docs/MCP-DIST-POST.md` | Recipe · bootstrap vs steady-state OIDC |
| `docs/OIDC-TRUSTED-PUBLISHING-BRIEF.md` | Original OIDC implementation brief (workflow landed) |
| `/pubcrate` | crates.io quality protocol (unchanged scope) |
| `/pubbetter` | mcp-better-specific ship motion (still valid) |
| Trusted Publishing | crates.io + npm configured for `Wolfe-Jam/mcp-better` · workflow `release.yml` · env `release` |

**Design invariants (hold these):**
- OIDC is **publisher-side only** — zero friction for consumers or other authors
- `/pubcrate` stays crates.io-focused; does not absorb dual-package/OIDC
- `mcp-dist-post` stays the thin MCP glue; does not publish
- Three-file lockstep is mechanical and hard-fail
- Volume is not the goal; quality and teachability are

---

## 3. Current phase position

```
Phase 0  — schema hygiene          CANCELLED / already true
Phase 1  — dual-package proof      CLOSED (0.4.1)
Phase 2A — recipe + post-step + OIDC   CLOSED on mcp-better (0.4.2)
Phase 2B — thin CLI / skill        NOT STARTED
Phase 3  — docs to npm parity      NOT STARTED
Phase 4  — quality bar (wjttc)     NOT STARTED
```

Cascade remains: **prove → repeatable → teachable → checkable.**

---

## 4. Recommended next work (ordered)

### A. Immediate hygiene (cheap, high value)
1. Ensure `docs/MCP-DIST-POST.md` reflects the **lived** 0.4.2 path (any delta from the brief).
2. Record a one-line receipt: “OIDC dual-package steady-state proven on mcp-better@0.4.2”.
3. Do **not** expand scope into Phase 2B until a second server has used the recipe.

### B. Second-server receipt (Phase 2A completion criteria)
- Run **one other FAF Rust MCP server** through the same path:
  - dual-package (cargo + npm shim)
  - `mcp-dist-post`
  - OIDC if the crate already exists on crates.io (bootstrap token only for brand-new crates)
- Gate: `npx <name>` starts a session with no Rust toolchain; Registry shows dual packages.
- This is the proof the recipe is not `mcp-better`-specific.

### C. Phase 2B only after B
- Promote `mcp-dist-post` into a thin CLI/skill if and only if the second server shows hand-rolling friction.
- Prefer adopt + document over green-field tooling.

### D. Explicitly not next
- No `/pubcrate` rewrite
- No Registry schema PRs required for this path
- No Phase 3 docs push until the tool path is repeatable on ≥2 servers
- No Phase 4 `wjttc` work unless you deliberately parallelize 4a (binary launch lane)

---

## 5. Quality Standards (always)

Before any publish/tag:

1. Coherent `main` (branch consolidation if needed)
2. Docs gate (README / CHANGELOG / version identity)
3. Visible `mcp-name:` in README for MCP servers
4. fmt · clippy · test · release build · `cargo doc`
5. `cargo publish --dry-run` + `npm publish --dry-run` reviewed
6. `mcp-dist-post --dry-run` exit 0
7. Explicit **GO** / **GREEN LIGHT** from Wolfe-Jam
8. Named-file commit only (not blind `git add -A`)
9. Tag `vX.Y.Z` → watch `publish-registries`
10. Real `mcp-dist-post` + `mcp-publisher publish server.json`

Avoid `| tail` on long `cargo doc` / compile steps in gate scripts (false “stuck” signal).

---

## 6. OIDC reference (steady-state)

**Already configured**
- crates.io Trusted Publishing → `Wolfe-Jam/mcp-better` · `release.yml` · env `release`
- npm Trusted Publisher → same · allow `npm publish`
- GitHub Environment `release` exists

**Bootstrap rule (other crates)**
- First crates.io publish of a **brand-new** crate still needs a classic token once
- Then configure Trusted Publishing and drop the token
- npm can use OIDC from first publish if account 2FA is enabled

**Publisher-only**
- Consumers and other authors are never required to use OIDC

---

## 7. Success criteria for the next milestone

| Milestone | Done when |
|-----------|-----------|
| Recipe locked | Second FAF Rust MCP server ships dual-package + Registry via the same path |
| Phase 2A closed fully | Above + docs match lived reality |
| Phase 2B start | Explicit decision after second-server friction is observed |

---

## 8. One-line status for memory / index

> Rust-First Phase 2A: dual-package + OIDC steady-state proven on mcp-better@0.4.2 (2026-08-08). Next: second-server receipt, then decide on thin CLI (2B).

---

*Hand-off for Grok-Build · no speculative work · quality over volume · quiet and correct*
