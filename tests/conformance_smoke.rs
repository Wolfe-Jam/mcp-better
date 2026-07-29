//! Conformance stubs T3–T8 (unit-level; full wire Discover is examples/stdio-client).

use mcp_better::server::{BetterServer, LIST_TOOLS_TTL_MS};
use rmcp::model::CacheScope;

// Re-export server module for integration tests via lib — we test via binary crate.
// Until a lib target exists, these tests live next to src unit tests.
// This file exercises the public binary crate layout via path include pattern.

// Prefer `cargo test` unit tests in src/server.rs for stamp/order.
// This integration file verifies the binary builds and the public constants.

#[test]
fn list_ttl_constant_is_one_minute() {
    assert_eq!(LIST_TOOLS_TTL_MS, 60_000);
}

#[test]
fn stamped_list_public() {
    let server = BetterServer::new();
    let list = server.stamped_list_tools();
    assert_eq!(list.ttl_ms, Some(LIST_TOOLS_TTL_MS));
    assert_eq!(list.cache_scope, Some(CacheScope::Public));
    let names: Vec<_> = list.tools.iter().map(|t| t.name.to_string()).collect();
    assert_eq!(names, vec!["health".to_string(), "echo".to_string()]);
}
