/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Script {
    /// Unique script identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Parent page ID
    #[serde(rename = "page_id", skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    /// Script source (inline or external URL)
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// URLs where this script was observed
    #[serde(rename = "urls", skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    #[serde(rename = "first_seen_at", skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,
    #[serde(rename = "last_seen_at", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    /// Reason for authorization decision
    #[serde(rename = "justification", skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Current script content hash
    #[serde(rename = "current_hash", skip_serializing_if = "Option::is_none")]
    pub current_hash: Option<String>,
    /// Hash of authorized script content
    #[serde(rename = "authorized_hash", skip_serializing_if = "Option::is_none")]
    pub authorized_hash: Option<String>,
    /// Script authorization status
    #[serde(rename = "authorization_status", skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<AuthorizationStatus>,
    #[serde(rename = "authorized_at", skip_serializing_if = "Option::is_none")]
    pub authorized_at: Option<String>,
}

impl Script {
    pub fn new() -> Script {
        Script {
            id: None,
            page_id: None,
            source: None,
            urls: None,
            first_seen_at: None,
            last_seen_at: None,
            justification: None,
            current_hash: None,
            authorized_hash: None,
            authorization_status: None,
            authorized_at: None,
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

