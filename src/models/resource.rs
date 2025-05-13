/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resource {
    /// The ID of the underlying linked resource.
    #[serde(rename = "resource_id", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The name of the resource link. Note this is separate from the resource store name and might not match the store name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            resource_id: None,
            name: None,
        }
    }
}


