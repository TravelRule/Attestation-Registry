//! Travel-Rule Attestation Registry
//!
//! A minimal on-chain registry that lets allowlisted issuers/anchors record a
//! counterparty screening result (clear / flagged / pending) against a
//! Stellar address, and lets anyone read that result back.
//!
//! ## What this contract deliberately does NOT store
//!
//! No personally identifiable information (PII) is stored on-chain, ever.
//! An attestation record links a *Stellar address* to a *screening status*
//! and an *off-chain reference ID* — the reference ID is a pointer an
//! off-chain screening service can use to look up the underlying case, not
//! a place to put a name, document number, or any other identifying data.
//! See docs/SECURITY.md for the full data-model rationale.
//!
//! ## Reference implementation notice
//!
//! This contract is a reference implementation for the Stellar Wave
//! ecosystem, not a compliance/legal product. See the root README for scope
//! and limitations.

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScreeningStatus {
    Clear = 0,
    Flagged = 1,
    Pending = 2,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Attestation {
    pub counterparty: Address,
    pub status: ScreeningStatus,
    /// Ledger timestamp (seconds since epoch) the attestation was written at.
    pub timestamp: u64,
    /// Opaque pointer into the off-chain screening service's own records.
    /// Never PII — see docs/SECURITY.md.
    pub reference_id: String,
    /// Which issuer wrote this attestation, for audit trail purposes.
    pub issuer: Address,
}

#[contracttype]
enum DataKey {
    Admin,
    Issuer(Address),
    Attestation(Address), // keyed by counterparty address
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    NotAllowlistedIssuer = 4,
    IssuerAlreadyAllowlisted = 5,
    IssuerNotAllowlisted = 6,
    AttestationNotFound = 7,
}

/// Emitted whenever an attestation is written (including overwrites of an
/// existing attestation for the same counterparty).
#[contractevent]
pub struct AttestationWritten {
    #[topic]
    pub counterparty: Address,
    pub issuer: Address,
    pub status: ScreeningStatus,
    pub reference_id: String,
    pub timestamp: u64,
}

#[contractevent]
pub struct IssuerAllowlisted {
    #[topic]
    pub issuer: Address,
}

#[contractevent]
pub struct IssuerRemoved {
    #[topic]
    pub issuer: Address,
}

#[contract]
pub struct AttestationRegistry;

#[contractimpl]
impl AttestationRegistry {
    /// One-time setup. Must be called exactly once, immediately after
    /// deployment, by whoever will act as admin.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Admin-only: add an issuer to the allowlist. Only allowlisted issuers
    /// may write attestations.
    pub fn add_issuer(env: Env, issuer: Address) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Issuer(issuer.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::IssuerAlreadyAllowlisted);
        }
        env.storage().persistent().set(&key, &true);

        IssuerAllowlisted { issuer }.publish(&env);
        Ok(())
    }

    /// Admin-only: remove an issuer from the allowlist.
    pub fn remove_issuer(env: Env, issuer: Address) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Issuer(issuer.clone());
        if !env.storage().persistent().has(&key) {
            return Err(Error::IssuerNotAllowlisted);
        }
        env.storage().persistent().remove(&key);

        IssuerRemoved { issuer }.publish(&env);
        Ok(())
    }

    /// Read-only: check whether an address is currently an allowlisted issuer.
    pub fn is_issuer(env: Env, issuer: Address) -> bool {
        env.storage().persistent().has(&DataKey::Issuer(issuer))
    }

    /// Write (or overwrite) an attestation for a counterparty. Only callable
    /// by an allowlisted issuer, and the issuer must authorize the call.
    ///
    /// Overwriting an existing attestation for the same counterparty is
    /// allowed by design — screening status can legitimately change over
    /// time (e.g. pending -> clear). The previous record is not kept
    /// on-chain; if you need history, index `AttestationWritten` events
    /// off-chain (this is exactly what the soroban-indexer project is for).
    pub fn write_attestation(
        env: Env,
        issuer: Address,
        counterparty: Address,
        status: ScreeningStatus,
        reference_id: String,
    ) -> Result<(), Error> {
        issuer.require_auth();

        if !env.storage().persistent().has(&DataKey::Issuer(issuer.clone())) {
            return Err(Error::NotAllowlistedIssuer);
        }

        let timestamp = env.ledger().timestamp();

        let attestation = Attestation {
            counterparty: counterparty.clone(),
            status,
            timestamp,
            reference_id: reference_id.clone(),
            issuer: issuer.clone(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Attestation(counterparty.clone()), &attestation);

        AttestationWritten {
            counterparty,
            issuer,
            status,
            reference_id,
            timestamp,
        }
        .publish(&env);

        Ok(())
    }

    /// Read-only: fetch the current attestation for a counterparty, if any.
    /// Callable by anyone — attestation status is meant to be publicly
    /// checkable (that's the point of the registry), even though write
    /// access is restricted.
    pub fn read_attestation(env: Env, counterparty: Address) -> Result<Attestation, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Attestation(counterparty))
            .ok_or(Error::AttestationNotFound)
    }

    fn require_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }
}

mod test;
