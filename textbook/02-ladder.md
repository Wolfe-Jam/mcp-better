# 02 — The ladder

**Status:** SOLID  
**Read time:** ~8 minutes  
**Depends on:** [01 Era & names](./01-era-and-names.md)  
**Feeds:** [03 InterOp](./03-interop.md), [08 Claim = wire](./08-claim-equals-wire.md)

---

## The shape

```text
NONE  →  GOOD  →  BETTER  →  BEST
```

This is not a slogan. It is the **product shape** of the teaching lane:

| Rung | Meaning | Where it lives |
|------|---------|----------------|
| **NONE** | No MCP (or non-protocol tool glue only) | Named in docs |
| **GOOD** | Real MCP — tools work — often with session-era operational habits | Common in the wild; migration source |
| **BETTER** | Honest modern peer for **`2026-07-28`**: Discover, stamps, stable order, claimed transports tested | **This book · `mcp-better` · AAIF surface** |
| **BEST** | Persistent project DNA / agent context beyond protocol literacy | **[faf.one/agents](https://faf.one/agents)** — hop up, not required |

---

## GOOD is not an insult

**GOOD** means: you shipped a server people use. Tools run. Hosts attach. Value exists.

GOOD often includes habits the old world taught:

- session id as “we’re connected”  
- `initialize` as the main lifecycle event  
- unstamped `tools/list` (works, but no cache contract)  
- host-lucky behavior (works in one Desktop config)  
- banner claims ahead of wire proof  

Call those **session-era operational assumptions**, not moral failure.

---

## BETTER is not “more tools”

**BETTER** means:

```text
claim surface  ≤  proof surface
```

Concretely for a **tools** server in Season 1:

| BETTER requires | BETTER does not require on day one |
|-----------------|-------------------------------------|
| Honest protocol version | Forty domain tools |
| Discover-compatible behavior | Resources + prompts as first-class |
| Stamped list results where list cache applies | OAuth / CIMD production |
| Stable tool order | Tasks extension |
| Documented transports only if implemented | MRTR demos |
| CI/smoke that fails when you lie | Being listed on every host directory |

**BETTER is protocol honesty under a modern client.**

---

## BEST is a different product surface

BEST answers: *does the agent have durable project context?*

That is valuable. It is also **not** the definition of a correct MCP peer.

| Surface | Message |
|---------|---------|
| **AAIF / this book** | Learn BETTER. No FAF install required. |
| **faf.one/agents** | When you want BEST, start here. |
| **mcp-better main** | No `project.faf` required to clone or score the textbook. |

Do not collapse BETTER into BEST. Do not bury BEST with no path. One hop is enough.

---

## If MCP had an AGENTS.md

Recurring device (steal for every server README):

```markdown
## Operational contract (7/28)
- Protocol: 2026-07-28
- Lifecycle: Discover-compatible (not initialize-only)
- tools/list: ttlMs + cacheScope + stable order
- Transports: stdio (default) · Streamable HTTP (if enabled — document bind + auth posture)
- Non-goals: (resources / OAuth / Tasks — say so if out of scope)
- Prove it: (link CI / smoke that fails when you lie)
```

Short. Specific. Verifiable. Same discipline as a good AGENTS.md: **facts the peer can check**, not vibes.

---

## Anti-patterns on the ladder

| Anti-pattern | Why it fails |
|--------------|--------------|
| Skipping GOOD language and shaming operators | Loses the migration audience |
| Calling a megaserver BETTER because it has features | Features ≠ era honesty |
| Requiring BEST to “finish” the textbook | AAIF surface must stand alone |
| Using ladder as marketing tiers for paid plans | Corrupts the audit language |

---

## Check (self-test)

1. Place a server that works only via legacy initialize and unstamped lists: which rung?  
2. Place `mcp-better` 0.2 with Discover smoke green: which rung?  
3. Place a perfect 7/28 tools server with no project DNA file: is it incomplete BETTER, or simply not BEST?

---

## Next

→ [03 — InterOp](./03-interop.md)
