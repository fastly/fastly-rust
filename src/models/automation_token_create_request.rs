/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenCreateRequest {
    /// The name of the token.
    #[serde(rename = "name")]
    pub name: String,
    /// The role for the token. The `roles` field is the preferred way to assign roles. This field is supported for backwards compatibility with single-role tokens.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Array of role IDs to assign to the token. Use this field for tokens with multiple roles. When provided, this takes precedence over the `role` field. To get available role IDs, use [GET /roles](https://www.fastly.com/documentation/reference/api/account/roles/).
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// List of service IDs the token can access. Only used when `limit_services` is `true`.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// When `true`, restricts the token to only the services specified in the `services` array. When `false` (default), the token has access to all services.
    #[serde(rename = "limit_services", skip_serializing_if = "Option::is_none")]
    pub limit_services: Option<bool>,
    /// When `true` (default), restricts the token to only the workspaces specified in the `workspaces` array. When `false`, the token has access to all workspaces. Only applies to roles that support workspace access (such as Next-Gen WAF roles). For roles without workspace support, this field has no effect.
    #[serde(rename = "limit_workspaces", skip_serializing_if = "Option::is_none")]
    pub limit_workspaces: Option<bool>,
    /// List of workspace IDs the token can access. Only used when `limit_workspaces` is `true` and the token has roles that support workspace access.
    #[serde(rename = "workspaces", skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<String>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// A UTC timestamp of when the token will expire.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    /// Indicates whether TLS access is enabled for the token. Only applicable when using the single-role `role` field. For tokens with multiple roles, assign TLS roles (`tls_admin` or `tls_viewer`) via the `roles` field instead.
    #[serde(rename = "tls_access", skip_serializing_if = "Option::is_none")]
    pub tls_access: Option<bool>,
}

impl AutomationTokenCreateRequest {
    pub fn new(name: String, expires_at: Option<String>) -> AutomationTokenCreateRequest {
        AutomationTokenCreateRequest {
            name,
            role: None,
            roles: None,
            services: None,
            limit_services: None,
            limit_workspaces: None,
            workspaces: None,
            scope: None,
            expires_at,
            tls_access: None,
        }
    }
}

/// The role for the token. The `roles` field is the preferred way to assign roles. This field is supported for backwards compatibility with single-role tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Engineer
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "global:read")]
    Globalread,
    #[serde(rename = "purge_all")]
    PurgeAll,
    #[serde(rename = "purge_select")]
    PurgeSelect,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Global
    }
}

