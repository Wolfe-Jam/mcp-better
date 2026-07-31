<!-- faf: textbook | md | mcp-728-textbook | Canonical MCP 7/28 textbook — compounding knowledge volume; AAIF = BETTER only. -->
<!-- faf: doc=textbook-spine | lane=AAIF/MCP | related=../STATE.md,../mcp-good-better-best-plan-2026-07-28.md | canonical=this -->

# The MCP Textbook (7/28)

**Status:** Living book · Season 1 SOLID  
**Era:** **7/28** · wire protocol **`2026-07-28`**  
**Audience:** Implementers, reviewers, Ambassadors, anyone who must audit a server  
**Identity:** [**the book is the app is the book**](./DOCTRINE-book-is-app.md)

**This directory ships with the binary.** It is the book face of the app.

---

## Doctrine (read first)

```text
the book is the app is the book
lesson after lesson · version after version · knowledge compounds
```

`mcp-better` is not a server with docs. It is a **textbook that runs**.  
Full lock: [`DOCTRINE-book-is-app.md`](./DOCTRINE-book-is-app.md).

---

## What this book is

A **methodical, boring, definitive** guide to modern MCP after the **stateless core** release — written so it **is** the product shape of `mcp-better`.

It is not a product catalog.  
It is not a host marketing tour.  
It is not a tour of every SEP.

It teaches one test, applied repeatedly:

> **Does this server behave like a 7/28 peer under a modern client?**

If a chapter expands the **claim surface** past the **proof surface**, it does not belong in a shipped version.

---

## How to read

| Path | Start here |
|------|------------|
| **Doctrine** | [Book = app](./DOCTRINE-book-is-app.md) |
| **Cold start** | [00 Preface](./00-preface.md) → [01 Era & names](./01-era-and-names.md) → [03 InterOp](./03-interop.md) |
| **Implementer** | [04 Discover](./04-discover.md) → [05 Stamped lists](./05-stamped-lists.md) → [08 Claim = wire](./08-claim-equals-wire.md) → [09 Lab](./09-run-the-textbook.md) |
| **Auditor / reviewer** | [03 InterOp](./03-interop.md) → [08 Claim = wire](./08-claim-equals-wire.md) → Appendix A |
| **One sitting (~45 min)** | Doctrine → 00 → 01 → 02 → 03 → 09 |

Public blog posts (dev.to, etc.) are **extracts**.  
**Shipped lessons live in the app tree.** When prose drifts from the binary, fix toward the binary, then the book, then extracts.

---

## Ladder (load-bearing)

```text
NONE  →  GOOD  →  BETTER  →  BEST
```

| Rung | Meaning | This book |
|------|---------|-----------|
| **NONE** | No MCP | Named only |
| **GOOD** | Real MCP, often session-era habits | What *not* to keep as identity |
| **BETTER** | Honest `2026-07-28` tools peer | **Entire Season 1** |
| **BEST** | Persistent project DNA for agents | One hop: [faf.one/agents](https://faf.one/agents) — not the AAIF hero |

**AAIF surface = BETTER.** No FAF install tax to learn the protocol.

---

## Table of contents

### Season 1 — The operational contract (core)

| # | Chapter | Status | One-line |
|---|---------|--------|----------|
| 00 | [Preface](./00-preface.md) | **SOLID** | Why a textbook, not a thread |
| 01 | [Era & names](./01-era-and-names.md) | **SOLID** | 7/28 vs `2026-07-28` |
| 02 | [The ladder](./02-ladder.md) | **SOLID** | NONE · GOOD · BETTER · BEST |
| 03 | [InterOp](./03-interop.md) | **SOLID** | Same tool name ≠ same machine |
| 04 | [Discover](./04-discover.md) | **SOLID** | Lifecycle after sessions |
| 05 | [Stamped lists](./05-stamped-lists.md) | **SOLID** | `ttlMs` · `cacheScope` · order |
| 06 | [Stateless core](./06-stateless-core.md) | **SOLID** | No protocol session; handles for app state |
| 07 | [Transports](./07-transports.md) | **SOLID** | stdio default · Streamable HTTP road |
| 08 | [Claim = wire](./08-claim-equals-wire.md) | **SOLID** | Honesty as engineering |
| 09 | [Lab: run the textbook](./09-run-the-textbook.md) | **SOLID** | mcp-better 0.2 in ≤10 min |
| 10 | [What we resist](./10-what-we-resist.md) | **SOLID** | Non-goals that keep the book true |

### Appendices

| ID | Document | Status |
|----|----------|--------|
| A | [Spec map (7/28 → BETTER)](./APPENDIX-a-spec-map.md) | **SOLID** |
| B | [Glossary](./APPENDIX-b-glossary.md) | **SOLID** |
| C | [Chapter status ledger](./STATUS.md) | live |

### Season 2 — Host & matrix (after Season 1 solid)

Stubs only. Write when Season 1 has soaked and Part I score hygiene is clean.

| # | Working title | Status |
|---|---------------|--------|
| 11 | Claude-shaped MCP (host assumptions) | STUB |
| 12 | Non-Claude host matrix | STUB |
| 13 | Language runtimes (Rust / TS / Py) | STUB |
| 14 | Remote / Edge after stateless core | STUB |
| 15 | Extensions later (Tasks · MRTR demos · auth direction) | STUB |

Season 2 chapters must still pass the peer test. Flavor is illustration, not product pitch.

---

## Editorial rules (keep these sacred)

1. **Boring first.** Precision over punchlines. Punchlines may appear once per chapter max.
2. **Falsifiable.** Every “MUST” for BETTER has a check a stranger can run.
3. **Claim ≤ proof.** If we cannot smoke it, we do not claim it in Season 1.
4. **One era.** Versions of `mcp-better` add *road* inside 7/28; they do not invent new protocol eras.
5. **No FAF hero on AAIF pages.** BEST is a hop, not a funnel.
6. **Archive, don’t delete.** Superseded chapter text → note in STATUS, keep history in git.
7. **Public extracts.** When publishing a blog/tutorial, link back here as canonical when the vault is private; public repo may mirror selected chapters later.

---

## Companion artifacts

| Artifact | Role |
|----------|------|
| [mcp-better](https://github.com/Wolfe-Jam/mcp-better) | Runnable BETTER server (health + echo) |
| Spec [2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28) | Authority on the wire |
| [Changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog) | What changed vs prior |
| Part I extract (live) | [dev.to InterOp](https://dev.to/wolfejam/not-all-mcp-servers-are-equal-what-728-just-made-official-2f29) |
| Private lane STATE | [`../STATE.md`](../STATE.md) |

---

## One-line lock

```text
Write the contract. Prove the contract. Resist the platform.
Season 1 = operational honesty. Season 2 = matrix. BEST = one hop up.
```

*Opened 2026-07-31 · master lives under PLANET-FAF/AAIF/MCP/textbook/*
