# github-backup-rs

Native Rust GitHub backup tool with two execution modes:

- standalone CLI for local backups
- JSON-RPC plugin mode over stdin/stdout for Mainframe integration

## Project Status

This project is early and not feature-complete yet.

- Implemented: project scaffold, config/error model, plugin RPC shell, repository discovery + git mirror clone/fetch, atomic/smart JSON writes
- In progress: expanded coverage for issues, pull requests, releases, account data, attachments, retries/rate-limit handling, and auth edge cases

Until `1.0.0`, expect breaking changes.

## Why This Exists

GitHub backup workflows need predictable, portable tooling. This project targets:

- single static-ish binary distribution
- lower operational overhead (TLS/runtime/dependency model)
- easier integration with plugin hosts through JSON-RPC

## Quick Start

### Requirements

- Rust toolchain (stable)
- `git` available on `PATH`

### Build

```bash
cargo build
```

### Run CLI

```bash
cargo run -- <user-or-org> --repositories -o /tmp/backup
```

Example:

```bash
cargo run -- octocat --repositories -o /tmp/backup/octocat
```

### Run Plugin Mode

```bash
printf '{"jsonrpc":"2.0","method":"system.info","id":1}\n' | cargo run -- --plugin
```

## Development

Use these checks before opening a PR:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Repository Layout

High-level layout:

- `src/cli/` CLI args and orchestration
- `src/plugin/` JSON-RPC protocol handling
- `src/api/` GitHub API client, pagination, retry, throttling
- `src/backup/` backup routines per resource type
- `src/git/` git subprocess wrapper and URL helpers
- `src/io/` atomic/smart file writes
- `src/auth/` auth providers (token, file token, keychain, app)

## Open Source Notes

- License: MIT (`LICENSE`)
- Security policy: `SECURITY.md`
- Contribution guidelines: `CONTRIBUTING.md`
- Milestone plan: `ROADMAP.md`
- Release notes: `CHANGELOG.md`

## Acknowledgments

Thanks to everyone contributing bug reports, tests, and implementation work.
