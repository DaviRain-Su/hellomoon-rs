use super::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const API_URL: &str = "";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Response {
    // /// array of objects
    // data: Vec<CNameMapping>,
    // /// The pagination token to use to keep your position in the results
    // #[serde(rename = "paginationToken")]
    // pagination_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ExampleApi {
    // /// The name of the collection
    // #[serde(rename = "collectionName")]
    // collection_name: String,
    // /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    // #[serde(rename = "helloMoonCollectionId")]
    // hello_moon_collection_id: String,
    // /// Current volume of the collection in SOL (lamports) looking back 24 hours
    // #[serde(rename = "currentVolumeSOL")]
    // current_volume_sol: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Request {
    // /// To find the correct helloMoonCollectionId,
    // /// click here and search a collection name.
    // /// This list is continuously updated.
    // #[serde(rename = "helloMoonCollectionId")]
    // #[serde(skip_serializing_if = "String::is_empty")]
    // hello_moon_collection_id: String,
    // /// The name of the collection
    // #[serde(rename = "collectionName")]
    // #[serde(skip_serializing_if = "String::is_empty")]
    // collection_name: String,
    // /// The number of results to return per page
    // #[serde(skip_serializing_if = "limit_is_zero")]
    // limit: usize,
    // /// The page number to return
    // #[serde(skip_serializing_if = "page_is_zero")]
    // page: usize,
    // /// The pagination token to use to keep your position in the results
    // #[serde(rename = "paginationToken")]
    // #[serde(skip_serializing_if = "String::is_empty")]
    // pagination_token: String,
}

pub async fn nft_listings(
    api_key: &str,
    request: Option<Request>,
) -> anyhow::Result<Response> {
    core_call::<Request, Response>(
        request,
        API_URL,
        api_key,
    )
    .await
    .map_err(|_| anyhow::anyhow!("helloMoonCollectionId or nftMint must be provided in body"))
}

#[tokio::test]
async fn test_metaplex_metadata() {
    let mut request = Request::default();
    // request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = nft_listings(&api_key, Some(request))
        .await
        .unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: Response = serde_json::from_str(&r).unwrap();
    assert_eq!(left, right);
}

