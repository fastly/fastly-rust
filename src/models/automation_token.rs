/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationToken {
    /// The name of the token.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The role on the token.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// (Optional) The service IDs of the services the token will have access to. Separate service IDs with a space. If no services are specified, the token will have access to all services on the account. 
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
    /// A space-delimited list of authorization scope.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// A UTC time-stamp of when the token expires.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl AutomationToken {
    pub fn new() -> AutomationToken {
        AutomationToken {
            name: None,
            role: None,
            services: None,
            scope: None,
            expires_at: None,
        }
    }
}

/// The role on the token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "user")]
    User,
}

impl Default for Role {
    fn default() -> Role {
        Self::Billing
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

