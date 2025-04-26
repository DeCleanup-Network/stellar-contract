use soroban_sdk::{Address, String};
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub level: u32,
    pub owner: Address,
}
