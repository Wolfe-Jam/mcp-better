# SDK notes — honesty for implementers

## `serve` vs Discover (naming collision)

In `rmcp` 3.0:

| Call | Side | Meaning |
|------|------|---------|
| `service.serve(transport)` | **Server** entrypoint | Attach handler to stdio/HTTP transport and run. **Not** lifecycle Discover. |
| `ClientInfo::serve(transport)` | **Client** | **Legacy** lifecycle: `initialize` + `notifications/initialized`. GOOD-era default. |
| `ClientInfo::serve_with_lifecycle(..., Discover { .. })` | **Client** | **BETTER** path: `server/discover`, preferred `V_2026_07_28`. |

Official samples still often show `Calculator.serve(stdio())` on the **server**. That is correct for the server process. BETTER pairs it with a **Discover client** example and **stamped** list results — do not copy client `serve()` alone and call the setup modern.

## List cache is not automatic

`ServerHandler::list_tools` default / macro-generated paths often omit:

- `ttlMs`
- `cacheScope`

Wire omission is fine for old peers. For **2026-07-28 BETTER**, stamp explicitly:

```rust
ListToolsResult::with_all_items(tools)
    .with_ttl_ms(60_000)
    .with_cache_scope(CacheScope::Public)
```

This repo overrides `list_tools` and uses `BetterServer::stamped_list_tools()`.

## Claim surface by version

### v0.1 — 7/28 over stdio (foundation)

We claim:

- stdio transport  
- tools capability  
- Discover-compatible results  
- stamped list cache (`ttlMs` / `cacheScope`)  

We do **not** claim on v0.1:

- Streamable HTTP / `Mcp-Method` / `Mcp-Name`  
- Server-side “Discover API” beyond correct results under Discover clients  
- Resources, prompts, tasks, OAuth  

### v0.2 — same 7/28 era + Streamable HTTP

We claim **everything in v0.1**, plus:

- **Streamable HTTP** via `--http` (or env `MCP_TRANSPORT` / `MCP_BETTER_TRANSPORT=http`; `MCP_TRANSPORT` wins if both set)  
- Default bind **`127.0.0.1:8787/mcp`** · loopback **Host** guards  
- HTTP smoke with required routing headers **`Mcp-Method` / `Mcp-Name`**  
- Same tools (`health`, `echo`), same stamps, same era string `2026-07-28`  
- **SECURITY.md** posture: no auth/TLS — **local demo only**  

### v0.3 — deeper correctness

We claim **everything in v0.2**, plus:

- **Multi-list stability** — Discover stdio smoke asserts stamps + order across repeated `tools/list`  
- **Restart-stable order** — `order-restart-smoke` (two processes)  
- **Lying companion** — `mcp-worse` + `contrast-smoke` (better passes · worse fails)  
- Same tools on **mcp-better** (`health`, `echo`); worse is teaching-only  

### v0.4 — dual package (cargo + npm)

We claim **everything in v0.3**, plus:

- **npm shim** `mcp-better` downloads the native binary from GitHub Releases (`npx` / no Rust toolchain)  
- **`server.json` dual packages** — cargo + npm, both stdio, same version  
- Identity still **`io.github.Wolfe-Jam/mcp-better`** (not `one.faf/*`)

### v0.4.3 — MRTR + Agent Skills (live on registries)

We claim **everything in v0.4**, plus:

- Tool order **`health` → `echo` → `confirm_echo`**  
- **`confirm_echo`** — SEP-2322 MRTR textbook (mid-call confirm · sealed `requestState`)  
- Optional **Agent Skills** — extension `io.modelcontextprotocol/skills` · skill `mcp-better-lab`

### v0.4.4 — book matches 0.4.3 wire (this tree · not tagged)

Same claim surface as **v0.4.3**. No new tool. Docs / textbook name the catalog the wire already ships.  
Published crates.io / npm remain **0.4.3** until an explicit tag.

We do **not** claim on v0.4.3 / v0.4.4:

- Production remote HTTP (auth, TLS, open bind)  
- Registry package transport other than **stdio** (HTTP is opt-in in-binary, not a second Registry package)  
- Tasks, OAuth, or Skills as the product surface  
- Server-side Discover “API product” beyond correct wire results under Discover clients  
- That `mcp-worse` is a supported peer (it is intentionally dishonest)

**Rule:** versions **add road inside 7/28**. They do not rebrand each transport as a new protocol era.

## Pin

```toml
rmcp = { version = "3.0", features = ["server", "client", "transport-io", "transport-child-process", "macros", "schemars"] }
```

HTTP path uses the same `rmcp` 3.x stack with Streamable HTTP features enabled in the binary. Do **not** track `main` git deps for production pins.
