# Building, testing, and deploying

## Prerequisites

- Rust (stable) with the `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- The [Soroban CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli).
- A funded testnet account (use [Friendbot](https://friendbot.stellar.org)).

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM will be at:
```
target/wasm32-unknown-unknown/release/travel_rule_attestation_registry.wasm
```

## Test

```bash
cargo test
```

This runs the unit tests in `src/test.rs`, covering unauthorized writes,
allowlist add/remove, overwrite behavior, and event emission. All tests run
against an in-memory `Env` — no testnet connection needed.

> **Note on this reference implementation:** these tests were written
> against the soroban-sdk 21.x API surface as documented at the time of
> writing. Soroban's SDK evolves — if `cargo test` reports API mismatches,
> check the installed `soroban-sdk` version against `Cargo.toml` and consult
> the SDK's changelog for renamed methods (this has happened before with
> e.g. `env.register_contract` → `env.register`).

## Deploy to testnet

```bash
# 1. Generate/fund a deployer identity if you don't have one
soroban keys generate deployer --network testnet
soroban keys fund deployer --network testnet

# 2. Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/travel_rule_attestation_registry.wasm \
  --source deployer \
  --network testnet

# Save the returned contract ID — you'll need it for the other repos'
# .env files (screening-service, dashboard).
```

## Initialize

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize --admin <ADMIN_ADDRESS>
```

## Add your first issuer (e.g. the screening-service's own signing key)

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- add_issuer --issuer <ISSUER_ADDRESS>
```

## Sanity-check a read

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- read_attestation --counterparty <SOME_ADDRESS>
```

This should return `AttestationNotFound` (error code 7) until an issuer has
actually written a record for that address — that's expected, not a bug.
