use crate::structs::nft_metadata::NFTMetadata;
use soroban_sdk::{Address, Env};
pub trait NFTInterface {
    /// Mint a new NFT with unique metadata
    fn mint(env: Env, to: Address, metadata: NFTMetadata);

    /// Transfer an NFT from one address to another
    fn transfer(env: Env, from: Address, to: Address, nft_id: u32);

    /// Burn an NFT
    fn burn(env: Env, from: Address, nft_id: u32);

    /// Get the metadata for an NFT
    fn get_metadata(env: Env, nft_id: u32) -> NFTMetadata;
}
