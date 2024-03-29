//! # LP Balances
//!
//! POST `https://rest-api.hellomoon.io/v0/defi/liquidity-pools/balances`
//!
//! Current balance of Liquidity Pools
//!
use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct LpBalanceRequest {
    /// Public key of address holding information about the pool.
    ///     > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pool_address: String,
    /// The public key (address) of the account containing the program on chain.
    ///     > You can also visit https://www.hellomoon.io/id?search=program to search for a program using a user interface.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub program: String,
    /// Name of the pool in the format {Symbol A} - {Symbol B}
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
pub struct LpBalanceResponse {
    /// array of objects
    pub data: Option<Vec<IResponse>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    /// The public key (address) of the account containing the program on chain.
    /// > You can also visit https://www.hellomoon.io/id?search=program to search for a program using a user interface.
    pub program: Option<String>,
    /// Public key of address holding information about the pool.
    /// > You can also visit https://www.hellomoon.io/id?search=lp to search for a liquidity pool or provider using a user interface.
    #[serde(rename = "poolAddress")]
    pub pool_address: Option<String>,
    /// Name of the pool in the format {Symbol A} - {Symbol B}
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
    /// Mint address of the first token in the LP pair per the SPL token program
    #[serde(rename = "tokenAccountA")]
    pub token_account_a: Option<String>,
    /// Mint address of the first token in the LP pair per the SPL token program
    #[serde(rename = "tokenAccountB")]
    pub token_account_b: Option<String>,
    /// Balance of token A in the tokens native units (unconverted for decimals)
    #[serde(rename = "balanceTokenALamports")]
    pub balance_token_a_lamports: Option<String>,
    /// Balance of token B in the tokens native units (unconverted for decimals)
    #[serde(rename = "balanceTokenBLamports")]
    pub balance_token_b_lamports: Option<String>,
    /// Balance of token A in the tokens converted for decimals
    #[serde(rename = "balanceTokenA")]
    balance_token_a: Option<f64>,
    /// Balance of token B in the tokens converted for decimals
    #[serde(rename = "balanceTokenB")]
    pub balance_token_b: Option<f64>,
}

pub async fn lp_balance(
    request: Option<LpBalanceRequest>,
    api_key: &str,
) -> anyhow::Result<LpBalanceResponse> {
    let api_url = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/liquidity-pools/balances");
    core_call::<LpBalanceRequest, LpBalanceResponse>(request, api_url, api_key).await
}

pub async fn lp_balance_return_json_vale(
    request: Option<LpBalanceRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let api_url = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/liquidity-pools/balances");
    core_call::<LpBalanceRequest, serde_json::Value>(request, api_url, api_key).await
}

#[tokio::test]
async fn test_lp_balance_return_json_vale() {
    let request = LpBalanceRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_balance_return_json_vale(Some(request), &api_key)
        .await
        .unwrap();

    println!("{}", crate::pretty_json(left));
}

#[tokio::test]
async fn test_lp_balance() {
    let request = LpBalanceRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_balance(Some(request), &api_key).await.unwrap();

    println!("lp balance: {:#?}", left);
}
