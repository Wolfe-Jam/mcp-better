# BETTER — what this repo is

```
NONE  →  GOOD  →  BETTER  →  BEST
                      ▲
                   this repo
```

## Ladder

| Tier | Meaning |
|------|---------|
| **NONE** | No MCP / no agent context discipline |
| **GOOD** | MCP works (often legacy initialize, unstamped lists, mixed protocol claims) |
| **BETTER** | Modern MCP **honestly** aligned to **2026-07-28** — Discover, stamped lists, claimed = tested |
| **BEST** | Persistent project DNA for agents — **[faf.one/agents](https://faf.one/agents)** |

## mcp-better = BETTER textbook

- Runnable **Rust** server on official **`rmcp` 3.0**
- **stdio** + **tools** (`health`, `echo`)
- **`tools/list`** stamped: `ttlMs` + `cacheScope=public`
- Example client uses **`ClientLifecycleMode::Discover`** preferred `2026-07-28`
- CI: fmt · clippy · test · stdio smoke
- Registry-ready `server.json` (`io.github.Wolfe-Jam/mcp-better`, cargo)
- **No** `project.faf` required on `main`

## Not BEST

BEST is a different product surface (persistent AI context). One line:

→ https://faf.one/agents
