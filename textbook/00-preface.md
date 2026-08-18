# 00 — Preface

**Status:** SOLID  
**Read time:** ~8 minutes  
**Depends on:** nothing  
**Feeds:** entire book

---

## Purpose

```text
the book is the app is the book
```

This book is not documentation *about* `mcp-better`.  
**It is the same product.** The chapters are the curriculum; the binary is the lab.  
Lesson after lesson, version after version, the knowledge compounds — each release explains **what** shipped, **why** it matters for 7/28, and **how** to prove it.

Doctrine: [DOCTRINE-book-is-app.md](./DOCTRINE-book-is-app.md).

This book exists so that “modern MCP” stops meaning whatever the last README banner said.

MCP after **`2026-07-28`** changed the operational contract: **stateless core**, Discover, stamped list results, Streamable HTTP routing headers, and a clearer line between protocol state and application state. Samples, hosts, and SDKs roll out on different clocks. The board fills with thin “stateless” notes. None of that is a substitute for a **methodical** treatment of the peer contract.

The goal of this book is **compounding knowledge volume**:

- chapters that stay true next quarter  
- checks a stranger can re-run  
- a small runnable server that demonstrates the contract without becoming a platform  

---

## What “textbook” means here

| Textbook | Not textbook |
|----------|----------------|
| Small surface, full honesty | Many tools, thin protocol |
| Claim surface ≤ proof surface | README ahead of CI |
| Protocol concepts first | Domain product features |
| Stable vocabulary | Rebrand every minor version as a new “era” |
| Lab that fails when you lie | Demo that always greets |

The companion binary is **`mcp-better`**: three tools (`health` → `echo` → `confirm_echo`), official Rust `rmcp` 3, Discover-compatible path, stamped `tools/list`, stdio by default, opt-in Streamable HTTP for local teaching. It is deliberately boring. `confirm_echo` is the optional MRTR textbook — not a fourth product surface.

---

## Who this is for

1. **Implementers** writing or migrating a tools server to 7/28.  
2. **Reviewers** (including AAIF / open-source scoring) who need an audit checklist.  
3. **Client authors** who must know what a modern peer actually expects.  
4. **Operators** who have seen “works in Host A, flakes in Host B” and need language for *why*.

It is not written for end users of a particular chat product. Host UI steps appear only where they clarify the wire.

---

## How authority works

| Source | Role |
|--------|------|
| [MCP specification `2026-07-28`](https://modelcontextprotocol.io/specification/2026-07-28) | **Wire authority** |
| Official changelog / SEPs as cited | Change rationale |
| This book | **Pedagogy and audit discipline** for BETTER |
| `mcp-better` CI + examples | **Proof** for the claims we make about the companion |

If this book and the spec disagree, **the spec wins**. Fix the book.

If this book and a blog extract disagree, **this book wins**. Fix the extract.

If this book and the **mcp-better binary / smokes** disagree on companion behavior, **the binary wins**. Fix the book (or the product docs — never invent proof).

---

## Editorial voice

- Prefer tables over slogans.  
- Prefer “MUST / SHOULD / LATER / AVOID” over vibes.  
- Prefer one worked example over six product links.  
- Prefer “we do not claim X” over silent omission when X is commonly assumed.

---

## Series vs book

Public posts (InterOp, host matrix, remote/edge) are **season extracts**. They score, amplify, and recruit readers. They are not the master.

```text
textbook/     →  canonical chapters (this tree)
dev.to / LI   →  timed extracts + CTAs to lab
mcp-better    →  runnable proof
```

---

## Done-when for Season 1

Season 1 is “done enough” when:

1. A reader can explain GOOD vs BETTER without a diagram.  
2. A reader can list the InterOp questions and why each bites.  
3. A reader can install `mcp-better` and see Discover + stamps + stable order.  
4. A reader can reject a lying README using Chapter 08.

Then write Season 2. Not before the core is boring.

---

## Next

→ [01 — Era & names](./01-era-and-names.md)
