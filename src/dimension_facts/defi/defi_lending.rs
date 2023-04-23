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
    /// The pagination token to use to keep your position in the results
    /// optional field
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IResponse {
    /// Select the action types: borrow or repay.
    ///  >If borrow, the endpoint returns the amount of tokens the user account borrowed from the program.
    ///  >If repay, the endpoint returns the amount of tokens that are repaid to the user.
    /// borrow repay
    #[serde(rename = "actionType")]
    action_type: Option<String>,
    /// The amount of tokens that the user account is repaying or borrowing.
    /// The amount is in the source mint's native unit, which is differentiated by it's decimal value.
    /// For example, the amount of 1,000,000 for the source mint of USDC would be 1 USDC since its decimal value is 6.
    amount: Option<usize>,
    /// Unix epoch time (in seconds) of a block as calculated from validator votes.
    /// If you want to look at historical data, let's say 7 days in the past.
    /// 1. Change the operator to <
    /// 2. Get the current epochtime i.e, 1673831466 -> Jan 15, 2023
    /// 3. Subtract the current epochtime from ( 86400 * 7 ). Place the result of 1673831466 - ( 86400 * 7 ) = 1673226666 in the value input - this returns the epochtime time from 7 days ago
    #[serde(rename = "blockTime")]
    block_time: Option<usize>,
    /// HelloMoon unique identifier for specific instruction within a transaction
    #[serde(rename = "instructionId")]
    instruction_id: Option<String>,
    /// Name of this instruction
    /// optional field
    #[serde(rename = "instructionName")]
    instruction_name: Option<String>,
    /// The zero-indexed position of an instruction - subinstruction combination in the context of the transaction. This is generated by flattening all instruction/subinstruction/sub-subinstruction/... and numbering them from 0.
    #[serde(rename = "instructionOrdinal")]
    instruction_ordinal: Option<usize>,
    /// Zero-indexed position of the instruction within the context of a transaction
    #[serde(rename = "instructionPosition")]
    instruction_position: Option<usize>,
    /// The mint references the token that the user account is repaid or borrowing from the program.
    /// optional field
    mint: Option<String>,
    /// The programId references the lending protocol
    /// that the user account is borrowing or repaying to.
    /// optional field
    #[serde(rename = "programId")]
    program_id: Option<String>,
    /// The programName references the lending protocol name
    /// that the user account is borrowing or repaying to.
    /// optional field
    #[serde(rename = "programName")]
    program_name: Option<String>,
    /// Zero-indexed position of sub-instruction within the context of a main instruction
    #[serde(rename = "subInstructionPosition")]
    sub_instruction_position: Option<isize>,
    /// First signature in a transaction, which can be used to track and verify the transaction status across the complete ledger.
    /// It is a base-58 encoded string that is uniquely generated for each transaction.
    /// optional field
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    /// The user account is the public key of the wallet that
    /// is borrowing or repaying from the program.
    /// optional field
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
