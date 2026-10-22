/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectEndpoint : Shared Network Protect endpoint attributes.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectEndpoint {
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
    /// The GRE destination address, unique within the customer. It cannot be changed after endpoint creation.
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl NetworkProtectEndpoint {
    /// Shared Network Protect endpoint attributes.
    pub fn new(name: String, ip_address: String) -> NetworkProtectEndpoint {
        NetworkProtectEndpoint {
            name,
            ip_address,
        }
    }
}


