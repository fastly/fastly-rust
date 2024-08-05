/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dashboard {
    /// Dashboard identifier (UUID)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A human-readable name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A short description of the dashboard
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of [dashboard items](#dashboard-item).
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::DashboardItem>>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::DashboardPropertyCreatedBy>>,
    #[serde(rename = "updated_by", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Box<crate::models::DashboardPropertyUpdatedBy>>,
}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard {
            id: None,
            name: None,
            description: None,
            items: None,
            created_at: None,
            updated_at: None,
            created_by: None,
            updated_by: None,
        }
    }
}


