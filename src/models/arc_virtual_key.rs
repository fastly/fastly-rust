/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcVirtualKey : A virtual key used to authenticate requests through the AI Runtime Control gateway.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcVirtualKey {
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
}

impl ArcVirtualKey {
    /// A virtual key used to authenticate requests through the AI Runtime Control gateway.
    pub fn new(name: String) -> ArcVirtualKey {
        ArcVirtualKey {
            name,
            expires_at: None,
            security_enabled: None,
            security_action: None,
            rpm_limit: None,
            tpm_limit: None,
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

