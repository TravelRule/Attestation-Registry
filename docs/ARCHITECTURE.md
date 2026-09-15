# Architecture

## Data model

The registry stores three kinds of data, all in Soroban contract storage:

| Key | Storage type | Value | Purpose |
|---|---|---|---|
| `DataKey::Admin` | instance | `Address` | Set once at `initialize`. Controls issuer allowlist. |
| `DataKey::Issuer(Address)` | persistent | `bool` (presence = true) | Allowlist membership check for a given issuer address. |
| `DataKey::Attestation(Address)` | persistent | `Attestation` struct | Latest screening result for a counterparty address. |

**Why instance storage for admin, persistent for the rest:** the admin
address is small, read on nearly every admin call, and conceptually
"contract configuration" — instance storage is the right fit and is
included in the contract's own footprint automatically. Issuer allowlist
entries and attestations are per-entity, unbounded in count, and need
their own TTL/rent lifecycle independent of the contract's core
instance data — hence persistent storage. See Soroban's storage
documentation for the instance vs. persistent vs. temporary distinction if
you're new to this.

## The `Attestation` record

```rust
pub struct Attestation {
    pub counterparty: Address,
    pub status: ScreeningStatus,   // Clear | Flagged | Pending
    pub timestamp: u64,             // ledger timestamp at write time
    pub reference_id: String,       // opaque pointer to off-chain case — NEVER PII
    pub issuer: Address,            // which issuer wrote this, for audit trail
}
```

Only the **latest** attestation per counterparty is stored on-chain. There
is deliberately no on-chain history table. If you need history:

- The contract emits an `AttestationWritten` event on every write (including
  overwrites) — index these events with something like the
  `soroban-indexer` project to build an off-chain history view.
- This keeps the contract's storage footprint bounded and avoids putting a
  growing, potentially-sensitive audit trail permanently on a public ledger.

## How this fits with the other travel-rule repos

```
                     ┌─────────────────────────┐
  counterparty ─────▶│  screening-service       │
  identifier          │  (off-chain, stub OFAC)  │
                     └────────────┬─────────────┘
                                  │ write_attestation()
                                  ▼
                     ┌─────────────────────────┐
                     │  attestation-registry    │◀── read_attestation()
                     │  (this repo, on-chain)   │       (anyone can call)
                     └────────────┬─────────────┘
                                  │ AttestationWritten event
                                  ▼
                     ┌─────────────────────────┐
                     │  dashboard / indexer     │
                     │  (off-chain consumers)   │
                     └─────────────────────────┘
```

The `api` repo (IVMS101-style messaging) does **not** call this
contract directly in the v1 reference design — it's a separate concern
(institution-to-institution messaging vs. counterparty screening status).
A production system might tie the two together (e.g. gate message
acceptance on a clear attestation), but that's explicitly out of scope for
this reference implementation. See the root playbook for the reasoning
behind keeping these as loosely-coupled, independently-adoptable pieces.

## Upgrade path considerations

This contract has no built-in upgrade mechanism (no admin-triggered code
upgrade function). For a reference implementation this is intentional —
adding upgradeability is itself a security-sensitive decision (who can
upgrade, under what conditions, with what timelock) that shouldn't be
bundled into a starter contract without a team explicitly deciding on that
model. If you fork this for production use, decide on an upgrade strategy
before deploying with real issuers.
