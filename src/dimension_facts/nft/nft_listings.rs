//! # NFT Listings
//!
//! POST `https://rest-api.hellomoon.io/v0/nft/listings`
//!
//! The NFT Listings endpoint provides different degrees of perspective NFT datasets. Viewing NFTs from a specific market or collection, down to a single unique NFT mint.
//!
//! Hello Moon also delivers the nft listing actions - ask, cancel_ask, put_for_sale, and sale_cancel
//! So you know what is really happening.
//!
//! Data goes back 30 days from all support markets.
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const NFT_LISTING_API_URL: &str = "https://rest-api.hellomoon.io/v0/nft/listings";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NftListingsResponse {
    /// array of objects
    data: Option<Vec<NftListings>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NftListings {
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: Option<String>,
    /// nft listings based off their instruction's action: ask, cancel_ask, put_for_sale, sale_cancel, sale
    /// ask cancel_ask put_for_sale sale_cancel sale
    #[serde(rename = "instructionName")]
    instruction_name: Option<String>,
    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(rename = "nftMint")]
    nft_mint: Option<String>,
    /// A list of marketplaces and their data supported by HelloMoon
    /// MEv1 Solanart SMB MEv2 Yawww
    market: Option<String>,
    /// NFT listing price in lamports, will be zero if the nft is delisted cancel_ask - when converted the price is a float with 6 decimal places of precision.
    price: Option<String>,
    /// Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    block_time: Option<String>,
    /// Numeric identifier of a block describing the slot that the block was produced in
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    /// Zero-indexed position of the transaction within the block
    #[serde(rename = "transactionPosition")]
    transaction_position: Option<usize>,
    /// The zero-indexed position of an instruction - subinstruction combination in the context of the transaction. This is generated by flattening all instruction/subinstruction/sub-subinstruction/... and numbering them from 0.
    #[serde(rename = "instructionOrdinal")]
    instruction_ordinal: Option<usize>,
    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NftListingsRequest {
    /// To find the correct helloMoonCollectionId,
    /// click here and search a collection name.
    /// This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    hello_moon_collection_id: String,

    #[serde(rename = "instructionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    instruction_name: Option<InstructionName>,

    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    transaction_id: String,

    /// Numeric identifier of a block describing the slot that the block was produced in
    #[serde(rename = "blockId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<usize>,

    /// Mint address of nft per the spl token program.
    /// Each NFT has a unique mint address within the collection.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "nftMint")]
    nft_mint: String,

    /// A list of marketplaces and their data supported by HelloMoon
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<Market>,

    /// Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    block_time: Option<usize>,

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

//ask, cancel_ask, put_for_sale, and sale_cancel
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InstructionName {
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "cancel_ask")]
    CancleAsk,
    #[serde(rename = "put_for_sale")]
    PutForSale,
    #[serde(rename = "sale_cancel")]
    SaleCancel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Market {
    #[serde(rename = "MEv1")]
    Mev1,
    #[serde(rename = "MEv2")]
    Mev2,
    Solanart,
    #[serde(rename = "SMB")]
    Smb,
    Yawww,
}
pub async fn nft_listings(
    api_key: &str,
    request: Option<NftListingsRequest>,
) -> anyhow::Result<NftListingsResponse> {
    core_call::<NftListingsRequest, NftListingsResponse>(
        request,
        NFT_LISTING_API_URL.to_string(),
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_nft_listings() {
    let request = NftListingsRequest::default();
    // request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = nft_listings(&api_key, Some(request)).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: NftListingsResponse = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
