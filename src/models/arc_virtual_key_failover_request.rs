/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcVirtualKeyFailoverRequest : A failover policy for a virtual key. `targets` are tried in order after the primary request fails and one of the enabled error conditions is met. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcVirtualKeyFailoverRequest {
    /// The provider connections to retry against, in order.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::ArcFailoverTarget>,
    /// Whether a request timeout triggers failover.
    #[serde(rename = "timeout_enabled", skip_serializing_if = "Option::is_none")]
    pub timeout_enabled: Option<bool>,
    /// Whether a `429 Too Many Requests` response from the provider triggers failover.
    #[serde(rename = "rate_limit_429_enabled", skip_serializing_if = "Option::is_none")]
    pub rate_limit_429_enabled: Option<bool>,
    /// Whether a `5xx` server error response from the provider triggers failover.
    #[serde(rename = "server_error_5xx_enabled", skip_serializing_if = "Option::is_none")]
    pub server_error_5xx_enabled: Option<bool>,
}

impl ArcVirtualKeyFailoverRequest {
    /// A failover policy for a virtual key. `targets` are tried in order after the primary request fails and one of the enabled error conditions is met. 
    pub fn new(targets: Vec<crate::models::ArcFailoverTarget>) -> ArcVirtualKeyFailoverRequest {
        ArcVirtualKeyFailoverRequest {
            targets,
            timeout_enabled: None,
            rate_limit_429_enabled: None,
            server_error_5xx_enabled: None,
        }
    }
}


