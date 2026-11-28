# SDK notes — honesty for implementers

## `serve` vs Discover (naming collision)

In `rmcp` 3.10 (`ClientConfig` is the current name; `ClientInfo` is a deprecated alias):

| Call | Side | Meaning |
|------|------|---------|
| `service.serve(transport)` | **Server** entrypoint | Attach handler to stdio/HTTP transport and run. **Not** lifecycle Discover. |
| `ClientConfig::serve(transport)` | **Client** | **Legacy** lifecycle: `initialize` + `notifications/initialized`. GOOD-era default. |
| `ClientConfig::serve_with_lifecycle(..., Discover { .. })` | **Client** | **BETTER** path: `server/discover`, preferred `V_2026_07_28`. |

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

### v0.4.4 — book matches 0.4.3 wire

Same claim surface as **v0.4.3**. No new tool. Docs / textbook name the catalog the wire already ships.

### v0.5.0 — matching client completes MRTR (live)

We claim **everything in v0.4.3**, plus:

- **`mrtr-client` example** — Discover + `confirm_echo` R1→R2 via `call_tool_once`  
- Fail paths: wrong confirm text · tampered `requestState`  
- No fourth tool. No new protocol era.

Published crates.io / npm / GH Release = **0.5.0**.

### v0.6.0 — the official MCP conformance suite in CI

We claim **everything in v0.5.0**, plus:

- Cache hints by choice on every list and read the server answers (tools, resources, templates, `resources/read`, skills)
- Prompts and completions not declared, so not answered (`-32601`); rmcp's defaults answered both
- Skills on rmcp's native API (3.9+), `size` per file, `text/markdown` read as listed
- CI runs the official conformance suite: the conformance build (`conformance/`) passes every check; the textbook binary passes every scored check that needs no suite fixture
- `rmcp` 3.10 · `rust-version` 1.88

### v0.7.0 — tools say what they do

We claim **everything in v0.6.0**, plus:

- Every tool sets all four annotation hints through the `#[tool(annotations(...))]` attribute: `readOnlyHint: true`, `destructiveHint: false`, `idempotentHint: true`, `openWorldHint: false`
- `mcp-better --version` (and `-V`) prints `mcp-better <version>` on stdout and exits 0, wherever it appears; `mcp-worse --version` likewise
- An unknown argument or `MCP_TRANSPORT` / `MCP_BETTER_TRANSPORT` value exits 2 with the reason; nothing falls back to stdio silently (`mcp_better::cli`)
- `server/discover` overrides the SDK default (`ttlMs: 0` + `private`) with the catalog's `ttlMs=60000` + `public`

### v0.8.0 — confirmations that hold

We claim **everything in v0.7.0**, plus:

- `confirm_echo` seals round 1 with a 5-minute expiry (rmcp `SealOptions::ttl`); an expired, tampered or wrong-tool `requestState` gets one `-32602`
- Round 2 with a `message` different from round 1 is refused (`-32602`), not answered with the sealed one
- SKILL.md frontmatter loads only one-line plain-string values; anything a YAML parser reads differently is refused at load
- One file per skill, by design, held by a test

Not claimed: a one-time token. Within 5 minutes a round-1 token can complete round 2 more than once (no session on 7/28).

### v0.9.0 — a plugin from the registry

We claim **everything in v0.8.0**, plus:

- `mcp-better plugin <registry-name>` writes `plugin.json` + `mcp.json` (Agent Plugins 1.0.0) from the MCP Registry entry's latest version; `--from server.json` works offline
- Secrets, `{variable}` values and required values with no default are listed, never written
- `--check DIR` exits 1 only when the plugin no longer starts what the registry lists (another package or runner, a stale pin, another host)

The server's MCP surface is unchanged from v0.8.0: same tools, resources and Skills.

We do **not** claim on v0.4.3 / v0.5.0 / v0.6.0 / v0.7.0 / v0.8.0 / v0.9.0:

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
