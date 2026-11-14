//! Streamable HTTP settings for `mcp-better --http`, shared with the conformance build.

use std::net::SocketAddr;

use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;

/// Default bind for `--http` (loopback only).
pub const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:8787";

/// The bind address from `MCP_BETTER_HTTP_ADDR`, or [`DEFAULT_HTTP_ADDR`].
pub fn http_addr() -> anyhow::Result<SocketAddr> {
    std::env::var("MCP_BETTER_HTTP_ADDR")
        .unwrap_or_else(|_| DEFAULT_HTTP_ADDR.into())
        .parse()
        .map_err(|e| anyhow::anyhow!("bad MCP_BETTER_HTTP_ADDR: {e}"))
}

/// Stateless for 2026-07-28 peers, JSON responses, and Host guards for local binds.
pub fn http_config(addr: SocketAddr) -> StreamableHttpServerConfig {
    StreamableHttpServerConfig::default()
        // 7/28 is always stateless; this flag only affects legacy peers
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_allowed_hosts([
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "::1".to_string(),
            addr.ip().to_string(),
            format!("{}:{}", addr.ip(), addr.port()),
            format!("localhost:{}", addr.port()),
            format!("[::1]:{}", addr.port()),
        ])
}
