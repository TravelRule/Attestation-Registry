# Contract interface reference

All functions below are public entry points on the `AttestationRegistry`
contract. Types referenced (`ScreeningStatus`, `Attestation`, `Error`) are
defined in `src/lib.rs`.

---

### `initialize(admin: Address) -> Result<(), Error>`

One-time setup. Must be called exactly once, immediately after deployment.

- **Auth required:** `admin` must sign.
- **Errors:** `AlreadyInitialized` if called more than once.

---

### `add_issuer(issuer: Address) -> Result<(), Error>`

Adds `issuer` to the write-allowlist.

- **Auth required:** the stored admin must sign.
- **Errors:** `NotInitialized` if called before `initialize`; `IssuerAlreadyAllowlisted` if already present.
- **Events:** emits `IssuerAllowlisted { issuer }`.

---

### `remove_issuer(issuer: Address) -> Result<(), Error>`

Removes `issuer` from the write-allowlist. Existing attestations they wrote
are **not** deleted or invalidated — this only affects future write access.

- **Auth required:** the stored admin must sign.
- **Errors:** `NotInitialized`; `IssuerNotAllowlisted` if not present.
- **Events:** emits `IssuerRemoved { issuer }`.

---

### `is_issuer(issuer: Address) -> bool`

Read-only. No auth required. Returns whether `issuer` is currently
allowlisted.

---

### `write_attestation(issuer: Address, counterparty: Address, status: ScreeningStatus, reference_id: String) -> Result<(), Error>`

Writes (or overwrites) the attestation for `counterparty`.

- **Auth required:** `issuer` must sign.
- **Errors:** `NotAllowlistedIssuer` if `issuer` isn't on the allowlist.
- **Events:** emits `AttestationWritten { counterparty, issuer, status, reference_id, timestamp }`.
- **Note:** overwriting an existing attestation is allowed by design —
  status can legitimately change (pending → clear, clear → flagged on a
  later sanctions-list update, etc). Only the latest record is kept
  on-chain; see `docs/ARCHITECTURE.md` for how to get history.

---

### `read_attestation(counterparty: Address) -> Result<Attestation, Error>`

Read-only. No auth required — attestation status is meant to be publicly
checkable by any anchor, wallet, or dashboard, even though writing is
gated.

- **Errors:** `AttestationNotFound` if no attestation exists yet for this
  counterparty.

---

## Error reference

| Variant | Value | Meaning |
|---|---|---|
| `AlreadyInitialized` | 1 | `initialize` called more than once |
| `NotInitialized` | 2 | Admin-gated call made before `initialize` |
| `NotAdmin` | 3 | Reserved; not currently returned (admin check happens via `require_admin` returning `NotInitialized`) |
| `NotAllowlistedIssuer` | 4 | `write_attestation` called by a non-allowlisted address |
| `IssuerAlreadyAllowlisted` | 5 | `add_issuer` called on an address already allowlisted |
| `IssuerNotAllowlisted` | 6 | `remove_issuer` called on an address not allowlisted |
| `AttestationNotFound` | 7 | `read_attestation` called for a counterparty with no record |

## Events

| Event | Topic | Fields |
|---|---|---|
| `AttestationWritten` | `counterparty` | `issuer`, `status`, `reference_id`, `timestamp` |
| `IssuerAllowlisted` | `issuer` | — |
| `IssuerRemoved` | `issuer` | — |

Index these with an off-chain indexer (e.g. the companion
`soroban-indexer` project) if you need attestation history, issuer-change
audit logs, or real-time notification when a counterparty's status changes.
