/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceCreate {
    /// The name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the linked resource.
    #[serde(rename = "resource_id", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl ResourceCreate {
    pub fn new() -> ResourceCreate {
        ResourceCreate {
            name: None,
            resource_id: None,
        }
    }
}


