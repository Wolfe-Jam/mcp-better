//! Discover-first stdio client smoke against the mcp-better server binary.
//!
//! Uses `ClientLifecycleMode::Discover` with preferred protocol **2026-07-28**.
//! This is the BETTER client path — not bare legacy `serve()` initialize.
//!
//! Run:
//!   cargo build
//!   cargo run --example stdio-client
//!
//! Or point MCP_BETTER_BIN at a built binary.

use rmcp::model::{CacheScope, CallToolRequestParams, ClientInfo, ProtocolVersion};
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{ClientLifecycleMode, ClientServiceExt};
use tokio::process::Command;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let bin = std::env::var("MCP_BETTER_BIN").unwrap_or_else(|_| {
        // Prefer the just-built binary next to this example's profile dir.
        let mut path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("target/debug"));
        // examples run from target/debug/examples/ — server is target/debug/mcp-better
        if path.ends_with("examples") {
            path.pop();
        }
        path.push("mcp-better");
        path.to_string_lossy().into_owned()
    });

    tracing::info!("spawning server: {bin}");

    let transport = TokioChildProcess::new(Command::new(&bin).configure(|_c| {}))?;

    let client = ClientInfo::default()
        .serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await?;

    tracing::info!("Discover lifecycle complete");

    // Full ListToolsResult so we can assert cache stamps (T4/T5).
    let list = client.list_tools(Default::default()).await?;
    let names: Vec<_> = list.tools.iter().map(|t| t.name.as_ref()).collect();
    tracing::info!(
        "tools/list names={names:?} ttl_ms={:?} cache_scope={:?}",
        list.ttl_ms,
        list.cache_scope
    );

    // Must match BETTER_TOOL_ORDER (health → echo → confirm_echo).
    if names != ["health", "echo", "confirm_echo"] {
        anyhow::bail!("stable tool order must be [health, echo, confirm_echo], got {names:?}");
    }

    match list.ttl_ms {
        Some(ms) if ms > 0 => {}
        other => anyhow::bail!("expected positive ttl_ms on tools/list, got {other:?}"),
    }
    if list.cache_scope != Some(CacheScope::Public) {
        anyhow::bail!(
            "expected cache_scope=public on tools/list, got {:?}",
            list.cache_scope
        );
    }

    // Same-process N lists — order + stamps must not drift (0.3 louder).
    for i in 1..=5 {
        let list_n = client.list_tools(Default::default()).await?;
        let names_n: Vec<_> = list_n.tools.iter().map(|t| t.name.as_ref()).collect();
        if names_n != names {
            anyhow::bail!("tool order not stable at list #{i}: first={names:?} got={names_n:?}");
        }
        match list_n.ttl_ms {
            Some(ms) if ms > 0 => {}
            other => anyhow::bail!("positive ttl_ms required at list #{i}, got {other:?}"),
        }
        if list_n.cache_scope != Some(CacheScope::Public) {
            anyhow::bail!(
                "cache_scope=public required at list #{i}, got {:?}",
                list_n.cache_scope
            );
        }
    }

    let health = client
        .call_tool(CallToolRequestParams::new("health"))
        .await?;
    tracing::info!("health result: {health:?}");

    let echo = client
        .call_tool(
            CallToolRequestParams::new("echo").with_arguments(rmcp::model::object(
                serde_json::json!({
                    "message": "hello-better"
                }),
            )),
        )
        .await?;
    tracing::info!("echo result: {echo:?}");

    client.cancel().await?;
    println!("stdio-client: OK (Discover + stamped list ×6 + health + echo; catalog includes confirm_echo)");
    Ok(())
}
