# Security

## v0.1 threat posture (stdio)

| Threat | Posture |
|--------|---------|
| Untrusted tool args | Schema-validated via `rmcp` + schemars; `echo` is pure |
| Path / exec tools | **Out of scope** — no shell, no filesystem tools |
| Session assumptions | stdio process-local; no multi-session correctness claim |
| Secrets in logs | Logs on stderr only; tools return no secrets |
| Supply chain | Pin `rmcp` 3.0 · commit `Cargo.lock` · CI |

## v0.2 Streamable HTTP (local demo)

`--http` is a **local textbook transport**, not a production multi-tenant server.

| Threat | Posture |
|--------|---------|
| Auth / TLS | **None.** No tokens, no mTLS, no OAuth. Anyone who can reach the bind address can call tools. |
| Bind address | Default **`127.0.0.1:8787`**. Do **not** set `MCP_BETTER_HTTP_ADDR=0.0.0.0:…` (or a public interface) unless you fully understand you are exposing an **unauthenticated** MCP endpoint. |
| Host guards | Loopback-oriented `Host` allowlist (localhost / 127.0.0.1 / ::1 + bound host:port). This is **not** a network ACL — it does not replace bind restriction or a reverse proxy. |
| Session mode | Stateless for 7/28 (`legacy_session_mode(false)`). No multi-session isolation claim. |
| SSRF / open proxy | Server does not fetch arbitrary URLs; tools are local (`health`, `echo`, `confirm_echo`). Still treat any non-loopback bind as hostile. |
| Production use | **Out of v0.2.** Use a proper edge (TLS, auth, rate limits) if you front an MCP server for real clients. |

### Operator checklist (HTTP)

1. Prefer loopback only: `MCP_BETTER_HTTP_ADDR=127.0.0.1:8787`
2. Never advertise `--http` as “safe for the internet”
3. Registry package transport remains **stdio** — see README “Registry identity”

## mcp-worse (lying companion)

`mcp-worse` is a **teaching binary** that deliberately omits list stamps and reverses tool order. It is **not** a secure or honest MCP peer. Do not expose it on a network. Do not point production hosts at it. Use only with `contrast-smoke` / local curriculum.

## Reporting

Open a private security advisory on the GitHub repository when available, or contact the maintainer listed in `Cargo.toml`.

Do not file public issues for exploitable protocol bugs until coordinated disclosure is agreed.
