#![no_std]
mod errors;
mod types;
mod storage;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, Address};
use crate::{errors::NFTError};



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
    pub fn mint(env: Env, to: Address) -> Result<(), NFTError> {
        
    }
}

mod test;
