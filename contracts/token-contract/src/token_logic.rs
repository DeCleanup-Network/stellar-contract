use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};

use crate::token_trait::TokenInterface;

//==== Constants for storage keys ====//
const STORAGE_NAME: Symbol = symbol_short!("NAME");
const STORAGE_SYMBOL: Symbol = symbol_short!("SYMBOL");
const STORAGE_DECIMALS: Symbol = symbol_short!("DECIMALS");
//==== Token contract struct ====//
#[contract]
pub struct Token;

//==== Implement the Token Interface for Token Contract ====//
#[contractimpl]
impl TokenInterface for Token {
    //==== Allowance Function ====//
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = (symbol_short!("allowance"), from, spender);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    //==== Approve Function (Sets allowance and publishes event) ====//
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        let key = (symbol_short!("allowance"), from.clone(), spender.clone());
        env.storage().persistent().set(&key, &amount);

        let expiration_key = (symbol_short!("expire"), from.clone(), spender.clone());
        env.storage()
            .persistent()
            .set(&expiration_key, &expiration_ledger);

        env.events().publish(
            (symbol_short!("approve"), from, spender),
            (amount, expiration_ledger),
        );
    }

    //==== Balance Function (Returns balance of an address) ====//
    fn balance(env: Env, id: Address) -> i128 {
        let key = (symbol_short!("balance"), id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    //==== transfer function ====//
    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth(); //==== Ensure the 'from' address has authorization to perform the transfer.

        let from_key = (symbol_short!("balance"), from.clone());
        let mut from_balance = env.storage().persistent().get(&from_key).unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");
        assert!(amount > 0, "Amount must be positive");

        from_balance -= amount;
        env.storage().persistent().set(&from_key, &from_balance);

        let to_key = (symbol_short!("balance"), to.clone());
        let mut to_balance = env.storage().persistent().get(&to_key).unwrap_or(0);
        to_balance += amount;
        env.storage().persistent().set(&to_key, &to_balance);

        env.events()
            .publish((symbol_short!("transfer"), from, to), amount);
    }

    //==== transfer_from function ====//
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance_key = (symbol_short!("allowance"), from.clone(), spender.clone());
        let mut allowance = env.storage().persistent().get(&allowance_key).unwrap_or(0);

        // Check if the allowance has expired
        let expiration_key = (symbol_short!("expire"), from.clone(), spender.clone());
        if let Some(expiration_ledger) = env.storage().persistent().get(&expiration_key) {
            let current_block = env.ledger().sequence();
            assert!(current_block <= expiration_ledger, "Allowance expired");
        }

        //==== Check if there is enough allowance for the transfer ====//
        assert!(allowance >= amount, "insufficient allowance");
        assert!(amount > 0, "Amount must be positive");

        allowance -= amount;
        env.storage().persistent().set(&allowance_key, &allowance);

        let from_key = (symbol_short!("balance"), from.clone());
        let mut from_balance = env.storage().persistent().get(&from_key).unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");

        from_balance -= amount;
        env.storage().persistent().set(&from_key, &from_balance);

        let to_key = (symbol_short!("balance"), to.clone());
        let mut to_balance = env.storage().persistent().get(&to_key).unwrap_or(0);
        to_balance += amount;
        env.storage().persistent().set(&to_key, &to_balance);

        // Emit the transfer event
        env.events()
            .publish((symbol_short!("transfer"), from, to), amount);
    }

    //==== burn function ====//
    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth(); //==== Ensure the 'from' address has authorization to burn the tokens.

        let from_key = (symbol_short!("balance"), from.clone());
        let mut from_balance = env.storage().persistent().get(&from_key).unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");
        assert!(amount > 0, "Amount must be positive");

        from_balance -= amount;
        env.storage().persistent().set(&from_key, &from_balance);

        // Emit burn event
        env.events().publish((symbol_short!("burn"), from), amount);
    }

    //==== burn_from function ====//
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth(); //==== Ensure the spender has authorization to burn the tokens on behalf of 'from'.

        let allowance_key = (symbol_short!("allowance"), from.clone(), spender.clone());
        let mut allowance = env.storage().persistent().get(&allowance_key).unwrap_or(0);
        assert!(allowance >= amount, "insufficient allowance");
        assert!(amount > 0, "Amount must be positive");

        allowance -= amount;
        env.storage().persistent().set(&allowance_key, &allowance);

        let from_key = (symbol_short!("balance"), from.clone());
        let mut from_balance = env.storage().persistent().get(&from_key).unwrap_or(0);
        assert!(from_balance >= amount, "insufficient balance");

        from_balance -= amount;
        env.storage().persistent().set(&from_key, &from_balance);

        // Emit burn event
        env.events().publish((symbol_short!("burn"), from), amount);
    }

    //==== decimals function ====//
    fn decimals(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&STORAGE_DECIMALS)
            .unwrap_or(18) // Default to 18 if not set
    }

    //==== name function ====//
    fn name(env: Env) -> String {
        env.storage()
            .persistent()
            .get(&STORAGE_NAME)
            .unwrap_or_else(|| String::from_str(&env, "My Token")) // Default to "My Token" if not initialized
    }

    //==== symbol function ====//
    fn symbol(env: Env) -> String {
        env.storage()
            .persistent()
            .get(&STORAGE_SYMBOL)
            .unwrap_or_else(|| String::from_str(&env, "MYTOKEN")) // Default to "MYTOKEN" if not initialized
    }
}
