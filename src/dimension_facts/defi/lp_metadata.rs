//! LP Metadata
//! POST
//! https://rest-api.hellomoon.io/v0/defi/liquidity-pools/metadata
//! Metadata on Liquidity Pools such as pool name and token names
//! 
use serde::{Deserialize, Serialize};

use crate::{core_call, limit_is_zero, page_is_zero};

const API_URL: &str = "https://rest-api.hellomoon.io/v0/defi/liquidity-pools/metadata";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Request {
    #[serde(rename = "poolAddress")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pool_address: String,
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "String::is_empty")]
    program_name: String,
    #[serde(rename = "poolName")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pool_name: String,
    #[serde(rename = "mintTokenA")]
    #[serde(skip_serializing_if = "String::is_empty")]
    mint_token_a: String,
    #[serde(rename = "mintTokenB")]
    #[serde(skip_serializing_if = "String::is_empty")]
    mint_token_b: String,
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
    #[serde(rename = "programName")]
    program_name: Option<String>,
    #[serde(rename = "poolAddress")]
    pool_address: Option<String>,
    #[serde(rename = "poolName")]
    pool_name: Option<String>,
    #[serde(rename = "mintTokenA")]
    mint_token_a: Option<String>,
    #[serde(rename = "nameTokenA")]
    name_token_a: Option<String>,
    #[serde(rename = "mintTokenB")]
    mint_token_b: Option<String>,
    #[serde(rename = "nameTokenB")]
    name_token_b: Option<String>,
    #[serde(rename = "tokenAccountA")]
    token_account_a: Option<String>,
    #[serde(rename = "tokenAccountB")]
    token_account_b: Option<String>,
}
pub async fn example(request: Option<Request>, api_key: &str) -> anyhow::Result<Response> {
    core_call::<Request, Response>(request, API_URL, api_key).await
}

#[tokio::test]
async fn test_lp_metadata_example() {
    let request = Request::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = example(Some(request), &api_key).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: Response = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
