/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenCreateRequestAttributes {
    /// name of the token
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// List of service ids to limit the token
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// A UTC time-stamp of when the token will expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Indicates whether TLS access is enabled for the token.
    #[serde(rename = "tls_access", skip_serializing_if = "Option::is_none")]
    pub tls_access: Option<bool>,
}

impl AutomationTokenCreateRequestAttributes {
    pub fn new() -> AutomationTokenCreateRequestAttributes {
        AutomationTokenCreateRequestAttributes {
            name: None,
            role: None,
            services: None,
            scope: None,
            expires_at: None,
            tls_access: None,
        }
    }
}

/// 
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

