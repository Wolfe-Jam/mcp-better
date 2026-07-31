//! Process-restart order smoke: two sequential server processes must list tools identically.
//!
//! Proves the static catalog order survives restart (not only same-process N lists).
//!
//!   cargo build --bins
//!   cargo run --example order-restart-smoke

use rmcp::model::{CacheScope, ClientInfo, ProtocolVersion};
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{ClientLifecycleMode, ClientServiceExt};
use tokio::process::Command;

fn resolve_better_bin() -> String {
    std::env::var("MCP_BETTER_BIN").unwrap_or_else(|_| {
        let mut path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("target/debug"));
        if path.ends_with("examples") {
            path.pop();
        }
        path.push("mcp-better");
        path.to_string_lossy().into_owned()
    })
}

async fn list_names(bin: &str) -> anyhow::Result<(Vec<String>, Option<u64>, Option<CacheScope>)> {
    let transport = TokioChildProcess::new(Command::new(bin).configure(|_c| {}))?;
    let client = ClientInfo::default()
        .serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await?;

    let list = client.list_tools(Default::default()).await?;
    let names: Vec<String> = list.tools.iter().map(|t| t.name.to_string()).collect();
    let ttl = list.ttl_ms;
    let scope = list.cache_scope;
    client.cancel().await?;
    Ok((names, ttl, scope))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let bin = resolve_better_bin();
    if !std::path::Path::new(&bin).is_file() {
        anyhow::bail!(
            "mcp-better binary not found at {bin}\nBuild first: cargo build --bins\nOr set MCP_BETTER_BIN"
        );
    }

    tracing::info!("process A: {bin}");
    let (names_a, ttl_a, scope_a) = list_names(&bin).await?;
    tracing::info!("process B: {bin}");
    let (names_b, ttl_b, scope_b) = list_names(&bin).await?;

    if names_a != ["health".to_string(), "echo".to_string()] {
        anyhow::bail!("process A order must be [health, echo], got {names_a:?}");
    }
    if names_a != names_b {
        anyhow::bail!("order not restart-stable: A={names_a:?} B={names_b:?}");
    }
    match ttl_a {
        Some(ms) if ms > 0 => {}
        other => anyhow::bail!("process A expected positive ttl_ms, got {other:?}"),
    }
    if ttl_a != ttl_b {
        anyhow::bail!("ttl_ms not restart-stable: A={ttl_a:?} B={ttl_b:?}");
    }
    if scope_a != Some(CacheScope::Public) || scope_b != Some(CacheScope::Public) {
        anyhow::bail!("cache_scope must be public both processes: A={scope_a:?} B={scope_b:?}");
    }

    println!("order-restart-smoke: OK (two processes · same order · same stamps)");
    Ok(())
}
