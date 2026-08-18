# NONE · GOOD · BETTER · BEST

How this project sits on the [AGENTS.md](https://agents.md) ladder — short version for contributors and reviewers.

## The model

```text
NONE  →  GOOD  →  BETTER  →  BEST
```

| State | Meaning |
|-------|---------|
| **NONE** | No agent instruction file |
| **GOOD** | Some `AGENTS.md` (or peer file) exists — quality varies |
| **BETTER** | A short, current, **facts-based** `AGENTS.md` an agent can trust |
| **BEST** | BETTER **plus** durable project DNA (`.faf`) that authors/refreshes instruction files from verified facts |

This repository is intentionally a **real protocol textbook**, not a toy fixture — see [`README.md`](../README.md) and [`BETTER.md`](../BETTER.md).

## This repo

| State | Where |
|-------|--------|
| **BETTER** | **`main`** and release tags (e.g. `v0.4.3`) — no `project.faf` |
| **BEST** | Branch/tag **`better-best/2026-08-18`** — same product software **plus** `project.faf` (falsifiable git diff) |

**Product default = BETTER.** You do not need FAF (or any other stack) to install or run `mcp-better`.

```bash
cargo install mcp-better --version 0.4.3   # or: npx mcp-better (no Rust toolchain)
mcp-better --help
```

## Why BETTER matters

Official SDK defaults often ship `tools/list` results without cache stamps (`ttlMs` / `cacheScope`) and without Discover-lifecycle support — honest on the wire, but not modern. `mcp-better` claims only what it implements and tests for the **2026-07-28** protocol era: stdio + Streamable HTTP transports, stamped list cache, stable tool order, `confirm_echo` MRTR (SEP-2322). Nothing claimed, nothing invented — see [`docs/SDK-NOTES.md`](./SDK-NOTES.md)'s claim-surface-by-version table.

## Falsifiable checks

**BETTER baseline (`v0.4.3`):**

```bash
git checkout v0.4.3
test ! -f project.faf   # product BETTER does not require project DNA
cargo test
```

**BEST print slice (`better-best/2026-08-18`):**

```bash
git fetch origin
git diff v0.4.3..better-best/2026-08-18 --stat
git diff v0.4.3..better-best/2026-08-18 -- project.faf docs/BETTER-BEST.md

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
- [BETTER.md](../BETTER.md) — the claim-surface ladder this repo is built for
- Optional depth on facts-driven project DNA: [faf.one/agents](https://faf.one/agents)
