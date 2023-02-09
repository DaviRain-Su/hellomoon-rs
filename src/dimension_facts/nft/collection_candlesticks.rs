//! # Collection Candlesticks
//!
//! POST: `https://rest-api.hellomoon.io/v0/collection/listing/candlesticks`
//!
//! The Collection Candlesticks endpoint allows you to choose the floor price period with
//! the provided granularities of `ONE_MIN`, `FIVE_MIN`, `ONE_HOUR`, `ONE_DAY`, `ONE_WEEK`.
//!
//! A candle stick consists of four main components: the open, high, low, close and volume -
//! the total number of transactions occured within the candlestick.
//!
//! The open is the price at which the period opened
//! The high is the highest price reached during the period
//! The low is the lowest price reached during the period
//! The close is the price at which the period closed
//!
use serde::{Deserialize, Serialize};

use crate::{core_call, limit_is_zero, page_is_zero};

const COLLECTION_CANDLESTICKS_API: &str =
    "https://rest-api.hellomoon.io/v0/collection/listing/candlesticks";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionCandlesticksResponse {
    /// array of objects
    data: Option<Vec<CollectionCandlesticks>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionCandlesticks {
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    pub hello_moon_collection_id: Option<String>,
    /// The time granularity (period) of the candlestick: `ONE_MIN`, `FIVE_MIN`, `ONE_HOUR`, `ONE_DAY`, `ONE_WEEK`.
    /// For example, the ONE_MIN granularity will return a candlestick for every minute in the time period - as long as there is volume.
    ///
    /// `ONE_MIN` `FIVE_MIN` `ONE_HOUR` `ONE_DAY` `ONE_WEEK`
    pub granularity: Option<String>,
    /// Numeric identifier of a block describing the slot that the block was produced in
    pub lastblockid: Option<usize>,
    /// Epoch start time of time period in seconds
    #[serde(rename = "startTime")]
    pub start_time: Option<usize>,
    /// The high price of a candlestick is the highest price reached during the time period.
    pub high: Option<String>,
    /// The low price of a candlestick is the lowest price reached during the time period.
    pub low: Option<String>,
    /// The opening price of a candlestick is the price at which the period opened.
    /// It is usually represented by the top of the candlestick body, and is the starting point for the period's price action.
    pub open: Option<String>,
    /// The closing price of a candlestick is the price at which the period closed.
    /// It is usually represented by the bottom of the candlestick body, and is the ending point for the period's price action.
    pub close: Option<String>,
    /// The volume of a candlestick is the total number of coins or tokens traded during the period.
    /// It is usually represented by the size of the candlestick body, with larger bodies indicating higher volumes.
    /// Volume is an important indicator of market activity and can be used to confirm price movements and identify potential reversal points.
    pub volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollectionCandlesticksRequest {
    /// To find the correct helloMoonCollectionId, click here and search a collection name.
    /// This list is continuously updated.
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "helloMoonCollectionId")]
    pub hello_moon_collection_id: String,
    /// Epoch start time of time period in seconds
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "startTime")]
    pub start_time: String, // todo startTime can improve
    /// The time granularity (period) of the candlestick: ONE_MIN, FIVE_MIN, ONE_HOUR, ONE_DAY, ONE_WEEK.
    /// For example, the ONE_MIN granularity will return a candlestick for every minute in the time period
    /// - as long as there is volume.
    // todo startTime can improve
    #[serde(skip_serializing_if = "granularity_is_empty")]
    pub granularity: Option<Granularity>,
    /// The number of results to return per page
    #[serde(skip_serializing_if = "limit_is_zero")]
    pub limit: usize,
    /// The page number to return
    #[serde(skip_serializing_if = "page_is_zero")]
    pub page: usize,
    /// The pagination token to use to keep your position in the results
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "paginationToken")]
    pub pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Granularity {
    #[serde(rename = "ONE_MIN")]
    OneMin,
    #[serde(rename = "FIVE_MIN")]
    FiveMin,
    #[serde(rename = "ONE_HOUR")]
    OneHour,
    #[serde(rename = "ONE_DAY")]
    OneDay,
    #[serde(rename = "ONE_WEEK")]
    OneWeek,
}
impl Default for Granularity {
    fn default() -> Self {
        Granularity::OneMin
    }
}

fn granularity_is_empty(value: &Option<Granularity>) -> bool {
    if let Some(v) = value {
        match serde_json::to_string(&v) {
            Ok(e) => e.is_empty(),
            Err(_) => true,
        }
    } else {
        true
    }
}

/// The Collection Candlesticks endpoint allows you to choose the floor price period with
/// the provided granularities of ONE_MIN, FIVE_MIN, ONE_HOUR, ONE_DAY, ONE_WEEK.
pub async fn collection_candlesticks(
    api_key: &str,
    request: Option<CollectionCandlesticksRequest>,
) -> anyhow::Result<CollectionCandlesticksResponse> {
    core_call::<CollectionCandlesticksRequest, CollectionCandlesticksResponse>(
        request,
        COLLECTION_CANDLESTICKS_API,
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_collection_candlesticks() {
    let mut request = CollectionCandlesticksRequest::default();
    request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();
    request.limit = 1;

    let api_key = dotenv::var("api_keys").unwrap();

    let left = collection_candlesticks(&api_key, Some(request))
        .await
        .unwrap();
    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: CollectionCandlesticksResponse = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
