/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccessKey {
    /// A description of the access key.
    #[serde(rename = "description")]
    pub description: String,
    /// The permissions granted to an access key.
    #[serde(rename = "permission")]
    pub permission: String,
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<String>>,
}

impl AccessKey {
    pub fn new(description: String, permission: String) -> AccessKey {
        AccessKey {
            description,
            permission,
            buckets: None,
        }
    }
}


