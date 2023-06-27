/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutomationTokenCreateResponseAllOf {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<crate::models::ReadOnlyId>>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Box<crate::models::ReadOnlyUserId>>,
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Box<crate::models::ReadOnlyCustomerId>>,
    #[serde(rename = "sudo_expires_at", skip_serializing_if = "Option::is_none")]
    pub sudo_expires_at: Option<String>,
    /// A UTC time-stamp of when the token was created. 
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// A UTC time-stamp of when the token was last used.
    #[serde(rename = "last_used_at", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    /// The User-Agent header of the client that last used the token.
    #[serde(rename = "user_agent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl AutomationTokenCreateResponseAllOf {
    pub fn new() -> AutomationTokenCreateResponseAllOf {
        AutomationTokenCreateResponseAllOf {
            id: None,
            user_id: None,
            customer_id: None,
            sudo_expires_at: None,
            created_at: None,
            access_token: None,
            last_used_at: None,
            user_agent: None,
        }
    }
}


