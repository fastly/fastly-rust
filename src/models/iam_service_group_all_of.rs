/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IamServiceGroupAllOf {
    /// Alphanumeric string identifying the service group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the object.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Name of the service group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the service group.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Number of services in the service group.
    #[serde(rename = "services_count", skip_serializing_if = "Option::is_none")]
    pub services_count: Option<i32>,
}

impl IamServiceGroupAllOf {
    pub fn new() -> IamServiceGroupAllOf {
        IamServiceGroupAllOf {
            id: None,
            object: None,
            name: None,
            description: None,
            services_count: None,
        }
    }
}


