#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Vec};

#[contract]
pub struct DAOTreasury;

#[contractimpl]
impl DAOTreasury {

    // Initialize DAO with admin
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    // Deposit function (tracks balance)
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let key = Symbol::new(&env, "BALANCE");
        let mut balance: i128 = env.storage().instance().get(&key).unwrap_or(0);
        balance += amount;

        env.storage().instance().set(&key, &balance);
    }

    // Withdraw (only admin)
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "ADMIN")).unwrap();
        admin.require_auth();

        let key = Symbol::new(&env, "BALANCE");
        let mut balance: i128 = env.storage().instance().get(&key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient funds");
        }

        balance -= amount;
        env.storage().instance().set(&key, &balance);
    }

    // Check treasury balance
    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::new(&env, "BALANCE")).unwrap_or(0)
    }
}