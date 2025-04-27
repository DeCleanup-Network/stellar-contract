use crate::types::{TokenKey, NFTMetadata, ADMIN_KEY};
use soroban_sdk::{Address, Env};

pub struct NFTStorageLayer;

impl NFTStorageLayer {
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&ADMIN_KEY, admin);
    }

    pub fn retrieve_admin(env: &Env) -> Address {
        env.storage().instance().get(&ADMIN_KEY).unwrap()
    }

    pub fn increment_balance(env: &Env, address: &Address) {
        let balance: u32 = env
            .storage()
            .instance()
            .get(&TokenKey::UserTokenBalance(address.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&TokenKey::UserTokenBalance(address.clone()), &(balance + 1));
    }

    pub fn decrement_balance(env: &Env, address: &Address) {
        let balance: u32 = env
            .storage()
            .instance()
            .get(&TokenKey::UserTokenBalance(address.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&TokenKey::UserTokenBalance(address.clone()), &(balance - 1));
    }


    pub fn set_token_owner(env: &Env, token_id: &u32, owner: &Address) {
        env.storage().instance().set(&TokenKey::TokenOwner(*token_id), owner);
    }

    pub fn get_token_owner(env: &Env, token_id: &u32) -> Option<Address> {
        env.storage().instance().get(&TokenKey::TokenOwner(*token_id))
    }

    pub fn set_token_metadata(env: &Env, token_id: &u32, metadata: &NFTMetadata) {
        env.storage().instance().set(&TokenKey::TokenMetadata(*token_id), metadata);
    }

    pub fn get_token_metadata(env: &Env, token_id: &u32) -> Option<NFTMetadata> {
        env.storage().instance().get(&TokenKey::TokenMetadata(*token_id))
    }

    
}