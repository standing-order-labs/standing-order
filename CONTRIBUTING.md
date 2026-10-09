# Contributing to Standing Order

Thanks for helping! This project handles money-adjacent logic, so we favor small, well-tested, well-explained changes.

## Ground rules

- **Pick an issue first.** Comment to be assigned before you start; do not work on issues assigned to someone else.
- **Keep scope tight.** One issue, one PR. Do not bundle refactors or unrelated fixes.
- **Understand what you submit.** AI-assisted code is fine only if you have read, run and can explain every line. Low-quality, untested or unexplained submissions will be closed.
- **No drive-by trivia.** Pure typo or whitespace PRs made to collect credit will not be merged.

## Setup

```bash
rustup target add wasm32v1-none
cargo install --locked stellar-cli   # 25.2+
cargo test --workspace
```

## Workflow

1. Fork the repo and create a branch: `feat/<short-name>` or `fix/<short-name>`.
2. Make your change with tests.
3. Run the full check locally:
   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. Commit using [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).
5. Open a PR using the template and link the issue with `Closes #<n>`.

## Contract change checklist

- Every new public function has tests for the success path and each error path.
- Every state-changing function calls `require_auth()` for the right party, or documents why it is permissionless.
- New storage keys are documented in `docs/ARCHITECTURE.md`.
- User-visible changes are recorded in `CHANGELOG.md`.

## Reviews

Maintainers aim to respond within a few days. Be kind in review; see the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing you agree that your contributions are licensed under the Apache License 2.0.
