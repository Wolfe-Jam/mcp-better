//! mcp-better — modern MCP textbook (BETTER) — built for 7/28
//!
//! stdio tools server on protocol **2026-07-28**.
//! See README.md and BETTER.md for the claim surface (v0.1 = stdio + tools + list cache).

use mcp_better::BetterServer;
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

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
