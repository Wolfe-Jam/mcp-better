<!-- faf: textbook-status | md | chapter-ledger | Which chapters are SOLID vs STUB; refresh when prose hardens. -->

# Textbook status ledger

**Last refresh:** 2026-07-31 (doctrine **book = app** locked · mirrored into `~/FAF/mcp-better/textbook/`)  
**Season 1 goal:** All core chapters **SOLID** before heavy Season 2 writing.  
**Shipped home:** `mcp-better/textbook/` — the book is the app is the book.

| Chapter | Title | Status | Notes |
|---------|-------|--------|-------|
| 00 | Preface | SOLID | |
| 01 | Era & names | SOLID | README-lag note tightened |
| 02 | Ladder | SOLID | |
| 03 | InterOp | SOLID | Header row: Method always; Name when applicable |
| 04 | Discover | SOLID | stdio-client exact OK line; HTTP smoke ≠ Discover |
| 05 | Stamped lists | SOLID | Restart-stable vs same-process smoke split |
| 06 | Stateless core | SOLID | SDK session plumbing vs protocol identity |
| 07 | Transports | SOLID | Env precedence; http-smoke happy-path only |
| 08 | Claim = wire | SOLID | 0.3 edge list matches real smoke gaps |
| 09 | Lab | SOLID | Exact OK lines; health JSON shape; HTTP spawn note |
| 10 | What we resist | SOLID | |
| A | Spec map | SOLID | Header precision |
| B | Glossary | SOLID | |
| 11–15 | Season 2 | STUB | Do not flesh until S1 soak |

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
| Version = lesson map | 0.1 stdio · 0.2 HTTP road · 0.3+ louder proof / pedagogy |
| Rule | Every version: code + smoke + book what/why/how |

### Product honesty pass (2026-07-31) — closed

| Surface | Fix |
|---------|-----|
| `GETTING-STARTED.md` | Exact OK lines for stdio-client + http-smoke |
| README / `--help` | Env precedence matches binary |
| README routing headers | Contract language + smoke reference |
| `textbook/` | Ships in app tree |

Binary remains source of truth if docs and wire disagree again.

## Extract map (public posts)

| Extract | Canonical chapter | Live URL | AAIF |
|---------|-------------------|----------|------|
| Part I InterOp | 03 (+ 09 lab slice) | [dev.to](https://dev.to/wolfejam/not-all-mcp-servers-are-equal-what-728-just-made-official-2f29) | [#363](https://github.com/aaif/ambassadors/issues/363) |

When a new public piece ships: add a row here. Prefer **one extract per chapter**, not six posts per chapter.

## Next editorial work (not product surface)

1. Soak Season 1; fix factual drift only.  
2. Optional public mirror of selected chapters into `mcp-better/docs/textbook/` (product decision).  
3. Season 2 starts with host assumptions (Ch 11) or Remote/Edge (Ch 14) — pick one.  
4. Code road 0.3 (deeper correctness) feeds **Ch 04–05–08** smokes, not new chapters.
