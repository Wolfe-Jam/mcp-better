# 01 — Era & names

**Status:** SOLID  
**Read time:** ~6 minutes  
**Depends on:** [00 Preface](./00-preface.md)  
**Feeds:** every chapter that mentions “7/28”

---

## Two names, one release

| Name | Kind | Use when |
|------|------|----------|
| **7/28** | Era name (human) | Speech, titles, teaching: “built for 7/28” |
| **`2026-07-28`** | Protocol version (machine) | Wire, SDKs, negotiation, logs, README protocol fields |

Humans say **7/28**.  
Machines negotiate **`2026-07-28`**.

Do not invent a third brand for the same release (“728” as slang is optional and means the same era; it is not a second protocol).

---

## What “era” does *not* mean

| Wrong | Right |
|-------|--------|
| Each product version is a new protocol era | Product versions add **road** inside the same era |
| “v0.2 is more 7/28 than v0.1” | v0.1 and v0.2 can both be honest 7/28; v0.2 may add HTTP *road* |
| “Modern” without a date | Name the wire version you implement and test |
| README era ahead of SDK pin | Pin the SDK that speaks the era you claim |

**mcp-better** example (fact, not marketing):

| Product | Era | Road |
|---------|-----|------|
| **v0.1** | 7/28 | stdio foundation: tools, Discover path, stamped list |
| **v0.2** | **same** 7/28 | + Streamable HTTP + routing-header smoke |

The era string on the wire does not change because HTTP shipped.

---

## Prior line (context only)

The previous stable line commonly referenced in migration talk is **`2025-11-25`**. This book does not re-litigate that release. It only needs you to know:

- Old samples often teach **initialize / session** muscle memory.  
- 7/28 makes **request/response + Discover** the core story.  
- Deprecations (Roots, Sampling, protocol Logging as greenfield features, etc.) have a documented window — see Appendix A.

Official repo READMEs or schema path examples can lag (they have pointed at older schema folders after 7/28 shipped). **Version authority is the specification site and the dated release tag**, not a stale README path.

---

## Naming discipline in prose

| Prefer | Avoid |
|--------|--------|
| “protocol `2026-07-28`” | “latest MCP” without date |
| “7/28 peer” | “full MCP” (undefined) |
| “Streamable HTTP (local demo)” when unauthenticated | “production remote MCP” for loopback demos |
| “Discover-compatible” | “supports MCP” alone |

---

## Check (self-test)

1. Write one sentence that uses **7/28** correctly for a human audience.  
2. Write one sentence that uses **`2026-07-28`** correctly for a wire/log audience.  
3. Explain why adding HTTP to a server is not, by itself, a new protocol era.

---

## Next

→ [02 — The ladder](./02-ladder.md)
