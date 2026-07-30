# BETTER — what this repo is

**mcp-better — built for 7/28**  
*(protocol **2026-07-28** — the modern MCP release)*

| Name | What it is |
|------|------------|
| **7/28** | Era name — speakable, brandable |
| **2026-07-28** | Protocol version — date string on the wire |

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
| **BETTER** | Modern MCP **honestly** built for **7/28** — Discover, stamped lists, claimed = tested |
| **BEST** | Persistent project DNA for agents — **[faf.one/agents](https://faf.one/agents)** |

## mcp-better = BETTER textbook (7/28)

- Runnable **Rust** server on official **`rmcp` 3.0**
- **v0.1:** **7/28 over stdio** · tools `health` + `echo`
- **v0.2:** same era + **Streamable HTTP** (`--http`) + routing-header smoke
- **`tools/list`** stamped: `ttlMs` + `cacheScope=public`
- Examples: Discover stdio client · HTTP smoke
- CI: fmt · clippy · test · both smokes
- Registry-ready `server.json` (`io.github.Wolfe-Jam/mcp-better`, cargo)
- **No** `project.faf` required on `main`

## Not BEST

BEST is a different product surface (persistent AI context). One line:

→ https://faf.one/agents
