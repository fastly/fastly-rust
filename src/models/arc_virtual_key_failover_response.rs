/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcVirtualKeyFailoverResponse : A virtual key's failover policy. `targets` are tried in order after the primary request fails and one of the enabled error conditions is met. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcVirtualKeyFailoverResponse {
    /// The provider connections to retry against, in order.
    #[serde(rename = "targets")]
    pub targets: Vec<crate::models::ArcFailoverTarget>,
    /// Whether a request timeout triggers failover.
    #[serde(rename = "timeout_enabled")]
    pub timeout_enabled: bool,
    /// Whether a `429 Too Many Requests` response from the provider triggers failover.
    #[serde(rename = "rate_limit_429_enabled")]
    pub rate_limit_429_enabled: bool,
    /// Whether a `5xx` server error response from the provider triggers failover.
    #[serde(rename = "server_error_5xx_enabled")]
    pub server_error_5xx_enabled: bool,
}

impl ArcVirtualKeyFailoverResponse {
    /// A virtual key's failover policy. `targets` are tried in order after the primary request fails and one of the enabled error conditions is met. 
    pub fn new(targets: Vec<crate::models::ArcFailoverTarget>, timeout_enabled: bool, rate_limit_429_enabled: bool, server_error_5xx_enabled: bool) -> ArcVirtualKeyFailoverResponse {
        ArcVirtualKeyFailoverResponse {
            targets,
            timeout_enabled,
            rate_limit_429_enabled,
            server_error_5xx_enabled,
        }
    }
}


