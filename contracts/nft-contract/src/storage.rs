use crate::types::{DataKey, NFTMetadata, ADMIN_KEY};
use soroban_sdk::{Address, Env};

pub struct NFTStorage;

impl NFTStorage {
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&ADMIN_KEY, admin);
    }

    pub retrieve_admin()
}