//! # Secondary Sales
//!
//! POST `https://rest-api.hellomoon.io/v0/nft/sales/secondary`
//!
//! The NFT Secondary Sales endpoint captures transactions that occured after the minting of a NFT.
//!
//! For example, the Primary Sales endpoint visualizes who minted the NFT and the program that was used. While the Secondary Sales endpoint visualizes who the minter sold the NFT to and the price it was transacted at.
//!
//! Data goes back 30 days for supported launchpads and mint programs.
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const SECONDARY_SALES_API_URL: &str = "https://rest-api.hellomoon.io/v0/nft/sales/secondary";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SecondarySalesResponse {
    /// array of objects
    data: Option<Vec<SecondarySales>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SecondarySales {
    /// Numeric identifier of a block describing the slot that the block was produced in
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    /// A list of marketplaces and their data supported by HelloMoon
    ///
    /// SMB ME_V1 ME_V2 YAWWW Elixir SolSea OpenSea Solanart Hadeswap CoralCube Coral Cube Exchange.Art
    marketplace: Option<String>,
    ///Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    block_time: Option<String>,
    /// The mint address of the NFT that was sold to the buyer by the seller.
    #[serde(rename = "nftMint")]
    nft_mint: Option<String>,
    /// PublicKey of the seller of the NFT
    seller: Option<String>,
    /// PublicKey of the buyer of the NFT
    buyer: Option<String>,
    /// The price of the NFT that was sold to the buyer.
    /// price is denoted in Solana
    price: Option<String>,
    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SecondarySalesRequest {
    /// To find the correct helloMoonCollectionId,
    /// click here and search a collection name.
    /// This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    hello_moon_collection_id: String,
    /// The mint address of the NFT that was sold to the buyer by the seller.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    nft_mint: String,
    /// PublicKey of the buyer of the NFT
    #[serde(skip_serializing_if = "String::is_empty")]
    buyer: String,
    /// PublicKey of the seller of the NFT
    #[serde(skip_serializing_if = "String::is_empty")]
    seller: String,
    /// A list of marketplaces and their data supported by HelloMoon
    #[serde(skip_serializing_if = "marketplace_is_empty")]
    marketplace: Option<Marketplace>,
    /// The price of the NFT that was sold to the buyer.
    /// price is denoted in Solana
    #[serde(skip_serializing_if = "String::is_empty")]
    price: String,
    /// Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    #[serde(skip_serializing_if = "String::is_empty")]
    block_time: String,
    /// Numeric identifier of a block describing the slot that the block was produced in
    #[serde(rename = "blockId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    block_id: String,
    /// The number of results to return per page
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: isize,
    /// The page number to return
    #[serde(skip_serializing_if = "page_is_zero")]
    page: isize,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Marketplace {
    #[serde(rename = "SMB")]
    Smb,
    #[serde(rename = "ME_V1")]
    Mev1,
    #[serde(rename = "ME_V2")]
    Mev2,
    #[serde(rename = "YAWWW")]
    Yawww,
    #[serde(rename = "ELIXIR")]
    Elixir,
    #[serde(rename = "SOLSEA")]
    Solsea,
    #[serde(rename = "OPENSEA")]
    Opensea,
    #[serde(rename = "SOLANART")]
    Solanart,
    #[serde(rename = "HADESWAP")]
    Hadeswap,
    #[serde(rename = "CORALCUBE")]
    Coralcube,
    #[serde(rename = "CORAL_CUBE")]
    CoralCube,
    #[serde(rename = "Exchange.art")]
    ExchangeArt,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self::Elixir
    }
}

fn marketplace_is_empty(value: &Option<Marketplace>) -> bool {
    if let Some(v) = value {
        match serde_json::to_string(&v) {
            Ok(e) => e.is_empty(),
            Err(_) => true,
        }
    } else {
        true
    }
}

pub async fn scondary_sales(
    api_key: &str,
    request: Option<SecondarySalesRequest>,
) -> anyhow::Result<SecondarySalesResponse> {
    core_call::<SecondarySalesRequest, SecondarySalesResponse>(
        request,
        SECONDARY_SALES_API_URL,
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_scondary_sales() {
    let request = SecondarySalesRequest::default();
    // request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = scondary_sales(&api_key, Some(request)).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: SecondarySalesResponse = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
