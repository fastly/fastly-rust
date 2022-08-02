/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IamRoleAllOf {
    /// Alphanumeric string identifying the role.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the object.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Name of the role.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the role.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This attribute is set to `true` if the role is managed by the customer. It is set to `false` if the role was created by Fastly.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<bool>,
    /// Number of permissions assigned to the role.
    #[serde(rename = "permissions_count", skip_serializing_if = "Option::is_none")]
    pub permissions_count: Option<i32>,
}

impl IamRoleAllOf {
    pub fn new() -> IamRoleAllOf {
        IamRoleAllOf {
            id: None,
            object: None,
            name: None,
            description: None,
            custom: None,
            permissions_count: None,
        }
    }
}


