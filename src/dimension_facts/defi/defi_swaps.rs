//! # DeFi Swaps
//!
//! POST https://rest-api.hellomoon.io/v0/defi/swaps
//!
//! DeFi Swaps endpoint can be used to track and verify both large and small
//! swap amounts that occur on the Solana blockchain. Hello Moon also provides key data fields such as the time of the swap, who made the swap, and the program and aggregator that was used.
//!
use std::default;

use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, is_zero, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct DefiSwapsRequest {
    #[serde(rename = "userAccount")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub user_account: String,
    #[serde(rename = "sourceMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source_mint: String,
    #[serde(rename = "destinationMint")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub destination_mint: String,
    #[serde(rename = "aggregatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_name: Option<AggregatorName>,
    #[serde(rename = "programId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub program_id: String,
    #[serde(rename = "sourceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<SourceAmount>,
    #[serde(rename = "destinationAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_amount: Option<DestinationAmount>,
    #[serde(rename = "blockId")]
    #[serde(skip_serializing_if = "is_zero")]
    pub block_id: usize,
    #[serde(rename = "blockTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<BlockTimeType>,
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    #[serde(skip_serializing_if = "page_is_zero")]
    page: usize,
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum AggregatorName {
    #[serde(rename = "Jupiter v2")]
    JupiterV2,
    #[serde(rename = "Jupiter v3")]
    JupiterV3,
    #[default]
    #[serde(rename = "Jupiter v4")]
    JupiterV4,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SourceAmount {
    Option1 {
        operator: String,
        /// The amount of the source mint sent for the swap.
        /// The amount is in the source mint's native unit, which is differentiated by it's decimal value.
        /// For example, the amount of 1,000,000 for the source mint of USDC would be 1 USDC since its decimal value is 6.
        value: usize,
    },
    /// The amount of the source mint sent for the swap.
    /// The amount is in the source mint's native unit, which is differentiated by it's decimal value.
    /// For example, the amount of 1,000,000 for the source mint of USDC would be 1 USDC since its decimal value is 6.
    Option2(usize),
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DestinationAmount {
    Option1 {
        operator: String,
        /// The amount of destination mints received from the swap.
        /// The amount is in the source mint's native unit, which is differentiated by it's decimal value.
        /// For example, the amount of 1,000,000 for the source mint of USDC would be 1 USDC since its decimal value is 6.
        value: usize,
    },
    /// The amount of destination mints received from the swap.
    /// The amount is in the source mint's native unit, which is differentiated by it's decimal value.
    /// For example, the amount of 1,000,000 for the source mint of USDC would be 1 USDC since its decimal value is 6.
    Option2(usize),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum BlockTimeType {
    Comparator {
        value: usize,
        operator: Operator,
    },
    Range {
        #[serde(rename = "greaterThan")]
        greater_than: usize,
        #[serde(rename = "LessThan")]
        less_than: usize,
        operator: Operator,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Equal,
    NoEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Between,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DefiSwapsResponse {
    /// array of objects
    data: Option<Vec<IResponse>>,
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {}

pub async fn defi_swaps(
    request: Option<DefiSwapsRequest>,
    api_key: &str,
) -> anyhow::Result<DefiSwapsResponse> {
    let api = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/swaps");
    core_call::<DefiSwapsRequest, DefiSwapsResponse>(request, api, api_key).await
}

pub async fn defi_swaps_return_json_value(
    request: Option<DefiSwapsRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let api = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/swaps");
    core_call::<DefiSwapsRequest, serde_json::Value>(request, api, api_key).await
}

#[tokio::test]
async fn test_defi_swaps() {
    let request = DefiSwapsRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = defi_swaps(Some(request), &api_key).await.unwrap();

    println!("defi swaps: {:?}", left)
}

#[tokio::test]
async fn test_defi_swaps_return_json_value() {
    let request = DefiSwapsRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = defi_swaps_return_json_value(Some(request), &api_key)
        .await
        .unwrap();

    println!("defi swaps: {}", crate::pretty_json(left))
}
