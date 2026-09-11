# AGENTS.md

## What this is

`mcp-better` is a Rust MCP server (crate `mcp-better`), distributed two ways: directly via `cargo`/crates.io, and via an npm shim (`bin/mcp-better.js`) that downloads the native binary from GitHub Releases. Rust is the source of truth — the npm package does not reimplement anything.

Three tools: `health`, `echo`, `confirm_echo`. `confirm_echo` is the interesting one — it demonstrates SEP-2322 MRTR (`input_required` → client retry) with a sealed, tamper-checked `requestState` (see `src/server.rs`).

## Build

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release
```

## Test — two different questions, two different commands

`cargo test --all-targets` answers **"does the code work"** — unit tests, including the four `confirm_echo` round-trip tests co-located in `src/server.rs` (`confirm_echo_round1_input_required`, `confirm_echo_round2_complete_after_confirm`, `confirm_echo_rejects_wrong_confirm_text`, `confirm_echo_rejects_tampered_state`).

It does **not** answer **"does the tool still promise what it promised."** `cargo test --all-targets` compiles the `examples/` binaries as part of its build but runs **zero test functions in them** (each reports `0 passed; 0 failed`) — the actual contract check only happens if you separately run the binary. That's a distinct, named layer:

- `cargo run --example mrtr-client` — the real `confirm_echo` contract check: MRTR round 1→2, sealed `requestState` integrity, elicitation response shape.
- `cargo run --example contrast-smoke` — cross-checks against the sibling `mcp-worse` binary (better passes, worse fails, on purpose).
- `cargo run --example order-restart-smoke` — two-process ordering check.
- `cargo run --example http-smoke` — Streamable HTTP transport check.

**⚠️ `tests/conformance_smoke.rs`, despite its name, only asserts tool-listing metadata** (tool order, names, cache TTL). It does not cover `confirm_echo`'s actual request/response contract. A green `cargo test` is not proof the contract held — do not treat it as one.

**Any change to a tool's request/response shape** (new field, renamed key, changed schema, changed elicitation form) **requires updating `cargo test` AND the matching example above, in the same PR.** If you only touched `src/server.rs` and `cargo test` is green, you have not yet verified the contract — run the relevant example next.

Full gate, in the order CI actually runs it:

```bash
bash scripts/ci.sh
# doc-gate → fmt → clippy -D warnings → test → release build →
# BETTER-purity check → all four example-based contract smokes above
```

## Code style

- `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings` must be clean before a PR.
- **BETTER purity:** no `project.faf` or `.faf` on `main` — enforced by `scripts/ci.sh` (`test ! -f project.faf`). See `BETTER.md` / `CONTRIBUTING.md` for why.

## PR instructions

- Run `bash scripts/ci.sh` green, locally, before opening.
- See `CONTRIBUTING.md` for scope and PR norms; see `docs/PUBBETTER.md` for the release protocol (`/pubbetter` only — never publish by hand).
