/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingHttpsAllOf {
    /// The URL to send logs to. Must use HTTPS. Required.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum number of logs sent in one request. Defaults `0` (10k).
    #[serde(rename = "request_max_entries", skip_serializing_if = "Option::is_none")]
    pub request_max_entries: Option<i32>,
    /// The maximum number of bytes sent in one request. Defaults `0` (100MB).
    #[serde(rename = "request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub request_max_bytes: Option<i32>,
    /// Content type of the header sent with the request.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Name of the custom header sent with the request.
    #[serde(rename = "header_name", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<crate::models::LoggingMessageType>,
    /// Value of the custom header sent with the request.
    #[serde(rename = "header_value", skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
    /// HTTP method used for request.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /// Enforces valid JSON formatting for log entries.
    #[serde(rename = "json_format", skip_serializing_if = "Option::is_none")]
    pub json_format: Option<JsonFormat>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl LoggingHttpsAllOf {
    pub fn new() -> LoggingHttpsAllOf {
        LoggingHttpsAllOf {
            url: None,
            request_max_entries: None,
            request_max_bytes: None,
            content_type: None,
            header_name: None,
            message_type: None,
            header_value: None,
            method: None,
            json_format: None,
            format: None,
        }
    }
}

/// HTTP method used for request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
}

impl Default for Method {
    fn default() -> Method {
        Self::POST
    }
}
/// Enforces valid JSON formatting for log entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JsonFormat {
    #[serde(rename = "0")]
    Disabled,
    #[serde(rename = "1")]
    JsonArray,
    #[serde(rename = "2")]
    NewlineDelimitedJson,
}

impl Default for JsonFormat {
    fn default() -> JsonFormat {
        Self::Disabled
    }
}

