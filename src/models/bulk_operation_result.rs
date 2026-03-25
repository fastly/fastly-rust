/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkOperationResult {
    /// The operation ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP status code for this operation.
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    /// Error reason if the operation failed.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl BulkOperationResult {
    pub fn new() -> BulkOperationResult {
        BulkOperationResult {
            id: None,
            status_code: None,
            reason: None,
        }
    }
}


