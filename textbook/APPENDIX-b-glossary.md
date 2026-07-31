# Appendix B — Glossary

**Status:** SOLID  
**Rule:** Prefer these terms in all MCP textbook prose and extracts.

---

| Term | Definition |
|------|------------|
| **7/28** | Human era name for the modern MCP release. |
| **`2026-07-28`** | Wire / SDK protocol version string for that release. |
| **AAIF** | Agentic AI Foundation (Linux Foundation). Public teaching lane for BETTER. |
| **BETTER** | Honest modern MCP peer: claim ≤ proof; Discover; stamps; tested transports. |
| **book = app** | Identity doctrine: textbook and binary are one artifact; versions are lessons. |
| **BEST** | Persistent project / agent context beyond protocol literacy; hop to faf.one/agents. |
| **cacheScope** | List-cache field: `public` or `private`. |
| **Claim surface** | Everything you assert in docs, blurbs, blogs, Registry text. |
| **Discover** | Modern lifecycle entry (`server/discover`); not legacy initialize-only. |
| **GOOD** | Real MCP with value; often session-era operational habits. |
| **Handle** | Explicit app-state token passed as tool args after sessions left the core. |
| **InterOp** | Behaving as a 7/28 peer under a modern client — not merely sharing tool names. |
| **mcp-better** | Runnable Rust textbook server (health + echo) for 7/28. |
| **MRTR** | Mid-request tool result pattern (`input_required` / `complete` via `resultType`). |
| **NONE** | No MCP (or non-protocol glue only). |
| **Proof surface** | Tests, smokes, and checks that falsify claims when broken. |
| **Road** | Additional transport or demo path **inside** the same protocol era. |
| **Season 1** | Core operational contract chapters (00–10 + appendices). |
| **Season 2** | Host/matrix/remote chapters after Season 1 soak. |
| **Stateless core** | Protocol not centered on session identity; request/response self-description. |
| **Stamped list** | List result with intentional `ttlMs` + `cacheScope`. |
| **Streamable HTTP** | Modern HTTP transport for MCP; requires routing headers when used. |
| **ttlMs** | List-cache time-to-live in milliseconds. |
| **UnsupportedProtocolVersion** | Clear rejection when peer versions cannot negotiate. |

---

## Phrases to avoid

| Avoid | Prefer |
|-------|--------|
| “Full MCP support” | Named capabilities + version |
| “Enterprise remote” (for loopback demos) | “Local HTTP demo · no auth” |
| “Guaranteed InterOp” | “Passes these checks” (no guarantee language) |
| “Above 7/28” / new era for minor versions | “More road inside 7/28” |

---

## Back

→ [README](./README.md)
