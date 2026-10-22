/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleMatchDestinationPorts : Requires a destination-port match field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleMatchDestinationPorts {
    #[serde(rename = "destination_ports")]
    pub destination_ports: Box<crate::models::NetworkProtectRuleDestinationPorts>,
}

impl NetworkProtectRuleMatchDestinationPorts {
    /// Requires a destination-port match field.
    pub fn new(destination_ports: crate::models::NetworkProtectRuleDestinationPorts) -> NetworkProtectRuleMatchDestinationPorts {
        NetworkProtectRuleMatchDestinationPorts {
            destination_ports: Box::new(destination_ports),
        }
    }
}


