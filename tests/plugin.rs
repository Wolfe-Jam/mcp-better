//! `mcp-better plugin`: a registry entry in, an Agent Plugins 1.0.0 folder out.
//! Offline: every entry here is a fixture (names under `io.example` are made up).

use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use mcp_better::plugin::{
    check, generate, plugin_name, to_json, Pin, Via, MCP_SCHEMA, PLUGIN_SCHEMA,
};
use serde_json::{json, Value};

const BETTER: &str = env!("CARGO_BIN_EXE_mcp-better");

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/plugin/mcp-better.json")
}

fn server(extra: Value) -> Value {
    let mut s =
        json!({ "name": "io.example/demo-server", "version": "1.2.0", "description": "A demo" });
    s.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    s
}

fn npm(extra: Value) -> Value {
    let mut p = json!({ "registryType": "npm", "identifier": "@example/demo", "version": "1.2.0", "transport": { "type": "stdio" } });
    p.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    p
}

fn entry(p: &mcp_better::plugin::Plugin) -> &Value {
    &p.mcp["mcpServers"][&p.name]
}

#[test]
fn plugin_names_follow_the_spec() {
    assert_eq!(
        plugin_name("io.github.Wolfe-Jam/mcp-better").unwrap(),
        "mcp-better"
    );
    assert_eq!(plugin_name("io.example/My_Server").unwrap(), "my-server");
    assert_eq!(plugin_name("io.example/--a..b--").unwrap(), "a.b");
    assert_eq!(plugin_name("io.example/x").unwrap(), "x");
    assert_eq!(
        plugin_name(&format!("io.example/{}", "a".repeat(80)))
            .unwrap()
            .len(),
        64
    );
    assert!(plugin_name("io.example/___").is_err());
}

#[test]
fn our_own_entry_runs_via_npm_and_says_why_not_cargo() {
    let s: Value = serde_json::from_str(&std::fs::read_to_string(fixture()).unwrap()).unwrap();
    let p = generate(&s, None, Pin::Exact).unwrap();
    assert_eq!(p.name, "mcp-better");
    assert_eq!(p.via, Via::Npm);
    let version = s["version"].as_str().unwrap();
    assert_eq!(
        entry(&p),
        &json!({ "type": "stdio", "command": "npx", "args": ["-y", format!("mcp-better@{version}")] })
    );
    assert_eq!(p.manifest["$schema"], PLUGIN_SCHEMA);
    assert_eq!(p.manifest["version"], version);
    assert_eq!(
        p.manifest["repository"],
        "https://github.com/Wolfe-Jam/mcp-better"
    );
    assert_eq!(p.mcp["$schema"], MCP_SCHEMA);
    assert!(p
        .skipped
        .iter()
        .any(|s| s.starts_with("cargo mcp-better: crates have no one-command runner")));
}

#[test]
fn latest_starts_at_latest() {
    let s = server(json!({ "packages": [npm(json!({}))] }));
    let p = generate(&s, None, Pin::Latest).unwrap();
    assert_eq!(entry(&p)["args"], json!(["-y", "@example/demo@latest"]));
}

#[test]
fn secrets_and_missing_values_are_listed_never_written() {
    let s = server(json!({ "packages": [npm(json!({ "environmentVariables": [
        { "name": "DEMO_KEY", "description": "API key", "isRequired": true, "isSecret": true },
        { "name": "DEMO_TOKEN", "isSecret": true },
        { "name": "DEMO_MODE", "default": "fast" },
        { "name": "DEMO_HOST", "isRequired": true },
        { "name": "DEMO_URL", "value": "https://{host}/api" },
        { "name": "DEMO_OFF" },
        { "name": "PLUGIN_ROOT", "value": "/x" }
    ] }))] }));
    let p = generate(&s, None, Pin::Exact).unwrap();
    assert_eq!(entry(&p)["env"], json!({ "DEMO_MODE": "fast" }));
    assert_eq!(
        p.needs,
        vec!["env DEMO_KEY: API key (secret)", "env DEMO_HOST"]
    );
    assert_eq!(p.optional, vec!["env DEMO_TOKEN (secret)", "env DEMO_URL"]);
    assert!(p
        .skipped
        .iter()
        .any(|s| s.starts_with("env PLUGIN_ROOT: reserved")));
}

#[test]
fn arguments_keep_their_place() {
    let s = server(json!({ "packages": [npm(json!({
        "runtimeArguments": [{ "type": "named", "name": "--quiet", "value": "true" }],
        "packageArguments": [
            { "type": "named", "name": "--port", "default": "8080" },
            { "type": "positional", "valueHint": "workspace", "isRequired": true },
            { "type": "positional", "value": "serve" }
        ]
    }))] }));
    let p = generate(&s, None, Pin::Exact).unwrap();
    assert_eq!(
        entry(&p)["args"],
        json!([
            "-y",
            "--quiet",
            "true",
            "@example/demo@1.2.0",
            "--port",
            "8080",
            "serve"
        ])
    );
    assert_eq!(p.needs, vec!["argument <workspace>"]);
}

#[test]
fn pypi_oci_and_nuget_runners() {
    let pypi = server(
        json!({ "packages": [{ "registryType": "pypi", "identifier": "demo-mcp", "version": "0.3.1", "transport": { "type": "stdio" } }] }),
    );
    assert_eq!(
        entry(&generate(&pypi, None, Pin::Exact).unwrap())["args"],
        json!(["demo-mcp==0.3.1"])
    );
    assert_eq!(
        entry(&generate(&pypi, None, Pin::Latest).unwrap())["args"],
        json!(["demo-mcp@latest"])
    );

    let oci = server(
        json!({ "packages": [{ "registryType": "oci", "identifier": "ghcr.io/example/demo", "version": "2.0.0",
        "transport": { "type": "stdio" }, "environmentVariables": [{ "name": "MODE", "value": "ro" }] }] }),
    );
    let p = generate(&oci, None, Pin::Exact).unwrap();
    assert_eq!(
        entry(&p),
        &json!({ "type": "stdio", "command": "docker", "args": ["run", "-i", "--rm", "-e", "MODE=ro", "ghcr.io/example/demo:2.0.0"] })
    );
    let tagged = server(
        json!({ "packages": [{ "registryType": "oci", "identifier": "ghcr.io/example/demo:1.0", "transport": { "type": "stdio" } }] }),
    );
    assert_eq!(
        entry(&generate(&tagged, None, Pin::Exact).unwrap())["args"][3],
        "ghcr.io/example/demo:1.0"
    );

    let nuget = server(
        json!({ "packages": [{ "registryType": "nuget", "identifier": "Example.Demo", "version": "1.0.0", "transport": { "type": "stdio" } }] }),
    );
    assert_eq!(
        entry(&generate(&nuget, None, Pin::Exact).unwrap())["args"],
        json!(["Example.Demo@1.0.0", "--yes"])
    );
}

#[test]
fn remotes_are_https_and_untemplated() {
    let s = server(json!({ "remotes": [
        { "type": "streamable-http", "url": "https://{tenant}.example.com/mcp" },
        { "type": "streamable-http", "url": "http://example.com/mcp" },
        { "type": "streamable-http", "url": "https://mcp.example.com/mcp", "headers": [
            { "name": "X-Client", "value": "plugin" },
            { "name": "Authorization", "isRequired": true, "isSecret": true }
        ] }
    ] }));
    let p = generate(&s, None, Pin::Exact).unwrap();
    assert_eq!(p.via, Via::Remote);
    assert_eq!(
        entry(&p),
        &json!({ "type": "streamable-http", "url": "https://mcp.example.com/mcp", "headers": { "X-Client": "plugin" } })
    );
    assert_eq!(p.needs, vec!["header Authorization (secret)"]);
    assert!(p.skipped.iter().any(|s| s.contains("has a {variable}")));
    assert!(p.skipped.iter().any(|s| s.contains("not HTTPS")));

    let local =
        server(json!({ "remotes": [{ "type": "sse", "url": "http://127.0.0.1:8787/sse" }] }));
    assert_eq!(
        entry(&generate(&local, None, Pin::Exact).unwrap())["type"],
        "sse"
    );
}

#[test]
fn order_and_via() {
    let s = server(json!({
        "packages": [{ "registryType": "pypi", "identifier": "demo-mcp", "version": "1.2.0", "transport": { "type": "stdio" } }, npm(json!({}))],
        "remotes": [{ "type": "streamable-http", "url": "https://mcp.example.com/mcp" }]
    }));
    assert_eq!(generate(&s, None, Pin::Exact).unwrap().via, Via::Npm);
    assert_eq!(
        generate(&s, Some(Via::Remote), Pin::Exact).unwrap().via,
        Via::Remote
    );
    let e = generate(&s, Some(Via::Oci), Pin::Exact).unwrap_err();
    assert!(e.contains("has nothing a plugin can start via oci"), "{e}");
}

#[test]
fn nothing_to_start_says_why() {
    let e = generate(&server(json!({})), None, Pin::Exact).unwrap_err();
    assert!(e.contains("lists no packages and no remotes"), "{e}");
    let http_pkg = server(
        json!({ "packages": [npm(json!({ "transport": { "type": "streamable-http", "url": "http://localhost:3000/mcp" } }))] }),
    );
    let e = generate(&http_pkg, None, Pin::Exact).unwrap_err();
    assert!(e.contains("started by hand"), "{e}");
}

#[test]
fn check_finds_drift_and_only_drift() {
    let s = server(json!({ "packages": [npm(json!({}))] }));
    let p = generate(&s, None, Pin::Exact).unwrap();
    assert!(check(&s, &p.manifest, &p.mcp).unwrap().is_empty());

    // The author's own fields and other servers are not drift.
    let mut m = p.manifest.clone();
    m["keywords"] = json!(["demo"]);
    let mut mcp = p.mcp.clone();
    mcp["mcpServers"]["other"] = json!({ "type": "stdio", "command": "other" });
    assert!(check(&s, &m, &mcp).unwrap().is_empty());

    // The registry moved on.
    let newer =
        server(json!({ "version": "1.3.0", "packages": [npm(json!({ "version": "1.3.0" }))] }));
    let drift = check(&newer, &p.manifest, &p.mcp).unwrap();
    assert_eq!(drift.len(), 2, "{drift:?}");
    assert!(drift[0].contains("@example/demo@1.3.0"));
    assert!(drift[1].starts_with("plugin.json version"));

    // A plugin on @latest is never bumped.
    let latest = generate(&s, None, Pin::Latest).unwrap();
    assert!(check(&newer, &latest.manifest, &latest.mcp)
        .unwrap()
        .is_empty());
}

#[test]
fn json_keys_in_spec_order() {
    let s = server(
        json!({ "packages": [npm(json!({ "environmentVariables": [{ "name": "A", "value": "1" }] }))] }),
    );
    let p = generate(&s, None, Pin::Exact).unwrap();
    let text = to_json(&p.mcp);
    let at = |k: &str| text.find(k).unwrap();
    assert!(at("$schema") < at("mcpServers"));
    assert!(
        at("\"type\"") < at("\"command\"")
            && at("\"command\"") < at("\"args\"")
            && at("\"args\"") < at("\"env\"")
    );
    assert!(text.ends_with("}\n"));
    assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), p.mcp);
}

fn run(args: &[&str]) -> Output {
    Command::new(BETTER)
        .args(args)
        .env("MCP_REGISTRY_URL", "http://127.0.0.1:9/")
        .stdin(Stdio::null())
        .output()
        .expect("binary runs")
}

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("mcp-better-plugin-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn the_command_writes_checks_and_refuses() {
    let dir = tmp("cmd");
    let f = fixture();
    let (from, out) = (f.to_str().unwrap(), dir.to_str().unwrap());

    let o = run(&["plugin", "--from", from, "--out", out]);
    assert_eq!(
        o.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert!(String::from_utf8_lossy(&o.stdout).contains("via npm"));
    let mcp: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("mcp.json")).unwrap()).unwrap();
    assert_eq!(mcp["mcpServers"]["mcp-better"]["command"], "npx");

    // Never replaces a file unasked.
    assert_eq!(
        run(&["plugin", "--from", from, "--out", out]).status.code(),
        Some(3)
    );
    assert_eq!(
        run(&["plugin", "--from", from, "--out", out, "--force"])
            .status
            .code(),
        Some(0)
    );

    assert_eq!(
        run(&["plugin", "--from", from, "--check", out])
            .status
            .code(),
        Some(0)
    );
    let pinned = std::fs::read_to_string(dir.join("mcp.json"))
        .unwrap()
        .replace("mcp-better@", "mcp-better@0.0.");
    std::fs::write(dir.join("mcp.json"), pinned).unwrap();
    let o = run(&["plugin", "--from", from, "--check", out]);
    assert_eq!(o.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&o.stdout).starts_with("drift in"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn usage_errors_exit_2_and_fetch_errors_exit_3() {
    assert_eq!(run(&["plugin"]).status.code(), Some(2));
    assert_eq!(
        run(&["plugin", "--via", "cargo", "x"]).status.code(),
        Some(2)
    );
    assert_eq!(run(&["plugin", "a", "b"]).status.code(), Some(2));
    assert_eq!(run(&["plugin", "a", "--from", "f"]).status.code(), Some(2));
    assert_eq!(
        run(&["plugin", "a", "--check", "d", "--latest"])
            .status
            .code(),
        Some(2)
    );
    assert_eq!(run(&["plugin", "--out"]).status.code(), Some(2));
    let h = run(&["plugin", "--help"]);
    assert_eq!(h.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&h.stderr).contains("Agent Plugins"));
    let v = run(&["plugin", "--from", "f", "-V"]);
    assert_eq!(
        (
            v.status.code(),
            String::from_utf8_lossy(&v.stdout).trim().to_string()
        ),
        (Some(0), format!("mcp-better {}", env!("CARGO_PKG_VERSION")))
    );
    // Nothing listens on port 9: a fetch failure, not a crash.
    let o = run(&["plugin", "io.example/demo-server"]);
    assert_eq!(
        o.status.code(),
        Some(3),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
}
