# Ship a skill with your MCP server

mcp-better serves one Agent Skill, `mcp-better-lab`, next to its tools, using the MCP Skills extension (`io.modelcontextprotocol/skills`, [SEP-2640](https://modelcontextprotocol.io/seps/2640-skills-extension)). This page walks through how it does that, shows what goes over the wire, and ends with the four checks that prove a host loads the skill. The code is Rust (rmcp); the wire format and the checks are the same in any language.

Tested on 2026-11-25: mcp-better 0.7.0 (rmcp 3.10) in Claude Code 2.1.332, interactive terminal session, macOS (Intel Core i5, 32 GB). Claude Code listed the skill as `mcp-better:mcp-better-lab` and loaded it.

```text
server/discover → skills/list → resources/read(skill://mcp-better-lab/SKILL.md) → tools/call
```

Skills guide; tools act. The skill tells the model how to use `health`, `echo` and `confirm_echo`; the tools stay the same.

## Before you start: your SDK must speak 2026-07-28

A host that supports Skills over MCP asks for skills only on a connection at protocol revision `2026-07-28` or later. Claude Code's debug log says so when it doesn't: `skills/list not sent: this connection is not known to use MCP protocol version 2026-07-28 or later`.

On `2026-07-28` the host opens with **`server/discover`**, not `initialize`. The request carries the protocol version and the client's capabilities in `_meta`; the server answers with the versions it supports and its capabilities. Every later request carries the same `_meta` fields, because the server keeps no session. If your server doesn't implement `server/discover`, the host falls back to `initialize` on an older revision and never asks for skills.

**The `initialize` trap:** if you test your server with a plain `initialize` asking for `2026-07-28`, rmcp answers `2025-11-25`. That doesn't mean the server can't do `2026-07-28`; that revision isn't negotiated through `initialize`. Test with `server/discover` (see [Test it yourself](#6-test-it-yourself-before-a-host-does)).

| SDK | `2026-07-28` | Skills support | Status |
|---|---|---|---|
| Rust, `rmcp` 3.10 | yes, `server/discover` | native `list_skills` / `get_skill` handler methods | mcp-better; measured |
| TypeScript, `@modelcontextprotocol/sdk` 1.x (1.32.2) | no: newest revision `2025-11-25`, `server/discover` answers "Method not found" | none; methods registered by hand | measured on a 1.x server, 2026-11-25 |
| TypeScript, `@modelcontextprotocol/server` 2.x (2.4.0) | yes, per Claude Code's docs ("MCP TypeScript SDK 2.0, which adds MCP protocol revision 2026-07-28") | `loadSkills(root)` and a `SkillsExtension` in `ext/skills` | read from the package; not run in a host here |
| Python | not checked | not checked | |

**A fast start matters too.** When a server shows no Protocol row, Claude Code's troubleshooting table says: if a stdio server takes over 3 seconds to start, run the installed program instead of `npx`, then reconnect. mcp-better is a single binary: it answered `server/discover` in 15 ms when measured directly, and Claude Code logged `Successfully connected (transport: stdio) in 1186ms`.

## 1. Write the skill

[`skills/mcp-better-lab/SKILL.md`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/skills/mcp-better-lab/SKILL.md) is a normal [Agent Skill](https://agentskills.io/specification): YAML frontmatter with `name` and `description`, then the instructions.

- **Name:** SEP-2640 requires the last segment of the skill's path to equal its `name`, as the Agent Skills spec requires `name` to equal the folder name. See the [Agent Skills spec](https://agentskills.io/specification#name-field) for the allowed characters and length.
- **Description:** it decides when a host loads the skill, so write it as the situation the skill is for. mcp-better's lab skill names its own server on purpose ("Use when connecting to mcp-better or testing its tools over protocol 2026-07-28"); a skill for real users should describe their task instead.
- **Tools:** name only tools your server actually lists.
- The `<!-- faf:start -->` block in mcp-better's SKILL.md is mcp-better's own metadata. The extension doesn't need it; leave it out of your skill.

## 2. Load it into a catalog

[`src/skills.rs`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs) builds the catalog once:

- The file is embedded at compile time with [`include_str!`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L26), so `cargo install` works with no `skills/` folder next to the binary, and the bytes served can never differ from the bytes hashed. **In other languages the rule is the same:** read the skill files from your own package, never from the working directory (that's the user's project), and make sure the folder ships (npm `files`, and check `.gitignore` isn't hiding `*.md`).
- [`load_lab`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L50) checks that the frontmatter `name` equals the name in the URI, `skill://mcp-better-lab/SKILL.md`.
- The [digest](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L153) is `sha256:<hex>` over the whole file's bytes, frontmatter included, and the size is its byte length.
- [`parse_frontmatter`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L168) requires `name` and `description`. **It is not a YAML parser:** it reads only one-line `key: value` pairs as strings. That's enough for this skill. For anything richer (quoted values, multi-line descriptions, nested `metadata:`), use a real YAML parser: the host parses your SKILL.md with one, and the frontmatter you list should match what it reads.

mcp-better serves one single-file skill, by design: the catalog labels each file as its skill's SKILL.md, and a test fails if a skill gains a second file. A skill with more files (`references/`, `scripts/`) lists every file with its own digest, size and media type, and can be browsed with `resources/directory/read` when the server declares `directoryRead`. See SEP-2640 for those rules; mcp-better doesn't exercise them.

## 3. Declare the extension

In [`get_info`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L318), add `io.modelcontextprotocol/skills` to `capabilities.extensions` next to `tools` and `resources`, and set the protocol version to `2026-07-28` ([line 332](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L332)). Declare `resources` too: the skill's files are read with `resources/read`. [`discover`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L349) answers `server/discover` from the same information. On the wire:

```json
{
  "supportedVersions": ["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25", "2026-07-28"],
  "capabilities": {
    "extensions": { "io.modelcontextprotocol/skills": {} },
    "resources": {},
    "tools": {}
  },
  "ttlMs": 60000,
  "cacheScope": "public"
}
```

## 4. Answer `skills/list` and `skills/get`

mcp-better uses rmcp 3.10, which has these as native `ServerHandler` methods:

- [`list_skills`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L425) returns every skill: its `SKILL.md` URI, the frontmatter, and a file list with each file's URI, digest and size ([`to_skill`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L114)).
- [`get_skill`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L436) returns one skill by URI, or `invalid_params` naming the URI.

The `skills/list` result mcp-better sends:

```json
{
  "resultType": "complete",
  "ttlMs": 60000,
  "cacheScope": "public",
  "skills": [
    {
      "uri": "skill://mcp-better-lab/SKILL.md",
      "frontmatter": {
        "name": "mcp-better-lab",
        "description": "Lab playbook for the mcp-better server — claim equals wire; calls health and echo, and the optional confirm_echo for MRTR. Use when connecting to mcp-better or testing its tools over protocol 2026-07-28."
      },
      "resources": [
        {
          "uri": "skill://mcp-better-lab/SKILL.md",
          "digest": "sha256:d4583c7999696433f755e1307a3a38be9401ee48bb450b519cdc4fff5098301c",
          "size": 1635
        }
      ]
    }
  ]
}
```

**Caching.** Both methods stamp `ttlMs: 60000` and `cacheScope: public` ([`CATALOG_TTL_MS`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L35)): the skill is fixed at build time and the same for everyone, so hosts may cache it. rmcp's default is `ttlMs: 0` and `private`. If your skills vary by user or account, keep `private`.

**Updates.** Changing a skill file changes its digest. Hosts may reuse a listing until its `ttlMs` runs out. In Claude Code, a user's "Allow this version" approval holds until the server changes the skill's files.

## 5. Serve the files

- [`list_resources`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L385) lists each skill file with the skill's name and description, `text/markdown` and its size ([`list_resources_meta`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L90)), so it also shows up as an ordinary resource. (It labels every file with the skill's name and `text/markdown`, which is right for a single SKILL.md; a multi-file skill needs a name and media type per file.)
- [`read_resource`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/server.rs#L397) returns the file's text for its `skill://` URI, and `invalid_params` with the URI in `data.uri` for anything else.

The host compares what it reads with the digest and size from `skills/list`. mcp-better's own test, [`lab_skill_loads_with_matching_digest`](https://github.com/Wolfe-Jam/mcp-better/blob/561356f/src/skills.rs#L207), checks the same thing: the served bytes hash to the listed digest and match the listed size. Write that test for your server too.

## 6. Test it yourself before a host does

Send the host's first two requests straight to your server over stdio. With mcp-better:

```bash
M='"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientInfo":{"name":"probe","version":"0"},"io.modelcontextprotocol/clientCapabilities":{"extensions":{"io.modelcontextprotocol/skills":{}}}}'
{ printf '%s\n' "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"server/discover\",\"params\":{$M}}" \
                "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"skills/list\",\"params\":{$M}}"; sleep 1; } | mcp-better
```

You should get the two results shown in steps 3 and 4. If `server/discover` comes back "Method not found", the server isn't on `2026-07-28` yet, and no host will ask it for skills.

## 7. Prove it in a host: four checks

Run these in an interactive Claude Code session (v2.1.331 or later). `claude -p` can't load MCP skills.

```text
claude mcp add mcp-better -- /absolute/path/to/mcp-better
claude --debug
```

Register the installed binary by its absolute path. Claude Code's docs advise the installed program over `npx` for stdio servers; `cargo run` would also check the build before starting. `claude --debug` prints the path of its log file at the top of the session (`~/.claude/debug/<session>.txt`); check 2 is a line in that file.

| # | Check | What mcp-better showed |
|---|---|---|
| 1 | `/mcp` → the server shows a **Protocol** row | `Protocol: 2026-07-28` · `Capabilities: 3 tools · 1 resource · 1 skill` |
| 2 | The debug log shows `skills/list` was sent | `MCP server "mcp-better": Found 1 skill via skills/list` |
| 3 | `/mcp` → the server → View skills lists it as `<server>:<skill>` | `mcp-better:mcp-better-lab` |
| 4 | A prompt that fits the description loads it | Prompt: "Connect to the mcp-better server and test its tools over protocol 2026-07-28." Screen: `Skill(mcp-better:mcp-better-lab) · Loaded skill`. Log: `Loaded MCP skill 'mcp-better-lab' from skill://mcp-better-lab/SKILL.md` |

After loading the skill, Claude called the tools in the skill's order: `health`, `echo`, then `confirm_echo`, whose mid-call confirmation reached the user as a form (`Type CONFIRM to echo the sealed message.`).

If a check fails, start with the **Troubleshoot MCP skills** section of [Claude Code's MCP docs](https://code.claude.com/docs/en/mcp). No Protocol row means the server isn't on `2026-07-28` or started too slowly.

## Where it works

From Claude Code's MCP docs, section "Use skills from MCP servers" (read 2026-11-25):

- **Supported:** the terminal (including JetBrains IDEs) and local sessions in the Desktop app; rolling out in the VS Code extension. Needs Claude Code v2.1.331 or later, a recent model, and a direct connection to Anthropic with feature-flag fetching on.
- **Plans:** on for Pro, Max and Ultra when you sign in with `/login`; off for Team and Enterprise until the organization turns them on; on with a long-lived token; rolling out (off for now) with an API key or a Console login.
- **Not supported:** cloud sessions, `claude -p`, `claude remote-control` server mode, GitHub Actions and the Agent SDK; in the Desktop app, connectors, Desktop extensions and servers from `claude_desktop_config.json`; cloud providers, a custom `ANTHROPIC_BASE_URL`, `apiKeyHelper` and `ANTHROPIC_AUTH_TOKEN`; organizations with compliance settings such as HIPAA or zero data retention.

Check the current docs before relying on this list: the feature is in public beta.

## Security

The skill is public textbook data, not credentials, and nothing in it is a permission. Treat skill content from any server as untrusted input: a digest shows the bytes match what the server listed, not that the content is safe. Claude Code shows the same warning on its skills page: the provider can change its skills at any time, and a skill can direct Claude to run code.

SEP-2640's security rules for hosts, worth knowing as a server author:

- Hosts must ignore `allowed-tools` in an MCP-served skill unless the user has explicitly approved that grant for that skill. Don't rely on it to give your skill tools.
- Hosts must not let MCP-served skill content cause host-side code execution without explicit per-skill user approval.

The skill drives `confirm_echo`, whose mid-call confirmation carries a `requestState` token. That token is signed, not encrypted: tampering is caught, but anyone can read what's inside, so never put private data in it. And on 2026-07-28 the server keeps no session, so a token can be used more than once ([#12](https://github.com/Wolfe-Jam/mcp-better/issues/12)). That's harmless for an echo; before copying the pattern to a tool with real effects, add an expiry or a one-time nonce.
