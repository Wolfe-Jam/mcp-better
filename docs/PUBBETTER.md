# PUBBETTER — how this repo ships

This is the release protocol in short form.

## Ship bar (local ≡ CI)

```bash
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/ci.sh
```

| Gate | What |
|------|------|
| lockstep | `Cargo.toml` · `package.json` · `server.json` = same version |
| fmt | `cargo fmt --check` |
| clippy | `-D warnings` |
| test | unit + integration |
| purity | no `project.faf` |
| smoke | stdio Discover client · mrtr-client · order-restart · contrast · Streamable HTTP |
| conformance | official MCP conformance suite: textbook binary vs `conformance-baseline.yml`; conformance build with no baseline |
| preflight | `rust-mcp-preflight` on `server.json` (Registry listing) |
| msrv | `cargo check` on Rust 1.88 (CI only) |
| cargo-dist | `dist-workspace.toml` present |

## Full publish motion (atomic · dual package)

```text
bump to X.Y.Z (Cargo.toml · package.json · server.json · CHANGELOG · README)
  → doc-gate (three-file lockstep)
  → scripts/ci.sh
  → push main · GitHub CI green
  → cargo publish --dry-run
  → npm publish --dry-run
  → GO! from wolfejam
  → git tag vX.Y.Z + push the tag
      Release workflow (.github/workflows/release.yml), on the tag:
        build → platform binaries attached to the GitHub Release
        test  → cargo test --locked --all-targets on the tagged commit
        publish-registries (needs build + test, OIDC, no tokens):
          cargo publish --locked → npm publish
  → mcp-publisher publish server.json   # cargo + npm packages
  → release-verify (incl. npx zero-toolchain)
```

**Never** tag without publishing in the same motion.  
**Never** use `one.faf/*` — Registry name is `io.github.Wolfe-Jam/mcp-better`.  
**Never** require FAF Trophy / `project.faf` to ship (BETTER purity).  
**Never** ship with version drift across the three lockstep files.

## Dry-run

```bash
cargo publish --dry-run
npm publish --dry-run
```

Mandatory before GO! and again if any commit landed after the first dry-run.

## Identity

| Surface | Value |
|---------|--------|
| Crate | `mcp-better` |
| npm | `mcp-better` |
| Registry | `io.github.Wolfe-Jam/mcp-better` |
| Visible README token | `mcp-name: io.github.Wolfe-Jam/mcp-better` |
| Packages | cargo + npm (both stdio) |

## Zero-toolchain gate

After npm publish + GH Release binaries:

```bash
npx mcp-better --help
# must print help without requiring rustc/cargo on PATH
```

## Related

- `/pubcrate` — other FAF crates  
- `/pubpro` — FAF npm MCP servers  
- `/pubbetter` — **this** product only  
