# MCP Dual-Package Post-Step (Phase 2A)

Thin companion to **cargo-dist** and **`/pubcrate`**.

It does **not** publish to crates.io or npm.  
It does **not** replace `/pubcrate`.  
It only owns the MCP-specific dual-package surface.

---

## Where it sits

```
/pubcrate  (or manual cargo publish)     → crates.io live
cargo-dist (or equivalent)               → binaries + npm package live
         ↓
scripts/mcp-dist-post.sh                 ← this step
         ↓
mcp-publisher publish server.json        → Registry dual entry live
```

---

## What it does

1. **Three-file lockstep**  
   `Cargo.toml` version == `package.json` version == `server.json` version  
   Any drift → exit 1 (hard fail, no warning-and-continue).

2. **`mcpName` gate**  
   Confirms the published (or about-to-be-published) npm `package.json` contains:
   ```json
   "mcpName": "io.github.<owner>/<server>"
   ```
   This is distinct from the README `mcp-name:` token that `/pubcrate` Step 1.6 already enforces.

3. **Dual-package `server.json`**  
   Writes / updates a `server.json` that declares **both**:
   - `registryType: "cargo"`
   - `registryType: "npm"`  
   under the same server identity, with matching versions.

4. **Optional paste-ready block**  
   With `--print-mcp-json` it prints a ready-to-paste `mcpServers` snippet.

---

## Prerequisites

- `Cargo.toml` and `package.json` already exist and share the same version.
- The npm `package.json` already contains the correct `mcpName` field  
  (add it before `npm publish` — the Registry rejects packages that lack it).
- `python3` available (used for JSON read/write).

---

## Usage

```bash
chmod +x scripts/mcp-dist-post.sh

./scripts/mcp-dist-post.sh \
  --crate mcp-better \
  --mcp-name io.github.Wolfe-Jam/mcp-better \
  --print-mcp-json
```

### Useful flags

| Flag | Meaning |
|------|---------|
| `--dry-run` | Check lockstep + mcpName and print the would-be `server.json`; write nothing |
| `--print-mcp-json` | Emit a paste-ready `mcpServers` block on success |
| `--server-json PATH` | Override path (default `./server.json`) |
| `--package-json PATH` | Override path (default `./package.json`) |
| `--cargo-toml PATH` | Override path (default `./Cargo.toml`) |

### Exit codes

| Code | Meaning |
|------|---------|
| 0 | Lockstep clean, `server.json` ready |
| 1 | Version drift or missing/invalid `mcpName` |
| 2 | Missing inputs / bad arguments |

---

## Recommended `dist-workspace.toml` skeleton

```toml
[workspace]
members = ["cargo:."]

[dist]
cargo-dist-version = "0.32.0"
ci = "github"
hosting = "github"

installers = ["shell", "powershell", "npm"]
publish-jobs = ["npm"]

targets = [
  "aarch64-apple-darwin",
  "x86_64-apple-darwin",
  "x86_64-unknown-linux-gnu",
  "aarch64-unknown-linux-gnu",
  "x86_64-pc-windows-msvc"
]

install-path = "CARGO_HOME"
install-updater = false
github-attestations = true
```

---

## Full dual-package motion (with existing skills)

```
1. /pubcrate                  # crates.io quality + docs gate + approval
2. cargo-dist release         # binaries + npm downloader package
3. scripts/mcp-dist-post.sh   # lockstep + mcpName + dual server.json
4. mcp-publisher publish server.json
```

`/pubcrate` stays the crates.io gate.  
This post-step stays the MCP dual-package companion.  
They do not merge.

---

## OIDC Trusted Publishing (steady-state)

Goal: **zero long-lived registry tokens** after bootstrap.  
Brief: [`OIDC-TRUSTED-PUBLISHING-BRIEF.md`](./OIDC-TRUSTED-PUBLISHING-BRIEF.md).

### Bootstrap (first-time / until OIDC works)

- crates.io may still need a classic token for the **first** human publish of a new crate.
- npm Trusted Publisher is configured after the package exists.
- **Do not delete** classic repo secrets until a tag-triggered OIDC dual publish has succeeded.

### Steady-state (this repo)

On `v*` tag push, `.github/workflows/release.yml`:

1. Builds multi-arch binaries → GitHub Release assets  
2. Job `publish-registries` (Environment **`release`**, `id-token: write`):
   - crates.io via `rust-lang/crates-io-auth-action` → short-lived `CARGO_REGISTRY_TOKEN`
   - `mcp-dist-post --dry-run` (lockstep + mcpName)
   - `npm publish --access public` with **no** `NODE_AUTH_TOKEN` (npm Trusted Publishing)

`workflow_dispatch` rebuilds **assets only** — it does **not** re-publish registries.

### After OIDC is proven

1. Confirm crates.io version shows **VIA GITHUB**  
2. Confirm npm published without a classic token  
3. Remove any remaining `CARGO_REGISTRY_TOKEN` / `NPM_TOKEN` / `NODE_AUTH_TOKEN` repo secrets  
4. MCP Registry remains `mcp-publisher publish server.json` (separate auth — not OIDC here)

### What this post-step still does

`mcp-dist-post` never publishes. It only enforces lockstep / mcpName and writes dual `server.json` for the Registry handoff.

---

## Explicit non-goals

- Does not run `cargo publish` or `npm publish`
- Does not generate the platform matrix or binary download logic
- Does not touch the README `mcp-name:` token
- Does not decide public/private status
- Does not create GitHub tags or releases

---

## Phase 2 path

- **2A (this)** — documented recipe + small post-step script  
- **2B (later)** — promote into a thin CLI / skill once the recipe is proven on a second server

---

*Phase 2A · Rust-First · 2026-08-08*
