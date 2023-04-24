//! # LP Metadata
//!
//! POST https://rest-api.hellomoon.io/v0/defi/liquidity-pools/metadata
//!
//! Metadata on Liquidity Pools such as pool name and token names
//!
use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

// const API_URL: &str = "https://rest-api.hellomoon.io/v0/defi/liquidity-pools/metadata";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct LpMetadataRequest {
    /// Public key of address holding information about the pool.
    /// > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pool_address: String,
    /// Program name
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub program_name: String,
    /// Token pair of the LP
    #[serde(rename = "poolName")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pool_name: String,
    /// Mint address of the first token in the LP pair per the SPL token program
    #[serde(rename = "mintTokenA")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mint_token_a: String,
    /// Mint address of the second token in the LP pair per the SPL token program
    #[serde(rename = "mintTokenB")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mint_token_b: String,
    /// The number of results to return per page
    #[serde(skip_serializing_if = "limit_is_zero")]
    pub limit: usize,
    /// The page number to return
    #[serde(skip_serializing_if = "page_is_zero")]
    pub page: usize,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LpMetadataResponse {
    /// array of objects
    pub data: Option<Vec<IResponse>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    /// Program name
    #[serde(rename = "programName")]
    pub program_name: Option<String>,
    /// Public key of address holding information about the pool.
    /// > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    pub pool_address: Option<String>,
    /// Token pair of the LP
    #[serde(rename = "poolName")]
    pub pool_name: Option<String>,
    /// Mint address of the first token in the LP pair per the SPL token program
    #[serde(rename = "mintTokenA")]
    pub mint_token_a: Option<String>,
    /// Name of the first token in the LP pair per our spl_token_list metadata
    #[serde(rename = "nameTokenA")]
    pub name_token_a: Option<String>,
    /// Mint address of the second token in the LP pair per the SPL token program
    #[serde(rename = "mintTokenB")]
    pub mint_token_b: Option<String>,
    /// Name of the second token in the LP pair per our spl_token_list metadata
    #[serde(rename = "nameTokenB")]
    pub name_token_b: Option<String>,
    /// Token account of a mint per the SPL token program that holds the first token in the LP pair
    #[serde(rename = "tokenAccountA")]
    pub token_account_a: Option<String>,
    ///Token account of a mint per the SPL token program that holds the second token in the LP pair
    #[serde(rename = "tokenAccountB")]
    pub token_account_b: Option<String>,
}
pub async fn lp_metadata_return_json_value(
    request: Option<LpMetadataRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let api_url = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/liquidity-pools/metadata");
    core_call::<LpMetadataRequest, serde_json::Value>(request, api_url, api_key).await
}

pub async fn lp_metadata(
    request: Option<LpMetadataRequest>,
    api_key: &str,
) -> anyhow::Result<LpMetadataResponse> {
    let api_url = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/liquidity-pools/metadata");
    core_call::<LpMetadataRequest, LpMetadataResponse>(request, api_url, api_key).await
}

#[tokio::test]
async fn test_lp_metadata() {
    let request = LpMetadataRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_metadata(Some(request), &api_key).await.unwrap();

    println!("lp metaata: {:#?}", left);
}

#[tokio::test]
async fn test_lp_metadata_return_json_value() {
    let request = LpMetadataRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_metadata_return_json_value(Some(request), &api_key)
        .await
        .unwrap();

    println!("{}", crate::pretty_json(left));
}
