//! # Lending
//!
//! POST `https://rest-api.hellomoon.io/v0/defi/lending`
//!
//! The DeFi Lending endpoint can be used to track and verify borrowing and repayment of digital assets including SPL tokens and stablecoins on the Solana blockchain.
//!
//! For example, get activity on popular lending protocols like Solend, so you know where and what token to lend.

use serde::{Deserialize, Serialize};

use crate::{core_call, limit_is_zero, page_is_zero};

const API_URL: &str = "";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Request {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {}

pub async fn example(request: Option<Request>, api_key: &str) -> anyhow::Result<Response> {
    core_call::<Request, Response>(request, API_URL, api_key).await
}

#[tokio::test]
async fn test_example() {
    let request = Request::default();

    let api_key = dotenv::var("api_keys").unwrap();

    let left = example(Some(request), &api_key).await.unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: Response = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
