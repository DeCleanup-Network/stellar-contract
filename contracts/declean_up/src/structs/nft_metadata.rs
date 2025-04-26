
use soroban_sdk::{String, Address};
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub level: u32,
    pub owner: Address,
}
