//! LP Emissions
//!
//! POST https://rest-api.hellomoon.io/v0/defi/liquidity-pools/emissions
//!
//! Current reward emissions per token and LP pool for programs
//!
use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, is_zero, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct LpEmissionsRequest {
    /// Public key of address holding information about the pool.
    /// > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pool_address: String,
    /// The spl mint of the reward being emitted by the lp
    #[serde(skip_serializing_if = "String::is_empty")]
    mint: String,
    #[serde(rename = "blockTime")]
    #[serde(skip_serializing_if = "is_zero")]
    block_time: usize,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LpEmissionsResponse {
    /// array of objects
    data: Option<Vec<IResponse>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    /// Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    block_time: Option<usize>,
    /// Numeric identifier of a block describing the slot that the block was produced in
    #[serde(rename = "blockId")]
    block_id: Option<usize>,
    /// Transaction ID of the the transaction where the reward config was set
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    /// Public key of address holding information about the pool.
    /// > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    pool_address: Option<String>,
    ///The spl mint of the reward being emitted by the lp
    mint: Option<String>,
    ///The amount of token given per day to liquidity providers of the pool in tokens native units
    #[serde(rename = "emissionsPerDay")]
    emissions_per_day: Option<f64>,
    /// The amount of token given per day to liquidity providers of the pool converted for token decimals
    #[serde(rename = "emissionsPerDayConverted")]
    emissions_per_day_converted: Option<f64>,
    /// Name of token per the Metaplex standard
    #[serde(rename = "mintName")]
    mint_aame: Option<String>,
    /// Public Key of tokenaccount rewards are paid out from
    #[serde(rename = "rewardVault")]
    reward_vault: Option<String>,
}
pub async fn lp_emission(
    request: Option<LpEmissionsRequest>,
    api_key: &str,
) -> anyhow::Result<LpEmissionsResponse> {
    let apr_url = format!(
        "{}{}",
        HELLOMOON_ROOT_URL, "/defi/liquidity-pools/emissions"
    );
    core_call::<LpEmissionsRequest, LpEmissionsResponse>(request, apr_url, api_key).await
}

pub async fn lp_emission_return_json_value(
    request: Option<LpEmissionsRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let apr_url = format!(
        "{}{}",
        HELLOMOON_ROOT_URL, "/defi/liquidity-pools/emissions"
    );
    core_call::<LpEmissionsRequest, serde_json::Value>(request, apr_url, api_key).await
}

#[tokio::test]
async fn test_lp_emissions_example() {
    let request = LpEmissionsRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_emission_return_json_value(Some(request), &api_key)
        .await
        .unwrap();

    println!("{}", crate::pretty_json(left));
}
