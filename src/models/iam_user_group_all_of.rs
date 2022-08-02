/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IamUserGroupAllOf {
    /// Alphanumeric string identifying the user group.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the user group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the user group.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Number of invitations added to the user group.
    #[serde(rename = "invitations_count", skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i32>,
    /// Number of users added to the user group.
    #[serde(rename = "users_count", skip_serializing_if = "Option::is_none")]
    pub users_count: Option<i32>,
    /// Number of roles added to the user group.
    #[serde(rename = "roles_count", skip_serializing_if = "Option::is_none")]
    pub roles_count: Option<i32>,
}

impl IamUserGroupAllOf {
    pub fn new() -> IamUserGroupAllOf {
        IamUserGroupAllOf {
            id: None,
            name: None,
            description: None,
            invitations_count: None,
            users_count: None,
            roles_count: None,
        }
    }
}


