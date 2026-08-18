# BETTER vs BEST

**Scale:** `NONE | GOOD | [BETTER] | BEST`  
**mcp-better** is the **BETTER** textbook for **AGENTS.md** — modern MCP honesty on the wire. **BEST** is persistent agent DNA (AGENTS.md / FAF), not this repo.

| | **BETTER** (this repo) | **BEST** |
|--|------------------------|----------|
| Goal | Modern MCP protocol textbook **that runs** · **AGENTS.md**-ready | Persistent AI project context |
| Artifact | Binary + [`textbook/`](../textbook/) (book = app) | FAF / agents stack · **AGENTS.md** at scale |
| Required files | README · BETTER docs · **textbook/** · code · smokes | project DNA at faf.one/agents |
| Install tax for AAIF | Zero FAF install | Optional next step |
| Main branch | No `project.faf` | May use project context elsewhere |

## How they relate

```
NONE → GOOD → BETTER → BEST
                 ▲
            mcp-better
         (AGENTS.md · MCP)

mcp-better  ──BETTER──►  honest 2026-07-28 MCP  (book is the app is the book)
                │
                └── one hop ──BEST──►  https://faf.one/agents
```

Do not collapse them: BETTER wins by protocol clarity and compounding lessons for agents; BEST wins by persistent context. This repository stays on the **[BETTER]** step.

Doctrine: [`../textbook/DOCTRINE-book-is-app.md`](../textbook/DOCTRINE-book-is-app.md).

## BEST print slice (this repo)

"Elsewhere" (above) now has a concrete location: **`better-best/2026-08-18`** — same `v0.4.3` software, plus `project.faf` (Trophy 100%). Same pattern already proven and archived-with-notes in `agents-md-facts` (`better-best/2026-07-21`): kept **off `main`** so the AAIF-facing/public surface stays BETTER-only, nothing invented, nothing bolted on. `main` still requires **zero** FAF install — this is a falsifiable branch a reader can diff against, not a claim.

```bash
git fetch origin
git diff v0.4.3..better-best/2026-08-18 --stat
git diff v0.4.3..better-best/2026-08-18 -- project.faf docs/BETTER-BEST.md
```

Or GitHub compare: `v0.4.3...better-best/2026-08-18`. Full falsifiable-check recipe: [`docs/BETTER-BEST-CHECKS.md`](./BETTER-BEST-CHECKS.md).
