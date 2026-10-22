/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectNetworkCreate : Attributes accepted when creating a network.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectNetworkCreate {
    /// Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
    /// The CIDR block to protect, masked to its network address. It cannot be changed after creation.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// Custom rule-set attachment is unavailable. Omit this value or send an empty string to use the backend-assigned default rule set.
    #[serde(rename = "rule_set_id", skip_serializing_if = "Option::is_none")]
    pub rule_set_id: Option<String>,
    /// Complete endpoint policy configuration. A network has at most one policy per region and may have no policies.
    #[serde(rename = "endpoint_policies", skip_serializing_if = "Option::is_none")]
    pub endpoint_policies: Option<Vec<crate::models::NetworkProtectEndpointPolicy>>,
}

impl NetworkProtectNetworkCreate {
    /// Attributes accepted when creating a network.
    pub fn new(name: String, prefix: String) -> NetworkProtectNetworkCreate {
        NetworkProtectNetworkCreate {
            description: None,
            name,
            prefix,
            rule_set_id: None,
            endpoint_policies: None,
        }
    }
}


