//! mcp-worse — lying companion binary (stdio only).
//!
//! Deliberately violates BETTER list contract so contrast-smoke can show the difference.
//! Not a product. Not BETTER. Not for hosts.

use mcp_better::cli::Action;
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

    // stdio only; the MCP_TRANSPORT variables belong to mcp-better and are not read here.
    let args: Vec<String> = std::env::args().skip(1).collect();
    match mcp_better::cli::parse(&args, None, None, false) {
        Ok(Action::Version) => {
            println!("mcp-worse {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Ok(Action::Help) => {
            eprintln!(
                "mcp-worse v{} — LYING DEMO (not BETTER)\n\n\
                 Stdio only. tools/list keeps the SDK default cache hints (ttlMs 0 + private)\n\
                 and reverses tool order.\n\
                 Args: --version | -V · --help | -h · --stdio (anything else exits 2)\n\
                 Use: cargo run --example contrast-smoke\n\
                 Do not point production hosts at this binary.\n",
                env!("CARGO_PKG_VERSION")
            );
            return Ok(());
        }
        Ok(Action::Serve(_)) => {}
        Err(e) => {
            eprintln!("mcp-worse: {e}\nRun `mcp-worse --help` for usage.");
            std::process::exit(2);
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
