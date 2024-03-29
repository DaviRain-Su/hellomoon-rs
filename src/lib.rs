#![allow(unused_imports)]

use serde::{de::DeserializeOwned, Serialize};

pub mod dimension_facts;
pub mod hello_moon_pub_api;
pub mod summary;

pub const HELLOMOON_ROOT_URL: &str = "https://rest-api.hellomoon.io/v0";

pub async fn core_call<Request: Serialize, Response: DeserializeOwned>(
    request: Option<Request>,
    api_url: String,
    api_key: &str,
) -> anyhow::Result<Response> {
    let response = if let Some(request) = request {
        let body = serde_json::to_string_pretty(&request)?;
        // todo
        log::info!("Body: [{}]", body);
        println!("Body: [{}]", body);
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

pub fn limit_is_zero(value: &usize) -> bool {
    is_zero(value)
}

pub fn page_is_zero(value: &usize) -> bool {
    is_zero(value)
}

pub fn is_zero(value: &usize) -> bool {
    *value == usize::MIN
}

pub fn pretty_json(value: serde_json::Value) -> String {
    use serde_json::json;
    format!("{:#}", json!(value))
}
