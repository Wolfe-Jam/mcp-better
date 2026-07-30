//! mcp-better — AAIF-verified modern MCP (BETTER textbook)
//!
//! **v0.2** — same 7/28 era, more road:
//! - default: **stdio**
//! - `--http`: Streamable HTTP (stateless 7/28 · local host guards)
//!
//! See README.md / BETTER.md.

use std::net::SocketAddr;
use std::sync::Arc;

use mcp_better::BetterServer;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let mode = transport_mode();
    match mode.as_str() {
        "http" => run_http().await,
        _ => run_stdio().await,
    }
}

fn transport_mode() -> String {
    // CLI wins over env
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--http" | "http" => return "http".into(),
            "--stdio" | "stdio" => return "stdio".into(),
            "--help" | "-h" => {
                eprintln!(
                    "mcp-better v{} — built for 7/28\n\n\
                     Usage:\n\
                       mcp-better              stdio (default)\n\
                       mcp-better --http       Streamable HTTP (local demo)\n\
                       mcp-better --stdio      force stdio\n\n\
                     Args (CLI wins over env):\n\
                       --http | http           Streamable HTTP\n\
                       --stdio | stdio         stdio\n\n\
                     Env:\n\
                       MCP_BETTER_HTTP_ADDR    default 127.0.0.1:8787 (loopback only recommended)\n\
                       MCP_BETTER_TRANSPORT    stdio | http  (alias: MCP_TRANSPORT)\n\
                       RUST_LOG                tracing filter\n\n\
                     Security: --http has no auth/TLS. Do not bind 0.0.0.0. See SECURITY.md.\n",
                    env!("CARGO_PKG_VERSION")
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }
    std::env::var("MCP_TRANSPORT")
        .or_else(|_| std::env::var("MCP_BETTER_TRANSPORT"))
        .unwrap_or_else(|_| "stdio".into())
        .to_lowercase()
}

async fn run_stdio() -> anyhow::Result<()> {
    tracing::info!(
        "mcp-better v{} — BETTER MCP (2026-07-28) stdio starting",
        env!("CARGO_PKG_VERSION")
    );

    let service = BetterServer::new()
        .serve(rmcp::transport::stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {e:?}");
        })?;

    service.waiting().await?;
    Ok(())
}

async fn run_http() -> anyhow::Result<()> {
    let addr: SocketAddr = std::env::var("MCP_BETTER_HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8787".into())
        .parse()
        .map_err(|e| anyhow::anyhow!("bad MCP_BETTER_HTTP_ADDR: {e}"))?;

    // Host header often includes port for non-80 binds
    let host_port = format!("{}:{}", addr.ip(), addr.port());
    let host_only = addr.ip().to_string();

    let config = StreamableHttpServerConfig::default()
        // 7/28 is always stateless; this flag only affects legacy peers
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_allowed_hosts([
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "::1".to_string(),
            host_only,
            host_port.clone(),
            format!("localhost:{}", addr.port()),
            format!("[::1]:{}", addr.port()),
        ]);

    let service = StreamableHttpService::new(
        || Ok(BetterServer::new()),
        Arc::new(LocalSessionManager::default()),
        config,
    );

    let app = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!(
        "mcp-better v{} — BETTER MCP (2026-07-28) Streamable HTTP on http://{addr}/mcp \
         (stateless for 7/28 · json_response · local Host guards)",
        env!("CARGO_PKG_VERSION")
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
            tracing::info!("shutdown signal");
        })
        .await?;

    Ok(())
}
