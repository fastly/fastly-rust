/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RoutingConfigVersionResponse : All attributes for a routing config version response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoutingConfigVersionResponse {
    /// Alphanumeric string identifying the version.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A freeform comment describing the version.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp of when the version was most recently activated. `null` if the version has never been activated.
    #[serde(rename = "activated_at", skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,
}

impl RoutingConfigVersionResponse {
    /// All attributes for a routing config version response.
    pub fn new() -> RoutingConfigVersionResponse {
        RoutingConfigVersionResponse {
            id: None,
            comment: None,
            created_at: None,
            activated_at: None,
        }
    }
}


