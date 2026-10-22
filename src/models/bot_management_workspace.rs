/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementWorkspace : A container for bot management settings and policy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementWorkspace {
    /// The display name of the workspace.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A description of the workspace.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "protection_mode", skip_serializing_if = "Option::is_none")]
    pub protection_mode: Option<crate::models::BotManagementProtectionMode>,
}

impl BotManagementWorkspace {
    /// A container for bot management settings and policy.
    pub fn new() -> BotManagementWorkspace {
        BotManagementWorkspace {
            name: None,
            description: None,
            protection_mode: None,
        }
    }
}


