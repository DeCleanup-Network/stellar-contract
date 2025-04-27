#![no_std]
mod errors;
mod types;
mod storage;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, Address};
use storage::NFTStorageLayer;
use crate::{errors::NFTError, types::*};



#[contract]
pub struct NFTContract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl NFTContract {

    pub fn init_admin(env: Env, admin: Address) -> Result<(), NFTError> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(NFTError::AdminAlreadyExists)
        }

        NFTStorageLayer::set_admin(&env, &admin);
        env.storage().instance().set(&COUNTER_KEY, &0u32);
        Ok(())
    }


    // pub fn mint(env: Env, to: Address) -> Result<(), NFTError> {
        
    // }
}

mod test;
