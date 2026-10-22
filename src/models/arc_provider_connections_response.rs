/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcProviderConnectionsResponse {
    /// The provider connections returned by the request.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::ArcProviderConnectionListItemResponse>,
    #[serde(rename = "meta")]
    pub meta: Box<crate::models::ArcProviderConnectionsMeta>,
}

impl ArcProviderConnectionsResponse {
    pub fn new(data: Vec<crate::models::ArcProviderConnectionListItemResponse>, meta: crate::models::ArcProviderConnectionsMeta) -> ArcProviderConnectionsResponse {
        ArcProviderConnectionsResponse {
            data,
            meta: Box::new(meta),
        }
    }
}


