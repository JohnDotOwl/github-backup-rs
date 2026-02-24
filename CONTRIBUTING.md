# Contributing

Thanks for contributing to `github-backup-rs`.

## Setup

```bash
git clone <your-fork-or-repo-url>
cd github-backup-rs
cargo build
```

## Development Workflow

1. Create a focused branch per change.
2. Keep PRs small enough to review quickly.
3. Add or update tests when behavior changes.
4. Run local checks before pushing:

```bash
cargo fmt --check
cargo test
```

## Pull Request Checklist

- Change is scoped and documented.
- Public behavior changes are reflected in `README.md`.
- New flags/options include help text.
- Error handling paths are covered where practical.

## Design Expectations

- Prefer explicit, typed configuration and errors.
- Preserve compatibility with documented behavior unless intentionally changed.
- Avoid silent data loss; favor atomic writes and clear logs.

## Reporting Issues

Use GitHub Issues for bugs and feature requests. Include:

- command used
- expected vs actual behavior
- logs/error output
- OS and Rust version
