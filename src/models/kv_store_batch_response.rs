/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvStoreBatchResponse {
    /// A descriptor for the response of the entire batch
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// If an error is present in any of the requests, this field will describe that error
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Errors which occurred while handling the items in the request
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::KvStoreBatchResponseErrors>>,
}

impl KvStoreBatchResponse {
    pub fn new() -> KvStoreBatchResponse {
        KvStoreBatchResponse {
            title: None,
            _type: None,
            errors: None,
        }
    }
}


