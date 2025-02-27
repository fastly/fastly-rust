/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvStoreBatchResponseErrors {
    /// The key that the error corresponds to. This field will be empty if the object or one of its fields was not parseable.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The line number of the batch request body on which the error occurred (starting from 0 for the first line).
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The HTTP response code for the item, or a 400 if the item's operation was not completed.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A descriptor of this particular item's error.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl KvStoreBatchResponseErrors {
    pub fn new() -> KvStoreBatchResponseErrors {
        KvStoreBatchResponseErrors {
            key: None,
            index: None,
            code: None,
            reason: None,
        }
    }
}


