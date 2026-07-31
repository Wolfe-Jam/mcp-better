# 05 — Stamped lists

**Status:** SOLID  
**Read time:** ~10 minutes  
**Depends on:** [04 Discover](./04-discover.md)  
**Feeds:** [08 Claim = wire](./08-claim-equals-wire.md), [09 Lab](./09-run-the-textbook.md)

---

## The problem unstamped lists create

`tools/list` can “work” forever without teaching the client anything about cache:

| Symptom | Cause |
|---------|--------|
| Client re-lists on every turn | No `ttlMs` |
| Multi-instance inconsistency myths | No `cacheScope` guidance |
| Prompt-cache thrash | Unstable order |
| “Flaky tools” reports | Catalog identity drifted without a stamp story |

7/28 list cache fields make the contract **explicit**.

---

## Fields that matter

| Field | Role | BETTER default for a static tool catalog |
|-------|------|------------------------------------------|
| **`ttlMs`** | How long a client may treat this list as fresh (ms) | **Positive** (e.g. `60000`) — not omitted if you claim BETTER |
| **`cacheScope`** | Who may share the cached list | **`public`** when the catalog is not user-private |
| **Order** | Determinism of the tool array | **Stable** for the same process; static catalogs should survive restart |

Applies in the same family to other list/read surfaces in the full protocol (`prompts/list`, `resources/list`, …). Season 1 textbook focuses on **`tools/list`**.

---

## `cacheScope` in plain language

| Value | Meaning |
|-------|---------|
| **`public`** | Safe to reuse across users/instances when the catalog is the same (typical static demo server) |
| **`private`** | Cache is not freely shareable (user- or tenant-specific catalog) |

A two-tool textbook with fixed `health` + `echo` is a **public** catalog. Marking it private without reason teaches the wrong default.

---

## Stable order

**SHOULD** in the spirit of prompt-cache friendliness; **MUST** for BETTER pedagogy.

| Guarantee | Level |
|-----------|--------|
| Same process, N consecutive `tools/list` | Same order |
| Process restart, static catalog | Same order when the catalog is code-fixed (e.g. `mcp-better` sorts by a constant `TOOL_ORDER`). **Prove with smoke if you claim restart-stable in public docs**; v0.2 automated smoke proves **same-process** N lists |
| Dynamic catalogs | Document when order may change; bump understanding of ttl |

**mcp-better** order: `health`, then `echo`. Always.

---

## SDK trap

Many SDK defaults / macro paths return a list **without** stamps. Wire omission is fine for old peers. For **BETTER** claims, **stamp explicitly**.

Conceptual shape (Rust `rmcp` 3 style):

```text
ListToolsResult
  .with_all_items(tools)
  .with_ttl_ms(60_000)
  .with_cache_scope(Public)
```

If the framework “helps” you omit stamps, override. Document the override. Test the override.

---

## What stamps are not

| Not this | Why |
|----------|-----|
| A performance product claim | They are a **protocol contract** |
| Proof of tool *correctness* | A stamped list can still implement bad tools |
| A substitute for Discover | Both are required for modern posture |
| Permission to skip capability honesty | Still advertise only what you implement |

---

## Lying patterns (teach by contrast)

| Lie | Detection |
|-----|-----------|
| README: “cacheable lists”; wire: no `ttlMs` | Inspect list JSON |
| `ttlMs: 0` or null while claiming modern cache | Assert `ttlMs > 0` in smoke |
| `cacheScope` omitted on a static public catalog | Assert scope present |
| Order shuffles between calls | N-list smoke |

A future **lying companion** binary (product road 0.3) exists to make these differences visceral. Season 1 does not require it to understand the rule.

---

## Lab expectation (`mcp-better`)

| Assert | Expected (0.2.0) |
|--------|------------------|
| `ttlMs` | `60000` (constant `LIST_TOOLS_TTL_MS`) |
| `cacheScope` | `public` |
| Names | `["health", "echo"]` |

Unit tests + `conformance_smoke` + Discover client smoke should all agree. If they disagree, the release is not honest.

---

## Check (self-test)

1. Capture one real `tools/list` result from a server you use. Are stamps present?  
2. If absent: is that documented, or is the README over-claiming?  
3. Write the one-line smoke assert you would add for stamps.

---

## Next

→ [06 — Stateless core](./06-stateless-core.md)
