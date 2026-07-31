# 07 — Transports

**Status:** SOLID  
**Read time:** ~10 minutes  
**Depends on:** [06 Stateless core](./06-stateless-core.md)  
**Feeds:** [08 Claim = wire](./08-claim-equals-wire.md), [09 Lab](./09-run-the-textbook.md)

---

## Two roads, one era

| Transport | Role in Season 1 | Production posture |
|-----------|------------------|--------------------|
| **stdio** | **Default** · host-spawned local servers | Normal for Desktop / IDE local MCP |
| **Streamable HTTP** | **Opt-in road** · same 7/28 era | Only as honest as your auth/TLS/bind story |

Adding HTTP does **not** create a new protocol era. It adds **road** inside 7/28.

---

## stdio (default textbook path)

| Property | Note |
|----------|------|
| Process | Host spawns server; JSON-RPC on stdio |
| Logs | **stderr** (never corrupt stdout framing) |
| Auth | Process boundary + OS user — not OAuth |
| Registry package | Often **stdio** for installable local servers |

**mcp-better:** `mcp-better` with no flags → stdio.

Hosts (Cursor, Claude Desktop, etc.) typically want an absolute path to the binary and empty args.

---

## Streamable HTTP (teaching road)

When claimed, BETTER requires:

| Requirement | Meaning |
|-------------|---------|
| Real Streamable HTTP implementation | Not “HTTP somewhere” folklore |
| **`Mcp-Method`** on POSTs; **`Mcp-Name`** when the method names a tool (etc.) | Routing headers (SEP-2243) — happy-path smoke sends them |
| Protocol version on the wire | e.g. `MCP-Protocol-Version: 2026-07-28` plus `_meta` on requests (as in `http-smoke`) |
| Honest security docs | No auth/TLS → **local demo only** |

### mcp-better v0.2 local demo

| Item | Value |
|------|--------|
| Enable | `--http` or `MCP_BETTER_TRANSPORT=http` |
| Default bind | `127.0.0.1:8787` (path `/mcp`) |
| Override | `MCP_BETTER_HTTP_ADDR` |
| Auth / TLS | **None** — see SECURITY.md |
| Open bind `0.0.0.0` | Dangerous without auth — do not treat as “easy prod” |

**http-smoke** (v0.2) spawns `--http` on a free loopback port, then POSTs `tools/list` + `tools/call` with **`Mcp-Method` / `Mcp-Name`** (and `MCP-Protocol-Version`) present. It asserts stamps and tool results. It is a **happy-path** proof — it does **not** yet assert that missing/wrong headers are rejected (that belongs in louder 0.3 / classroom work).

---

## Header routing (concept)

Streamable HTTP POSTs carry routing information in headers so infrastructure can route without always parsing the full body:

| Header family | Role |
|---------------|------|
| **`Mcp-Method`** | Which MCP method |
| **`Mcp-Name`** | Name discriminator where required |
| Protocol version header(s) | As specified / SDK-emitted |

Classroom extensions (later): show failure modes when headers are missing; optional local-only auth *stub* to show identity headers. Never pretend a stub is production auth.

---

## Registry vs binary capability

| Surface | Honest statement for mcp-better 0.2 |
|---------|-------------------------------------|
| **MCP Registry package** | stdio install path |
| **Same binary** | Can also `--http` for local teaching |
| **Not claimed** | Production remote multi-tenant HTTP |

Claiming “remote MCP” because `--http` exists is a **claim > proof** failure (Ch 08).

---

## Transport selection matrix (mcp-better 0.2)

| How | Value |
|-----|--------|
| CLI | `mcp-better` · `--http` · `--stdio` |
| Bare args | `http` / `stdio` (same meaning as flags) |
| Env | `MCP_TRANSPORT` or `MCP_BETTER_TRANSPORT` → `stdio` \| `http` |

**Precedence (current binary):** CLI flags/args win. If no CLI transport flag, env is read as **`MCP_TRANSPORT` first**, then **`MCP_BETTER_TRANSPORT`**. If neither is set → **stdio**.

---

## Anti-patterns

| Anti-pattern | Fix |
|--------------|-----|
| HTTP default on a public interface with no auth | Default loopback; document risk |
| Second product version branded as “the HTTP era” | Same era, more road |
| Only testing stdio while README leads with remote | Prove both or claim one |
| Logging on stdout in stdio mode | stderr only |

---

## Check (self-test)

1. Which transports does your server **claim**?  
2. Which transports does CI **prove**?  
3. If they differ, which side will you change — docs or tests?

---

## Next

→ [08 — Claim = wire](./08-claim-equals-wire.md)
