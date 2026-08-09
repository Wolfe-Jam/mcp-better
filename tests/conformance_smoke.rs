//! Crate-boundary checks via the **public lib** (`mcp_better::…`).
//!
//! | Layer | Where |
//! |-------|--------|
//! | Unit (stamp · order · tool count) | `src/server.rs` `#[cfg(test)]` |
//! | Lib re-export / constant | **this file** |
//! | Wire Discover (stdio) | `examples/stdio-client` |
//! | Wire Streamable HTTP | `examples/http-smoke` |
//! | Restart order | `examples/order-restart-smoke` |
//! | Lying contrast | `examples/contrast-smoke` + `mcp-worse` |
//!
//! Intentionally thin — not a second copy of the unit suite.

use mcp_better::{BetterServer, WorseServer, BETTER_TOOL_ORDER, LIST_TOOLS_TTL_MS};
use rmcp::model::CacheScope;

#[test]
fn public_lib_exports_list_ttl_one_minute() {
    assert_eq!(LIST_TOOLS_TTL_MS, 60_000);
}

#[test]
fn public_lib_exports_better_tool_order() {
    assert_eq!(BETTER_TOOL_ORDER, &["health", "echo", "confirm_echo"]);
}

/// Crate-boundary recheck: consumers of the lib see the same BETTER stamps as unit tests.
#[test]
fn public_lib_stamped_list_contract() {
    let server = BetterServer::new();
    let list = server.stamped_list_tools();
    assert_eq!(list.ttl_ms, Some(LIST_TOOLS_TTL_MS));
    assert_eq!(list.cache_scope, Some(CacheScope::Public));
    let names: Vec<_> = list.tools.iter().map(|t| t.name.to_string()).collect();
    assert_eq!(
        names,
        vec![
            "health".to_string(),
            "echo".to_string(),
            "confirm_echo".to_string()
        ]
    );
}

/// Lying companion is exported and fails the BETTER list contract on purpose.
#[test]
fn public_lib_worse_is_unstamped_and_reversed() {
    let list = WorseServer::new().lying_list_tools();
    assert_eq!(list.ttl_ms, None);
    assert_eq!(list.cache_scope, None);
    let names: Vec<_> = list.tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(names, vec!["echo", "health"]);
}
