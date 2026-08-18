# BETTER / BEST — falsifiable checks

Companion to [`BETTER-BEST.md`](./BETTER-BEST.md) — the reusable check recipe, same shape used across every repo that follows the [agents.md](https://agents.md) NONE → GOOD → BETTER → BEST ladder (origin: `agents-md-facts`).

## BETTER baseline (`v0.4.3`)

```bash
git checkout v0.4.3
test ! -f project.faf   # product BETTER does not require project DNA
cargo test
```

## BEST print slice (`better-best/2026-08-18`)

```bash
git fetch origin
git diff v0.4.3..better-best/2026-08-18 --stat
git diff v0.4.3..better-best/2026-08-18 -- project.faf docs/BETTER-BEST.md docs/BETTER-BEST-CHECKS.md

# Optional worktrees (side-by-side)
# git worktree add ../mcp-better-better v0.4.3
# git worktree add ../mcp-better-best   better-best/2026-08-18
```

Or GitHub compare: `v0.4.3...better-best/2026-08-18`.

## Naming note

We do **not** call production slices "demos."
Evidence is **tags, SHAs, and diffs** of real software.

## Further reading

- [agents.md](https://agents.md) — the open standard
- [README](../README.md) — install, usage, protocol claims
- [BETTER.md](../BETTER.md) — this repo's claim-surface ladder
- Optional depth on facts-driven project DNA: [faf.one/agents](https://faf.one/agents)
