/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BotManagementWorkspaceResponseExtra {
    /// A UUID used to uniquely identify the workspace.
    #[serde(rename = "id")]
    pub id: String,
    /// The IDs of the services attached to this workspace.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// The date and time the workspace was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the workspace was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl BotManagementWorkspaceResponseExtra {
    pub fn new(id: String, created_at: String, updated_at: String) -> BotManagementWorkspaceResponseExtra {
        BotManagementWorkspaceResponseExtra {
            id,
            services: None,
            created_at,
            updated_at,
        }
    }
}


