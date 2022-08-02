/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<Box<String>>,
    /// The real life name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Indicates that the user has limited access to the customer's services.
    #[serde(rename = "limit_services", skip_serializing_if = "Option::is_none")]
    pub limit_services: Option<bool>,
    /// Indicates whether the is account is locked for editing or not.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Indicates if a new password is required at next login.
    #[serde(rename = "require_new_password", skip_serializing_if = "Option::is_none")]
    pub require_new_password: Option<bool>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::RoleUser>,
    /// Indicates if 2FA is enabled on the user.
    #[serde(rename = "two_factor_auth_enabled", skip_serializing_if = "Option::is_none")]
    pub two_factor_auth_enabled: Option<bool>,
    /// Indicates if 2FA is required by the user's customer account.
    #[serde(rename = "two_factor_setup_required", skip_serializing_if = "Option::is_none")]
    pub two_factor_setup_required: Option<bool>,
}

impl User {
    pub fn new() -> User {
        User {
            login: None,
            name: None,
            limit_services: None,
            locked: None,
            require_new_password: None,
            role: None,
            two_factor_auth_enabled: None,
            two_factor_setup_required: None,
        }
    }
}


