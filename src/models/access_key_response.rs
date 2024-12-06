/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessKeyResponse {
    /// Generated access key.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Generated secret key.
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// Description for the access key.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Permissions granted to an access key.
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<String>>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl AccessKeyResponse {
    pub fn new() -> AccessKeyResponse {
        AccessKeyResponse {
            access_key: None,
            secret_key: None,
            description: None,
            permission: None,
            buckets: None,
            created_at: None,
        }
    }
}


