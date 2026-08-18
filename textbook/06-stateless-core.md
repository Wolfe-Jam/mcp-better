# 06 — Stateless core

**Status:** SOLID  
**Read time:** ~10 minutes  
**Depends on:** [04 Discover](./04-discover.md)  
**Feeds:** [07 Transports](./07-transports.md), Season 2 remote/edge

---

## The headline

**7/28 makes the protocol core request/response oriented.**  
Protocol-level sessions are not the identity of the peer.

That is the “stateless core” story — the reason production HTTP and load balancing become thinkable without a sticky session brain.

---

## What went away (conceptual)

| Prior muscle | 7/28 posture |
|--------------|--------------|
| `Mcp-Session-Id` as protocol identity | **Do not** invent session stickiness as protocol identity |
| `initialize` as required handshake | Discover + self-describing requests |
| “Connected” = long-lived shared state on the transport | “Correct” = this request can be answered |

Exact wire removals and replacements are in the [official changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog). This chapter teaches **operational consequences**.

---

## Self-describing traffic

Requests carry what the peer needs (version, client info, capabilities — via `_meta` and, on HTTP, headers as specified).

| Implication | Practice |
|-------------|----------|
| Server should not depend on “we already shook hands five minutes ago” | Treat each request as complete enough |
| Client should not assume ambient session fields | Send required metadata each time the protocol says so |
| Debugging improves | Logs show version per request, not only at connect |

---

## Application state ≠ protocol session

Stateless **protocol** does not mean “your app may never remember anything.”

| Kind of state | Where it belongs |
|---------------|------------------|
| Protocol identity / routing | Headers, `_meta`, method names — **not** hidden session bags |
| Multi-call workflow | **Explicit handles** minted by tools and passed as arguments |
| User secrets | Auth layer (when you actually ship remote auth) |
| Process-local caches | Fine if any instance can still answer correctly or regenerate |

### Handle pattern (pedagogy)

```text
call tool → result includes handle H
later call → argument includes H
server resolves H to app state (DB, memory map, object store)
```

No handle, no shared workflow.  
That is the post-session model in one diagram.

`health` and `echo` stay **pure** (no multi-call state). `confirm_echo` is the optional MRTR demo: one sealed `requestState` for a single retry — not session identity, not a handle store. Do not treat that retry token as a sticky session.

---

## SDK plumbing vs protocol identity

A server binary may still construct session-related **SDK types** for transport compatibility (for example a local session manager used by a Streamable HTTP stack). That is not the same as teaching clients to treat **`Mcp-Session-Id` as protocol identity**.

For **mcp-better** HTTP: `legacy_session_mode(false)` — 7/28 is treated as stateless; SECURITY.md states no multi-session isolation claim. The textbook lesson remains: **do not rebuild handshake-as-identity**.

---

## Load balancing consequence (HTTP)

If the protocol does not require sticky sessions:

| Design | Consequence |
|--------|-------------|
| Any healthy instance may take the next request | Horizontal scale becomes plausible |
| Instance-local only memory for “protocol session” | **Wrong** — will break under LB |
| Instance-local memory for handle **if** handles are not shared | Breaks multi-instance — store handles where all instances can resolve them |

stdio single-process demos hide this. Remote chapters (Season 2) re-open it.

---

## What still streams

Stateless core is not “everything is a single JSON blob with no streams.”

- Progress and partial results may still live on the **request’s** response stream where the protocol allows.  
- Long-lived **subscription** listen patterns are a separate opt-in surface (LATER for the textbook core).  
- Broken streams generally mean **re-issue** with a new request id — design tools with that in mind.

Do not reintroduce SSE-resumability folklore as a BETTER requirement for Season 1 tools.

---

## Health vs removed `ping`

Protocol `ping` is gone. Prefer an application tool named **`health`** (or similar) for liveness **as a tool**, not as a fake protocol keepalive that reintroduces session thinking.

`mcp-better` ships `health` on purpose.

---

## Check (self-test)

1. Does your server require a session header to answer `tools/list`? If yes, why?  
2. Do you store “protocol connection state” in process memory that another instance would lack?  
3. If you need multi-call workflows, do you have a handle story written down?

---

## Next

→ [07 — Transports](./07-transports.md)
