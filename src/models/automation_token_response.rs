/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenResponse {
    /// The name of the token.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// (Optional) The service IDs of the services the token will have access to. Separate service IDs with a space. If no services are specified, the token will have access to all services on the account. 
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// A space-delimited list of authorization scope.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// (optional) A UTC time-stamp of when the token will expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// A UTC time-stamp of when the token was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<crate::models::ReadOnlyId>>,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<crate::models::ReadOnlyCustomerId>>,
    /// The IP address of the client that last used the token.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The User-Agent header of the client that last used the token.
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "sudo_expires_at", skip_serializing_if = "Option::is_none")]
    pub sudo_expires_at: Option<String>,
    /// A UTC time-stamp of when the token was last used.
    #[serde(rename = "last_used_at", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
}

impl AutomationTokenResponse {
    pub fn new() -> AutomationTokenResponse {
        AutomationTokenResponse {
            name: None,
            role: None,
            services: None,
            scope: None,
            expires_at: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            id: None,
            customer_id: None,
            ip: None,
            user_agent: None,
            sudo_expires_at: None,
            last_used_at: None,
        }
    }
}

/// A space-delimited list of authorization scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "purge_select")]
    PurgeSelect,
    #[serde(rename = "purge_all")]
    PurgeAll,
    #[serde(rename = "global:read")]
    Globalread,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Global
    }
}

