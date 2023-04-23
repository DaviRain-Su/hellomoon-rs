//! # Lending
//!
//! POST `https://rest-api.hellomoon.io/v0/defi/lending`
//!
//! The DeFi Lending endpoint can be used to track and verify borrowing and repayment of digital assets including SPL tokens and stablecoins on the Solana blockchain.
//!
//! For example, get activity on popular lending protocols like Solend, so you know where and what token to lend.

use serde::{Deserialize, Serialize};
use crate::{core_call, limit_is_zero, page_is_zero};

const API_URL: &str = "https://rest-api.hellomoon.io/v0/defi/lending";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Request {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "programId")]
    program_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "programName")]
    program_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "transactionId")]
    transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionType")]
    action_type: Option<ActionType>,
    #[serde(skip_serializing_if = "String::is_empty")]
    user_account: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    mint: String,
    #[serde(skip_serializing_if = "limit_is_zero")]
    limit: usize,
    #[serde(skip_serializing_if = "page_is_zero")]
    page: usize,
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "String::is_empty")]
    pagination_token: String,
}

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
    #[serde(rename = "programId")]
    program_id: Option<String>,
    #[serde(rename = "actionType")]
    action_type: Option<String>,
    #[serde(rename = "userAccount")]
    user_account: Option<String>,
    mint: Option<String>,
    amount: Option<String>,
    #[serde(rename = "transactionId")]
    transaction_id: Option<String>,
    #[serde(rename = "blockTime")]
    block_time: Option<String>,
    #[serde(rename = "instructionName")]
    instruction_name: Option<String>,
    #[serde(rename = "instructionId")]
    instruction_id: Option<String>,
    #[serde(rename = "instructionPosition")]
    instruction_position: Option<usize>,
    #[serde(rename = "subInstructionPosition")]
    sub_instruction_position: Option<isize>,
    #[serde(rename = "instructionOrdinal")]
    instruction_ordinal: Option<usize>,
}

pub async fn example(request: Option<Request>, api_key: &str) -> anyhow::Result<Response> {
    core_call::<Request, Response>(request, API_URL, api_key).await
}

#[tokio::test]
async fn test_lending_example() {
    let request = Request::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = example(Some(request), &api_key).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: Response = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
