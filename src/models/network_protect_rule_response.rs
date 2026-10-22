/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleResponse : A rule as returned by the API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleResponse {
    #[serde(rename = "ip_protocol")]
    pub ip_protocol: crate::models::NetworkProtectRuleIpProtocol,
    /// Match traffic whose source appears in this prefix list.
    #[serde(rename = "source_prefix_list_id")]
    pub source_prefix_list_id: String,
    /// Match traffic sent to this address; omit for any destination. An address outside a Network using this Rule Set never matches and still consumes one of the 64 rules. The API does not enforce containment in a network prefix; attached Rule Sets require a compatible IP family.
    #[serde(rename = "destination_ip_address")]
    pub destination_ip_address: String,
    #[serde(rename = "source_ports")]
    pub source_ports: Box<crate::models::NetworkProtectRuleSourcePorts>,
    #[serde(rename = "destination_ports")]
    pub destination_ports: Box<crate::models::NetworkProtectRuleDestinationPorts>,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// An ISO 8601 timestamp in UTC.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl NetworkProtectRuleResponse {
    /// A rule as returned by the API.
    pub fn new(ip_protocol: crate::models::NetworkProtectRuleIpProtocol, source_prefix_list_id: String, destination_ip_address: String, source_ports: crate::models::NetworkProtectRuleSourcePorts, destination_ports: crate::models::NetworkProtectRuleDestinationPorts) -> NetworkProtectRuleResponse {
        NetworkProtectRuleResponse {
            ip_protocol,
            source_prefix_list_id,
            destination_ip_address,
            source_ports: Box::new(source_ports),
            destination_ports: Box::new(destination_ports),
            created_at: None,
            updated_at: None,
        }
    }
}


