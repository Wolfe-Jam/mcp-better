//! mcp-better library surface — server handler + stamped list helpers.
//!
//! The binary (`src/main.rs`) is the stdio entrypoint. Tests and examples use this crate.

pub mod server;

pub use server::{BetterServer, LIST_TOOLS_TTL_MS};
