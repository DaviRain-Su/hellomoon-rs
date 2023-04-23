//! # LP Withdrawal/Deposit
//!
//! POST https://rest-api.hellomoon.io/v0/defi/liquidity-pools/withdrawals-deposits
//!
//! The LP Withdrawal/Deposit endpoint provides the pair of token data that were deposited or withdrawn from a liquidity pool.
//!
//! Alongside the token data, the endpoint also provides the user account, amount of tokens deposited or withdrawn, and the program id that was used to execute the transaction.
//!
use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct LpWithdrawalDepositRequest {
    /// The program id references the lending program that the user account is removing or adding tokens from
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "programId")]
    program_id: String,
    /// The user account is the account that is removing or adding tokens.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "userAccount")]
    user_account: String,
    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "transactionId")]
    transaction_id: String,
    /// Name of this instruction
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "instructionName")]
    instruction_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionType")]
    action_type: Option<ActionType>,
    /// Token mint A is the token address of the first token provided to the liquidity pool.
    /// The USD value of the deposit always needs to be split between the two tokens to ensure equal value on each side.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "tokenMintA")]
    token_mint_a: String,
    /// Token mint B is the token address of the second token provided to the liquidity pool.
    /// The USD value of the deposit always needs to be split between the two tokens to ensure equal value on each side.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "tokenMintB")]
    token_mint_b: String,
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

/// Select the action types: addLiquidity or removeLiquidity
/// If addLiquidity, the endpoint returns the amount of tokens that were added to the liquidity pool.
/// If removeLiquidity, the endpoint returns the amount of tokens that were removed from the liquidity pool.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    #[serde(rename = "addLiquidity")]
    AddLiquidity,
    #[serde(rename = "removeLiquidity")]
    RemoveLiquidity,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LpWithdrawalDepositResponse {
    /// array of objects
    data: Option<Vec<IResponse>>,
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {}

pub async fn lp_withdrawal_deposit(
    request: Option<LpWithdrawalDepositRequest>,
    api_key: &str,
) -> anyhow::Result<LpWithdrawalDepositResponse> {
    let api_url = format!(
        "{}{}",
        HELLOMOON_ROOT_URL, "/defi/liquidity-pools/withdrawals-deposits"
    );
    core_call::<LpWithdrawalDepositRequest, LpWithdrawalDepositResponse>(request, api_url, api_key)
        .await
}

pub async fn lp_withdrawal_deposit_returen_json_value(
    request: Option<LpWithdrawalDepositRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    let api_url = format!(
        "{}{}",
        HELLOMOON_ROOT_URL, "/defi/liquidity-pools/withdrawals-deposits"
    );
    core_call::<LpWithdrawalDepositRequest, serde_json::Value>(request, api_url, api_key).await
}

#[tokio::test]
async fn test_lp_withdrawal_deposit() {
    let request = LpWithdrawalDepositRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = lp_withdrawal_deposit_returen_json_value(Some(request), &api_key)
        .await
        .unwrap();

    println!("{:#?}", left);
}
