/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectNetwork : Shared Network Protect network attributes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectNetwork {
    /// Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
    /// The CIDR block to protect, masked to its network address. It cannot be changed after creation.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// The backend-assigned rule set applied to this network.
    #[serde(rename = "rule_set_id", skip_serializing_if = "Option::is_none")]
    pub rule_set_id: Option<String>,
    /// Complete endpoint policy configuration. A network has at most one policy per region and may have no policies.
    #[serde(rename = "endpoint_policies")]
    pub endpoint_policies: Vec<crate::models::NetworkProtectEndpointPolicy>,
    /// Desired advertisement state, which is not the same as observed route state. Enabling it requires approved LOA, valid ROA, and a ready default-region delivery policy. A later ROA failure does not turn an existing advertisement off; losing LOA approval or the last ready default endpoint does.
    #[serde(rename = "advertise")]
    pub advertise: bool,
}

impl NetworkProtectNetwork {
    /// Shared Network Protect network attributes.
    pub fn new(name: String, prefix: String, endpoint_policies: Vec<crate::models::NetworkProtectEndpointPolicy>, advertise: bool) -> NetworkProtectNetwork {
        NetworkProtectNetwork {
            description: None,
            name,
            prefix,
            rule_set_id: None,
            endpoint_policies,
            advertise,
        }
    }
}


