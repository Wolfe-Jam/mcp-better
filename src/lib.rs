//! mcp-better library surface — server handler + stamped list helpers.
//!
//! Process entry is the binary (`src/main.rs`): **stdio** by default, **Streamable HTTP** via `--http`.
//! Unit tests, integration tests, and examples depend on this crate (not the binary).
//!
//! Companion **`mcp-worse`** (lying demo) is a separate binary — see `src/bin/mcp_worse.rs`.

pub mod server;
pub mod worse;

pub use server::{BetterServer, BETTER_TOOL_ORDER, LIST_TOOLS_TTL_MS};
pub use worse::WorseServer;
