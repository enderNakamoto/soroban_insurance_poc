#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address;
use soroban_sdk::testutils::Ledger;

fn build_insurance<'a>(env: &Env) -> InsuranceContractClient<'a> {
    env.mock_all_auths();

    let contract_id = env.register_contract(None, InsuranceContract);
    let client = InsuranceContractClient::new(&env, &contract_id);

    let moderator = soroban_sdk::Address::generate(&env);
    client.set_moderator(&moderator);

    let timestamp = 1725177875_u64; // 2024-09-01
    // mock the ledger timestamp
    env.ledger().with_mut(|li| {
        li.timestamp = timestamp;
    });

    let liquidation = 1000;
    let start = timestamp + 3600; // add one hour
    let end = timestamp + 2629743;  // add one month

    client.init_insurance(
        &liquidation,
        &start,
        &end
    );

    client
}

#[test]
fn test_set_moderator() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InsuranceContract);
    let client  = InsuranceContractClient::new(&env, &contract_id);

    let moderator = soroban_sdk::Address::generate(&env);
    client.set_moderator(&moderator);
}

#[test]
fn test_get_insurance_details() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InsuranceContract);
    let client  = InsuranceContractClient::new(&env, &contract_id);

    let details: InsuranceDetails = client.get_insurance_details();

    assert_eq!(details.liquidation, 0);
    assert_eq!(details.status, InsuranceStatus::UnInitialized);
    assert_eq!(details.start, 0);
    assert_eq!(details.end, 0);
}


#[test]
fn test_init_insurance(){
    let env = Env::default();
    let client = build_insurance(&env);

    let details: InsuranceDetails = client.get_insurance_details();

    assert_eq!(details.liquidation, 1000);
    assert_eq!(details.status, InsuranceStatus::Initialized);
    assert_eq!(details.start, 1725181475);
    assert_eq!(details.end, 1727807618);
}