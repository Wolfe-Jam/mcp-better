# MCP Dual-Package Post-Step (Phase 2A)

Thin companion to **cargo-dist** and **`/pubcrate`**.

It does **not** publish to crates.io or npm.  
It does **not** replace `/pubcrate`.  
It only owns the MCP-specific dual-package surface.

**Receipt (2026-08-08):** dual-package + OIDC steady-state proven on **`mcp-better@0.4.2`**.  
Post-ship brief: [`GB-BRIEF-POST-0.4.2.md`](./GB-BRIEF-POST-0.4.2.md).

---

## Where it sits

### Steady-state (mcp-better after 0.4.2)

```
tag vX.Y.Z on main
  → .github/workflows/release.yml
       · build multi-arch binaries → GitHub Release assets
       · publish-registries (Environment release, OIDC)
           cargo publish via crates-io-auth-action
           mcp-dist-post --dry-run (CI lockstep check)
           npm publish (Trusted Publishing, no NODE_AUTH_TOKEN)
  → scripts/mcp-dist-post.sh   (local: write dual server.json)
  → mcp-publisher publish server.json
```

### Human quality path (still valid)

```
/pubcrate or local gates     → dry-run reviewed → GO
cargo/npm via OIDC on tag    → crates.io + npm live
scripts/mcp-dist-post.sh     ← this step (real run)
mcp-publisher publish server.json
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
   Registry **rejects** npm packages that lack `mcpName` (lived 0.4.0 → fixed 0.4.1).

3. **Dual-package `server.json`**  
   Writes / updates a `server.json` that declares **both**:
   - `registryType: "cargo"`
   - `registryType: "npm"`  
   under the same server identity, with matching versions and `transport: stdio` by default.

4. **Optional paste-ready block**  
   With `--print-mcp-json` it prints a ready-to-paste `mcpServers` snippet.

---

## Prerequisites

- `Cargo.toml` and `package.json` already exist and share the same version.
- The npm `package.json` already contains the correct `mcpName` field  
  (add it **before** `npm publish` — the Registry rejects packages that lack it).
- `python3` available (used for JSON read/write).
- For OIDC CI: Trusted Publishing configured on crates.io + npm for this repo; GitHub Environment `release`.

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

## OIDC Trusted Publishing (lived path)

Goal: **zero long-lived registry tokens** after bootstrap.  
Brief: [`OIDC-TRUSTED-PUBLISHING-BRIEF.md`](./OIDC-TRUSTED-PUBLISHING-BRIEF.md).

### Proven on mcp-better@0.4.2

| Gate | Result |
|------|--------|
| crates.io | OIDC · `trustpub_data.provider=github` (VIA GITHUB) |
| npm | Trusted Publishing · no CI token · `mcpName` present |
| MCP Registry | dual cargo + npm @ 0.4.2 |
| Repo classic secrets | **none** — scrub not required |

### Proven on rust-faf-mcp@0.4.1 (second-server · 2A-recipe closed)

| Gate | Result |
|------|--------|
| crates.io | OIDC · `publish-crate.yml` · env `crates-io` |
| npm | bootstrap OTP once · Trusted Publisher proven · `mcpName: one.faf/rust-faf-mcp` |
| MCP Registry | dual cargo + npm @ 0.4.1 · DNS login (`FAF_ONE_MCP_PRIVATE_KEY`) |
| Identity | **`one.faf/*`** (not `io.github`) |
| Workflows | split surface (not single `release.yml`) — see recipe deltas |

### Bootstrap (brand-new crate / brand-new npm name)

- First crates.io publish of a **new** crate still needs a classic token once.
- Then configure Trusted Publishing (workflow filename + Environment must match exactly).
- **New npm package name:** first publish is often **human OTP / recovery code** (`npm publish --access public --otp=…`) when OIDC returns E404 “package not found.” Then add Trusted Publisher on npmjs.com.
- Registry dual publish **requires** both package registries to already host that version (npm 404 → registry 400).
- Do **not** delete classic tokens until a successful OIDC dual publish.

### Recipe deltas (mcp-better → rust-faf-mcp)

| Area | mcp-better | rust-faf-mcp (portable pattern) |
|------|------------|----------------------------------|
| Release jobs | one `release.yml` + env `release` | `release.yml` (binaries) + `publish-crate.yml` + `publish-npm.yml` + `publish-mcp-registry.yml` |
| GH Environments | `release` | `crates-io` + `npm` |
| Registry auth | GitHub OIDC for `io.github…` | **DNS** for `one.faf/*` (`login dns --domain faf.one`) |
| Order | cargo/npm then registry | **npm must exist before** dual `server.json` registry publish |

`mcp-dist-post` still never publishes — lockstep + dual `server.json` only.
### Steady-state (this repo)

On `v*` tag push, `.github/workflows/release.yml`:

1. Multi-arch binaries → GitHub Release assets  
2. Job **`publish-registries`** (Environment **`release`**, `id-token: write`):
   - crates.io via `rust-lang/crates-io-auth-action` → short-lived token only  
   - `mcp-dist-post --dry-run` in CI  
   - `npm publish --access public` with **no** `NODE_AUTH_TOKEN`

`workflow_dispatch` rebuilds **assets only** — no registry re-publish.

### Lived CI lessons (0.4.2)

1. **SHA-pinning `dtolnay/rust-toolchain`:** floating `@stable` implies the toolchain; a **commit SHA does not**. Always pass:
   ```yaml
   with:
     toolchain: stable
   ```
2. **Gate scripts:** do **not** pipe long steps through `| tail` (`cargo doc` looked “stuck” while still running).
3. **Tag retarget** only when nothing has published yet for that version (we retargeted `v0.4.2` after the toolchain fix).

### What this post-step still does

`mcp-dist-post` never publishes. It only enforces lockstep / mcpName and writes dual `server.json` for the Registry handoff.

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

## Explicit non-goals

- Does not run `cargo publish` or `npm publish` (CI OIDC jobs do that)
- Does not generate the platform matrix or binary download logic
- Does not touch the README `mcp-name:` token
- Does not decide public/private status
- Does not create GitHub tags or releases
- Does not rewrite `/pubcrate`

---

## Phase 2 path

- **2A-flagship** — closed on `mcp-better@0.4.2` (dual-package + OIDC)  
- **2A-recipe** — open until a **second** FAF Rust MCP server uses the same path  
- **2B** — thin CLI/skill only after second-server friction is observed  

---

*Phase 2A · Rust-First · lived 2026-08-08 (mcp-better@0.4.2)*
