/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcVirtualKeyResponse {
    /// A display name for the virtual key.
    #[serde(rename = "name")]
    pub name: String,
    /// The date and time the key expires, in RFC 3339 format. Omit for a key that doesn't expire.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Whether AI Firewall security is enabled for requests made with this key.
    #[serde(rename = "security_enabled", skip_serializing_if = "Option::is_none")]
    pub security_enabled: Option<bool>,
    /// The action AI Firewall takes when it detects a violation on a request made with this key. Only meaningful when `security_enabled` is `true`.
    #[serde(rename = "security_action", skip_serializing_if = "Option::is_none")]
    pub security_action: Option<SecurityAction>,
    /// The requests-per-minute limit for this key. Omit for no limit.
    #[serde(rename = "rpm_limit", skip_serializing_if = "Option::is_none")]
    pub rpm_limit: Option<i32>,
    /// The tokens-per-minute limit for this key. Omit for no limit.
    #[serde(rename = "tpm_limit", skip_serializing_if = "Option::is_none")]
    pub tpm_limit: Option<i32>,
    /// The unique identifier of the virtual key.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the resource.
    #[serde(rename = "type")]
    pub _type: String,
    /// The ID of the user who created the key.
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// The display name of the user who created the key.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The date and time the key was last used to make a request, in RFC 3339 format. `null` if the key has never been used.
    #[serde(rename = "last_used_at", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    /// The date and time the key was deleted, in RFC 3339 format. `null` unless the key was deleted and `include_deleted` was used to retrieve it.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// The date and time the key was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the key was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Whether a spend limit is enabled for this key.
    #[serde(rename = "spend_limit_enabled", skip_serializing_if = "Option::is_none")]
    pub spend_limit_enabled: Option<bool>,
    /// The date and time `spend_limit_enabled` was last set to `true`, in RFC 3339 format. `null` if the spend limit has never been enabled. This timestamp is not cleared when `spend_limit_enabled` is later set back to `false`, so it can be non-null even when the spend limit is currently disabled. 
    #[serde(rename = "spend_limit_enabled_on", skip_serializing_if = "Option::is_none")]
    pub spend_limit_enabled_on: Option<String>,
}

impl ArcVirtualKeyResponse {
    pub fn new(name: String, id: String, _type: String, user_id: String, created_at: String, updated_at: String) -> ArcVirtualKeyResponse {
        ArcVirtualKeyResponse {
            name,
            expires_at: None,
            security_enabled: None,
            security_action: None,
            rpm_limit: None,
            tpm_limit: None,
            id,
            _type,
            user_id,
            created_by: None,
            last_used_at: None,
            deleted_at: None,
            created_at,
            updated_at,
            spend_limit_enabled: None,
            spend_limit_enabled_on: None,
        }
    }
}

/// The action AI Firewall takes when it detects a violation on a request made with this key. Only meaningful when `security_enabled` is `true`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityAction {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "replace")]
    Replace,
}

impl Default for SecurityAction {
    fn default() -> SecurityAction {
        Self::Log
    }
}

