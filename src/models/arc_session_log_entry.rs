/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcSessionLogEntry : A single request and response pair recorded within a session.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcSessionLogEntry {
    /// The raw request data.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    /// The raw response data.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// Additional attributes recorded for this request/response pair.
    #[serde(rename = "attrs", skip_serializing_if = "Option::is_none")]
    pub attrs: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ArcSessionLogEntry {
    /// A single request and response pair recorded within a session.
    pub fn new() -> ArcSessionLogEntry {
        ArcSessionLogEntry {
            request: None,
            response: None,
            attrs: None,
        }
    }
}


