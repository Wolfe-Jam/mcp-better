# PUBBETTER — how this repo ships

Canonical skill: **`/pubbetter`** → `~/.claude/skills/pubbetter/SKILL.md`

This doc is the human short form. Agents run the skill end-to-end.

## Ship bar (local ≡ CI)

```bash
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/ci.sh
```

| Gate | What |
|------|------|
| fmt | `cargo fmt --check` |
| clippy | `-D warnings` |
| test | unit + integration |
| purity | no `project.faf` |
| smoke | Discover client · stamped list · health · echo |

## Full publish motion (atomic)

```text
bump versions
  → doc-gate + README/CHANGELOG
  → scripts/ci.sh
  → push main · GitHub CI green
  → cargo publish --dry-run
  → GO! from wolfejam
  → cargo publish
  → git tag v* + gh release
  → mcp-publisher publish server.json
  → release-verify truth-table
```

**Never** tag without publishing in the same motion.  
**Never** use `one.faf/*` — Registry name is `io.github.Wolfe-Jam/mcp-better`.  
**Never** require FAF Trophy / `project.faf` to ship (BETTER purity).

## Dry-run

```bash
cargo publish --dry-run
```

Mandatory before GO! and again if any commit landed after the first dry-run.

## Identity

| Surface | Value |
|---------|--------|
| Crate | `mcp-better` |
| Registry | `io.github.Wolfe-Jam/mcp-better` |
| Visible README token | `mcp-name: io.github.Wolfe-Jam/mcp-better` |

## Related

- `/pubcrate` — other FAF crates  
- `/pubpro` — FAF npm MCP servers  
- `/pubbetter` — **this** product only  
