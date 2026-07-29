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

## v0.1 claim surface

We claim:

- stdio transport  
- tools capability  
- Discover-compatible results  
- stamped list cache  

We do **not** claim on day 0:

- Streamable HTTP / `Mcp-Method` / `Mcp-Name`  
- Server-side “Discover API” beyond correct results under Discover clients  
- Resources, prompts, tasks, OAuth  

## Pin

```toml
rmcp = { version = "3.0", features = ["server", "client", "transport-io", "transport-child-process", "macros", "schemars"] }
```

Do not track `main` git deps for production pins.
