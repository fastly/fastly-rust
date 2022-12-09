/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// PublishItem : An individual message.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublishItem {
    /// The channel to publish the message on.
    #[serde(rename = "channel")]
    pub channel: String,
    /// The ID of the message.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the previous message published on the same channel.
    #[serde(rename = "prev-id", skip_serializing_if = "Option::is_none")]
    pub prev_id: Option<String>,
    #[serde(rename = "formats")]
    pub formats: Box<crate::models::PublishItemFormats>,
}

impl PublishItem {
    /// An individual message.
    pub fn new(channel: String, formats: crate::models::PublishItemFormats) -> PublishItem {
        PublishItem {
            channel,
            id: None,
            prev_id: None,
            formats: Box::new(formats),
        }
    }
}


