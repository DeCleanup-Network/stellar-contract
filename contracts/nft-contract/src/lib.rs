#![no_std]
mod errors;
mod types;
mod storage;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec, Address, symbol_short};
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

        let topics = (symbol_short!("init"), &admin);
        env.events().publish(topics, &admin);

        Ok(())
    }


    pub fn mint(env: Env, 
                to: Address,
                name: String,
                description: String,
                image_url: String,
                level: u32,
    ) -> Result<(), NFTError> {
        let admin = NFTStorageLayer::retrieve_admin(&env);

        admin.require_auth();

        let token_id: u32 = env.storage().instance().get(&COUNTER_KEY).unwrap();
        env.storage().instance().set(&COUNTER_KEY, &(token_id + 1));

        let metadata = NFTMetadata {
            name,
            description,
            image_url,
            level,
            owner: to.clone()
        };

        NFTStorageLayer::set_token_owner(&env, &token_id, &to);
        NFTStorageLayer::set_token_metadata(&env, &token_id, &metadata);

        NFTStorageLayer::increment_balance(&env, &to);

        let mint_topic = (symbol_short!("mint"), to);
        env.events().publish(mint_topic, token_id);

        Ok(())
    }


    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        token_id: u32,
    ) -> Result<(), NFTError> {
       
        let token_owner = NFTStorageLayer::get_token_owner(&env, &token_id)
            .ok_or(NFTError::TokenDoesNotExist)?;
        
        if token_owner != from {
            return Err(NFTError::InvalidTokenOwner);
        }

        
        from.require_auth();

       
        NFTStorageLayer::set_token_owner(&env, &token_id, &to);
        
        
        NFTStorageLayer::decrement_balance(&env, &from);
        NFTStorageLayer::increment_balance(&env, &to);

        
        let topics = (symbol_short!("transfer"), from, to);
        env.events().publish(topics, token_id);

        Ok(())
    }

    pub fn get_metadata(env: Env, token_id: u32) -> NFTMetadata {
        let _owner = NFTStorageLayer::get_token_owner(&env, &token_id)
            .unwrap_or_else(|| panic!("NFT Token not found"));

        let metadata = NFTStorageLayer::get_token_metadata(&env, &token_id)
            .unwrap_or_else(|| panic!("NFT Metadata not found"));

        metadata

    }

    pub fn burn(env: Env, from: Address, token_id: u32) -> Result<(), NFTError> {
        let token_owner = NFTStorageLayer::get_token_owner(&env, &token_id)
            .ok_or(NFTError::TokenDoesNotExist)?;


        if token_owner != from {
            return Err(NFTError::InvalidTokenOwner);
        }

        
        from.require_auth();

         // Delete the token owner
        env.storage().instance().remove(&TokenKey::TokenOwner(token_id));

        // Delete the token metadata
        env.storage().instance().remove(&TokenKey::TokenMetadata(token_id));

        // Decrease user's balance
        NFTStorageLayer::decrement_balance(&env, &from);

        let topics = (symbol_short!("burn"), from);
        env.events().publish(topics, token_id);

        Ok(())

    }
}

mod test;
