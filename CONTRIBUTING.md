# Contributing to Travel-Rule Attestation Registry

Thanks for your interest in contributing! This document covers how to get
started.

## Development Setup

```bash
git clone https://github.com/TravelRule/Attestation-Registry.git
cd Attestation-Registry
```

Ensure you have Rust installed with the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

## Scripts

| Command | Description |
|---|---|
| `cargo test` | Run unit tests |
| `cargo fmt --all` | Format code |
| `cargo clippy --all-targets --all-features` | Run lints |
| `cargo build --target wasm32-unknown-unknown --release` | Build WASM |

## Contract Functions

See `docs/CONTRACT_INTERFACE.md` for the full API reference.

## Pull Request Process

1. Create a feature branch from `main`.
2. Make your changes and ensure `cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test` pass.
3. Open a PR with a clear description of what changed and why.
4. Wait for CI to pass and for a maintainer review.

## Code Style

- We use `rustfmt` for formatting. Run `cargo fmt --all` before committing.
- We use `clippy` for lints. Ensure no warnings before submitting.
- Follow the no-PII rule: this contract must never store personal data.
