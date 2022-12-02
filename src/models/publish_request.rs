/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// PublishRequest : Contains a batch of messages to publish.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublishRequest {
    /// The messages to publish.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::PublishItem>,
}

impl PublishRequest {
    /// Contains a batch of messages to publish.
    pub fn new(items: Vec<crate::models::PublishItem>) -> PublishRequest {
        PublishRequest {
            items,
        }
    }
}


