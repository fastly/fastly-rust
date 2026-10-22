/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectEndpointUpdate : Attributes accepted when updating an endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectEndpointUpdate {
    /// A human-readable name, unique within the customer.
    #[serde(rename = "name")]
    pub name: String,
}

impl NetworkProtectEndpointUpdate {
    /// Attributes accepted when updating an endpoint.
    pub fn new(name: String) -> NetworkProtectEndpointUpdate {
        NetworkProtectEndpointUpdate {
            name,
        }
    }
}


