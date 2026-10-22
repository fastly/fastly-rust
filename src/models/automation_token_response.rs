/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenResponse {
    /// The name of the token.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// All roles assigned to the token. Each role includes an `id` and `name`.
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::AutomationTokenRole>>,
    /// The service IDs of the services the token has access to. Empty when `limit_services` is `false` (full access) or when the token has no service access.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// When `true`, the token is restricted to only the services in the `services` array. When `false`, the token has access to all services.
    #[serde(rename = "limit_services", skip_serializing_if = "Option::is_none")]
    pub limit_services: Option<bool>,
    /// When `true`, the token is restricted to only the workspaces in the `workspaces` array. When `false`, the token has access to all workspaces. Only applies to roles that support workspace access.
    #[serde(rename = "limit_workspaces", skip_serializing_if = "Option::is_none")]
    pub limit_workspaces: Option<bool>,
    /// The workspace IDs of the workspaces the token has access to. Returns an empty array for tokens without workspace access.
    #[serde(rename = "workspaces", skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<String>>,
    /// A space-delimited list of authorization scope.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// (optional) A UTC timestamp of when the token will expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
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
    /// Indicates whether TLS access is enabled for the token.
    #[serde(rename = "tls_access", skip_serializing_if = "Option::is_none")]
    pub tls_access: Option<bool>,
    /// A UTC timestamp of when the token was last used.
    #[serde(rename = "last_used_at", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    /// A UTC timestamp of when the token was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl AutomationTokenResponse {
    pub fn new() -> AutomationTokenResponse {
        AutomationTokenResponse {
            name: None,
            role: None,
            roles: None,
            services: None,
            limit_services: None,
            limit_workspaces: None,
            workspaces: None,
            scope: None,
            expires_at: None,
            id: None,
            customer_id: None,
            ip: None,
            user_agent: None,
            tls_access: None,
            last_used_at: None,
            created_at: None,
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

