//! Matching textbook client — completes SEP-2322 MRTR on `confirm_echo`.
//!
//! 0.5 lesson: the peer is two-sided. 0.4.3 proved the server can pause;
//! this client Discover-negotiates 7/28, then finishes the mid-call retry.
//! Uses `call_tool_once` so R1/R2 stay visible (does not hide behind auto-MRTR).
//!
//! Run:
//!   cargo build --bins
//!   cargo run --example mrtr-client
//!
//! Or: MCP_BETTER_BIN=/path/to/mcp-better cargo run --example mrtr-client

use rmcp::model::{
    CacheScope, CallToolRequestParams, CallToolResponse, ClientInfo, InputResponses,
    ProtocolVersion,
};
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use rmcp::{ClientLifecycleMode, ClientServiceExt};
use serde_json::json;
use tokio::process::Command;

const CONFIRM_INPUT_KEY: &str = "confirm";

fn server_bin() -> String {
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

fn confirm_args(message: &str) -> CallToolRequestParams {
    CallToolRequestParams::new("confirm_echo").with_arguments(rmcp::model::object(json!({
        "message": message
    })))
}

fn accept_confirm(text: &str) -> InputResponses {
    let mut responses = InputResponses::new();
    responses.insert(
        CONFIRM_INPUT_KEY.to_string(),
        json!({
            "action": "accept",
            "content": { "confirm": text }
        }),
    );
    responses
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let bin = server_bin();
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

    let list = client.list_tools(Default::default()).await?;
    let names: Vec<_> = list.tools.iter().map(|t| t.name.as_ref()).collect();
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

    // Happy path: R1 pause → echo sealed state + CONFIRM → complete.
    let r1 = client.call_tool_once(confirm_args("Claim = Wire")).await?;
    let CallToolResponse::InputRequired(ir) = r1 else {
        anyhow::bail!("round 1 must be input_required, got {r1:?}");
    };
    if !ir.result_type.is_input_required() {
        anyhow::bail!("round 1 resultType must be input_required");
    }
    let sealed = ir
        .request_state
        .clone()
        .ok_or_else(|| anyhow::anyhow!("round 1 missing requestState"))?;
    let reqs = ir
        .input_requests
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("round 1 missing inputRequests"))?;
    if !reqs.contains_key(CONFIRM_INPUT_KEY) {
        anyhow::bail!("round 1 inputRequests missing `{CONFIRM_INPUT_KEY}`");
    }

    let r2 = client
        .call_tool_once(
            confirm_args("Claim = Wire")
                .with_input_responses(accept_confirm("CONFIRM"))
                .with_request_state(sealed),
        )
        .await?;
    let CallToolResponse::Complete(done) = r2 else {
        anyhow::bail!("round 2 must be complete, got {r2:?}");
    };
    let body = format!("{done:?}");
    if !body.contains("Claim = Wire") && !body.contains("confirmed") {
        anyhow::bail!("round 2 complete body missing echo/confirmed: {body}");
    }

    // Fail: wrong confirm text.
    let r1 = client.call_tool_once(confirm_args("nope")).await?;
    let CallToolResponse::InputRequired(ir) = r1 else {
        anyhow::bail!("wrong-confirm setup must be input_required, got {r1:?}");
    };
    let sealed = ir
        .request_state
        .clone()
        .ok_or_else(|| anyhow::anyhow!("wrong-confirm missing requestState"))?;
    let wrong = client
        .call_tool_once(
            confirm_args("nope")
                .with_input_responses(accept_confirm("yes"))
                .with_request_state(sealed),
        )
        .await;
    if wrong.is_ok() {
        anyhow::bail!("wrong confirm text must fail, got {wrong:?}");
    }

    // Fail: tampered requestState.
    let r1 = client.call_tool_once(confirm_args("sealed")).await?;
    let CallToolResponse::InputRequired(_) = r1 else {
        anyhow::bail!("tamper setup must be input_required, got {r1:?}");
    };
    let tampered = client
        .call_tool_once(
            confirm_args("sealed")
                .with_input_responses(accept_confirm("CONFIRM"))
                .with_request_state("not-a-valid-seal"),
        )
        .await;
    if tampered.is_ok() {
        anyhow::bail!("tampered requestState must fail, got {tampered:?}");
    }

    client.cancel().await?;
    println!(
        "mrtr-client: OK (Discover + confirm_echo R1 input_required → R2 complete · reject wrong confirm · reject tampered state)"
    );
    Ok(())
}
