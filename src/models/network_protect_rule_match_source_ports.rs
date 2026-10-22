/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleMatchSourcePorts : Requires a source-port match field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleMatchSourcePorts {
    #[serde(rename = "source_ports")]
    pub source_ports: Box<crate::models::NetworkProtectRuleSourcePorts>,
}

impl NetworkProtectRuleMatchSourcePorts {
    /// Requires a source-port match field.
    pub fn new(source_ports: crate::models::NetworkProtectRuleSourcePorts) -> NetworkProtectRuleMatchSourcePorts {
        NetworkProtectRuleMatchSourcePorts {
            source_ports: Box::new(source_ports),
        }
    }
}


