//! # Metaplex Metadata
//! POST `https://rest-api.hellomoon.io/v0/nft/mint_information`
//! On-chain NFT Mint Information from the Metaplex Token Standard

use super::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const METAPLEX_METADATA_API_URL: &str = "https://rest-api.hellomoon.io/v0/nft/mint_information";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MetaplexMetadataResponse {
    /// array of objects
    data: Vec<MetaplexMetadata>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MetaplexMetadata {
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    nft_mint: String,
    /// Public key of address holding NFT metadata
    #[serde(rename = "nftMetadataAdress")]
    nft_metadata_adress: String,
    /// The NFT on chain metadata
    #[serde(rename = "nftMetadataJson")]
    nft_metadata_json: NftMetadataJson,
    /// The public key of the Collection NFT's Mint Account
    #[serde(rename = "nftCollectionMint")]
    nft_collection_mint: Option<String>,
    /// Array of verified creators
    #[serde(rename = "nftVerifiedCreatorsArray")]
    nft_verified_creators_array: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NftMetadataJson {
    /// The on-chain name of the token, limited to 32 bytes
    name: String,
    /// The on-chain symbol of the token, limited to 10 bytes
    symbol: String,
    /// The URI of the token, limited to 200 bytes. This URI points to an off-chain JSON file that contains additional data following a certain standard.
    uri: String,
    /// The royalties shared by the creators in basis points — i.e. 550 means 5.5%. Whilst this field is used by virtually all NFT marketplaces, it is not enforced by the Token Metadata program itself.
    #[serde(rename = "sellerFeeBasisPoints")]
    seller_fee_basis_points: usize,
    /// An array of creators and their share of the royalties. This array is limited to 5 creators.
    #[serde(skip_serializing_if = "Option::is_none")] // TODO, this I meet error.
    creators: Option<Vec<Creator>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Creator {
    /// The publicKey of the creator
    address: String,
    /// A boolean indicating if the creator signed the NFT. It is important to check this field to ensure the authenticity of the creator.
    verified: bool,
    /// The share of the royalties that the creator gets. This is a number between 0 and 100. The sum of all shares must be 100.
    share: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MetaplexMetadataRequest {
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
    nft_mint: String,
    /// The public key of the Collection NFT's Mint Account
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "nftCollectionMint")]
    nft_collection_mint: String,
    /// The number of results to return per page
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    /// The page number to return
    #[serde(skip_serializing_if = "page_is_zero")]
    page: usize,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

pub async fn metaplex_metadata(
    api_key: &str,
    request: Option<MetaplexMetadataRequest>,
) -> anyhow::Result<MetaplexMetadataResponse> {
    core_call::<MetaplexMetadataRequest, MetaplexMetadataResponse>(request, METAPLEX_METADATA_API_URL, api_key)
        .await
        .map_err(|_| anyhow::anyhow!("helloMoonCollectionId or nftMint must be provided in body"))
}

#[tokio::test]
async fn test_metaplex_metadata() {
    let request = MetaplexMetadataRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = metaplex_metadata(&api_key, Some(request)).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: MetaplexMetadataResponse = serde_json::from_str(&r).unwrap();
    // println!("{:#?}", right);
    assert_eq!(left, right);
}
