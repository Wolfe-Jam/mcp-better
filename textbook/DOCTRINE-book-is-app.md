<!-- faf: doctrine | md | book-is-app | Load-bearing identity for mcp-better + MCP textbook. -->

# Doctrine — the book is the app is the book

**Status:** LOCKED  
**Dated:** 2026-07-31  
**Applies to:** `mcp-better` · this textbook tree · every future version

---

## One line

```text
the book is the app is the book
```

Not two products. Not “docs about a server.”  
**One artifact** with two faces that must never diverge.

| Face | What it is |
|------|------------|
| **App** | Runnable peer — binary, smokes, CI, Registry crate |
| **Book** | Lessons — what · why · how — in the same repo, versioned with the app |

If the book claims it, the app proves it.  
If the app ships it, the book explains it.  
If either lies, the version is not honest.

---

## Compounding

```text
lesson after lesson
version after version
knowledge compounds
```

| Axis | Meaning |
|------|---------|
| **Lesson** | One falsifiable unit of protocol understanding |
| **Version** | The app release that **is** that lesson (or a tight bundle of lessons) |
| **Compounds** | Next version assumes prior lessons; does not rebrand the era; does not delete the curriculum |

Readers grow by walking versions.  
The tree is a textbook you can `cargo install`.

---

## What · why · how (every lesson)

Every lesson (chapter **and** version notes) answers three questions about the **app**:

| Question | Book answers | App proves |
|----------|--------------|------------|
| **What** | What surface exists (tools, transport, stamps, …) | Wire + smoke |
| **Why** | Why 7/28 / BETTER needs it | Spec map + InterOp card |
| **How** | How to run, assert, and reject lies | Examples · CI · lab chapter |

Missing any leg = incomplete lesson.  
Marketing without how = claim > proof.  
Code without why = unteachable road.

---

## Version = lesson (map)

Versions add **road inside 7/28**. They do not invent a new protocol era.

| Version | Lesson (what the version *is*) | Book locus |
|---------|--------------------------------|------------|
| **0.1** | 7/28 over **stdio** — Discover path, stamped list, stable order, `health`+`echo` | Ch 04–05 · 09 (stdio) |
| **0.2** | Same era + **Streamable HTTP** road — headers, local-demo honesty | Ch 07 · 09 (HTTP) |
| **0.3** | **Deeper correctness** — louder smokes, edge rejection, harder to lie | Ch 04–05 · 08 (deepen) |
| **0.4** | **Pedagogical tools** (≤2) — protocol concepts only | New micro-lessons + Ch 10 filter |
| **0.5** | **Matching tiny client** — what a modern client expects | Lab + client chapter |
| Later | Extension demos only if core stays pure | Ch 15 · marked extension |

When you cut a version, you cut a **lesson release**:

1. Code + smoke green  
2. Book chapter(s) updated for what/why/how  
3. CHANGELOG names the lesson, not a feature dump  
4. Claim surface ≤ proof surface  

---

## Anti-patterns (break the identity)

| Break | Why it fails |
|-------|----------------|
| Book in a vault only; app is silent | Book is not the app |
| App grows tools; book stays frozen | App is not the book |
| Blog extract invents claims the app never ships | Extract is not the book |
| Version bumps with no lesson | Version inflation / platform cosplay |
| Lesson with no smoke | Claim > proof |
| FAF/BEST required to “finish” the textbook | Collapses ladder; AAIF surface dies |

---

## Editorial + engineering rule (same rule)

```text
claim surface  ≤  proof surface
book surface   =  app surface   (for shipped lessons)
```

Private drafts may lead.  
**Shipped** lessons live **in the app tree** so installers read the book that matches the binary.

| Tree | Role |
|------|------|
| **`~/FAF/mcp-better/textbook/`** | **Public compounding home** — book face of the app |
| **`PLANET-FAF/AAIF/MCP/textbook/`** | Editorial / STATUS / series ops — keep in sync |

When they drift, **reconcile toward the binary**, then update both textbook trees.

---

## The peer test (unchanged)

Every lesson still collapses to:

> Does this server behave like a 7/28 peer under a modern client?

The book teaches the test.  
The app is the worked answer.  
The next version is the next problem set.

---

## Lock

```text
mcp-better is not a server with docs.
mcp-better is a textbook that runs.
Lesson after lesson. Version after version.
The book explains the app — what, why, how.
The app proves the book — wire and smoke.
```
