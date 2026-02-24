# github-backup-rs

Native Rust GitHub backup tool.

## Project Status

This project is early and not feature-complete yet.

- Implemented: project scaffold, config/error model, repository discovery + git mirror clone/fetch, atomic/smart JSON writes
- In progress: expanded coverage for issues, pull requests, releases, account data, attachments, retries/rate-limit handling, and auth edge cases

Until `1.0.0`, expect breaking changes.

## Why This Exists

GitHub backup workflows need predictable, portable tooling. This project targets:

- single static-ish binary distribution
- lower operational overhead (TLS/runtime/dependency model)

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

## Development

Use these checks before opening a PR:

```bash
cargo fmt --check
cargo test
```

## Repository Layout

High-level layout:

- `src/cli/` CLI args and orchestration
- `src/api/` GitHub API client, pagination, retry, throttling
- `src/backup/` backup routines per resource type
- `src/git/` git subprocess wrapper and URL helpers
- `src/io/` atomic/smart file writes
- `src/auth/` auth providers (token, file token, keychain, app)

## Project Notes

- License: MIT (`LICENSE`)
- Security policy: `SECURITY.md`
- Contribution guidelines: `CONTRIBUTING.md`
- Milestone plan: `ROADMAP.md`
- Release notes: `CHANGELOG.md`

## Acknowledgments

Thanks to everyone contributing bug reports, tests, and implementation work.
