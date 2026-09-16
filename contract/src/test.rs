#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::Env;

fn setup(env: &Env) -> (Address, AttestationRegistryClient<'_>) {
    // soroban-sdk 21.x: register_contract returns the contract Address.
    // (22.x renamed this to env.register.)
    let contract_id = env.register_contract(None, AttestationRegistry);
    let client = AttestationRegistryClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (admin, client)
}

#[test]
fn initialize_can_only_run_once() {
    let env = Env::default();
    env.mock_all_auths();

    // soroban-sdk 21.x: register_contract returns the contract Address.
    // (22.x renamed this to env.register.)
    let contract_id = env.register_contract(None, AttestationRegistry);
    let client = AttestationRegistryClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    let second_admin = Address::generate(&env);
    let result = client.try_initialize(&second_admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn allowlist_add_and_remove() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    let issuer = Address::generate(&env);
    assert!(!client.is_issuer(&issuer));

    client.add_issuer(&issuer);
    assert!(client.is_issuer(&issuer));

    // Adding the same issuer twice is rejected, not silently accepted.
    let result = client.try_add_issuer(&issuer);
    assert_eq!(result, Err(Ok(Error::IssuerAlreadyAllowlisted)));

    client.remove_issuer(&issuer);
    assert!(!client.is_issuer(&issuer));

    // Removing an issuer that isn't allowlisted is rejected.
    let result = client.try_remove_issuer(&issuer);
    assert_eq!(result, Err(Ok(Error::IssuerNotAllowlisted)));
}

#[test]
fn write_attestation_rejects_non_allowlisted_issuer() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    // `issuer` is never added to the allowlist.
    let issuer = Address::generate(&env);
    let counterparty = Address::generate(&env);
    let reference_id = String::from_str(&env, "case-001");

    let result = client.try_write_attestation(
        &issuer,
        &counterparty,
        &ScreeningStatus::Clear,
        &reference_id,
    );

    assert_eq!(result, Err(Ok(Error::NotAllowlistedIssuer)));
}

#[test]
fn write_attestation_then_read_it_back() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    let issuer = Address::generate(&env);
    client.add_issuer(&issuer);

    let counterparty = Address::generate(&env);
    let reference_id = String::from_str(&env, "case-002");

    client.write_attestation(&issuer, &counterparty, &ScreeningStatus::Flagged, &reference_id);

    let attestation = client.read_attestation(&counterparty);
    assert_eq!(attestation.status, ScreeningStatus::Flagged);
    assert_eq!(attestation.issuer, issuer);
    assert_eq!(attestation.reference_id, reference_id);
}

#[test]
fn writing_a_second_attestation_overwrites_the_first() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    let issuer = Address::generate(&env);
    client.add_issuer(&issuer);

    let counterparty = Address::generate(&env);
    let first_ref = String::from_str(&env, "case-003a");
    let second_ref = String::from_str(&env, "case-003b");

    client.write_attestation(&issuer, &counterparty, &ScreeningStatus::Pending, &first_ref);
    client.write_attestation(&issuer, &counterparty, &ScreeningStatus::Clear, &second_ref);

    // The registry only ever holds the latest status for a counterparty —
    // this is by design (see the doc comment on write_attestation). History
    // lives in the emitted events, not in registry storage.
    let attestation = client.read_attestation(&counterparty);
    assert_eq!(attestation.status, ScreeningStatus::Clear);
    assert_eq!(attestation.reference_id, second_ref);
}

#[test]
fn reading_a_missing_attestation_returns_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    let counterparty = Address::generate(&env);
    let result = client.try_read_attestation(&counterparty);
    assert_eq!(result, Err(Ok(Error::AttestationNotFound)));
}

#[test]
fn write_attestation_emits_an_event() {
    let env = Env::default();
    env.mock_all_auths();
    let (_, client) = setup(&env);

    let issuer = Address::generate(&env);
    client.add_issuer(&issuer);

    let counterparty = Address::generate(&env);
    let reference_id = String::from_str(&env, "case-004");

    client.write_attestation(&issuer, &counterparty, &ScreeningStatus::Clear, &reference_id);

    let events = env.events().all();
    // At least one event was published as part of this call. A production
    // test suite should also decode the event payload and assert its
    // fields match what was written; kept minimal here for the reference
    // implementation.
    assert!(events.len() > 0);
}
