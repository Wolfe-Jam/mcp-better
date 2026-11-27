//! A plugin is packaging: an MCP Registry entry (`server.json`) in, an Agent Plugins
//! 1.0.0 folder (`plugin.json` + `mcp.json`) out, and a drift check between the two.
//!
//! The rules come from the Agent Plugins spec (§5, §7.2, §9): `mcp.json` has no secret
//! field and expands only `${PLUGIN_ROOT}` and `${PLUGIN_DATA}`. So a secret, a value
//! with a `{variable}` in it, or a required input with no value is never written: it is
//! named in [`Plugin::needs`], for the person installing the plugin to set in their client.

use serde_json::{json, Map, Value};

pub const PLUGIN_SCHEMA: &str = "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json";
pub const MCP_SCHEMA: &str = "https://agent-plugins.org/schemas/1.0.0/mcp.schema.json";

/// Which version the server starts at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pin {
    /// The version the registry lists (default): reproducible, and `--check` flags it
    /// once the registry moves on.
    Exact,
    /// `@latest` (`:latest` for OCI): the plugin never needs a bump.
    Latest,
}

/// The way the plugin runs the server, in the order tried by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Via {
    Npm,
    Pypi,
    Remote,
    Oci,
    Nuget,
}

impl Via {
    pub const ORDER: [Via; 5] = [Via::Npm, Via::Pypi, Via::Remote, Via::Oci, Via::Nuget];

    pub fn parse(s: &str) -> Option<Via> {
        Some(match s {
            "npm" => Via::Npm,
            "pypi" => Via::Pypi,
            "remote" => Via::Remote,
            "oci" => Via::Oci,
            "nuget" => Via::Nuget,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Via::Npm => "npm",
            Via::Pypi => "pypi",
            Via::Remote => "remote",
            Via::Oci => "oci",
            Via::Nuget => "nuget",
        }
    }
}

/// A generated plugin.
#[derive(Debug, Clone, PartialEq)]
pub struct Plugin {
    /// The plugin name (§5.5), also the server's key in `mcpServers`.
    pub name: String,
    pub via: Via,
    pub manifest: Value,
    pub mcp: Value,
    /// Inputs the server needs that the plugin can't carry; set them in the client.
    pub needs: Vec<String>,
    /// Optional inputs only the user can supply (secrets, `{variables}`).
    pub optional: Vec<String>,
    /// Packages and remotes not used, and why.
    pub skipped: Vec<String>,
}

/// A plugin name (§5.5) from a registry name: the part after the last `/`, lowercased,
/// anything outside `a-z 0-9 - .` turned into `-`, no `--` or `..`, alphanumeric at both
/// ends, at most 64 characters.
pub fn plugin_name(registry_name: &str) -> Result<String, String> {
    let last = registry_name.rsplit('/').next().unwrap_or(registry_name);
    let mut out = String::new();
    for c in last.chars().flat_map(char::to_lowercase) {
        let c = if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' {
            c
        } else {
            '-'
        };
        let prev = out.chars().last();
        if (c == '-' || c == '.') && matches!(prev, None | Some('-') | Some('.')) {
            continue;
        }
        out.push(c);
    }
    out.truncate(64);
    let name = out.trim_end_matches(['-', '.']).to_string();
    if name.is_empty() {
        Err(format!("no plugin name can be made from `{registry_name}`"))
    } else {
        Ok(name)
    }
}

fn str_of<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str).filter(|s| !s.is_empty())
}

fn arr<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    v.get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}

/// A `{variable}` the registry would fill in from the user: not expandable in `mcp.json`.
fn templated(s: &str) -> bool {
    s.contains('{') && s.contains('}')
}

/// An input's fixed value: `value`, else `default`; never a secret or a template.
enum Input {
    Fixed(String),
    Needed(String),
    /// Optional, but only the user can supply it (a secret or a `{variable}`).
    Optional(String),
    Absent,
}

fn input(v: &Value, label: &str) -> Input {
    let required = v
        .get("isRequired")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let secret = v.get("isSecret").and_then(Value::as_bool).unwrap_or(false);
    let what = str_of(v, "description")
        .map(|d| format!("{label}: {d}"))
        .unwrap_or_else(|| label.to_string());
    if secret {
        return if required {
            Input::Needed(format!("{what} (secret)"))
        } else {
            Input::Optional(format!("{what} (secret)"))
        };
    }
    match str_of(v, "value").or_else(|| str_of(v, "default")) {
        Some(s) if !templated(s) => Input::Fixed(s.to_string()),
        _ if required => Input::Needed(what),
        Some(_) => Input::Optional(what),
        None => Input::Absent,
    }
}

/// `runtimeArguments` / `packageArguments` → argv, collecting what can't be written.
fn args_of(list: &[Value], needs: &mut Vec<String>, optional: &mut Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for a in list {
        let named = str_of(a, "type") == Some("named");
        let label = match (named, str_of(a, "name"), str_of(a, "valueHint")) {
            (true, Some(n), _) => format!("argument {n}"),
            (_, _, Some(h)) => format!("argument <{h}>"),
            _ => "an argument".to_string(),
        };
        match input(a, &label) {
            Input::Fixed(v) => {
                if let (true, Some(n)) = (named, str_of(a, "name")) {
                    out.push(n.to_string());
                }
                out.push(v);
            }
            Input::Needed(w) => needs.push(w),
            Input::Optional(w) => optional.push(w),
            Input::Absent => {}
        }
    }
    out
}

/// `environmentVariables` → `(name, value)` pairs, collecting what can't be written.
fn env_of(
    list: &[Value],
    needs: &mut Vec<String>,
    optional: &mut Vec<String>,
    skipped: &mut Vec<String>,
) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for e in list {
        let Some(name) = str_of(e, "name") else {
            continue;
        };
        if name == "PLUGIN_ROOT" || name == "PLUGIN_DATA" {
            skipped.push(format!("env {name}: reserved by Agent Plugins (§9.2)"));
            continue;
        }
        match input(e, &format!("env {name}")) {
            Input::Fixed(v) => out.push((name.to_string(), v)),
            Input::Needed(w) => needs.push(w),
            Input::Optional(w) => optional.push(w),
            Input::Absent => {}
        }
    }
    out
}

fn package_via(p: &Value) -> Option<Via> {
    match str_of(p, "registryType")? {
        "npm" => Some(Via::Npm),
        "pypi" => Some(Via::Pypi),
        "oci" => Some(Via::Oci),
        "nuget" => Some(Via::Nuget),
        _ => None,
    }
}

/// One package as a stdio server entry.
fn stdio_entry(
    p: &Value,
    via: Via,
    pin: Pin,
    needs: &mut Vec<String>,
    optional: &mut Vec<String>,
    skipped: &mut Vec<String>,
) -> Value {
    let id = str_of(p, "identifier").unwrap_or_default();
    let version = match pin {
        Pin::Exact => str_of(p, "version"),
        Pin::Latest => None,
    };
    let runtime = args_of(arr(p, "runtimeArguments"), needs, optional);
    let package = args_of(arr(p, "packageArguments"), needs, optional);
    let env = env_of(arr(p, "environmentVariables"), needs, optional, skipped);

    let (command, mut args): (&str, Vec<String>) = match via {
        Via::Npm => ("npx", vec!["-y".into()]),
        Via::Pypi => ("uvx", vec![]),
        Via::Nuget => ("dnx", vec![]),
        Via::Oci => ("docker", vec!["run".into(), "-i".into(), "--rm".into()]),
        Via::Remote => unreachable!("remotes are not stdio"),
    };
    args.extend(runtime);
    let spec = match via {
        Via::Npm | Via::Nuget => format!("{id}@{}", version.unwrap_or("latest")),
        Via::Pypi => match version {
            Some(v) => format!("{id}=={v}"),
            None => format!("{id}@latest"),
        },
        Via::Oci => {
            // An OCI identifier may carry its own tag or digest.
            let tail = id.rsplit('/').next().unwrap_or(id);
            if tail.contains(':') || tail.contains('@') {
                id.to_string()
            } else {
                format!("{id}:{}", version.unwrap_or("latest"))
            }
        }
        Via::Remote => unreachable!(),
    };
    let mut entry = Map::new();
    entry.insert("type".into(), json!("stdio"));
    entry.insert("command".into(), json!(command));
    if via == Via::Oci {
        // The container gets its environment from `-e`, not from the docker process.
        for (k, v) in &env {
            args.push("-e".into());
            args.push(format!("{k}={v}"));
        }
        args.push(spec);
        args.extend(package);
    } else {
        args.push(spec);
        if via == Via::Nuget {
            args.push("--yes".into());
        }
        args.extend(package);
        if !env.is_empty() {
            let m: Map<String, Value> = env.into_iter().map(|(k, v)| (k, json!(v))).collect();
            entry.insert("env".into(), Value::Object(m));
        }
    }
    entry.insert("args".into(), json!(args));
    Value::Object(entry)
}

/// HTTPS, or HTTP on a loopback host (§7.2.1).
fn url_allowed(url: &str) -> bool {
    if let Some(rest) = url.strip_prefix("https://") {
        return !rest.is_empty();
    }
    let Some(rest) = url.strip_prefix("http://") else {
        return false;
    };
    let host = rest.split(['/', '?']).next().unwrap_or("");
    let host = host.rsplit_once(':').map_or(host, |(h, p)| {
        if p.chars().all(|c| c.is_ascii_digit()) {
            h
        } else {
            host
        }
    });
    host == "localhost" || host == "[::1]" || host.starts_with("127.")
}

/// One remote as a streamable-http / sse entry, or why not.
fn remote_entry(
    r: &Value,
    needs: &mut Vec<String>,
    optional: &mut Vec<String>,
) -> Result<Value, String> {
    let kind = str_of(r, "type").unwrap_or("?");
    if kind != "streamable-http" && kind != "sse" {
        return Err(format!("remote ({kind}): not an Agent Plugins transport"));
    }
    let url = str_of(r, "url").unwrap_or_default();
    if templated(url) {
        return Err(format!(
            "remote {url}: the URL has a {{variable}}, which mcp.json can't fill in"
        ));
    }
    if !url_allowed(url) {
        return Err(format!(
            "remote {url}: not HTTPS (only a loopback host may use HTTP)"
        ));
    }
    let mut entry = Map::new();
    entry.insert("type".into(), json!(kind));
    entry.insert("url".into(), json!(url));
    let mut headers = Map::new();
    for h in arr(r, "headers") {
        let Some(name) = str_of(h, "name") else {
            continue;
        };
        match input(h, &format!("header {name}")) {
            Input::Fixed(v) => {
                headers.insert(name.to_string(), json!(v));
            }
            Input::Needed(w) => needs.push(w),
            Input::Optional(w) => optional.push(w),
            Input::Absent => {}
        }
    }
    if !headers.is_empty() {
        entry.insert("headers".into(), Value::Object(headers));
    }
    Ok(Value::Object(entry))
}

/// Build the plugin for a registry `server` object (the `server` member of a registry
/// response, or a `server.json`).
pub fn generate(server: &Value, via: Option<Via>, pin: Pin) -> Result<Plugin, String> {
    let reg_name = str_of(server, "name").ok_or("server.json has no `name`")?;
    let name = plugin_name(reg_name)?;
    let mut skipped = Vec::new();

    // Everything the entry offers, in its own order, tagged with how it would run.
    let mut options: Vec<(Via, &Value)> = Vec::new();
    for p in arr(server, "packages") {
        let id = str_of(p, "identifier").unwrap_or("?");
        let rt = str_of(p, "registryType").unwrap_or("?");
        let transport = p
            .pointer("/transport/type")
            .and_then(Value::as_str)
            .unwrap_or("stdio");
        match package_via(p) {
            _ if transport != "stdio" => skipped.push(format!(
                "{rt} {id}: a {transport} package is started by hand, so no plugin can start it"
            )),
            Some(v) => options.push((v, p)),
            None if rt == "cargo" => skipped.push(format!(
                "cargo {id}: crates have no one-command runner (cargo install first)"
            )),
            None if rt == "mcpb" => {
                skipped.push(format!("mcpb {id}: a bundle file, not a command"))
            }
            None => skipped.push(format!("{rt} {id}: no known runner")),
        }
    }
    for r in arr(server, "remotes") {
        options.push((Via::Remote, r));
    }

    let order: Vec<Via> = match via {
        Some(v) => vec![v],
        None => Via::ORDER.to_vec(),
    };
    for want in order {
        for (v, item) in options.iter().filter(|(v, _)| *v == want) {
            let (mut needs, mut optional) = (Vec::new(), Vec::new());
            let entry = if *v == Via::Remote {
                match remote_entry(item, &mut needs, &mut optional) {
                    Ok(e) => e,
                    Err(why) => {
                        skipped.push(why);
                        continue;
                    }
                }
            } else {
                stdio_entry(item, *v, pin, &mut needs, &mut optional, &mut skipped)
            };
            let mut manifest = Map::new();
            manifest.insert("$schema".into(), json!(PLUGIN_SCHEMA));
            manifest.insert("name".into(), json!(name));
            for (to, from) in [
                ("version", "version"),
                ("description", "description"),
                ("homepage", "websiteUrl"),
            ] {
                if let Some(s) = str_of(server, from) {
                    manifest.insert(to.into(), json!(s));
                }
            }
            if let Some(s) = server.pointer("/repository/url").and_then(Value::as_str) {
                manifest.insert("repository".into(), json!(s));
            }
            let mcp = json!({ "$schema": MCP_SCHEMA, "mcpServers": { name.clone(): entry } });
            return Ok(Plugin {
                name,
                via: *v,
                manifest: Value::Object(manifest),
                mcp,
                needs,
                optional,
                skipped,
            });
        }
    }
    let wanted = via.map_or(String::new(), |v| format!(" via {}", v.as_str()));
    if options.is_empty() && skipped.is_empty() {
        skipped.push("the entry lists no packages and no remotes".into());
    }
    let mut msg = format!("`{reg_name}` has nothing a plugin can start{wanted}");
    for s in &skipped {
        msg.push_str(&format!("\n  - {s}"));
    }
    Err(msg)
}

/// Whether an on-disk entry starts the server at `@latest` / `:latest`.
pub fn pin_of(entry: &Value) -> Pin {
    let latest = arr(entry, "args")
        .iter()
        .filter_map(Value::as_str)
        .any(|a| a.ends_with("@latest") || a.ends_with(":latest"));
    if latest {
        Pin::Latest
    } else {
        Pin::Exact
    }
}

/// The plugin's way of running the server, read back from an on-disk entry.
pub fn via_of(entry: &Value) -> Option<Via> {
    match (str_of(entry, "type")?, str_of(entry, "command")) {
        ("streamable-http" | "sse", _) => Some(Via::Remote),
        ("stdio", Some("npx")) => Some(Via::Npm),
        ("stdio", Some("uvx")) => Some(Via::Pypi),
        ("stdio", Some("docker")) => Some(Via::Oci),
        ("stdio", Some("dnx")) => Some(Via::Nuget),
        _ => None,
    }
}

/// Compare an on-disk plugin with what the registry lists now. Returns one line per
/// difference; empty means no drift. Only what [`generate`] writes is compared: other
/// servers, and fields such as `keywords` or `author`, are the author's own.
pub fn check(server: &Value, manifest: &Value, mcp: &Value) -> Result<Vec<String>, String> {
    let reg_name = str_of(server, "name").ok_or("server.json has no `name`")?;
    let name = plugin_name(reg_name)?;
    let on_disk = mcp
        .pointer(&format!(
            "/mcpServers/{}",
            name.replace('~', "~0").replace('/', "~1")
        ))
        .ok_or_else(|| format!("mcp.json has no server named `{name}`"))?;
    let via = via_of(on_disk)
        .ok_or_else(|| format!("mcp.json `{name}`: not a command this tool writes"))?;
    let want = generate(server, Some(via), pin_of(on_disk))?;

    let mut drift = Vec::new();
    let expected = &want.mcp["mcpServers"][&name];
    if on_disk != expected {
        drift.push(format!(
            "mcp.json `{name}`: on disk {on_disk}, registry now {expected}"
        ));
    }
    let pin = pin_of(on_disk);
    for key in [
        "$schema",
        "name",
        "version",
        "description",
        "homepage",
        "repository",
    ] {
        // A plugin on @latest is never bumped, so its version is the author's.
        if key == "version" && pin == Pin::Latest {
            continue;
        }
        let (got, exp) = (manifest.get(key), want.manifest.get(key));
        if got != exp {
            let show = |v: Option<&Value>| v.map_or("(none)".to_string(), Value::to_string);
            drift.push(format!(
                "plugin.json {key}: on disk {}, registry now {}",
                show(got),
                show(exp)
            ));
        }
    }
    if mcp.get("$schema").and_then(Value::as_str) != Some(MCP_SCHEMA) {
        drift.push(format!("mcp.json $schema: expected {MCP_SCHEMA}"));
    }
    Ok(drift)
}

/// The order keys are written in: the spec's own order, `$schema` and `type` first.
const KEY_ORDER: [&str; 17] = [
    "$schema",
    "name",
    "version",
    "description",
    "author",
    "homepage",
    "repository",
    "license",
    "keywords",
    "mcpServers",
    "type",
    "command",
    "args",
    "env",
    "cwd",
    "url",
    "headers",
];

/// Pretty JSON (two spaces, trailing newline) with keys in [`KEY_ORDER`], the rest after
/// in their own order. `serde_json`'s map is sorted, which would put `args` before `type`.
pub fn to_json(v: &Value) -> String {
    fn write(v: &Value, indent: usize, out: &mut String) {
        let pad = |n: usize| "  ".repeat(n);
        match v {
            Value::Object(m) if !m.is_empty() => {
                let mut keys: Vec<&String> = m.keys().collect();
                keys.sort_by_key(|k| {
                    KEY_ORDER
                        .iter()
                        .position(|o| o == k)
                        .unwrap_or(KEY_ORDER.len())
                });
                out.push_str("{\n");
                for (i, k) in keys.iter().enumerate() {
                    out.push_str(&pad(indent + 1));
                    out.push_str(&Value::String((*k).clone()).to_string());
                    out.push_str(": ");
                    write(&m[*k], indent + 1, out);
                    out.push_str(if i + 1 < keys.len() { ",\n" } else { "\n" });
                }
                out.push_str(&pad(indent));
                out.push('}');
            }
            Value::Array(a) if !a.is_empty() => {
                out.push_str("[\n");
                for (i, x) in a.iter().enumerate() {
                    out.push_str(&pad(indent + 1));
                    write(x, indent + 1, out);
                    out.push_str(if i + 1 < a.len() { ",\n" } else { "\n" });
                }
                out.push_str(&pad(indent));
                out.push(']');
            }
            other => out.push_str(&other.to_string()),
        }
    }
    let mut out = String::new();
    write(v, 0, &mut out);
    out.push('\n');
    out
}
