//! # Collection Name Mapping
//! 
//! POST `https://rest-api.hellomoon.io/v0/nft/collection/name`
//! 
//! The Collection Name Mapping endpoint maps a unique Hello Moon helloMoonCollectionId to an unique NFT Collection Name.
//!
//! helloMoonCollectionId or collectionName is required to receive a successful query response.
//!
//!
use super::{core_call, limit_is_zero, page_is_zero};
use serde::{Deserialize, Serialize};

const COLLECTION_NAME_MAPPING_API: &str = "https://rest-api.hellomoon.io/v0/nft/collection/name";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CollectionNameMappingResponse {
    /// array of objects
    data: Option<Vec<CollectionNameMapping>>,
    /// The pagination token to use to keep your position in the results
    #[serde(rename = "paginationToken")]
    pagination_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CollectionNameMapping {
    /// The name of the collection
    #[serde(rename = "collectionName")]
    collection_name: Option<String>,
    /// To find the correct helloMoonCollectionId, click here and search a collection name. This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    hello_moon_collection_id: Option<String>,
    /// Current volume of the collection in SOL (lamports) looking back 24 hours
    #[serde(rename = "currentVolumeSOL")]
    current_volume_sol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CollectionNameMappingRequest {
    /// To find the correct helloMoonCollectionId,
    /// click here and search a collection name.
    /// This list is continuously updated.
    #[serde(rename = "helloMoonCollectionId")]
    #[serde(skip_serializing_if = "String::is_empty")]
    hello_moon_collection_id: String,
    /// The name of the collection
    #[serde(rename = "collectionName")]
    #[serde(skip_serializing_if = "String::is_empty")]
    collection_name: String,
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

pub async fn collection_name_mapping(
    api_key: &str,
    request: Option<CollectionNameMappingRequest>,
) -> anyhow::Result<CollectionNameMappingResponse> {
    core_call::<CollectionNameMappingRequest, CollectionNameMappingResponse>(
        request,
        COLLECTION_NAME_MAPPING_API,
        api_key,
    )
    .await
}

#[tokio::test]
async fn test_collection_name_mapping() {
    let mut request = CollectionNameMappingRequest::default();
    request.hello_moon_collection_id = "040de757c0d2b75dcee999ddd47689c4".to_string();

    let api_key = dotenv::var("api_keys").unwrap();
    let left = collection_name_mapping(&api_key, Some(request))
        .await
        .unwrap();

    let r = serde_json::to_string_pretty(&left).unwrap();
    let right: CollectionNameMappingResponse = serde_json::from_str(&r).unwrap();
    println!("{:#?}", right);
    assert_eq!(left, right);
}
