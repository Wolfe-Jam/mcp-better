//! `mcp-better plugin`: the command line over [`crate::plugin`].
//!
//! Exit codes: 0 written / no drift · 1 drift · 2 usage · 3 the entry can't be fetched,
//! read or turned into a plugin, or a file can't be written.

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::plugin::{check, generate, to_json, Pin, Plugin, Via};

pub const DEFAULT_REGISTRY: &str = "https://registry.modelcontextprotocol.io";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// A registry server name, fetched at its latest version.
    Name(String),
    /// A local `server.json` (or a registry response with a `server` member).
    File(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub source: Source,
    pub out: Option<PathBuf>,
    pub check: Option<PathBuf>,
    pub latest: bool,
    pub via: Option<Via>,
    pub force: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    Version,
    Help,
    Run(Args),
}

pub fn help() -> String {
    format!(
        "mcp-better plugin v{} — a registry entry in, an Agent Plugins folder out\n\n\
         Usage:\n\
           mcp-better plugin <registry-name> [--out DIR] [--latest] [--via TYPE] [--force]\n\
           mcp-better plugin --from server.json [...]\n\
           mcp-better plugin <registry-name> --check DIR\n\n\
         Writes DIR/plugin.json and DIR/mcp.json (Agent Plugins 1.0.0). DIR defaults to\n\
         the plugin name. Inputs a plugin can't carry (secrets, {{variables}}, required\n\
         values with no default) are never written: they are listed, to set in your client.\n\n\
         Options:\n\
           --out DIR        where to write (refuses to replace files without --force)\n\
           --latest         start the server at @latest instead of the registry's version\n\
           --via TYPE       npm | pypi | remote | oci | nuget (default: first that works,\n\
                            in that order)\n\
           --from FILE      read a server.json instead of the registry\n\
           --check DIR      compare DIR's plugin with the registry; exit 1 on drift\n\
           --force          replace existing files\n\n\
         Env:\n\
           MCP_REGISTRY_URL  default {DEFAULT_REGISTRY}\n\n\
         Exit: 0 ok · 1 drift · 2 usage · 3 fetch, read or write failed\n",
        env!("CARGO_PKG_VERSION")
    )
}

/// Parse the arguments after `plugin`.
pub fn parse(args: &[String]) -> Result<Cmd, String> {
    // As for the server: --version / --help win wherever they appear; the first wins.
    for a in args {
        match a.as_str() {
            "--version" | "-V" => return Ok(Cmd::Version),
            "--help" | "-h" => return Ok(Cmd::Help),
            _ => {}
        }
    }
    let mut name = None;
    let mut from = None;
    let (mut out, mut check, mut via) = (None, None, None);
    let (mut latest, mut force) = (false, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut value = |flag: &str| {
            it.next()
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .ok_or_else(|| format!("{flag} needs a value"))
        };
        match a.as_str() {
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--check" => check = Some(PathBuf::from(value("--check")?)),
            "--from" => from = Some(PathBuf::from(value("--from")?)),
            "--via" => {
                let v = value("--via")?;
                via = Some(Via::parse(&v).ok_or(format!(
                    "--via {v}: expected npm, pypi, remote, oci or nuget"
                ))?);
            }
            "--latest" => latest = true,
            "--force" => force = true,
            s if s.starts_with('-') => return Err(format!("unknown argument `{s}`")),
            s if name.is_none() => name = Some(s.to_string()),
            s => return Err(format!("unexpected argument `{s}`")),
        }
    }
    let source = match (name, from) {
        (Some(n), None) => Source::Name(n),
        (None, Some(f)) => Source::File(f),
        (Some(_), Some(_)) => return Err("give a registry name or --from, not both".into()),
        (None, None) => return Err("give a registry name, or --from server.json".into()),
    };
    if check.is_some() && (out.is_some() || force) {
        return Err("--check only reads: drop --out and --force".into());
    }
    if check.is_some() && (latest || via.is_some()) {
        return Err("--check reads --latest and --via from the plugin itself".into());
    }
    Ok(Cmd::Run(Args {
        source,
        out,
        check,
        latest,
        via,
        force,
    }))
}

/// The registry's latest entry for `name`.
pub fn fetch(registry: &str, name: &str) -> Result<Value, String> {
    let url = format!(
        "{}/v0/servers/{}/versions/latest",
        registry.trim_end_matches('/'),
        name.replace('%', "%25").replace('/', "%2F")
    );
    let mut resp = ureq::get(&url)
        .header("Accept", "application/json")
        .header(
            "User-Agent",
            concat!("mcp-better/", env!("CARGO_PKG_VERSION")),
        )
        .call()
        .map_err(|e| match e {
            ureq::Error::StatusCode(404) => format!("`{name}` is not in the registry ({url})"),
            e => format!("{url}: {e}"),
        })?;
    let body: Value = resp
        .body_mut()
        .read_json()
        .map_err(|e| format!("{url}: not JSON ({e})"))?;
    server_of(body)
}

/// The server object: a registry response's `server`, or a bare `server.json`.
pub fn server_of(v: Value) -> Result<Value, String> {
    let server = match v.get("server") {
        Some(s) if s.is_object() => s.clone(),
        _ => v,
    };
    if server.get("name").and_then(Value::as_str).is_none() {
        return Err("not a registry server entry: no `name`".into());
    }
    Ok(server)
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: not JSON ({e})", path.display()))
}

/// Write the plugin's two files into `dir`.
pub fn write(plugin: &Plugin, dir: &Path, force: bool) -> Result<(), String> {
    let files = [("plugin.json", &plugin.manifest), ("mcp.json", &plugin.mcp)];
    if !force {
        for (f, _) in files {
            if dir.join(f).exists() {
                return Err(format!(
                    "{} exists (use --force to replace it)",
                    dir.join(f).display()
                ));
            }
        }
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for (f, v) in files {
        let p = dir.join(f);
        std::fs::write(&p, to_json(v)).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    Ok(())
}

/// Run the command; returns the exit code. Output goes to stdout, problems to stderr.
pub fn run(args: Args, registry: &str) -> i32 {
    let fail = |e: String| {
        eprintln!("mcp-better plugin: {e}");
        3
    };
    let server = match &args.source {
        Source::Name(n) => fetch(registry, n),
        Source::File(f) => read_json(f).and_then(server_of),
    };
    let server = match server {
        Ok(s) => s,
        Err(e) => return fail(e),
    };

    if let Some(dir) = &args.check {
        let read = read_json(&dir.join("plugin.json"))
            .and_then(|m| Ok((m, read_json(&dir.join("mcp.json"))?)));
        let (manifest, mcp) = match read {
            Ok(x) => x,
            Err(e) => return fail(e),
        };
        return match check(&server, &manifest, &mcp) {
            Ok(r) => {
                if r.drift.is_empty() {
                    println!(
                        "no drift: {} (server `{}`) starts what the registry lists",
                        dir.display(),
                        r.server
                    );
                } else {
                    println!("drift in {} (server `{}`):", dir.display(), r.server);
                    for d in &r.drift {
                        println!("  - {d}");
                    }
                }
                for n in &r.notes {
                    println!("note: {n}");
                }
                i32::from(!r.drift.is_empty())
            }
            Err(e) => fail(e),
        };
    }

    let pin = if args.latest { Pin::Latest } else { Pin::Exact };
    let plugin = match generate(&server, args.via, pin) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let dir = args
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from(&plugin.name));
    if let Err(e) = write(&plugin, &dir, args.force) {
        return fail(e);
    }
    println!(
        "wrote {}/plugin.json and mcp.json ({} via {})",
        dir.display(),
        plugin.name,
        plugin.via.as_str()
    );
    if !plugin.needs.is_empty() {
        println!("set these in your client (a plugin can't carry them):");
        for n in &plugin.needs {
            println!("  - {n}");
        }
    }
    if !plugin.optional.is_empty() {
        println!("optional, set in your client if you want them:");
        for n in &plugin.optional {
            println!("  - {n}");
        }
    }
    for s in &plugin.skipped {
        println!("not used: {s}");
    }
    0
}
