# Skills over MCP (J1 textbook)

**mcp-better** serves one Agent Skill so a client can:

```text
initialize → skills/list → resources/read(SKILL.md) → tools/call
```

Same process as tools. Textbook receipt. **Not** a registry/ORAS publish.

## Advertise

On initialize, `capabilities.extensions`:

```json
"io.modelcontextprotocol/skills": {}
```

Also advertises `capabilities.resources` so `resources/read` is in-band.

## Methods

| Method | How |
|--------|-----|
| `skills/list` | Custom request (rmcp has no first-class skills API yet) |
| `skills/get` | Custom request · params `{ "uri": "skill://…" }` |
| `resources/list` | Lists skill file URIs |
| `resources/read` | Returns SKILL.md text · SHA-256 matches `resources[].digest` |

## Skill on disk / embed

```text
skills/mcp-better-lab/SKILL.md
```

- Frontmatter: `name`, `description` (required)
- URI: `skill://mcp-better-lab/SKILL.md` (path name = frontmatter `name`)
- Digest: `sha256:<hex>` over full file bytes (including frontmatter)

Embedded at compile time (`include_str!`) so `cargo install` still works without a nearby `skills/` tree.

## Tools

Unchanged: `health` · `echo` · `confirm_echo`. Skills **guide**; tools **act**.

## Security posture

Skill body is published textbook data — not host execution, not secret material. Do not treat remote `allowed-tools` as authority.

## Not required

Dual-package authors do **not** need skills. See dual-package guides for launch-only path.
