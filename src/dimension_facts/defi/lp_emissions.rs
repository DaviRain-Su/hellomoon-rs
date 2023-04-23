//! LP Emissions
//! POST https://rest-api.hellomoon.io/v0/defi/liquidity-pools/emissions
//! Current reward emissions per token and LP pool for programs

use serde::{Deserialize, Serialize};

use crate::{core_call, is_zero, limit_is_zero, page_is_zero};

const API_URL: &str = "https://rest-api.hellomoon.io/v0/defi/liquidity-pools/emissions";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Request {
    #[serde(rename = "poolAddress")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pool_address: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mint: String,
    #[serde(rename = "blockTime")]
    #[serde(skip_serializing_if = "is_zero")]
    block_time: usize,
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    #[serde(skip_serializing_if = "page_is_zero")]
    page: usize,
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// array of objects
    data: Option<Vec<IResponse>>,
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    #[serde(rename = "blockTime")]
    block_time: Option<String>,
    #[serde(rename = "blockId")]
    block_id: Option<String>,
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    #[serde(rename = "poolAddress")]
    pool_address: Option<String>,
    mint: Option<String>,
    #[serde(rename = "emissionsPerDay")]
    emissions_per_day: Option<String>,
    #[serde(rename = "emissionsPerDayConverted")]
    emissions_per_day_converted: Option<String>,
    #[serde(rename = "mintName")]
    mint_aame: Option<String>,
    #[serde(rename = "rewardVault")]
    reward_vault: Option<String>,
}
pub async fn example(request: Option<Request>, api_key: &str) -> anyhow::Result<Response> {
    core_call::<Request, Response>(request, API_URL.to_string(), api_key).await
}

#[tokio::test]
async fn test_lp_emissions_example() {
    let request = Request::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = example(Some(request), &api_key).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: Response = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
