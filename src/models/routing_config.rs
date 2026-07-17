/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RoutingConfig : All attributes for creating a routing config.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// The user-defined name for the routing config.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "initial_version", skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<Box<crate::models::InitialVersion>>,
}

impl RoutingConfig {
    /// All attributes for creating a routing config.
    pub fn new(name: String) -> RoutingConfig {
        RoutingConfig {
            name,
            initial_version: None,
        }
    }
}


