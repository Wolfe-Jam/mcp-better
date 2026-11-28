//! mcp-better — modern MCP (BETTER textbook)
//!
//! **v0.2** — same 7/28 era, more road:
//! - default: **stdio**
//! - `--http`: Streamable HTTP (stateless 7/28 · local host guards)
//!
//! See README.md / BETTER.md.

use std::net::SocketAddr;
use std::sync::Arc;

use mcp_better::cli::{Action, Transport};
use mcp_better::BetterServer;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpService,
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

    // `mcp-better plugin …`: registry entry → Agent Plugins folder (see `plugin_cli`).
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("plugin") {
        use mcp_better::plugin_cli::{self, Cmd};
        let code = match plugin_cli::parse(&args[1..]) {
            Ok(Cmd::Version) => {
                println!("mcp-better {}", env!("CARGO_PKG_VERSION"));
                0
            }
            Ok(Cmd::Help) => {
                eprint!("{}", plugin_cli::help());
                0
            }
            Ok(Cmd::Run(a)) => {
                let registry = std::env::var("MCP_REGISTRY_URL")
                    .ok()
                    .filter(|v| !v.trim().is_empty())
                    .unwrap_or_else(|| plugin_cli::DEFAULT_REGISTRY.to_string());
                plugin_cli::run(a, &registry)
            }
            Err(e) => {
                eprintln!("mcp-better plugin: {e}\nRun `mcp-better plugin --help` for usage.");
                2
            }
        };
        std::process::exit(code);
    }

    match mcp_better::cli::from_env(true) {
        Ok(Action::Version) => println!("mcp-better {}", env!("CARGO_PKG_VERSION")),
        Ok(Action::Help) => eprint!("{}", help()),
        Ok(Action::Serve(Transport::Http)) => run_http().await?,
        Ok(Action::Serve(Transport::Stdio)) => run_stdio().await?,
        Err(e) => {
            eprintln!("mcp-better: {e}\nRun `mcp-better --help` for usage.");
            std::process::exit(2);
        }
    }
    Ok(())
}

fn help() -> String {
    format!(
        "mcp-better v{} — built for 7/28\n\n\
         Usage:\n\
           mcp-better              stdio (default)\n\
           mcp-better --http       Streamable HTTP (local demo)\n\
           mcp-better --stdio      force stdio\n\
           mcp-better --version    print the version and exit\n\
           mcp-better plugin …     registry entry → Agent Plugins folder (plugin --help)\n\n\
         Args (CLI wins over env; anything else is an error, exit 2):\n\
           --http | http           Streamable HTTP\n\
           --stdio | stdio         stdio\n\
           --version | -V          version\n\
           --help | -h             this help\n\n\
         Env:\n\
           MCP_BETTER_HTTP_ADDR    default 127.0.0.1:8787 (loopback only recommended)\n\
           MCP_TRANSPORT           stdio | http  (read first if both set)\n\
           MCP_BETTER_TRANSPORT    same values (fallback)\n\
           RUST_LOG                tracing filter\n\n\
         Security: --http has no auth/TLS. Do not bind 0.0.0.0. See SECURITY.md.\n",
        env!("CARGO_PKG_VERSION")
    )
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
    let addr: SocketAddr = mcp_better::http_addr()?;
    let config = mcp_better::http_config(addr);

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
