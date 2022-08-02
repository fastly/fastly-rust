/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IamPermission {
    /// Alphanumeric string identifying the permission.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the object.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// Name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the permission.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the resource the operation will be performed on.
    #[serde(rename = "resource_name", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// The description of the resource.
    #[serde(rename = "resource_description", skip_serializing_if = "Option::is_none")]
    pub resource_description: Option<String>,
    /// Permissions are either \"service\" level or \"account\" level.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl IamPermission {
    pub fn new() -> IamPermission {
        IamPermission {
            id: None,
            object: None,
            name: None,
            description: None,
            resource_name: None,
            resource_description: None,
            scope: None,
        }
    }
}


