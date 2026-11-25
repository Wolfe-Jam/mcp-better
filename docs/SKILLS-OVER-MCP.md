# Ship a skill with your MCP server

mcp-better serves one Agent Skill, `mcp-better-lab`, next to its tools, using the MCP Skills extension (`io.modelcontextprotocol/skills`, [SEP-2640](https://modelcontextprotocol.io/seps/2640-skills-extension)). This page walks through how it does that, with a link to each line, and ends with the four checks that prove a host loads the skill.

Tested on 2026-11-25: mcp-better 0.7.0 (rmcp 3.10) in Claude Code 2.1.332, interactive terminal session, macOS (Intel Core i5, 32 GB). Claude Code listed the skill as `mcp-better:mcp-better-lab` and loaded it.

```text
server/discover → skills/list → resources/read(skill://mcp-better-lab/SKILL.md) → tools/call
```

Skills guide; tools act. The skill tells the model how to use `health`, `echo` and `confirm_echo`; the tools stay the same.

## Two things a host checks before it asks for skills

1. **Protocol `2026-07-28`.** Claude Code opens with `server/discover` at revision `2026-07-28` and asks a server for skills only on a connection at that revision or later. Its debug log says so when it doesn't: `skills/list not sent: this connection is not known to use MCP protocol version 2026-07-28 or later`. mcp-better advertises the revision in [`get_info`](../src/server.rs#L332) and answers [`discover`](../src/server.rs#L349).
2. **A fast start.** When a server shows no Protocol row, Claude Code's troubleshooting table says: if a stdio server takes over 3 seconds to start, run the installed program instead of `npx`, then reconnect. mcp-better is a single Rust binary: it answered `server/discover` in 15 ms when measured directly, and Claude Code logged `Successfully connected (transport: stdio) in 1186ms`.

## 1. Write the skill

[`skills/mcp-better-lab/SKILL.md`](../skills/mcp-better-lab/SKILL.md) is a normal Agent Skill: YAML frontmatter with `name` and `description`, then the instructions. The `description` decides when a host loads it ("Use when connecting to mcp-better or testing its tools over protocol 2026-07-28"), so write it as the situation the skill is for. Name only tools your server actually lists.

## 2. Load it into a catalog

[`src/skills.rs`](../src/skills.rs) builds the catalog once:

- The file is embedded at compile time with [`include_str!`](../src/skills.rs#L26), so `cargo install` works with no `skills/` folder next to the binary, and the bytes served can never differ from the bytes hashed.
- [`load_lab`](../src/skills.rs#L50) checks that the frontmatter `name` equals the name in the URI, `skill://mcp-better-lab/SKILL.md`.
- The [digest](../src/skills.rs#L153) is `sha256:<hex>` over the whole file, frontmatter included, and the size is its byte length.
- [`parse_frontmatter`](../src/skills.rs#L168) requires `name` and `description`.

## 3. Declare the extension

In [`get_info`](../src/server.rs#L318), add `io.modelcontextprotocol/skills` to `capabilities.extensions` next to `tools` and `resources`, and set the protocol version to `2026-07-28`. Declare `resources` too: the skill's files are read with `resources/read`.

## 4. Answer `skills/list` and `skills/get`

rmcp 3.9+ has these as native `ServerHandler` methods:

- [`list_skills`](../src/server.rs#L425) returns every skill: its `SKILL.md` URI, the frontmatter as written, and a file list with each file's URI, digest and size ([`to_skill`](../src/skills.rs#L114)).
- [`get_skill`](../src/server.rs#L436) returns one skill by URI, or `invalid_params` naming the URI.
- Both stamp `ttlMs: 60000` and `cacheScope: public` ([`CATALOG_TTL_MS`](../src/server.rs#L35)). The skill is fixed at build time, so the server says hosts may cache it. rmcp's default is `ttlMs: 0` and `private`: correct, but it tells hosts not to cache.

## 5. Serve the files

- [`list_resources`](../src/server.rs#L385) lists each skill file with the skill's name and description, `text/markdown` and its size ([`list_resources_meta`](../src/skills.rs#L90)), so a host without the extension still sees it.
- [`read_resource`](../src/server.rs#L397) returns the file's text for its `skill://` URI, and `invalid_params` with the URI in `data.uri` for anything else.

The host compares what it reads with the digest and size from `skills/list`.

## 6. Prove it in a host: four checks

Run these in an interactive Claude Code session (v2.1.331 or later). `claude -p` can't load MCP skills.

```text
claude mcp add mcp-better -- /absolute/path/to/mcp-better
claude --debug
```

Use the installed binary by its absolute path, not `cargo run` or `npx`: both start too slowly.

| # | Check | What mcp-better showed |
|---|---|---|
| 1 | `/mcp` → the server shows a **Protocol** row | `Protocol: 2026-07-28` · `Capabilities: 3 tools · 1 resource · 1 skill` |
| 2 | The debug log shows `skills/list` was sent | `MCP server "mcp-better": Found 1 skill via skills/list` |
| 3 | `/mcp` → the server → View skills lists it as `<server>:<skill>` | `mcp-better:mcp-better-lab` |
| 4 | A prompt that fits the description loads it | Prompt: "Connect to the mcp-better server and test its tools over protocol 2026-07-28." Screen: `Skill(mcp-better:mcp-better-lab) · Loaded skill`. Log: `Loaded MCP skill 'mcp-better-lab' from skill://mcp-better-lab/SKILL.md` |

After loading the skill, Claude called the tools in the skill's order: `health`, `echo`, then `confirm_echo`, whose mid-call confirmation reached the user as a form (`Type CONFIRM to echo the sealed message.`).

If a check fails, start with the **Troubleshoot MCP skills** section of Claude Code's MCP docs. No Protocol row means the server isn't on `2026-07-28` or started too slowly.

## Where it works

From Claude Code's MCP docs, section "Use skills from MCP servers" (read 2026-11-25):

- **Supported:** the terminal (including JetBrains IDEs) and local sessions in the Desktop app; rolling out in the VS Code extension. Needs Claude Code v2.1.331 or later, a recent model, and a direct connection to Anthropic with feature-flag fetching on.
- **Not supported:** cloud sessions, `claude -p`, `claude remote-control` server mode, GitHub Actions and the Agent SDK; in the Desktop app, connectors, Desktop extensions and servers from `claude_desktop_config.json`; cloud providers, a custom `ANTHROPIC_BASE_URL`, `apiKeyHelper` and `ANTHROPIC_AUTH_TOKEN`; organizations with compliance settings such as HIPAA or zero data retention.

Check the current docs before relying on this list: the feature is in public beta.

## Security

The skill is public textbook data, not credentials, and nothing in it is a permission. Treat skill content from any server as untrusted input: a digest shows the bytes match what the server listed, not that the content is safe. Claude Code shows the same warning on its skills page: the provider can change its skills at any time, and a skill can direct Claude to run code.

## Not required

Dual-package authors (Rust crate + npm launcher) don't need skills. See the dual-package guides.
