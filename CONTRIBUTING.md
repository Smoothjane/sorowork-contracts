# Contributing to SoroWork Contracts

## Prerequisites

- Rust toolchain (rustup)
- Soroban CLI: `cargo install stellar-cli --features opt`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

## Development

```bash
# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets
```

## Commit Convention

Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`.

## Pull Requests

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes with tests
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a PR with a clear description
