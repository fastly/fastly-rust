/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleMatchDestinationIp : Requires a destination address match field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleMatchDestinationIp {
    /// Match traffic sent to this address; omit for any destination. An address outside a Network using this Rule Set never matches and still consumes one of the 64 rules. The API does not enforce containment in a network prefix; attached Rule Sets require a compatible IP family.
    #[serde(rename = "destination_ip_address")]
    pub destination_ip_address: String,
}

impl NetworkProtectRuleMatchDestinationIp {
    /// Requires a destination address match field.
    pub fn new(destination_ip_address: String) -> NetworkProtectRuleMatchDestinationIp {
        NetworkProtectRuleMatchDestinationIp {
            destination_ip_address,
        }
    }
}


