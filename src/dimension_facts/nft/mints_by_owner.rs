//！# Mints By Owner
//!
//！POST `https://rest-api.hellomoon.io/v0/nft/mints-by-owner`
//!
//！All NFT Mints owned by a wallet with on chain attached
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const MINTS_BY_OWNER_API_URL: &str = "https://rest-api.hellomoon.io/v0/nft/mints-by-owner";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MintsByOwnerResponse {
    /// array of objects
    data: Option<Vec<MintsByOwner>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MintsByOwner {
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    nft_mint: Option<String>,
    /// The token account of the NFT per the SPL token program
    #[serde(rename = "tokenAccount")]
    token_account: Option<String>,
    /// The owner account of the NFT per the SPL token program
    #[serde(rename = "ownerAccount")]
    owner_account: Option<String>,
    /// Public key of address holding NFT metadata
    #[serde(rename = "metadataAddress")]
    metadata_address: Option<String>,
    /// The NFT on chain metadata
    #[serde(rename = "metadataJson")]
    metadata_json: Option<MetadataJson>,
    /// The public key of the Collection NFT's Mint Account
    #[serde(rename = "nftCollectionMint")]
    nft_collection_mint: Option<String>,
    /// Array of verified creators
    #[serde(rename = "verifiedCreators")]
    verified_creators: Option<Vec<String>>,
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MetadataJson {
    /// The on-chain name of the token, limited to 32 bytes
    name: Option<String>,
    /// The on-chain symbol of the token, limited to 10 bytes
    symbol: Option<String>,
    /// The URI of the token, limited to 200 bytes. This URI points to an off-chain JSON file that contains additional data following a certain standard.
    uri: Option<String>,
    /// The royalties shared by the creators in basis points — i.e. 550 means 5.5%. Whilst this field is used by virtually all NFT marketplaces, it is not enforced by the Token Metadata program itself.
    #[serde(rename = "sellerFeeBasisPoints")]
    seller_fee_basis_points: Option<usize>,
    /// An array of creators and their share of the royalties. This array is limited to 5 creators.
    creators: Option<Vec<Creator>>,
    /// This field optionally links to the Mint address of another NFT that acts as a Collection NFT.
    collection: Option<Collection>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Creator {
    /// The publicKey of the creator
    address: Option<String>,
    /// A boolean indicating if the creator signed the NFT. It is important to check this field to ensure the authenticity of the creator.
    verified: Option<bool>,
    /// The share of the royalties that the creator gets. This is a number between 0 and 100. The sum of all shares must be 100.
    share: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Collection {
    /// A boolean indicating if the owner of the Collection NFT signed this NFT. It is important to check this field to ensure the authenticity of the collection.
    verified: Option<bool>,
    /// The public key of the Collection NFT's Mint Account
    key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MintsByOwnerRequest {
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
    nft_mint: String,
    /// To find the correct helloMoonCollectionId,
    /// click here and search a collection name.
    /// This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    hello_moon_collection_id: String,

    /// The owner account of the NFT per the SPL token program
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "String::is_empty")]
    owner_account: String,

    /// The public key of the Collection NFT's Mint Account
    #[serde(rename = "nftCollectionMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
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

pub async fn mints_by_owners(
    api_key: &str,
    request: Option<MintsByOwnerRequest>,
) -> anyhow::Result<MintsByOwnerResponse> {
    core_call::<MintsByOwnerRequest, MintsByOwnerResponse>(
        request,
        MINTS_BY_OWNER_API_URL.to_string(),
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_mints_by_owners() {
    let mut request = MintsByOwnerRequest::default();
    request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = mints_by_owners(&api_key, Some(request)).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: MintsByOwnerResponse = serde_json::from_str(&r).unwrap();
    assert_eq!(left, right);
}
