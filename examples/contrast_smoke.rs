//! Contrast smoke: mcp-better passes the BETTER list contract; mcp-worse deliberately fails it.
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

fn is_lying_surface(p: &ListProbe) -> bool {
    // Unstamped and/or wrong order — either is enough to fail BETTER.
    // mcp-worse keeps the two-tool reverse order (echo, health) on purpose.
    let unstamped = p.ttl_ms.is_none() || p.cache_scope.is_none();
    let wrong_order = p.names != better_names();
    unstamped || wrong_order
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
    if !is_lying_surface(&worse_probe) {
        anyhow::bail!(
            "mcp-worse must violate BETTER list contract for teaching, but it passed: names={:?} ttl={:?} scope={:?}",
            worse_probe.names,
            worse_probe.ttl_ms,
            worse_probe.cache_scope
        );
    }
    if is_better_contract(&worse_probe) {
        anyhow::bail!("mcp-worse unexpectedly satisfies BETTER contract — companion is broken");
    }

    println!("contrast-smoke: OK (mcp-better passes BETTER list contract · mcp-worse fails it)");
    Ok(())
}
