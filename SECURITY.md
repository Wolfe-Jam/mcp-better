# Security

## v0.1 threat posture

| Threat | Posture |
|--------|---------|
| Untrusted tool args | Schema-validated via `rmcp` + schemars; `echo` is pure |
| Path / exec tools | **Out of v0.1** — no shell, no filesystem tools |
| Session assumptions | stdio process-local; no multi-session correctness claim |
| Secrets in logs | Logs on stderr only; tools return no secrets |
| Supply chain | Pin `rmcp` 3.0 · commit `Cargo.lock` · CI |

## Reporting

Open a private security advisory on the GitHub repository when available, or contact the maintainer listed in `Cargo.toml`.

Do not file public issues for exploitable protocol bugs until coordinated disclosure is agreed.
