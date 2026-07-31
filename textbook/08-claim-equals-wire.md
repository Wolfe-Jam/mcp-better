# 08 — Claim = wire

**Status:** SOLID  
**Read time:** ~10 minutes  
**Depends on:** [03](./03-interop.md)–[07](./07-transports.md)  
**Feeds:** [09 Lab](./09-run-the-textbook.md), [10 Resist](./10-what-we-resist.md), product road 0.3

---

## The law

```text
claim surface  ≤  proof surface
```

If a sentence appears in README, Registry blurb, blog, or AAIF notes as a capability, there must be a **reproducible check** that fails when the sentence becomes false.

This is the entire BETTER rung in one inequality.

---

## Surfaces that make claims

| Surface | Example claim | Required proof |
|---------|---------------|----------------|
| README protocol line | “Built for 7/28” | Discover smoke · version on wire |
| README transports | “Streamable HTTP” | HTTP smoke + header asserts |
| `tools/list` docs | “cacheable” | `ttlMs` / `cacheScope` asserts |
| Tool list | “only health + echo” | Exact name vector in tests |
| SECURITY.md | “local demo only” | Default bind loopback · no auth features advertised as prod |
| Registry / crates blurb | Version pin | Published artifact matches tag |
| Blog extract | “10-minute textbook” | Cold path still works |

---

## Honesty close (process)

Before any release that changes claims:

1. List every public claim sentence (README, SECURITY, server.json, crate description).  
2. Map each to a test, smoke, or explicit **non-claim**.  
3. Run the full CI script cold.  
4. If a claim lacks a check: **delete the claim** or **add the check**. Never ship the gap.

**mcp-better** uses a release matrix (`docs/RELEASE-MATRIX.md`) so “FULL LIVE” means gates, not vibes.

---

## Failure modes (how servers lie without meaning to)

| Lie pattern | How it happens | Detection |
|-------------|----------------|-----------|
| **Banner inflation** | Marketing ahead of SDK pin | Compare README to `Cargo.toml` / package.json |
| **Transport fiction** | HTTP mentioned; only stdio tested | Matrix row empty |
| **Stamp theater** | Docs mention ttl; SDK default omits | Wire capture |
| **Era cosplay** | “2026-07-28” string hardcoded in one tool result only | Discover + list + tool all checked |
| **Directory cosplay** | Listed somewhere; wire is prior-era | InterOp card (Ch 03) |
| **Silent degrade** | Old client accepted with wrong shape | Version rejection tests |

---

## Louder smokes (why 0.3 exists)

Happy-path smoke is necessary and insufficient.

| Smoke class | Purpose |
|-------------|---------|
| Happy path | Proves the demo works |
| Edge: old client / bad version | Proves rejection honesty |
| Edge: missing/wrong HTTP headers | Proves transport contract (still happy-path only on `http-smoke`) |
| Edge: order across process restart | **v0.3** `order-restart-smoke` |
| Negative companion (`mcp-worse`) | **v0.3** `contrast-smoke` |

v0.3 ships the restart + lying contrast legs. Missing-header HTTP edges remain future classroom work. Principle: **fail loudly**.

---

## AAIF and scoring discipline

| Rule | Why |
|------|-----|
| Score **artifacts**, not vibes | Reviewers need URLs |
| New major teaching piece → **new URL** | Do not re-score the same crate URL for every semver as a second identical contribution type without board rules |
| BETTER-only lede on AAIF | BEST/FAF is a hop, not the submission hero |
| Concrete > generic agent essays | MCP project rewards protocol substance |

---

## Writing claims (templates)

**Good:**

> stdio default; Streamable HTTP via `--http` on `127.0.0.1` for local demo; no auth/TLS.

**Bad:**

> Enterprise-ready remote MCP with modern everything.

**Good:**

> `tools/list` returns `ttlMs=60000` and `cacheScope=public`; order is `health`, `echo`.

**Bad:**

> Fully cache-optimized agent platform.

---

## Check (self-test)

1. Open your server README. Highlight every capability adjective.  
2. For each, write the command that would falsify it.  
3. Run those commands. Record gaps. Fix docs or tests before the next tag.

---

## Next

→ [09 — Lab: run the textbook](./09-run-the-textbook.md)
