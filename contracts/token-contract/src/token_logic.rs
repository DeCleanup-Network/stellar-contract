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
}