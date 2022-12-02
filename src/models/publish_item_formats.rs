/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// PublishItemFormats : Transport-specific message payload representations to be used for delivery. At least one format (`http-response`, `http-stream`, and/or `ws-message`) must be specified. Messages are only delivered to subscribers interested in the provided formats. For example, the `ws-message` format will only be sent to WebSocket clients.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublishItemFormats {
    #[serde(rename = "http-response", skip_serializing_if = "Option::is_none")]
    pub http_response: Option<Box<crate::models::HttpResponseFormat>>,
    #[serde(rename = "http-stream", skip_serializing_if = "Option::is_none")]
    pub http_stream: Option<Box<crate::models::HttpStreamFormat>>,
    #[serde(rename = "ws-message", skip_serializing_if = "Option::is_none")]
    pub ws_message: Option<Box<crate::models::WsMessageFormat>>,
}

impl PublishItemFormats {
    /// Transport-specific message payload representations to be used for delivery. At least one format (`http-response`, `http-stream`, and/or `ws-message`) must be specified. Messages are only delivered to subscribers interested in the provided formats. For example, the `ws-message` format will only be sent to WebSocket clients.
    pub fn new() -> PublishItemFormats {
        PublishItemFormats {
            http_response: None,
            http_stream: None,
            ws_message: None,
        }
    }
}


