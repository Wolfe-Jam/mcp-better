//! Command line for the shipped binaries: `--version` and `--help` win wherever they
//! appear, an unknown argument or `MCP_TRANSPORT` value is an error (exit 2), and
//! nothing silently falls back to stdio.

/// What the binary should do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Print `<name> <version>` on stdout, exit 0.
    Version,
    /// Print usage on stderr, exit 0.
    Help,
    /// Serve over this transport.
    Serve(Transport),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Stdio,
    Http,
}

/// Parse the arguments (without the program name) and the two transport variables.
///
/// `http_allowed` is false for `mcp-worse`, which only speaks stdio.
pub fn parse(
    args: &[String],
    mcp_transport: Option<&str>,
    mcp_better_transport: Option<&str>,
    http_allowed: bool,
) -> Result<Action, String> {
    // --version / --help win wherever they appear; the first of the two wins.
    for a in args {
        match a.as_str() {
            "--version" | "-V" => return Ok(Action::Version),
            "--help" | "-h" => return Ok(Action::Help),
            _ => {}
        }
    }

    // CLI wins over env; the first transport argument wins.
    let mut chosen: Option<Transport> = None;
    for a in args {
        let t = match a.as_str() {
            "--stdio" | "stdio" => Transport::Stdio,
            "--http" | "http" if http_allowed => Transport::Http,
            other => return Err(format!("unknown argument `{other}`")),
        };
        chosen.get_or_insert(t);
    }
    if let Some(t) = chosen {
        return Ok(Action::Serve(t));
    }

    // MCP_TRANSPORT first, then MCP_BETTER_TRANSPORT; an empty value counts as unset.
    for (name, value) in [
        ("MCP_TRANSPORT", mcp_transport),
        ("MCP_BETTER_TRANSPORT", mcp_better_transport),
    ] {
        let Some(v) = value.map(str::trim).filter(|v| !v.is_empty()) else {
            continue;
        };
        return match v.to_ascii_lowercase().as_str() {
            "stdio" => Ok(Action::Serve(Transport::Stdio)),
            "http" if http_allowed => Ok(Action::Serve(Transport::Http)),
            _ => Err(format!(
                "{name}={v}: expected {}",
                if http_allowed {
                    "stdio or http"
                } else {
                    "stdio"
                }
            )),
        };
    }
    Ok(Action::Serve(Transport::Stdio))
}

/// [`parse`] over the real process arguments and environment.
pub fn from_env(http_allowed: bool) -> Result<Action, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let t = std::env::var("MCP_TRANSPORT").ok();
    let b = std::env::var("MCP_BETTER_TRANSPORT").ok();
    parse(&args, t.as_deref(), b.as_deref(), http_allowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(args: &[&str]) -> Result<Action, String> {
        let a: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        parse(&a, None, None, true)
    }

    #[test]
    fn version_and_help_win_anywhere() {
        assert_eq!(p(&["--version"]), Ok(Action::Version));
        assert_eq!(p(&["-V"]), Ok(Action::Version));
        assert_eq!(p(&["--http", "--version"]), Ok(Action::Version));
        assert_eq!(p(&["--http", "-h"]), Ok(Action::Help));
        assert_eq!(p(&["--bogus", "--help"]), Ok(Action::Help));
    }

    #[test]
    fn unknown_arguments_are_errors() {
        for a in ["--bogus", "-x", "help", "version", "extra"] {
            assert_eq!(p(&[a]), Err(format!("unknown argument `{a}`")));
        }
        assert!(p(&["--stdio", "extra"]).is_err());
    }

    #[test]
    fn transport_args_first_wins() {
        assert_eq!(p(&[]), Ok(Action::Serve(Transport::Stdio)));
        assert_eq!(p(&["--http"]), Ok(Action::Serve(Transport::Http)));
        assert_eq!(p(&["http"]), Ok(Action::Serve(Transport::Http)));
        assert_eq!(
            p(&["--stdio", "--http"]),
            Ok(Action::Serve(Transport::Stdio))
        );
    }

    #[test]
    fn env_transport_is_checked() {
        let e = |t: Option<&str>, b: Option<&str>| parse(&[], t, b, true);
        assert_eq!(e(Some("HTTP"), None), Ok(Action::Serve(Transport::Http)));
        assert_eq!(e(None, Some("http")), Ok(Action::Serve(Transport::Http)));
        assert_eq!(
            e(Some("stdio"), Some("http")),
            Ok(Action::Serve(Transport::Stdio))
        );
        assert_eq!(
            e(Some(""), Some("http")),
            Ok(Action::Serve(Transport::Http))
        );
        assert_eq!(
            e(Some("htp"), None),
            Err("MCP_TRANSPORT=htp: expected stdio or http".into())
        );
        // CLI wins over env, even a bad env value.
        assert_eq!(
            parse(&["--stdio".into()], Some("htp"), None, true),
            Ok(Action::Serve(Transport::Stdio))
        );
    }

    #[test]
    fn stdio_only_binary_refuses_http() {
        let a = vec!["--http".to_string()];
        assert_eq!(
            parse(&a, None, None, false),
            Err("unknown argument `--http`".into())
        );
        assert_eq!(
            parse(&[], Some("http"), None, false),
            Err("MCP_TRANSPORT=http: expected stdio".into())
        );
        assert_eq!(
            parse(&[], None, None, false),
            Ok(Action::Serve(Transport::Stdio))
        );
    }
}
