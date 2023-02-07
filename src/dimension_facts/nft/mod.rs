use serde::{de::DeserializeOwned, Serialize};

pub mod collection_candlesticks;
pub mod collection_mint_mapping;
pub mod collection_name_mapping;
pub mod listing_status;
pub mod metaplex_metadata;
pub mod mints_by_owner;
pub mod nft_listings;
pub mod primary_sales;
pub mod secondary_sales;

pub async fn core_call<Request: Serialize, Response: DeserializeOwned>(
    request: Option<Request>,
    api_url: &str,
    api_key: &str,
) -> anyhow::Result<Response> {
    let response = if let Some(request) = request {
        let body = serde_json::to_string_pretty(&request)?;
        reqwest::Client::new()
            .post(api_url)
            .header("accept", "application/json")
            .header("authorization", format!("Bearer {}", api_key))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await?
            .json::<Response>()
            .await?
    } else {
        reqwest::Client::new()
            .post(api_url)
            .header("accept", "application/json")
            .header("authorization", format!("Bearer {}", api_key))
            .header("content-type", "application/json")
            .send()
            .await?
            .json::<Response>()
            .await?
    };
    Ok(response)
}
