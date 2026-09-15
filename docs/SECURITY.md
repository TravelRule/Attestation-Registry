# Security notes

## This contract is unaudited

Nothing in this repo has been through a professional security audit. Treat
it as a well-tested starting point, not a production-ready system. If you
deploy a fork of this with real issuers and real counterparties, get an
audit first — same posture as recommended for the derivatives platform
project, just with lower fund-custody stakes (this contract never holds
funds).

## The no-PII rule, and why it's enforced at the schema level

The `Attestation` struct has exactly one field that could be misused to
smuggle in PII: `reference_id: String`. Everything else is either a Stellar
address (already public/pseudonymous by nature of being on a public ledger)
or a screening status enum.

**This is a deliberate, narrow surface.** The contract itself cannot
prevent an issuer from putting a name or document number into
`reference_id` — that's a call-site discipline problem, not something
Soroban storage typing can enforce. Mitigations:

1. The `screening-service` repo's integration code is the only
   thing that should ever call `write_attestation`, and its own docs
   (`docs/DATA_SOURCES.md` in that repo) are explicit that `reference_id`
   is an opaque case pointer, not a data field.
2. If you're reviewing a PR that touches the write path, treat any change
   that makes `reference_id` richer (e.g. structured instead of opaque
   string) as a PII-risk review item, not a routine change.
3. Consider adding a max-length check on `reference_id` at the contract
   level (e.g. reject anything over ~64 bytes) as a cheap guardrail against
   someone accidentally dumping a JSON blob of identity data into it. This
   isn't implemented in v1 — it's flagged here as a good first
   community-contributed hardening issue.

## Access control model

- **Admin** is a single address set once at `initialize` and never
  rotated in this version. A production fork should almost certainly
  replace this with a multisig or a timelocked admin-rotation function
  before going live with real issuers — a single admin key is a single
  point of failure for the entire allowlist.
- **Issuers** are added/removed only by admin. There's no self-service
  issuer registration, and no issuer can add another issuer.
- **Attestation writes** require the writing issuer's own signature
  (`issuer.require_auth()`), not just allowlist membership — an
  allowlisted issuer's key still has to actually sign the transaction.
- **Reads are fully public**, by design. Screening status is meant to be
  checkable by anyone (a wallet, a counterparty anchor, a dashboard)
  without needing special access. If your threat model requires
  gating *reads* too, that's a meaningful design change from this
  reference implementation, not a small tweak.

## What an audit would need to check

If you're preparing this for a real audit, the reviewer will want to look
closely at:

- Whether `require_auth()` is called on every state-changing path (it is,
  in this version — verify this hasn't regressed if you've modified the
  contract).
- Storage TTL/rent handling for persistent entries — this reference
  implementation does not implement `extend_ttl` calls, which a
  production deployment will need to avoid entries expiring unexpectedly.
- The lack of an upgrade mechanism (see `ARCHITECTURE.md`) — confirm this
  is an intentional decision for your deployment, not an oversight.
- The `reference_id` PII surface described above.

## Reporting a vulnerability

This is a reference implementation without a formal bug bounty. If you find
a serious issue in the pattern this contract demonstrates, please open an
issue rather than a PR with exploit details, so it can be discussed before
any live fork is affected.
