//! # Collection Mint Mapping
//!
//! POST `https://rest-api.hellomoon.io/v0/nft/collection/mints`
//!
//! The Collection Mint Mapping endpoint maps a unique Hello Moon helloMoonCollectionId to a list of on-chain mint addresses.
//!
//! helloMoonCollectionId or nftMint is required to receive a successful query response.
//!
//!
use super::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const COLLECTION_MINT_MAPPING_API: &str = "https://rest-api.hellomoon.io/v0/nft/collection/mints";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionMintMappingResponse {
    /// array of objects
    data: Option<Vec<CollectionMintMapping>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CollectionMintMapping {
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: Option<String>,
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    nft_mint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CollectionMintMappingRequest {
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    hello_moon_collection_id: String,
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
    nft_mint: String,
    /// The number of results to return per page
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    #[serde(skip_serializing_if = "page_is_zero")]
    /// The page number to return
    page: usize,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

pub async fn collection_mint_mapping(
    api_key: &str,
    request: Option<CollectionMintMappingRequest>,
) -> anyhow::Result<CollectionMintMappingResponse> {
    core_call::<CollectionMintMappingRequest, CollectionMintMappingResponse>(
        request,
        COLLECTION_MINT_MAPPING_API,
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_collection_mint_mapping() {
    let mut request = CollectionMintMappingRequest::default();
    request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = collection_mint_mapping(&api_key, Some(request))
        .await
        .unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: CollectionMintMappingResponse = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
