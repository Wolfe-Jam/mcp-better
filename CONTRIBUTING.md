# Contributing

## Scope

This is a **BETTER** textbook MCP server. Prefer:

- Protocol honesty (claim only what tests prove)
- Small surface area on `main`
- Discover + stamped lists over new transport sprawl

## Before a PR

```bash
export PATH="$HOME/.cargo/bin:$PATH"
bash scripts/ci.sh
# equivalent: fmt · clippy -D · test · purity · Discover smoke
```

## Releases

Use **`/pubbetter`** — dry-run, green CI, tag, crates.io, MCP Registry.  
Do **not** publish by hand without that protocol. See [`docs/PUBBETTER.md`](./docs/PUBBETTER.md).

## Do not

- Add `project.faf` to `main` (BEST is optional elsewhere; see BETTER.md)
- Rebrand identity to `one.faf/*` — Registry name stays `io.github.Wolfe-Jam/mcp-better`
- Claim HTTP/7/28 full surface without CI

## Code of conduct

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
