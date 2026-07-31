//! mcp-worse — lying companion binary (stdio only).
//!
//! Deliberately violates BETTER list contract so contrast-smoke can show the difference.
//! Not a product. Not BETTER. Not for hosts.

use mcp_better::WorseServer;
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    for a in std::env::args().skip(1) {
        if matches!(a.as_str(), "--help" | "-h") {
            eprintln!(
                "mcp-worse v{} — LYING DEMO (not BETTER)\n\n\
                 Stdio only. tools/list omits ttlMs/cacheScope and reverses tool order.\n\
                 Use: cargo run --example contrast-smoke\n\
                 Do not point production hosts at this binary.\n",
                env!("CARGO_PKG_VERSION")
            );
            std::process::exit(0);
        }
    }

    tracing::warn!(
        "mcp-worse v{} starting — intentional BETTER contract violations (teaching only)",
        env!("CARGO_PKG_VERSION")
    );

    let service = WorseServer::new()
        .serve(rmcp::transport::stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {e:?}");
        })?;

    service.waiting().await?;
    Ok(())
}
