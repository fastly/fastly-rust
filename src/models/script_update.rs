/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScriptUpdate {
    /// Script authorization status
    #[serde(rename = "authorization_status", skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<AuthorizationStatus>,
    /// Reason for authorization decision
    #[serde(rename = "justification", skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Hash of authorized script content
    #[serde(rename = "authorized_hash", skip_serializing_if = "Option::is_none")]
    pub authorized_hash: Option<String>,
}

impl ScriptUpdate {
    pub fn new() -> ScriptUpdate {
        ScriptUpdate {
            authorization_status: None,
            justification: None,
            authorized_hash: None,
        }
    }
}

/// Script authorization status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthorizationStatus {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "unauthorized")]
    Unauthorized,
}

impl Default for AuthorizationStatus {
    fn default() -> AuthorizationStatus {
        Self::Authorized
    }
}

