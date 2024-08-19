#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[contracttype]
pub enum InsuranceStatus {
    UnInitialized = 0,
    Initialized = 1,
    Liquidated = 2,
    Expired = 3,
}

#[contracttype]
#[derive(Clone)]
pub struct InsuranceDetails {
    pub liquidation: u32,
    pub status: InsuranceStatus,
    pub start: u64,
    pub end: u64,
}

const MODERATOR: Symbol = symbol_short!("MODERATOR");
const INSURANCE_DETAILS: Symbol = symbol_short!("DETAILS");

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {
    pub fn set_moderator(env: Env, moderator: Address) {
        assert!(
            env.storage().instance().has(&MODERATOR) == false,
            "moderator for the insurance is already set"
        );
        env.storage().instance().set(&MODERATOR, &moderator);
    }

    pub fn get_insurance_details(env: Env) -> InsuranceDetails {
        env.storage().instance().get(&INSURANCE_DETAILS).unwrap_or(InsuranceDetails {
            liquidation: 0,
            status: InsuranceStatus::UnInitialized,
            start: 0,
            end: 0,
        })
    }

    pub fn init_insurance(env: Env, liquidation: u32, start: u64, end: u64) {
        let moderator: Address = env.storage().instance().get(&MODERATOR).unwrap();
        moderator.require_auth();

        assert_eq!(
            Self::get_insurance_details(env.clone()).status,
            InsuranceStatus::UnInitialized,
            "Insurance already initialized"
        );
        assert!(
            start > env.ledger().timestamp(),
            "Start time must be in the future"
        );
        assert!(
            end > start,
            "End time must be greater than start time"
        );
        assert!(
            liquidation > 0,
            "Liquidation amount must be greater than 0"
        );

        // Set the insurance details
        let mut insurance = Self::get_insurance_details(env.clone());
        insurance.liquidation = liquidation;
        insurance.status = InsuranceStatus::Initialized;
        insurance.start = start;
        insurance.end = end;

        env.storage().instance().set(&INSURANCE_DETAILS, &insurance);
    }
}

mod test;
