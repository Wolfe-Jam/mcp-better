# Shipping a dual-package Rust MCP server

**Audience:** Rust developers who already have a working MCP **stdio** server and want hosts to start it with **no Rust toolchain**.  
**Status:** Public guide · 2026-08-08 · from lived n=2  
**Canonical public path:** [`mcp-better/docs/DUAL-PACKAGE-RUST-MCP.md`](https://github.com/Wolfe-Jam/mcp-better/blob/main/docs/DUAL-PACKAGE-RUST-MCP.md)  
**What this is not:** a new CLI, a protocol change, or a claim that cargo-only publish is invalid.

Lived examples (public):

| Example | Registry name | Version | crates + npm |
|---------|---------------|---------|--------------|
| Textbook | [`io.github.Wolfe-Jam/mcp-better`](https://github.com/Wolfe-Jam/mcp-better) | 0.4.2 | `mcp-better` |
| Product | [`one.faf/rust-faf-mcp`](https://github.com/Wolfe-Jam/rust-faf-mcp) | 0.4.1 | `rust-faf-mcp` |

Recipe notes in-repo: [`mcp-better/docs/MCP-DIST-POST.md`](https://github.com/Wolfe-Jam/mcp-better/blob/main/docs/MCP-DIST-POST.md) · [`rust-faf-mcp/docs/TRUSTED-PUBLISHING-NPM.md`](https://github.com/Wolfe-Jam/rust-faf-mcp/blob/main/docs/TRUSTED-PUBLISHING-NPM.md)

---

## 0. One-screen summary

| Role | Surface | Why |
|------|---------|-----|
| **Provenance** | crates.io (`registryType: "cargo"`) | Versioned Rust artifact + ownership path the cargo validator expects |
| **Launch bridge** | npm (`registryType: "npm"`) | Clients already know `npx <name>` |

**Dual-package** means:

1. The **same version** is published to crates.io **and** npm  
2. GitHub Releases hold **multi-arch binaries** the npm package can download  
3. `server.json` declares **both** packages for that version  

### Honest boundary

| True | Not a claim |
|------|-------------|
| Node is used **to fetch** the binary | “No runtime at all” |
| The **running** process is a native binary | “Rust runs on Node” |
| Dual-package is **optional** | “Cargo alone is invalid” |
| The pattern is portable | “Every cargo server must use npm” |

**When you need this:** hosts without Rust, or clients that resolve servers via `npx`.  
**When cargo-only is enough:** your users all run `cargo install` (or ship another launch path).

---

## 1. Prerequisites

If you only need **`cargo install` users**, stop after crates.io (and a normal cargo registry entry). **This guide is for the dual path** — npm launch bridge + dual `server.json`.

- Working Rust MCP server on **stdio**
- GitHub repository that can cut **Releases** with multi-arch binary assets  
- Ability to publish the crate on **crates.io**  
- **npm** account (turn on 2FA before Trusted Publishing)  
- **Registry identity** chosen up front:
  - `io.github.<owner>/<server>` — GitHub-backed namespace, **or**  
  - Domain DNS identity (example: `one.faf/<server>`) — publish via DNS login  

Tools you will use: `cargo`, `npm`, and [`mcp-publisher`](https://github.com/modelcontextprotocol/registry) (from registry releases). No special dual-package CLI is required.

**Skimmers:** full gate list is in [§10 Checklist](#10-checklist-copy-paste).

---

## 2. End-to-end order (do not reverse)

```
1. Lock three files to the same version
2. Put multi-arch binaries on a GitHub Release for that version
3. cargo publish  (or OIDC job)
4. npm publish    (or OIDC job)   ← package must exist before step 5
5. mcp-publisher publish server.json   ← dual cargo + npm entry
6. Smoke: npx <npm-name> with no Rust toolchain
```

**Why this order:** the registry checks that packages listed in `server.json` already exist at that version. Lived error when npm was missing:

```text
NPM package 'rust-faf-mcp' not found (status: 404)
```

**Why Release before smoke:** the npm package is a downloader. `npx` fails if the asset name/version is wrong even when crates.io is perfect.

---

## 3. Three-file lockstep

These three **versions** must match before you publish:

| File | Must agree |
|------|------------|
| `Cargo.toml` | `version` · crate `name` |
| `package.json` | same `version` · `"mcpName"` · `bin` entry for the shim |
| `server.json` | same `version` · **both** packages (cargo + npm) |

**Hard rule:** bump all three together. There is no cheap fix after crates.io or the registry bake a version.

### Manual check

```bash
# From repo root — expect one shared version string three times
grep -E '^version\s*=' Cargo.toml | head -1
node -p "require('./package.json').version"
node -p "require('./server.json').version"
```

### Optional helper

Some FAF repos ship `scripts/mcp-dist-post.sh`. It **never publishes**. It only checks lockstep / `mcpName` and can write dual `server.json`.

```bash
./scripts/mcp-dist-post.sh \
  --crate <crate-name> \
  --mcp-name <registry-name> \
  --dry-run
```

You can do the same checks by hand if you do not have that script.

---

## 4. Ownership markers

| Package type | Marker | Where | Rule |
|--------------|--------|--------|------|
| **cargo** | `mcp-name: <registry-name>` | **README.md body** | Must be **visible** markdown (a normal line or code span). An HTML comment alone is not enough. |
| **npm** | `"mcpName": "<registry-name>"` | **package.json** | Must be inside the published tarball. |

### Visible README example

```markdown
MCP Registry name: `mcp-name: io.github.YourUser/your-server`
```

### package.json fragment

```json
{
  "name": "your-server",
  "version": "1.2.3",
  "mcpName": "io.github.YourUser/your-server",
  "bin": {
    "your-server": "bin/your-server.js"
  }
}
```

The registry **rejects** dual publish when the npm package lacks `mcpName` (lived: fixed between mcp-better 0.4.0 and 0.4.1).

Live markers:

| Server | README `mcp-name` | npm `mcpName` |
|--------|-------------------|---------------|
| mcp-better | `io.github.Wolfe-Jam/mcp-better` | same |
| rust-faf-mcp | `one.faf/rust-faf-mcp` | same |

---

## 5. Binaries on GitHub Releases

The npm package downloads a **prebuilt** binary. You must:

1. Build for each OS/arch you claim (commonly darwin arm64/x64, linux gnu x64/arm64; Windows optional)  
2. Attach archives to the GitHub Release for tag `vX.Y.Z`  
3. Make the **shim’s expected names** match those assets  

### Naming is yours — document it

Two patterns that already work:

| Pattern | Example asset | Ship |
|---------|---------------|------|
| Version + triple | `rust-faf-mcp-0.4.1-x86_64-apple-darwin.tar.gz` | rust-faf-mcp |
| Release-matrix / cargo-dist | Multi-target archives from CI on `v*` tags | mcp-better |

`rust-faf-mcp`’s shim tries, in order:

1. `rust-faf-mcp-${VERSION}-${target}.tar.gz`  
2. `rust-faf-mcp-v${VERSION}-${target}.tar.gz`  
3. `rust-faf-mcp-${target}.tar.gz`  

**Rule:** change asset names and shim URL logic in the same PR. Drift breaks every `npx` user.

You may use **cargo-dist**, a hand-written release matrix, or any CI that uploads the same files. Dual-package does not require one CI layout.

---

## 6. npm package (thin shim)

This is **not** a Node rewrite of the MCP server.

Minimum behavior:

1. Resolve OS / CPU  
2. Download the matching GitHub Release asset for **this package version**  
3. Cache it, make it executable, **spawn** it with stdio inherited  

Also:

- Set `"mcpName"` before the first publish  
- Restrict `"files"` to the shim and helpers (never ship `target/`)  
- Prefer a tiny hand-rolled `bin/*.js` **or** a cargo-dist-generated installer — both are valid  

| Crate | npm name | `mcpName` | Shim entry |
|-------|----------|-----------|------------|
| mcp-better | `mcp-better` | `io.github.Wolfe-Jam/mcp-better` | `bin/mcp-better.js` |
| rust-faf-mcp | `rust-faf-mcp` | `one.faf/rust-faf-mcp` | `npm/rust-faf-mcp.js` |

---

## 7. Dual `server.json`

Minimal dual entry (stdio). Add description, repository, icons, etc. as you already do for single-package servers.

**Replace** `OWNER`, `SERVER`, and `X.Y.Z` with your registry identity, package name(s), and version. Field names (`registryType`, `identifier`, `transport`, …) stay as shown.

```json
{
  "name": "io.github.OWNER/SERVER",
  "description": "One-line description of the server.",
  "version": "X.Y.Z",
  "packages": [
    {
      "registryType": "cargo",
      "registryBaseUrl": "https://crates.io",
      "identifier": "SERVER",
      "version": "X.Y.Z",
      "transport": { "type": "stdio" }
    },
    {
      "registryType": "npm",
      "registryBaseUrl": "https://registry.npmjs.org",
      "identifier": "SERVER",
      "version": "X.Y.Z",
      "transport": { "type": "stdio" }
    }
  ]
}
```

`name` is the **registry identity** (not necessarily the crate name).  
`identifier` is the crates.io / npm package name.

Publish only after both packages exist:

```bash
# install mcp-publisher from modelcontextprotocol/registry releases
./mcp-publisher validate
# login depends on identity — see §8
./mcp-publisher publish
```

---

## 8. Bootstrap vs steady-state auth

| Surface | First time | Steady-state |
|---------|------------|--------------|
| **crates.io** | Classic API token if the crate name is brand-new | [Trusted Publishing](https://crates.io) (OIDC) — workflow filename + optional GitHub Environment must match exactly |
| **npm** | Often `npm publish --access public` with interactive 2FA / recovery code once (OIDC may return E404 until the **package name** exists) | [Trusted Publisher](https://docs.npmjs.com/trusted-publishers) on the package — GitHub org, repo, **exact** workflow filename, Environment |
| **MCP Registry** | `io.github.*` → GitHub device / OIDC-style publisher login · **domain** identity → `mcp-publisher login dns --domain <apex>` with the domain’s private key | Same |

All of that is **publisher-side**. Consumers only run `npx` or `cargo install`.

### npm prove-out

After Trusted Publisher is configured, re-running publish on an **already published** version fails with:

```text
You cannot publish over the previously published versions: X.Y.Z
```

That means **authentication worked**. Do not treat it as a broken package.

### DNS identity (sketch)

For a domain namespace (lived: `one.faf/*` on apex `faf.one`):

1. Public TXT on the domain: `v=MCPv1; k=ed25519; p=<public-key-base64>`  
2. Private key only in CI secret / local vault — never in git  
3. `mcp-publisher login dns --domain <apex> --private-key <64-hex>`  
4. `mcp-publisher publish`  

This is intentionally thinner than the OIDC tables above. For field-level DNS auth rules, follow the current **MCP Registry DNS authentication** docs in the [modelcontextprotocol/registry](https://github.com/modelcontextprotocol/registry) repository — keep the public TXT and private key as a matching pair.

---

## 9. Two worked examples

Same **pattern**, different **layout**. Do not force one workflow shape.

### A. Textbook — `mcp-better@0.4.2`

| | |
|--|--|
| Registry name | `io.github.Wolfe-Jam/mcp-better` |
| Packages | crates.io + npm `mcp-better` @ **0.4.2** |
| Registry auth | GitHub-linked `io.github.*` path |
| Binaries | cargo-dist targets; hand-rolled `bin/mcp-better.js` |
| CI shape | Primarily `release.yml` + Environment `release` (binaries + OIDC registries) |
| Character | Textbook / BETTER purity — dual-package without product identity constraints |

```bash
./scripts/mcp-dist-post.sh \
  --crate mcp-better \
  --mcp-name io.github.Wolfe-Jam/mcp-better \
  --dry-run
```

**Smoke:** `npx mcp-better` → native binary, no Rust toolchain.

### B. Product — `rust-faf-mcp@0.4.1`

| | |
|--|--|
| Registry name | `one.faf/rust-faf-mcp` |
| Packages | crates.io + npm `rust-faf-mcp` @ **0.4.1** |
| Registry auth | **DNS** for domain identity (`login dns --domain faf.one`) |
| Binaries | Hand release matrix; assets `rust-faf-mcp-${VERSION}-${target}.tar.gz` |
| CI shape | **Split:** `release.yml` · `publish-crate.yml` · `publish-npm.yml` · `publish-mcp-registry.yml` |
| Environments | `crates-io` + `npm` |
| Character | Product identity stays on `one.faf/*` (do not rewrite to `io.github/*` for “convenience”) |

```bash
./scripts/mcp-dist-post.sh \
  --crate rust-faf-mcp \
  --mcp-name one.faf/rust-faf-mcp \
  --dry-run
```

Lived publish order: crates OIDC → npm bootstrap (OTP once) → DNS registry publish.

**Smoke:** package bin / `npx` resolves `rust-faf-mcp@0.4.1` → downloads Release binary → MCP session.  
Implementation note for integrators: this server’s stdio framing is **NDJSON** (newline JSON-RPC), not Content-Length headers. That is server-specific, not a dual-package requirement.

---

## 10. Checklist (copy-paste)

- [ ] Stdio MCP server builds and runs locally  
- [ ] Stdio is NDJSON — validate: one `initialize` line in → one JSON line on stdout; nothing else on stdout  
- [ ] Registry identity chosen (`io.github.*` **or** DNS domain)  
- [ ] `Cargo.toml` · `package.json` · `server.json` versions match (**lockstep ×3**)  
- [ ] README has **visible** `mcp-name: <identity>`  
- [ ] `package.json` has `"mcpName": "<identity>"`  
- [ ] `server.json` lists cargo **and** npm packages at the same version  
- [ ] GitHub Release for `vX.Y.Z` has multi-arch assets  
- [ ] Shim URL logic matches asset names  
- [ ] `cargo publish --dry-run` (or equivalent CI)  
- [ ] Inspect npm pack / dry-run; confirm `mcpName` is in the tarball  
- [ ] **cargo** publish (or OIDC) green  
- [ ] **npm** publish (or OIDC) green — OTP/recovery code OK on first name create  
- [ ] `mcp-publisher validate` · login · **publish**  
- [ ] Smoke: `npx <npm-name>` (or package bin) **without** a Rust toolchain  
- [ ] (Steady-state) Trusted Publishing configured for crates + npm so the next version needs no classic tokens  
- [ ] (Optional) Score the native binary — [§11](#11-score-the-native-binary-optional)

---

## 11. Score the native binary (optional)

Dual-package is about **launch**. Quality is about the **binary**.

After you have a release build (or an installed crate binary), you can score the **running process** with [`wjttc`](https://www.npmjs.com/package/wjttc) — path-like files are spawned **directly** (not `npx -y ./path`):

```bash
npx wjttc certify --mcp ./target/release/your-server
npx wjttc certify --mcp /usr/local/bin/your-server
```

| Layer | Role |
|-------|------|
| **wjttc** | May run under Node |
| **Server under test** | Native process — path spawned directly, not `npx -y ./path` |

Still supported (npm launch path):

```bash
npx wjttc certify --mcp "npx -y your-server"
```

Optional. No registry change. No extra CLI beyond `wjttc`.  
See also: [wjttc Quick Start](https://github.com/Wolfe-Jam/wjttc/blob/main/README.md#quick-start) (native binary examples).

---

## 12. Stdio wire hygiene

MCP stdio uses **newline-delimited JSON-RPC** (one message per line). That is the standard binding — not LSP-style `Content-Length` headers.

| Channel | Use |
|---------|-----|
| **stdout** | MCP messages only |
| **stderr** | Logs and tracing |

Do not write banners, `println!` debug lines, or progress text to stdout. Hosts and the certification bar read lines as JSON-RPC; anything else breaks the session.

`rmcp`’s stdio transport and this dual-package path assume NDJSON. Prefer `\n` (LF) on the wire.

Optional smoke:

```bash
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2026-07-28","capabilities":{},"clientInfo":{"name":"probe","version":"0"}}}' \
  | ./target/release/your-server
```

Expect a single JSON line on stdout. This check is independent of publishing to npm or the registry.

---

## 13. What this is not

- Not a claim that cargo-only registry entries are wrong  
- Not a requirement that every Rust MCP author publish to npm  
- Not a change to the MCP protocol or registry schema  
- Not volume advice (“more cargo servers”)  
- Not advice to abandon a domain identity for `io.github.*`  
- Not a promise of a single CI template for every repo  

---

## One-line

> Publish the native binary to crates.io and GitHub Releases; publish a thin npm downloader with `mcpName`; keep three versions locked; publish cargo → npm → dual registry. **Node fetches; Rust runs.** Stdio is NDJSON (stdout pure). Optional: score the binary with `wjttc certify --mcp ./path`.

---

*Phase 3 public guide · teach loop + wire hygiene · canonical: [mcp-better/docs/DUAL-PACKAGE-RUST-MCP.md](https://github.com/Wolfe-Jam/mcp-better/blob/main/docs/DUAL-PACKAGE-RUST-MCP.md) · 2026-08-08*
