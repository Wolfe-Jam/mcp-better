//! HTTP smoke: spawn mcp-better --http, POST tools/list + tools/call, assert stamps.
//!
//! Run (after bin is built):
//!   cargo build --bins && cargo run --example http-smoke
//!
//! Or point at any binary:
//!   MCP_BETTER_BIN=/path/to/mcp-better cargo run --example http-smoke

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bin = resolve_bin()?;
    if !PathBuf::from(&bin).is_file() {
        anyhow::bail!(
            "mcp-better binary not found at {bin}\n\
             Build first: cargo build --bins\n\
             Or set MCP_BETTER_BIN=/path/to/mcp-better"
        );
    }

    // Ephemeral high port — avoid clashing with a long-lived --http on 8787
    let addr = "127.0.0.1:18787";
    let base = format!("http://{addr}/mcp");

    let mut child = spawn_http(&bin, addr)?;
    let ready = wait_for_port(addr, Duration::from_secs(8));
    if let Err(e) = ready {
        let stderr = read_child_stderr(&mut child);
        let _ = child.kill();
        let _ = child.wait();
        anyhow::bail!("server did not accept TCP on {addr}: {e}\n--- server stderr ---\n{stderr}");
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let result = run_checks(&client, &base).await;
    let _ = child.kill();
    let _ = child.wait();
    if let Err(ref err) = result {
        eprintln!("http-smoke failed: {err}");
    }
    result
}

fn resolve_bin() -> anyhow::Result<String> {
    if let Ok(p) = std::env::var("MCP_BETTER_BIN") {
        return Ok(p);
    }
    // CARGO_BIN_EXE_<name> is set for integration tests; examples often lack it.
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_mcp-better") {
        return Ok(p);
    }
    // Prefer target next to this example binary
    if let Ok(exe) = std::env::current_exe() {
        let mut path = exe;
        if path.file_name().is_some() {
            path.pop(); // .../examples or .../deps
        }
        if path.ends_with("examples") || path.ends_with("deps") {
            path.pop();
        }
        path.push("mcp-better");
        if path.is_file() {
            return Ok(path.to_string_lossy().into_owned());
        }
    }
    // Fallback: CARGO_MANIFEST_DIR/target/debug|release
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    for profile in ["debug", "release"] {
        let p = manifest.join("target").join(profile).join("mcp-better");
        if p.is_file() {
            return Ok(p.to_string_lossy().into_owned());
        }
    }
    anyhow::bail!("could not resolve mcp-better binary (set MCP_BETTER_BIN)")
}

fn spawn_http(bin: &str, addr: &str) -> anyhow::Result<Child> {
    let child = Command::new(bin)
        .arg("--http")
        .env("MCP_BETTER_HTTP_ADDR", addr)
        .env("RUST_LOG", "info")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("spawn {bin} --http: {e}"))?;
    Ok(child)
}

fn wait_for_port(addr: &str, budget: Duration) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    let mut last = None;
    while start.elapsed() < budget {
        match TcpStream::connect(addr) {
            Ok(_) => return Ok(()),
            Err(e) => last = Some(e),
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(anyhow::anyhow!(
        "timeout after {budget:?}: {}",
        last.map(|e| e.to_string())
            .unwrap_or_else(|| "no attempts".into())
    ))
}

fn read_child_stderr(child: &mut Child) -> String {
    use std::io::Read;
    let mut buf = String::new();
    if let Some(mut err) = child.stderr.take() {
        let _ = err.read_to_string(&mut buf);
    }
    if buf.is_empty() {
        "(empty)".into()
    } else {
        buf
    }
}

async fn run_checks(client: &reqwest::Client, base: &str) -> anyhow::Result<()> {
    // Discover-style tools/list with 7/28 meta (no session)
    let list_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {
            "_meta": {
                "io.modelcontextprotocol/protocolVersion": "2026-07-28",
                "io.modelcontextprotocol/clientInfo": {
                    "name": "http-smoke",
                    "version": "0.2.0"
                },
                "io.modelcontextprotocol/clientCapabilities": {}
            }
        }
    });

    let resp = client
        .post(base)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .header("MCP-Protocol-Version", "2026-07-28")
        .header("Mcp-Method", "tools/list")
        .json(&list_body)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        anyhow::bail!("tools/list HTTP {status}: {text}");
    }

    let v: Value = parse_json_or_sse(&text)?;
    let result = v
        .get("result")
        .ok_or_else(|| anyhow::anyhow!("no result in {v}"))?;

    let tools = result
        .get("tools")
        .and_then(|t| t.as_array())
        .ok_or_else(|| anyhow::anyhow!("no tools array"))?;
    let names: Vec<&str> = tools
        .iter()
        .filter_map(|t| t.get("name").and_then(|n| n.as_str()))
        .collect();
    if names != ["health", "echo"] {
        anyhow::bail!("expected [health, echo], got {names:?}");
    }

    let ttl = result.get("ttlMs").and_then(|t| t.as_u64());
    if ttl != Some(60_000) {
        anyhow::bail!("expected ttlMs=60000, got {ttl:?}");
    }
    let scope = result.get("cacheScope").and_then(|s| s.as_str());
    if scope != Some("public") {
        anyhow::bail!("expected cacheScope=public, got {scope:?}");
    }

    // Call health
    let health_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "health",
            "arguments": {},
            "_meta": {
                "io.modelcontextprotocol/protocolVersion": "2026-07-28",
                "io.modelcontextprotocol/clientInfo": {"name": "http-smoke", "version": "0.2.0"},
                "io.modelcontextprotocol/clientCapabilities": {}
            }
        }
    });

    let resp = client
        .post(base)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .header("MCP-Protocol-Version", "2026-07-28")
        // SEP-2243 style routing headers (client → server)
        .header("Mcp-Method", "tools/call")
        .header("Mcp-Name", "health")
        .json(&health_body)
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("tools/call health HTTP {status}: {text}");
    }
    if !text.contains("BETTER") && !text.contains("2026-07-28") {
        anyhow::bail!("health body missing BETTER/7-28 markers: {text}");
    }

    println!(
        "http-smoke: OK (Streamable HTTP · tools/list stamped · health · Mcp-Method/Mcp-Name on call)"
    );
    Ok(())
}

fn parse_json_or_sse(text: &str) -> anyhow::Result<Value> {
    if let Ok(v) = serde_json::from_str::<Value>(text) {
        return Ok(v);
    }
    // SSE: data: {...}
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("data:") {
            let data = rest.trim();
            if data.is_empty() || data == "[DONE]" {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<Value>(data) {
                return Ok(v);
            }
        }
    }
    anyhow::bail!("could not parse JSON or SSE from: {text}")
}
