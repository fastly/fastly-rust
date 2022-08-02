/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// RateLimiterResponse1 : Custom response to be sent when the rate limit is exceeded. Required if `action` is `response`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RateLimiterResponse1 {
    /// HTTP status code for custom limit enforcement response.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// MIME type for custom limit enforcement response.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Response body for custom limit enforcement response.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl RateLimiterResponse1 {
    /// Custom response to be sent when the rate limit is exceeded. Required if `action` is `response`.
    pub fn new() -> RateLimiterResponse1 {
        RateLimiterResponse1 {
            status: None,
            content_type: None,
            content: None,
        }
    }
}


