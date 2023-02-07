//! # Collection Mint Mapping
//!
//! POST `https://rest-api.hellomoon.io/v0/nft/collection/mints`
//!
//! The Collection Mint Mapping endpoint maps a unique Hello Moon helloMoonCollectionId to a list of on-chain mint addresses.
//!
//! helloMoonCollectionId or nftMint is required to receive a successful query response.
//!
//!
use serde::{Deserialize, Serialize};

const COLLECTION_MINT_MAPPING_API: &str = "https://rest-api.hellomoon.io/v0/nft/collection/mints";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CMintMappingResponse {
    data: Vec<CMintMapping>,
    pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CMintMapping {
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: String,
    #[serde(rename = "nftMint")]
    nft_mint: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CMintMappingRequest {}

pub async fn collection_mint_mapping(
    api_key: &str,
    request: Option<CMintMappingRequest>,
) -> anyhow::Result<CMintMappingResponse> {
    todo!()
}
