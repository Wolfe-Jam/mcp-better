# Mid-call confirmation — `confirm_echo` (SEP-2322 MRTR)

**Textbook tool on mcp-better.** Optional pattern — not required for dual-package, launch, or every tool.

## What MRTR is

**Multi Round-Trip Requests (MRTR)** let a server pause inside one logical operation and ask the client for more input (elicitation, sampling, or roots) **without** holding a long-lived server→client JSON-RPC stream as application state. The server returns `resultType: "input_required"`; the client fulfills `inputRequests`, then **retries** the same method with `inputResponses` and the echoed opaque `requestState`.

## Sequence

```text
1. Client  → tools/call confirm_echo { message }
2. Server  ← resultType: input_required
             + inputRequests.confirm (elicitation/create: “Type CONFIRM”)
             + requestState (opaque, sealed)
3. Client  → fulfills form (content.confirm = "CONFIRM")
4. Client  → tools/call confirm_echo { message,
                 inputResponses, requestState }   // same sealed state, echoed
5. Server  ← resultType: complete  (echoes sealed message)
```

| Round | Behavior |
|-------|----------|
| **1** | No `inputResponses` → `input_required` + form elicitation + sealed `requestState` |
| **2** | Integrity-check `requestState`; require accepted form with **`CONFIRM`** → complete |

## Minimal JSON (illustrative)

**Round 1 response (shape):**

```json
{
  "resultType": "input_required",
  "inputRequests": {
    "confirm": {
      "method": "elicitation/create",
      "params": {
        "message": "Type CONFIRM to echo the sealed message.",
        "requestedSchema": {
          "type": "object",
          "properties": {
            "confirm": { "type": "string", "description": "Type CONFIRM (exact) to continue" }
          },
          "required": ["confirm"]
        }
      }
    }
  },
  "requestState": "<opaque sealed token>"
}
```

**Round 2 request params (shape):**

```json
{
  "name": "confirm_echo",
  "arguments": { "message": "hello" },
  "requestState": "<same opaque token>",
  "inputResponses": {
    "confirm": {
      "action": "accept",
      "content": { "confirm": "CONFIRM" }
    }
  }
}
```

## Integrity

`requestState` is **untrusted** if echoed without verification. mcp-better seals a small JSON payload (`message`) with `rmcp::model::RequestStateCodec` (HMAC) and associated data bound to `confirm_echo`. Tampered or wrong-tool state → invalid params.

The demo signing key is a **constant for the textbook binary only**. Real deployments must use a secret key.

## Protocol gate

The `rmcp` SDK only delivers `InputRequiredResult` to peers that negotiated **`2026-07-28` or newer**. Older clients get a **protocol error** on that result (not a silent fallback). Document that honestly: MRTR is a **7/28** surface.

`health` and `echo` never need mid-call input — checklist **A8 = N/A** for them. Only tools that elicit/sample mid-call implement MRTR.

## Links

- Pattern: [SEP-2322 / Multi Round-Trip Requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports) (see also MCP SEP index for SEP-2322)
- SDK types: `rmcp` `model/mrtr.rs` · `RequestStateCodec` (`request-state` feature)
- Upstream example (when present in your `rmcp` tree): `examples/servers/src/mrtr.rs` / crate tests `test_mrtr_behavior.rs`
- Vault placement: PLANET-FAF `CHECKLIST-728-QUALITY.md` **A8** (conditional) · `MRTR-PLACEMENT.md`

## What this is not

- Not a dual-package requirement  
- Not required for every tool on this server  
- Not FAF-HTTP / Streamable HTTP session design (remote mid-call still uses POST retry, not held SSE as app state)

---

*Textbook on main · ship version when you GO publish (dual lockstep). Dual-package guide stays launch/wire — not an MRTR tax.*
