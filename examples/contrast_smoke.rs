//! Contrast smoke: mcp-better passes the BETTER list contract; mcp-worse fails
//! **each** teaching clause (unstamped · names≠better). Partial companion decay
//! fails closed and names the clause.
//!
//!   cargo build --bins
//!   cargo run --example contrast-smoke

use rmcp::model::{CacheScope, ClientInfo, ProtocolVersion};
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{ClientLifecycleMode, ClientServiceExt};
use tokio::process::Command;

struct ListProbe {
    names: Vec<String>,
    ttl_ms: Option<u64>,
    cache_scope: Option<CacheScope>,
}

fn resolve_bin(env_key: &str, default_name: &str) -> String {
    std::env::var(env_key).unwrap_or_else(|_| {
        let mut path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("target/debug"));
        if path.ends_with("examples") {
            path.pop();
        }
        path.push(default_name);
        path.to_string_lossy().into_owned()
    })
}

async fn probe(bin: &str) -> anyhow::Result<ListProbe> {
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
    let probe = ListProbe {
        names: list.tools.iter().map(|t| t.name.to_string()).collect(),
        ttl_ms: list.ttl_ms,
        cache_scope: list.cache_scope,
    };
    client.cancel().await?;
    Ok(probe)
}

fn better_names() -> [String; 3] {
    [
        "health".to_string(),
        "echo".to_string(),
        "confirm_echo".to_string(),
    ]
}

fn is_better_contract(p: &ListProbe) -> bool {
    p.names == better_names()
        && matches!(p.ttl_ms, Some(ms) if ms > 0)
        && p.cache_scope == Some(CacheScope::Public)
}

fn is_unstamped(p: &ListProbe) -> bool {
    p.ttl_ms.is_none() || p.cache_scope.is_none()
}

fn wrong_names(p: &ListProbe) -> bool {
    p.names != better_names()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let better = resolve_bin("MCP_BETTER_BIN", "mcp-better");
    let worse = resolve_bin("MCP_WORSE_BIN", "mcp-worse");

    for (label, path) in [("mcp-better", &better), ("mcp-worse", &worse)] {
        if !std::path::Path::new(path).is_file() {
            anyhow::bail!(
                "{label} binary not found at {path}\nBuild first: cargo build --bins\n\
                 Or set MCP_BETTER_BIN / MCP_WORSE_BIN"
            );
        }
    }

    let better_probe = probe(&better).await?;
    tracing::info!(
        "better names={:?} ttl={:?} scope={:?}",
        better_probe.names,
        better_probe.ttl_ms,
        better_probe.cache_scope
    );
    if !is_better_contract(&better_probe) {
        anyhow::bail!(
            "mcp-better must satisfy BETTER list contract, got names={:?} ttl={:?} scope={:?}",
            better_probe.names,
            better_probe.ttl_ms,
            better_probe.cache_scope
        );
    }

    let worse_probe = probe(&worse).await?;
    tracing::info!(
        "worse names={:?} ttl={:?} scope={:?}",
        worse_probe.names,
        worse_probe.ttl_ms,
        worse_probe.cache_scope
    );
    if !is_unstamped(&worse_probe) {
        anyhow::bail!(
            "mcp-worse stamp clause went green (must stay unstamped): ttl={:?} scope={:?}",
            worse_probe.ttl_ms,
            worse_probe.cache_scope
        );
    }
    if !wrong_names(&worse_probe) {
        anyhow::bail!(
            "mcp-worse names clause went green (must stay ≠ {:?}): names={:?}",
            better_names(),
            worse_probe.names
        );
    }
    if is_better_contract(&worse_probe) {
        anyhow::bail!("mcp-worse unexpectedly satisfies BETTER contract — companion is broken");
    }

    println!(
        "contrast-smoke: OK (better contract · worse unstamped · worse names≠health,echo,confirm_echo)"
    );
    Ok(())
}
