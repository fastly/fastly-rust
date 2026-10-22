/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectNetworkUpdate : Attributes accepted when updating a network.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectNetworkUpdate {
    /// Optional descriptive text for the network. This metadata does not change traffic handling. Null clears it on update.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Custom rule-set attachment is unavailable. Omission, null, and an empty string preserve the current backend-assigned rule set.
    #[serde(rename = "rule_set_id", skip_serializing_if = "Option::is_none")]
    pub rule_set_id: Option<String>,
    /// Complete endpoint policy configuration. A network has at most one policy per region and may have no policies.
    #[serde(rename = "endpoint_policies", skip_serializing_if = "Option::is_none")]
    pub endpoint_policies: Option<Vec<crate::models::NetworkProtectEndpointPolicy>>,
    /// Desired advertisement state, which is not the same as observed route state. Enabling it requires approved LOA, valid ROA, and a ready default-region delivery policy. A later ROA failure does not turn an existing advertisement off; losing LOA approval or the last ready default endpoint does.
    #[serde(rename = "advertise", skip_serializing_if = "Option::is_none")]
    pub advertise: Option<bool>,
}

impl NetworkProtectNetworkUpdate {
    /// Attributes accepted when updating a network.
    pub fn new() -> NetworkProtectNetworkUpdate {
        NetworkProtectNetworkUpdate {
            description: None,
            name: None,
            rule_set_id: None,
            endpoint_policies: None,
            advertise: None,
        }
    }
}


