use soroban_sdk::{contracttype, Address, String, Symbol, symbol_short};

pub const COUNTER_KEY: Symbol = symbol_short!("COUNTER");
pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_urls: String,
    pub level: u32,
    pub owner: Address
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    TokenOwner(u32),
    TokenMetadata(u32),
    TokenUri(u32),
    UserBalance(Address),
}
