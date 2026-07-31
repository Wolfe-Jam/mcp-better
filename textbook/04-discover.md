# 04 — Discover

**Status:** SOLID  
**Read time:** ~10 minutes  
**Depends on:** [03 InterOp](./03-interop.md)  
**Feeds:** [06 Stateless core](./06-stateless-core.md), [09 Lab](./09-run-the-textbook.md)

---

## The shift

| Era habit | 7/28 core story |
|-----------|-----------------|
| `initialize` / `initialized` as lifecycle | **`server/discover`** + self-describing requests |
| Session establishes “who we are” | Each request carries what the peer needs |
| Connected = long-lived channel identity | Connected = can complete a request correctly |

Clients **MAY** call Discover first. Servers **MUST** implement Discover.  
Pedagogy rule: **BETTER demos use a Discover (or Auto→7/28) client path**, not initialize-only.

---

## What Discover is for

Discover answers, in protocol terms: what version and capabilities can we speak?

It is **not**:

- a substitute for authentication  
- a long-lived session mint  
- permission to skip `_meta` / version on later traffic where the protocol requires self-description  

It is the modern entry for **capability and version selection**, replacing handshake-as-identity.

---

## Client posture (implementer notes)

| Posture | Meaning | Textbook stance |
|---------|---------|-----------------|
| **Discover** | Prefer modern lifecycle | **Required for BETTER lab clients** |
| **Auto** | SDK chooses; should land on 7/28 when available | Acceptable if it selects `2026-07-28` |
| **Legacy initialize** | GOOD-era path | Useful for *contrast* demos; not the hero path |

In Rust `rmcp` 3, names collide carefully:

| Call | Side | Meaning |
|------|------|---------|
| `service.serve(transport)` | **Server** | Attach handler and run — **not** Discover |
| `ClientInfo::serve(transport)` | **Client** | Often **legacy** lifecycle |
| `serve_with_lifecycle(..., Discover { .. })` | **Client** | **BETTER** path — preferred `V_2026_07_28` |

Do not copy a server `serve()` example, run a legacy client, and call the setup modern.

---

## Server posture

A BETTER tools server:

1. Implements Discover (typically via current SDK server stack).  
2. Does **not** require initialize to “become real.”  
3. Answers with honest version/capability information.  
4. Rejects or clearly fails closed when the client cannot meet required protocol version — **do not silently pretend**.

### Unsupported protocol version

When a client offers only an old protocol the server will not speak:

| Bad | Good |
|-----|------|
| Accept and half-speak 7/28 shapes | Clear **UnsupportedProtocolVersion** (or SDK equivalent) |
| Hang until timeout | Explicit error the client can surface |
| Log only on stderr | Error on the wire |

Season 1 pedagogy: document the behavior; deeper edge-case matrix is product road **0.3** (louder smokes), not a reason to delay this chapter’s principles.

---

## Capability filtering (concept)

Discover is also where capability negotiation lives. A modern client may filter what it will use. A honest server:

- advertises only what it implements  
- does not advertise resources/prompts/tasks if it does not serve them  
- keeps the **tools** capability truthful for a tools textbook  

**Claim fewer capabilities. Implement them fully.**

---

## Relation to stateless core

Discover does not reintroduce sessions.

| After Discover | Still true |
|----------------|------------|
| Client knows version | Next request must still be self-describing as required |
| Server is “ready” | Any HTTP instance may still handle the next request |
| Tools can be listed | List results should be stamped (Ch 05) |

---

## Lab expectation (`mcp-better`)

The **`stdio-client`** example (not the HTTP smoke):

1. Spawns the server binary as a child process.  
2. Connects with `ClientLifecycleMode::Discover` and preferred version **`V_2026_07_28`**.  
3. Lists tools twice; asserts stamps + stable order `health`, `echo`.  
4. Calls `health` and `echo`.  
5. Prints `stdio-client: OK (Discover + stamped list + health + echo)` and exits 0 only if all asserts pass.

If Discover or stamps fail, the example **bail!s** (non-zero) — not a soft warning.

**Note:** `http-smoke` proves Streamable HTTP + stamps + headers on a **self-describing request** path; it is not a second Discover client.

---

## Check (self-test)

1. In your client SDK, name the API that selects Discover vs initialize.  
2. Does your server README say “MCP” only, or does it say Discover / `2026-07-28`?  
3. What happens if a client offers only a pre-7/28 version — pass, reject, or unknown?

---

## Next

→ [05 — Stamped lists](./05-stamped-lists.md)
