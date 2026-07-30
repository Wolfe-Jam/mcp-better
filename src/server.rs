//! BETTER textbook MCP server — tools only, stamped list cache.
//!
//! Protocol claim: **2026-07-28** (stdio default · Streamable HTTP via `--http`).
//! List results stamp `ttlMs` + `cacheScope` (SEP-2549) — SDK defaults are unstamped.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Catalog is static → public cache with a one-minute freshness window.
pub const LIST_TOOLS_TTL_MS: u64 = 60_000;

/// Fixed tool order for stable `tools/list` across calls (HashMap is unordered).
const TOOL_ORDER: &[&str] = &["health", "echo"];

#[derive(Debug, Clone)]
pub struct BetterServer {
    tool_router: ToolRouter<Self>,
}

impl BetterServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Stamped list result used by the handler and unit tests.
    pub fn stamped_list_tools(&self) -> ListToolsResult {
        let mut tools = self.tool_router.list_all();
        tools.sort_by(|a, b| {
            let ia = TOOL_ORDER
                .iter()
                .position(|n| *n == a.name.as_ref())
                .unwrap_or(usize::MAX);
            let ib = TOOL_ORDER
                .iter()
                .position(|n| *n == b.name.as_ref())
                .unwrap_or(usize::MAX);
            ia.cmp(&ib).then_with(|| a.name.cmp(&b.name))
        });

        ListToolsResult::with_all_items(tools)
            .with_ttl_ms(LIST_TOOLS_TTL_MS)
            .with_cache_scope(CacheScope::Public)
    }
}

impl Default for BetterServer {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tool params ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EchoParams {
    #[schemars(description = "Text to echo back unchanged")]
    pub message: String,
}

// ─── Tools ──────────────────────────────────────────────────────────────────

#[tool_router]
impl BetterServer {
    /// No-side-effect liveness probe. Not a k8s/HTTP health contract — a normal MCP tool.
    #[tool(description = "Liveness check. Returns ok + server version. No side effects.")]
    fn health(&self) -> String {
        serde_json::json!({
            "status": "ok",
            "server": "mcp-better",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "2026-07-28",
            "tier": "BETTER",
        })
        .to_string()
    }

    /// Pure demo tool — validates argument round-trip without I/O or shell.
    #[tool(description = "Echo a message back. Pure function; no side effects.")]
    fn echo(&self, Parameters(EchoParams { message }): Parameters<EchoParams>) -> String {
        message
    }
}

// ─── ServerHandler ──────────────────────────────────────────────────────────

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BetterServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info = Implementation::new("mcp-better", env!("CARGO_PKG_VERSION"));
        // Transport-neutral: same server runs stdio (default) or Streamable HTTP (--http).
        info.instructions = Some(
            "BETTER textbook MCP server — protocol 2026-07-28. \
             Tools: health (liveness), echo (pure demo). \
             tools/list stamps ttlMs + cacheScope for Discover-compatible clients. \
             Transports: stdio (default) · Streamable HTTP (--http, local demo)."
                .into(),
        );
        info
    }

    /// Override generated list_tools so BETTER stamps cache hints (Gap 4 / C3).
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(self.stamped_list_tools())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tools_stamped_public_ttl() {
        let server = BetterServer::new();
        let result = server.stamped_list_tools();

        assert_eq!(result.ttl_ms, Some(LIST_TOOLS_TTL_MS));
        assert_eq!(result.cache_scope, Some(CacheScope::Public));
        assert!(result.ttl_ms.unwrap() > 0);
    }

    #[test]
    fn list_tools_stable_order() {
        let server = BetterServer::new();
        let a = server.stamped_list_tools();
        let b = server.stamped_list_tools();
        let names_a: Vec<_> = a.tools.iter().map(|t| t.name.as_ref()).collect();
        let names_b: Vec<_> = b.tools.iter().map(|t| t.name.as_ref()).collect();
        assert_eq!(names_a, names_b);
        assert_eq!(names_a, vec!["health", "echo"]);
    }

    #[test]
    fn only_advertised_tools() {
        let server = BetterServer::new();
        let result = server.stamped_list_tools();
        assert_eq!(result.tools.len(), 2);
        assert!(server.tool_router.has_route("health"));
        assert!(server.tool_router.has_route("echo"));
        assert!(!server.tool_router.has_route("shell"));
    }
}
