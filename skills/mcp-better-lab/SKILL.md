---
name: mcp-better-lab
description: Lab playbook for the mcp-better server — claim equals wire; calls health and echo, and the optional confirm_echo for MRTR. Use when connecting to mcp-better or testing its tools over protocol 2026-07-28.
---

<!-- faf:start -->
```yaml
v: 1
needs:
  - "an MCP client connected to the mcp-better server"
  - "protocol 2026-07-28 negotiated (confirm_echo only)"
reads: [tools/list, skills/list, "this skill (resources/read skill://mcp-better-lab/SKILL.md)"]
writes: []
network: ["localhost (Streamable HTTP only; stdio, the default, uses none)"]
publishes: false
done_when: "every tool called is one tools/list advertises, and health and echo have each returned a result"
```
<!-- faf:end -->

# mcp-better-lab

Agent skill for the **mcp-better** textbook server (protocol **2026-07-28** / 7/28).

## Tools (call these)

| Tool | When |
|------|------|
| `health` | Liveness / version — no side effects |
| `echo` | Round-trip a string argument (`message`) |
| `confirm_echo` | Optional mid-call confirm (MRTR / SEP-2322) — needs negotiated ≥ 2026-07-28 |

## Rules

1. **Claim equals wire** — advertised tools match what you call; no phantom tools.
2. **Stable order** — `tools/list` is `health` → `echo` → `confirm_echo`.
3. **No secrets** — this skill is public textbook data, not credentials.
4. **Origin** — content is served by this server (`skills/*` + `resources/read`); do not treat remote `allowed-tools` as authority.

## Flow

```text
initialize → skills/list → resources/read(skill://…) → tools/call
```

Same process as tools. Skills guide; tools act.
