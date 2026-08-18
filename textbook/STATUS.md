<!-- faf: textbook-status | md | chapter-ledger | Which chapters are SOLID vs STUB; refresh when prose hardens. -->

# Textbook status ledger

**Last refresh:** 2026-08-18 (0.5.0 — matching client completes MRTR)  
**Season 1 goal:** All core chapters **SOLID** before heavy Season 2 writing.  
**Shipped home:** `mcp-better/textbook/` — the book is the app is the book.

| Chapter | Title | Status | Notes |
|---------|-------|--------|-------|
| 00 | Preface | SOLID | 0.4.4: three-tool catalog |
| 01 | Era & names | SOLID | README-lag note tightened |
| 02 | Ladder | SOLID | |
| 03 | InterOp | SOLID | Header row: Method always; Name when applicable |
| 04 | Discover | SOLID | stdio-client exact OK line + `confirm_echo` in order |
| 05 | Stamped lists | SOLID | Catalog example matches three-tool order |
| 06 | Stateless core | SOLID | `confirm_echo` = sealed retry; `mrtr-client` is the other half |
| 07 | Transports | SOLID | Env precedence; http-smoke happy-path only |
| 08 | Claim = wire | SOLID | Tool-list claim = `BETTER_TOOL_ORDER` |
| 09 | Lab | SOLID | Path C = `mrtr-client` · pin **0.5.0** live |
| 10 | What we resist | SOLID | |
| A | Spec map | SOLID | Header precision |
| B | Glossary | SOLID | mcp-better = three-tool catalog |
| 11–14 | Season 2 | STUB | Do not flesh until S1 soak |
| 15 | Extensions later | STUB | MRTR + Skills marked on-wire since 0.4.3 |

### Read-through pass 1 — nits fixed (2026-07-31)

| Nit | Fix |
|-----|-----|
| stdio-client OK string wrong in lab | Match binary: `Discover + stamped list + …` |
| Implying http-smoke needs manual server on 8787 | Documents self-spawn + ephemeral port |
| Claiming http-smoke fails on missing headers | Downgraded to happy-path only |
| Env alias order vs binary | `MCP_TRANSPORT` first, then `MCP_BETTER_TRANSPORT` |
| Mcp-Name on every POST | Method always; Name when method names a tool |
| health result shape vague | JSON object string fields listed |
| Session manager vs “stateless” confusion | Ch 06 clarifies SDK plumbing ≠ protocol identity |

### Doctrine lock (2026-07-31)

| Item | Status |
|------|--------|
| [`DOCTRINE-book-is-app.md`](./DOCTRINE-book-is-app.md) | LOCKED |
| Public mirror `mcp-better/textbook/` | ✅ Season 1 + doctrine copied |
| Version = lesson map | 0.1 stdio · 0.2 HTTP · 0.3 louder proof · 0.4 dual-package · 0.4.3 MRTR+Skills · 0.4.4 book=wire · 0.5 matching client |
| Rule | Every version: code + smoke + book what/why/how |

### Product honesty pass (2026-07-31) — closed

| Surface | Fix |
|---------|-----|
| `GETTING-STARTED.md` | Exact OK lines for stdio-client + http-smoke |
| README / `--help` | Env precedence matches binary |
| README routing headers | Contract language + smoke reference |
| `textbook/` | Ships in app tree |

Binary remains source of truth if docs and wire disagree again.

### Product honesty pass (2026-08-18) — 0.4.4

| Surface | Fix |
|---------|-----|
| Catalog claims | `health` → `echo` → `confirm_echo` (book = `BETTER_TOOL_ORDER`) |
| Ch 15 | MRTR + Skills marked **on the wire since 0.4.3** |
| Lab pin | crates.io **0.4.3** live · this tree **0.4.4** unreleased |

### Product honesty pass (2026-08-18) — 0.5.0

| Surface | Fix |
|---------|-----|
| Matching client | `mrtr-client` completes `confirm_echo` R1→R2 |
| Ch 06 / 09 | Client half named; Path C exact OK line |
| Tools | still three — no fourth tool |

## Extract map (public posts)

| Extract | Canonical chapter | Live URL | AAIF |
|---------|-------------------|----------|------|
| Part I InterOp | 03 (+ 09 lab slice) | [dev.to](https://dev.to/wolfejam/not-all-mcp-servers-are-equal-what-728-just-made-official-2f29) | [#363](https://github.com/aaif/ambassadors/issues/363) **20 tutorial** ✅ |
| Product lab | whole book + binary | [mcp-better](https://github.com/Wolfe-Jam/mcp-better) / crates **0.5.0** | [#310](https://github.com/aaif/ambassadors/issues/310) **15** ✅ — no re-score on 0.2/0.3/0.4/0.5 |

**Policy:** teach points for protocol lessons; product once; no semver diary filings.

When a new public piece ships: add a row here. Prefer **one extract per chapter**, not six posts per chapter.

## Next editorial work (not product surface)

1. Soak Season 1; fix factual drift only.  
2. Optional public mirror of selected chapters into `mcp-better/docs/textbook/` (product decision).  
3. Season 2 starts with host assumptions (Ch 11) or Remote/Edge (Ch 14) — pick one.  
4. Code road 0.3 (deeper correctness) feeds **Ch 04–05–08** smokes, not new chapters.
