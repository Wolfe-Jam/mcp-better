# 03 — InterOp

**Status:** SOLID  
**Read time:** ~12 minutes  
**Depends on:** [02 Ladder](./02-ladder.md)  
**Feeds:** Chapters 04–08, lab 09  
**Public extract:** [Part I on dev.to](https://dev.to/wolfejam/not-all-mcp-servers-are-equal-what-728-just-made-official-2f29)

---

## Definition

**InterOp is not:** does `tools/list` return a string called `echo`?

**InterOp is:**

> Does this server behave like a **7/28 peer** under a **modern client**?

Same tool *name* across hosts does not imply same *operational* machine.

---

## Why names lie

A server can expose `health` on three hosts and still be three different machines:

| Dimension | Host A | Host B | Host C |
|-----------|--------|--------|--------|
| Lifecycle | Discover | initialize-only | mixed |
| List stamps | yes | no | partial |
| Tool order | stable | reshuffled | stable |
| HTTP headers | complete | missing | N/A (stdio) |
| Session assumptions | none | sticky id required | hidden |

Users report: “MCP is flaky.”  
Often: **the operational surface differed** while the catalog name stayed pretty.

---

## The InterOp questions (audit card)

Print this. Use it on every server review.

| # | Question | Why it bites |
|---|----------|----------------|
| 1 | Does the client use **Discover**, or only legacy `initialize`? | Handshake-as-identity is GOOD-era. 7/28 is request/response + `server/discover`. |
| 2 | Does `tools/list` return **`ttlMs` + `cacheScope`**? | List cache is part of the modern contract for BETTER claims — not optional polish. |
| 3 | Is tool order **stable** across repeated list calls (and ideally restarts for a static catalog)? | Prompt-cache and client caching assume determinism. |
| 4 | On Streamable HTTP: is **`Mcp-Method`** present (and **`Mcp-Name`** when the method names a tool/resource)? | Routing without full body parse — required shape for the transport (SEP-2243). |
| 5 | Does the server invent **session stickiness** as protocol identity? | Protocol sessions are gone. App state = explicit handles if anything. |
| 6 | Does **claimed** protocol version / transport match **tested** wire behavior? | Banner ≠ peer. |
| 7 | Are **deprecated greenfield** features sold as modern core (Roots, Sampling, protocol Logging, …)? | New impl should avoid; see Appendix A. |

If two servers both advertise `echo` but only one answers these honestly, they are **not** interchangeable.

---

## GOOD habits (pre-7/28 muscle memory)

None of this means “you’re bad.” It was the world many samples taught.

| Habit | What it looked like |
|-------|---------------------|
| **Session as identity** | Sticky `Mcp-Session-Id`; “connected” means long-lived peer |
| **Handshake as the event** | `initialize` / `initialized` as the main lifecycle story |
| **Unstamped lists** | `tools/list` works; no cache hints; clients re-poll forever |
| **Host-lucky InterOp** | Works in one Desktop config; mystery failure in another |
| **Banner claims** | README says “modern MCP”; wire is still prior-era shaped |

Call that **GOOD**: real MCP, real tools, real value — with **session-era operational assumptions**.

---

## BETTER checklist (auditable)

Humans say **7/28**. Machines negotiate **`2026-07-28`**.

| BETTER check | Spec-shaped meaning |
|--------------|---------------------|
| **Stateless core** | No protocol session; any request can hit any healthy instance (HTTP) |
| **Discover** | Clients prefer Discover / Auto → 7/28; servers answer discover correctly |
| **Self-describing traffic** | Version + capabilities travel with the request (`_meta` / headers) |
| **Stamped lists** | Positive `ttlMs`, intentional `cacheScope` (`public` for static catalogs) |
| **Stable tool order** | Same process, same order across N list calls |
| **HTTP road (if claimed)** | Streamable HTTP + routing headers (`Mcp-Method` / `Mcp-Name` as required); no fake “remote-prod” without auth story |
| **Honest claim surface** | Docs + CI prove claims; no deprecated core as greenfield features |
| **MRTR / Tasks / OAuth** | Real, but **not** required to be a BETTER *tools* textbook on day one |

**BETTER is not more tools. BETTER is claim = wire.**

---

## Worked micro-example

**Server S1**

- Lists `health`, `echo`  
- Discover path OK  
- `ttlMs=60000`, `cacheScope=public`  
- Order fixed  

**Server S2**

- Lists `health`, `echo`  
- initialize-only client path  
- No stamps  
- Order depends on map iteration  

**Catalog equality:** both “have echo.”  
**InterOp equality:** false.  
**Client impact:** caching, retries, multi-instance HTTP, and host A vs host B behavior diverge.

---

## What InterOp is not

- Matching tool *counts*  
- Sharing a registry name  
- Appearing in a host “connectors” brand shelf  
- Passing a single manual click test on one laptop  

Registry presence and directory presence are distribution surfaces. They are **not** substitutes for the peer test.

---

## Check (self-test)

1. Pick any MCP server you use. Answer InterOp questions 1–6 with evidence links or “unknown.”  
2. Mark each unknown as a documentation bug or a smoke gap.  
3. State one change that would move that server from GOOD-shaped toward BETTER without adding domain tools.

---

## Next

→ [04 — Discover](./04-discover.md)
