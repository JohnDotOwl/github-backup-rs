# github-backup-rs

Fast GitHub repository backup tool in Rust.

`v1.0.0` focuses on one thing: clone and update repositories.

## What It Does

- Clones all repositories for a GitHub user or organization
- Updates existing local clones on repeated runs
- Supports specific repository selection with `--repo owner/repo`

Repository clone-only scope in `v1.0.0`.

## Quick Start

### Requirements

- Rust toolchain (stable)
- `git` available on `PATH`

### Build

```bash
cargo build --release
```

### Backup All Repositories for a User

```bash
cargo run --release -- <github-username> -o ./backup
```

### Backup All Repositories for an Organization

```bash
cargo run --release -- <github-org> --organization -o ./backup
```

### Include Private Repositories

Set `GITHUB_TOKEN` first, then run the same command with your username:

```bash
export GITHUB_TOKEN=ghp_your_token_here
cargo run --release -- <your-username> -o ./backup
```

### Re-run to Update

Run the same command again. Existing repositories are fetched and fast-forwarded.

## Output Layout

```text
backup/
  repositories/
    owner-a/
      repo-one/
    owner-b/
      repo-two/
  repositories.json
```

## Development

```bash
cargo fmt --check
cargo test
```

## Notes

- License: MIT (`LICENSE`)
- Security policy: `SECURITY.md`
- Contribution guidelines: `CONTRIBUTING.md`
- Release notes: `CHANGELOG.md`
