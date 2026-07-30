//! Crate-boundary checks via the **public lib** (`mcp_better::…`).
//!
//! | Layer | Where |
//! |-------|--------|
//! | Unit (stamp · order · tool count) | `src/server.rs` `#[cfg(test)]` |
//! | Lib re-export / constant | **this file** |
//! | Wire Discover (stdio) | `examples/stdio-client` |
//! | Wire Streamable HTTP | `examples/http-smoke` |
//!
//! Intentionally thin — not a second copy of the unit suite.

use mcp_better::{BetterServer, LIST_TOOLS_TTL_MS};
use rmcp::model::CacheScope;

#[test]
fn public_lib_exports_list_ttl_one_minute() {
    assert_eq!(LIST_TOOLS_TTL_MS, 60_000);
}

/// Crate-boundary recheck: consumers of the lib see the same BETTER stamps as unit tests.
#[test]
fn public_lib_stamped_list_contract() {
    let server = BetterServer::new();
    let list = server.stamped_list_tools();
    assert_eq!(list.ttl_ms, Some(LIST_TOOLS_TTL_MS));
    assert_eq!(list.cache_scope, Some(CacheScope::Public));
    let names: Vec<_> = list.tools.iter().map(|t| t.name.to_string()).collect();
    assert_eq!(names, vec!["health".to_string(), "echo".to_string()]);
}
