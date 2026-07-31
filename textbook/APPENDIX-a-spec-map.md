# Appendix A — Spec map (7/28 → BETTER)

**Status:** SOLID  
**Source basis:** MCP `2026-07-28` changelog + alignment study (2026-07-28)  
**Authority:** If this appendix drifts from the [official spec](https://modelcontextprotocol.io/specification/2026-07-28), **fix this appendix**.

---

## Legend (mcp-better / Season 1 textbook)

| Tag | Meaning |
|-----|---------|
| **MUST** | Required to claim BETTER / `2026-07-28` honesty for a tools peer |
| **SHOULD** | Default; document if omitted |
| **LATER** | Real protocol; not Season 1 companion hero |
| **AVOID** | Deprecated or wrong for new greenfield impl |

---

## Sessions & handshake

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| No protocol session id as identity | **MUST** not invent session stickiness as protocol identity |
| No initialize-required “connected” | **MUST** not require init for readiness |
| Self-describing requests (`_meta`, etc.) | **MUST** use SDK paths that emit required metadata |
| `server/discover` | **MUST** implement (SDK may provide) |
| App state via explicit handles | **MUST** document if multi-call state exists; Season 1 companion has none |

---

## List cache

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| `ttlMs` + `cacheScope` on list results | **MUST** for tools/list under BETTER claims |
| Deterministic tool order | **MUST** for BETTER pedagogy |
| `public` for static catalogs | **SHOULD** |

---

## Streamable HTTP

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| `Mcp-Method` on POST; `Mcp-Name` when naming a tool/resource | **MUST** when HTTP is claimed |
| Protocol version on requests (`MCP-Protocol-Version` / `_meta` as required) | **MUST** document; happy-path smoke should send them |
| Production auth | **LATER** / out of Season 1 companion |
| Local unauthenticated demo | Allowed if **SECURITY** is honest |

---

## MRTR

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| `resultType` on results | **MUST** (`complete` for ordinary tools) |
| `input_required` flows | **LATER** for rich tools |

---

## Tasks

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| Tasks as extension | **LATER** · never core hero |

---

## Authorization

| Spec direction | Textbook / mcp-better |
|----------------|------------------------|
| CIMD preferred over DCR for modern remote | Document when remote auth is taught |
| stdio local without OAuth | **OK** for Season 1 |
| Fake OAuth | **AVOID** |

---

## Deprecations (new impl AVOID)

| Feature | Migration hint |
|---------|----------------|
| **Roots** | Tool params / URIs / config |
| **Sampling** | Direct LLM APIs |
| **Protocol Logging** feature | stderr / OpenTelemetry |
| **HTTP+SSE** legacy transport | Streamable HTTP |
| **DCR** as primary registration story | CIMD |
| Protocol **`ping`** | App-level `health` tool if needed |

Deprecation windows are defined by the lifecycle policy in the official docs (minimum window language there wins).

---

## Not in core protocol (BEST note)

Project DNA / persistent agent context / memory formats are **not** supplied by MCP `2026-07-28` as a standard context file format. That gap is why **BEST** is a separate hop ([faf.one/agents](https://faf.one/agents)), not a missing MCP method in Season 1.

---

## Back

→ [README](./README.md) · [Glossary](./APPENDIX-b-glossary.md)
