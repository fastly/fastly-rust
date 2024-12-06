/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogInsightsDimensions {
    /// The URL path for this dimension.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The client's country for this dimension.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The HTTP response code for this dimension.
    #[serde(rename = "status-code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// The HTTP reason phrase for this dimension.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// The client's browser for this dimension.
    #[serde(rename = "browser", skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    /// The content type of the response for this dimension.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The client's device type for this dimension.
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// The client's operating system for this dimension.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

impl LogInsightsDimensions {
    pub fn new() -> LogInsightsDimensions {
        LogInsightsDimensions {
            url: None,
            country: None,
            status_code: None,
            response: None,
            browser: None,
            content_type: None,
            device: None,
            os: None,
        }
    }
}


