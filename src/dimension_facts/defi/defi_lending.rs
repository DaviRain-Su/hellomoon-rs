//! # Lending
//!
//! POST `https://rest-api.hellomoon.io/v0/defi/lending`
//!
//! The DeFi Lending endpoint can be used to track and verify borrowing and repayment of digital assets including SPL tokens and stablecoins on the Solana blockchain.
//!
//! For example, get activity on popular lending protocols like Solend, so you know where and what token to lend.

use crate::HELLOMOON_ROOT_URL;
use crate::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

// const DEFI_LENGIN_URL: String = format!("{}{}", HELLOMOON_ROOT_URL, "/defi/lending");

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct DefiLendingRequest {
    /// The programId references the lending protocol
    /// that the user account is borrowing or repaying to.
    /// optional field
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "programId")]
    program_id: String,
    /// The programName references the lending protocol name
    /// that the user account is borrowing or repaying to.
    /// optional field
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "programName")]
    program_name: String,
    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    /// optional field
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "transactionId")]
    transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionType")]
    action_type: Option<ActionType>,
    /// The user account is the public key of the wallet that
    /// is borrowing or repaying from the program.
    /// optional field
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "userAccount")]
    user_account: String,
    /// The mint references the token that the user account is repaid or borrowing from the program.
    /// optional field
    #[serde(skip_serializing_if = "String::is_empty")]
    mint: String,
    /// The number of results to return per page
    /// optional field
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    /// The page number to return
    /// optional field
    #[serde(skip_serializing_if = "page_is_zero")]
    page: usize,
    /// The pagination token to use to keep your position in the results
    /// optional field
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

/// Select the action types: borrow or repay.
///  >If borrow, the endpoint returns the amount of tokens the user account borrowed from the program.
///  >If repay, the endpoint returns the amount of tokens that are repaid to the user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    #[serde(rename = "borrow")]
    Borrow,
    #[serde(rename = "repay")]
    Repay,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Borrow
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DefiLendingResponse {
    /// array of objects
    data: Option<Vec<IResponse>>,
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    #[serde(rename = "actionType")]
    action_type: Option<String>,
    amount: Option<usize>,
    #[serde(rename = "blockTime")]
    block_time: Option<usize>,
    #[serde(rename = "instructionId")]
    instruction_id: Option<String>,
    #[serde(rename = "instructionName")]
    instruction_name: Option<String>,
    #[serde(rename = "instructionOrdinal")]
    instruction_ordinal: Option<usize>,
    #[serde(rename = "instructionPosition")]
    instruction_position: Option<usize>,
    mint: Option<String>,
    #[serde(rename = "programId")]
    program_id: Option<String>,
    #[serde(rename = "programName")]
    program_name: Option<String>,
    #[serde(rename = "subInstructionPosition")]
    sub_instruction_position: Option<isize>,
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    #[serde(rename = "userAccount")]
    user_account: Option<String>,
}

pub async fn defi_lending(
    request: Option<DefiLendingRequest>,
    api_key: &str,
) -> anyhow::Result<DefiLendingResponse> {
    core_call::<DefiLendingRequest, DefiLendingResponse>(
        request,
        format!("{}{}", HELLOMOON_ROOT_URL, "/defi/lending"),
        api_key,
    )
    .await
}

pub async fn defi_lending_return_json_value(
    request: Option<DefiLendingRequest>,
    api_key: &str,
) -> anyhow::Result<serde_json::Value> {
    core_call::<DefiLendingRequest, serde_json::Value>(
        request,
        format!("{}{}", HELLOMOON_ROOT_URL, "/defi/lending"),
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_defi_lending() {
    let request = DefiLendingRequest::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = defi_lending(Some(request), &api_key).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: DefiLendingResponse = serde_json::from_str(&r).unwrap();
    // println!("{:#?}", left);
    assert_eq!(left, right);
}
