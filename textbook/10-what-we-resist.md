# 10 — What we resist

**Status:** SOLID  
**Read time:** ~8 minutes  
**Depends on:** Season 1 core  
**Feeds:** product roadmap discipline; Season 2 scope control

---

## Why resistance is part of the textbook

Every successful open lesson attracts feature gravity:

- “Just add resources.”  
- “Just add OAuth so it’s real.”  
- “Just make it useful with twenty tools.”  
- “Just fold in project context.”  

Each request can be reasonable **somewhere**.  
Most of them **destroy** a 7/28 peer textbook by expanding claim surface past proof surface.

This chapter is the immune system.

---

## Resist list (Season 1 sacred)

| Temptation | Why resist | Where it belongs instead |
|------------|------------|---------------------------|
| **Resources / prompts as first-class hero** | Different teaching surface; dilutes tools-peer clarity | Later specialized chapters or other servers |
| **OAuth / CIMD as day-one product** | Auth wrong is worse than auth absent on a local textbook | Remote chapter + real security design |
| **MCP Apps / SEP zoo chase** | Spec watching ≠ textbook churn | INTEL + LATER appendix |
| **Multi-tool “useful” server** | Domain features ≠ protocol literacy | Product servers; not this book’s companion |
| **FAF / project DNA on main** | Collapses BEST into BETTER; AAIF lede pollution | [faf.one/agents](https://faf.one/agents) · optional `better-best/*` print |
| **Version inflation as platform** | 0.x should stay textbook-shaped | Road inside 7/28 only |
| **Directory / connector shelf chasing** | Brand shelves ≠ wire honesty | Registry + smokes |
| **Re-score same URL endlessly** | Board hygiene | New artifacts / new URLs for new teaching |

---

## Allowed growth (still textbook)

| Growth | Why allowed |
|--------|-------------|
| **Deeper correctness** | Same tools; louder smokes; Discover edge cases |
| **1–2 pedagogical tools** | Only if each teaches a **protocol** concept (`_meta`, structured result, handle) |
| **Better HTTP classroom** | Failure modes, header demos — still local-demo honest |
| **Tiny matching client** | Proves what a modern client expects |
| **Marked extension demos** | Clearly “extension,” never the hero surface |
| **Season 2 host matrix chapters** | Illustration of InterOp — not product catalog |

Rule for any new tool:

> Does this teach a **protocol** concept, or a **domain** concept?  
> Domain → reject for the companion binary.

---

## The peer test (final filter)

Before accepting a feature into the companion or a new Season 1 chapter:

```text
Does this help answer:
  “Does this server behave like a 7/28 peer under a modern client?”
```

| Answer | Action |
|--------|--------|
| Yes, and we can smoke it | Consider |
| Yes, but we cannot smoke it | Document as non-claim or add smoke first |
| No, but users will like it | **Reject** for this textbook |
| No, but it scores points | **Reject** — points without honesty rot the lane |

---

## BEST remains one hop

```text
BETTER (this book + mcp-better)
        │
        └── hop ──► https://faf.one/agents  (BEST)
```

Do not require BEST to complete Season 1.  
Do not pretend BEST is “just more MCP methods.”

---

## Product version shape (reminder)

| Version | Focus | Still a textbook? |
|---------|--------|-------------------|
| 0.2 | stdio + HTTP road | Yes (shipped) |
| 0.3 | Deeper correctness + louder smokes | Yes |
| 0.4 | ≤2 pedagogical tools + HTTP classroom | Yes if capped |
| 0.5 | Matching tiny client | Yes |
| Later | Extension illustrations | Only if core stays pure |

---

## Check (self-test)

1. Name three features you will **not** add to `mcp-better` main this quarter.  
2. Name one feature that would pass the peer test.  
3. If someone asks “when does it become useful?”, answer with the InterOp definition — not a roadmap of domain tools.

---

## Season 1 close

You now have:

- vocabulary (7/28 vs wire date)  
- ladder (NONE → BEST)  
- InterOp card  
- Discover, stamps, stateless core, transports  
- claim = wire discipline  
- a lab that proves the companion  

**Next reading:** Appendix A (spec map) or Appendix B (glossary).  
**Next writing (editorial):** Season 2 only after soak.  
**Next code (optional):** 0.3 louder correctness — feeds chapters 04–05–08, does not replace them.

---

## Back to spine

→ [README — Table of contents](./README.md)
