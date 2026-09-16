# Playbook: `TravelRule/attestation-registry`

**Role in the toolkit:** on-chain Soroban contract — the source of truth for counterparty screening status
**Depends on:** nothing (no other repo)
**Depended on by:** `screening-service` (writes), `dashboard` (reads, indirectly via screening-service in v1)

---

## What this repo does

A minimal Soroban contract where allowlisted issuers write a screening
status (clear/flagged/pending) against a counterparty's Stellar address,
and anyone can read it back. No PII stored on-chain — only addresses, a
status enum, a timestamp, and an opaque off-chain case reference.

## Current state (as built)

- ✅ Contract logic complete: `initialize`, `add_issuer`, `remove_issuer`,
  `is_issuer`, `write_attestation`, `read_attestation`
- ✅ Unit tests covering unauthorized writes, allowlist add/remove,
  overwrite behavior, missing-record reads, event emission
- ✅ Docs: README, ARCHITECTURE, CONTRACT_INTERFACE, SECURITY, DEPLOYMENT
- ❌ **Never compiled or tested against a real toolchain** — this was
  built in a sandbox with no Rust installed. `cargo test` has not
  actually been run.
- ❌ Not deployed anywhere (testnet or otherwise)

## What's left before this repo is submission-ready

### Blocking
- [ ] Run `cargo build --target wasm32-unknown-unknown --release` and
  `cargo test` locally — fix any soroban-sdk API mismatches (version
  drift is likely; the code was written against the 21.x API surface
  from memory, not compiled)
- [ ] Deploy to Soroban testnet, run `initialize`, `add_issuer`, and a
  manual `write_attestation` / `read_attestation` round trip to confirm
  it actually works end to end
- [ ] Set up CI (GitHub Actions) running `cargo test` on push — none
  exists yet, unlike the indexer repo which already has this

### Should-do
- [ ] Generate real TS/JS bindings (`soroban contract bindings
  typescript`) once deployed — `screening-service` needs these to leave
  dry-run mode
- [ ] Add a max-length guard on `reference_id` (flagged in SECURITY.md as
  a good hardening issue, not yet implemented)
- [ ] Decide and document an upgrade strategy if you intend to iterate on
  this post-deployment (currently no upgrade mechanism at all)

### Nice-to-have
- [ ] Seed 3-5 "good first issue"-labeled GitHub issues (multisig admin,
  reference_id length guard, TTL/rent extension handling, a second
  status-history event decoder example)
- [ ] LICENSE, description, topics set in GitHub (see earlier checklist)

## Phased completion prompts

**Phase 1 — Get it compiling**
```
Run `cargo build --target wasm32-unknown-unknown --release` and `cargo
test` in this repo. Fix any compilation errors against the installed
soroban-sdk version — check Cargo.toml's pinned version against what's
actually resolved, and consult the soroban-sdk changelog for any renamed
APIs (e.g. env.register_contract vs env.register, contractevent macro
syntax). Do not change the contract's external behavior — only fix
compile/API issues. Report what had to change and why.
```

**Phase 2 — Deploy and smoke-test on testnet**
```
Follow docs/DEPLOYMENT.md to deploy this contract to Soroban testnet.
After deployment, run initialize, add_issuer for a test issuer address,
write_attestation for a test counterparty, and read_attestation to
confirm the round trip works. Record the deployed contract ID and the
commands used in a new docs/TESTNET_DEPLOYMENT_LOG.md for reference.
```

**Phase 3 — CI**
```
Add a GitHub Actions workflow (.github/workflows/ci.yml) that installs
the Rust toolchain with the wasm32-unknown-unknown target and runs
`cargo test` on every push and PR. Cache the cargo registry/target
directories to keep runs fast.
```

**Phase 4 — Generate bindings and issue-seeding**
```
Generate TypeScript bindings for this deployed contract using `soroban
contract bindings typescript`, commit them under generated/ or document
the exact command in README for consumers to run themselves. Then draft
5-8 GitHub issues covering: multisig/admin rotation, reference_id length
guard, TTL/rent extension automation, and any real audit-readiness gaps
found in docs/SECURITY.md.
```
