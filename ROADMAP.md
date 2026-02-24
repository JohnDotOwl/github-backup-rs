# Roadmap

## Milestone: 0.1.x (Foundation)

- [x] crate/module scaffolding
- [x] CLI entrypoint + plugin entrypoint
- [x] core config/error model
- [x] atomic/smart JSON writes
- [x] repository listing + mirror clone/fetch
- [ ] retry + rate-limit handling integrated in API client

## Milestone: 0.2.x (Resource Coverage)

- [ ] issues + comments + events
- [ ] pull requests + review comments + commits
- [ ] releases + asset downloads
- [ ] labels + milestones + hooks + security advisories
- [ ] wiki + gists + account resources

## Milestone: 0.3.x (Parity and Reliability)

- [ ] incremental modes
- [ ] attachment extraction/download + manifest
- [ ] platform auth parity (classic/fine PAT, app, token file, keychain)
- [ ] robust retry semantics (`Retry-After`, `X-RateLimit-Reset`, jitter)
- [ ] integration tests against mock GitHub API

## Milestone: 1.0.0

- [ ] feature coverage baseline for all documented resources
- [ ] stable CLI/plugin contracts
- [ ] signed release binaries and release notes
