//! mcp-better library surface — server handler + stamped list helpers.
//!
//! Process entry is the binary (`src/main.rs`): **stdio** by default, **Streamable HTTP** via `--http`.
//! Unit tests, integration tests, and examples depend on this crate (not the binary).

pub mod server;

pub use server::{BetterServer, LIST_TOOLS_TTL_MS};
