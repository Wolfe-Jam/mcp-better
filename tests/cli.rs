//! The binaries' command line, as a new user first types it.

use std::process::{Command, Output, Stdio};

fn run(bin: &str, args: &[&str], env: &[(&str, &str)]) -> Output {
    let mut c = Command::new(bin);
    c.args(args)
        .env_remove("MCP_TRANSPORT")
        .env_remove("MCP_BETTER_TRANSPORT")
        .stdin(Stdio::null());
    for (k, v) in env {
        c.env(k, v);
    }
    c.output().expect("binary runs")
}

const BETTER: &str = env!("CARGO_BIN_EXE_mcp-better");
const WORSE: &str = env!("CARGO_BIN_EXE_mcp-worse");

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).trim().to_string()
}
fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}

#[test]
fn version_prints_the_version_and_exits_zero_wherever_it_appears() {
    let want = format!("mcp-better {}", env!("CARGO_PKG_VERSION"));
    for args in [
        &["--version"][..],
        &["-V"],
        &["--http", "--version"],
        &["--version", "--http"],
    ] {
        let out = run(BETTER, args, &[]);
        assert!(out.status.success(), "{args:?} exits 0");
        assert_eq!(stdout(&out), want, "{args:?} prints the version on stdout");
    }
}

#[test]
fn help_lists_the_flags_and_exits_zero() {
    let out = run(BETTER, &["--http", "--help"], &[]);
    assert!(out.status.success());
    assert!(stderr(&out).contains("--version"));
}

#[test]
fn unknown_arguments_exit_two_without_serving() {
    for arg in ["--bogus", "-x", "help", "version"] {
        let out = run(BETTER, &[arg], &[]);
        assert_eq!(out.status.code(), Some(2), "{arg} exits 2");
        assert!(
            stderr(&out).contains(&format!("unknown argument `{arg}`")),
            "{arg} is named"
        );
    }
}

#[test]
fn unknown_transport_value_exits_two() {
    let out = run(BETTER, &[], &[("MCP_TRANSPORT", "htp")]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("MCP_TRANSPORT=htp: expected stdio or http"));
}

#[test]
fn mcp_worse_has_the_same_version_flag_and_refuses_unknown_arguments() {
    let out = run(WORSE, &["--version"], &[]);
    assert!(out.status.success());
    assert_eq!(
        stdout(&out),
        format!("mcp-worse {}", env!("CARGO_PKG_VERSION"))
    );

    let out = run(WORSE, &["--http"], &[]);
    assert_eq!(out.status.code(), Some(2), "mcp-worse is stdio only");
}
