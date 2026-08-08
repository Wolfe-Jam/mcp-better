# Grok-Build Brief — OIDC Trusted Publishing for Dual-Package Rust MCP

**Status:** Workflow landed — Environment `release` + Trusted Publishing must match  
**Owner:** Wolfe-Jam  
**Target:** `mcp-better` first (already dual-package @ 0.4.1)  
**Date:** 2026-08-08  
**Related:** Phase 2A (`mcp-dist-post`), `/pubcrate`, Rust-First plan  
**Implementation:** `.github/workflows/release.yml` job `publish-registries` (tag push only)

---

## Goal

After the one-time bootstrap, both halves of a dual-package Rust MCP server publish with **zero long-lived registry tokens**:

- crates.io → OIDC via `rust-lang/crates-io-auth-action`
- npm → OIDC Trusted Publishing (no `NPM_TOKEN` / `NODE_AUTH_TOKEN`)

The Phase 2A post-step (`mcp-dist-post`) stays unchanged — it never publishes.

---

## Non-goals

- Do not rewrite `/pubcrate`
- Do not build the Phase 2B CLI yet
- Do not change MCP Registry / `mcp-publisher` auth
- Do not alter the dual-package `server.json` shape
- Do not enable staged publishing unless explicitly asked

---

## Prerequisites (already true for mcp-better)

- [x] Crate published on crates.io (`mcp-better@0.4.1`)
- [x] Matching npm package published with `mcpName`
- [x] Dual-package entry live on the MCP Registry
- [x] GitHub repo: `Wolfe-Jam/mcp-better`

---

## Implementation steps

### 1. Configure Trusted Publishing (manual, one-time)

**crates.io**
1. Go to https://crates.io/crates/mcp-better/settings → Trusted Publishing → Add
2. Fill:
   - Owner: `Wolfe-Jam`
   - Repository: `mcp-better`
   - Workflow filename: the exact release workflow that will publish (e.g. `release.yml`)
   - Environment: `release` (recommended)
3. Save

**npm**
1. Go to the `mcp-better` package on npmjs.com → Settings → Trusted Publisher
2. Choose GitHub Actions
3. Fill:
   - Organization/user: `Wolfe-Jam`
   - Repository: `mcp-better`
   - Workflow filename: same release workflow
   - Environment: `release` (optional but recommended)
   - Allowed actions: `npm publish` (only)
4. Save

### 2. Create / harden the GitHub Environment

In the repo Settings → Environments → New environment: `release`

Recommended protection rules:
- Required reviewers (at least one)
- Restrict to tags matching `v*`

### 3. Update the release workflow

Target file: `.github/workflows/release.yml` (or the cargo-dist generated equivalent).

**Required permissions on the publish job(s):**
```yaml
permissions:
  id-token: write   # OIDC
  contents: read    # checkout only
```

**crates.io publish step (replace token-based auth):**
```yaml
- uses: actions/checkout@<pin-sha>
- uses: rust-lang/crates-io-auth-action@v1
  id: auth
- run: cargo publish
  env:
    CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
```

**npm publish step (remove NODE_AUTH_TOKEN):**
```yaml
- uses: actions/setup-node@<pin-sha>
  with:
    node-version: '24'
    registry-url: 'https://registry.npmjs.org'
- run: npm publish --access public
  # no NODE_AUTH_TOKEN when Trusted Publishing is configured
```

**Hardening requirements:**
- Pin third-party actions by commit SHA (not floating tags)
- Job must reference `environment: release`
- Trigger only on version tags (`v*`), never on `pull_request_target` or `workflow_run`
- Keep the publish job permission-minimal; separate any GitHub Release / asset-upload work into other jobs if needed

### 4. Remove classic secrets

After a successful OIDC publish of both packages:

- Delete `CARGO_REGISTRY_TOKEN` / crates.io API token from repo secrets (if present)
- Delete `NPM_TOKEN` / `NODE_AUTH_TOKEN` from repo secrets (if present)
- Confirm no workflow still references them

### 5. Verify

On the next version bump (or a dry-run tag if preferred):

| Gate | Expected |
|------|----------|
| Tag push triggers the release workflow | ✓ |
| crates.io publish succeeds via OIDC | ✓ (look for “VIA GITHUB” on the version) |
| npm publish succeeds with no token | ✓ |
| `mcp-dist-post` still passes lockstep + mcpName | ✓ |
| `mcp-publisher publish server.json` still works | ✓ |
| Repo secrets contain no long-lived registry tokens | ✓ |

---

## Deliverables

1. Updated release workflow (OIDC for both registries)
2. Short note added to `docs/MCP-DIST-POST.md` (or equivalent) documenting:
   - Bootstrap (first publish still needs a classic token for crates.io)
   - Steady-state OIDC path
   - That `mcp-dist-post` remains the post-publish MCP glue
3. Confirmation that classic registry tokens have been removed from the repo

---

## Order of operations (do not reorder)

```
1. Configure Trusted Publishing on crates.io + npm   (manual UI)
2. Create/harden GitHub Environment `release`
3. Update workflow to OIDC
4. Land the workflow change
5. Publish next version via the new path
6. Only then delete the classic secrets
```

Deleting secrets before a successful OIDC publish will lock you out.

---

## Reference — minimal secure jobs

**crates.io**
```yaml
- uses: rust-lang/crates-io-auth-action@v1
  id: auth
- run: cargo publish
  env:
    CARGO_REGISTRY_TOKEN: ${{ steps.auth.outputs.token }}
```

**npm**
```yaml
- uses: actions/setup-node@v6
  with:
    node-version: '24'
    registry-url: 'https://registry.npmjs.org'
- run: npm publish --access public
```

---

## Success criteria

- Next `mcp-better` release publishes both packages with zero long-lived registry tokens
- Phase 2A post-step continues to work unchanged
- Pattern is documented so the next dual-package Rust MCP server can copy it

---

*Rust-First · Phase 2A companion · 2026-08-08*
