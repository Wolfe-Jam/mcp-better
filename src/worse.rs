//! **Lying companion** — deliberately violates the BETTER list contract.
//!
//! Teaching only. Same tool *names* as mcp-better, wrong operational surface:
//! - **no** `ttlMs` / `cacheScope` stamps
//! - **reversed** tool order (`echo` then `health`)
//!
//! Use with `examples/contrast-smoke` so learners see claim ≠ wire.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Intentional anti-order (BETTER is health → echo).
const WORSE_TOOL_ORDER: &[&str] = &["echo", "health"];

#[derive(Debug, Clone)]
pub struct WorseServer {
    tool_router: ToolRouter<Self>,
}

impl WorseServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Unstamped list with reversed order — the lie.
    pub fn lying_list_tools(&self) -> ListToolsResult {
        let mut tools = self.tool_router.list_all();
        tools.sort_by(|a, b| {
            let ia = WORSE_TOOL_ORDER
                .iter()
                .position(|n| *n == a.name.as_ref())
                .unwrap_or(usize::MAX);
            let ib = WORSE_TOOL_ORDER
                .iter()
                .position(|n| *n == b.name.as_ref())
                .unwrap_or(usize::MAX);
            ia.cmp(&ib).then_with(|| a.name.cmp(&b.name))
        });
        // Deliberately omit with_ttl_ms / with_cache_scope.
        ListToolsResult::with_all_items(tools)
    }
}

impl Default for WorseServer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EchoParams {
    #[schemars(description = "Text to echo back unchanged")]
    pub message: String,
}

#[tool_router]
impl WorseServer {
    #[tool(description = "Liveness check (lying demo server — not BETTER).")]
    fn health(&self) -> String {
        serde_json::json!({
            "status": "ok",
            "server": "mcp-worse",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "2026-07-28",
            "tier": "LYING-DEMO",
            "warning": "This binary deliberately fails the BETTER list contract for teaching.",
        })
        .to_string()
    }

    #[tool(description = "Echo a message back. Pure function; no side effects.")]
    fn echo(&self, Parameters(EchoParams { message }): Parameters<EchoParams>) -> String {
        message
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for WorseServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info = Implementation::new("mcp-worse", env!("CARGO_PKG_VERSION"));
        info.instructions = Some(
            "LYING DEMO — not BETTER. Same tool names as mcp-better but tools/list \
             omits ttlMs/cacheScope and reverses order. For contrast-smoke only. \
             Do not use in production or as a template."
                .into(),
        );
        info
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(self.lying_list_tools())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lying_list_is_unstamped_and_reversed() {
        let list = WorseServer::new().lying_list_tools();
        assert_eq!(list.ttl_ms, None);
        assert_eq!(list.cache_scope, None);
        let names: Vec<_> = list.tools.iter().map(|t| t.name.as_ref()).collect();
        assert_eq!(names, vec!["echo", "health"]);
    }
}
