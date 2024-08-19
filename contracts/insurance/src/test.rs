#![cfg(test)]

use super::*;

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
