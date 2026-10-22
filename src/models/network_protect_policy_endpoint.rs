/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectPolicyEndpoint : One endpoint attached to a network policy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectPolicyEndpoint {
    /// A unique identifier for a Network Protect resource.
    #[serde(rename = "endpoint_id")]
    pub endpoint_id: String,
    /// Delivery preference within a policy. Lower values are preferred first. Two endpoints in the same policy cannot share a value.
    #[serde(rename = "preference")]
    pub preference: i32,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NetworkProtectPolicyEndpoint {
    /// One endpoint attached to a network policy.
    pub fn new(endpoint_id: String, preference: i32) -> NetworkProtectPolicyEndpoint {
        NetworkProtectPolicyEndpoint {
            endpoint_id,
            preference,
            created_at: None,
            updated_at: None,
        }
    }
}


