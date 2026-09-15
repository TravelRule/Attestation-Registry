# Travel-Rule Attestation Registry

Repo: `TravelRule/attestation-registry`

An on-chain Soroban contract where allowlisted issuers/anchors register a
counterparty screening result (clear / flagged / pending) against a Stellar
address, and anyone can read that result back.

> **Reference implementation notice.** This is not compliance or legal
> advice, and it is not audited. It's a starting point for teams building
> real travel-rule / screening infrastructure on Stellar. See
> [`docs/SECURITY.md`](docs/SECURITY.md) before using it with real funds or
> real counterparties.

## Why this exists

Anchors and issuers moving MGUSD/USDC/EURC on Stellar currently roll
screening/attestation logic in-house or off-chain, with no shared,
inspectable on-chain record of "has this counterparty been screened, and
what was the result." This registry is that shared record — deliberately
minimal, so it doesn't become the thing that leaks PII on a public ledger.

## Documentation

| Doc | Covers |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Data model, storage layout, how this fits with the other 3 repos |
| [`docs/CONTRACT_INTERFACE.md`](docs/CONTRACT_INTERFACE.md) | Every public function, its auth requirements, and error codes |
| [`docs/SECURITY.md`](docs/SECURITY.md) | The no-PII rule, access-control model, and what an audit would need to check |
| [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) | Building, testing, and deploying to testnet |

## Quickstart

```bash
# Build
cargo build --target wasm32-unknown-unknown --release

# Test
cargo test

# Deploy to testnet — see docs/DEPLOYMENT.md for the full flow
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/travel_rule_attestation_registry.wasm \
  --source <your-key> \
  --network testnet
```

## Relationship to the other travel-rule repos

This registry is one of four repos in the Travel-Rule Toolkit:

```
attestation-registry   <- you are here (this repo)
screening-service       -- writes attestations here after screening
api                     -- reference IVMS101-style messaging API
dashboard               -- reads attestation status for display
schema                  -- shared JSON schemas used by api + dashboard
```

The registry is intentionally the smallest, most conservatively-scoped piece
— it's the one piece that's actually on-chain and immutable-by-default, so
it carries the highest review bar of the four.
