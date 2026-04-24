#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, Address,
};

#[contracttype]
#[derive(Clone)]
pub struct PaymentConfig {
    pub payer: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub interval: u64,      // seconds (e.g. 30 days ≈ 2_592_000)
    pub last_payment: u64,
}

#[contract]
pub struct ChildSupportContract;

#[contractimpl]
impl ChildSupportContract {

    // Initialize payment agreement
    pub fn init(
        env: Env,
        payer: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        interval: u64,
    ) {
        payer.require_auth();

        let config = PaymentConfig {
            payer,
            recipient,
            token,
            amount,
            interval,
            last_payment: env.ledger().timestamp(),
        };

        env.storage().instance().set(&symbol_short!("CONFIG"), &config);
    }

    // Trigger payment if due
    pub fn pay(env: Env) {
        let mut config: PaymentConfig = env
            .storage()
            .instance()
            .get(&symbol_short!("CONFIG"))
            .expect("Contract not initialized");

        let now = env.ledger().timestamp();

        if now < config.last_payment + config.interval {
            panic!("Payment not due yet");
        }

        let token_client = soroban_sdk::token::Client::new(&env, &config.token);

        // Transfer from payer → recipient
        token_client.transfer(
            &config.payer,
            &config.recipient,
            &config.amount,
        );

        config.last_payment = now;

        env.storage().instance().set(&symbol_short!("CONFIG"), &config);
    }

    // View configuration
    pub fn get_config(env: Env) -> PaymentConfig {
        env.storage()
            .instance()
            .get(&symbol_short!("CONFIG"))
            .expect("Contract not initialized")
    }
}
