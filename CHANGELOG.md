# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog:
https://keepachangelog.com/en/1.1.0/

## [1.0.0] - 2026-02-24

### Added

- Repository backup flow focused on cloning and updating repositories
- User and organization repository discovery through the GitHub API
- Repository selection with `--repo owner/repo`
- Token support from `--token`, `--token-file`, keychain, and `GITHUB_TOKEN`
- Automatic update of existing local clones on repeat runs
