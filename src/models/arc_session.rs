/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ArcSession : A session log. Other than `firewall_enabled` (always present), `firewall_*` fields are only present when AI Firewall ran on a request in this session. They're absent (not merely empty) otherwise, so their absence and an empty value mean different things. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArcSession {
    /// The unique identifier of the session.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the virtual key used for this session.
    #[serde(rename = "virtual_key_id")]
    pub virtual_key_id: String,
    /// The display name of the virtual key used for this session.
    #[serde(rename = "virtual_key_name", skip_serializing_if = "Option::is_none")]
    pub virtual_key_name: Option<String>,
    /// The AI model identifier used.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the provider connection that ultimately served the session.
    #[serde(rename = "provider")]
    pub provider: String,
    /// The ID of the provider connection that ultimately served the session.
    #[serde(rename = "provider_connection_id")]
    pub provider_connection_id: String,
    /// The ID of the provider connection originally requested, before any failover occurred.
    #[serde(rename = "primary_provider", skip_serializing_if = "Option::is_none")]
    pub primary_provider: Option<String>,
    /// The number of requests made in this session.
    #[serde(rename = "requests")]
    pub requests: i32,
    /// The total input tokens consumed in this session.
    #[serde(rename = "input_tokens")]
    pub input_tokens: i32,
    /// The total output tokens generated in this session.
    #[serde(rename = "output_tokens")]
    pub output_tokens: i32,
    /// The number of AI Firewall violations detected in this session.
    #[serde(rename = "violations")]
    pub violations: i32,
    /// Whether AI Firewall was enabled for this session's virtual key.
    #[serde(rename = "firewall_enabled")]
    pub firewall_enabled: bool,
    /// The outcome AI Firewall reached for this session. Present only when AI Firewall ran.
    #[serde(rename = "firewall_outcome", skip_serializing_if = "Option::is_none")]
    pub firewall_outcome: Option<String>,
    /// The AI Firewall stage that produced the outcome (for example, the request or response stage). Present only when AI Firewall ran.
    #[serde(rename = "firewall_stage", skip_serializing_if = "Option::is_none")]
    pub firewall_stage: Option<String>,
    /// The AI Firewall policy that was applied. Present only when AI Firewall ran.
    #[serde(rename = "firewall_policy", skip_serializing_if = "Option::is_none")]
    pub firewall_policy: Option<String>,
    /// Why AI Firewall inspection was incomplete, if applicable. Present only when AI Firewall ran and inspection didn't complete.
    #[serde(rename = "firewall_incomplete_reason", skip_serializing_if = "Option::is_none")]
    pub firewall_incomplete_reason: Option<String>,
    /// The date and time the session was created, in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The date and time the session was last updated, in RFC 3339 format.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The request and response pairs recorded for this session.
    #[serde(rename = "logs")]
    pub logs: Vec<crate::models::ArcSessionLogEntry>,
}

impl ArcSession {
    /// A session log. Other than `firewall_enabled` (always present), `firewall_*` fields are only present when AI Firewall ran on a request in this session. They're absent (not merely empty) otherwise, so their absence and an empty value mean different things. 
    pub fn new(id: String, virtual_key_id: String, model: String, provider: String, provider_connection_id: String, requests: i32, input_tokens: i32, output_tokens: i32, violations: i32, firewall_enabled: bool, created_at: String, updated_at: String, logs: Vec<crate::models::ArcSessionLogEntry>) -> ArcSession {
        ArcSession {
            id,
            virtual_key_id,
            virtual_key_name: None,
            model,
            provider,
            provider_connection_id,
            primary_provider: None,
            requests,
            input_tokens,
            output_tokens,
            violations,
            firewall_enabled,
            firewall_outcome: None,
            firewall_stage: None,
            firewall_policy: None,
            firewall_incomplete_reason: None,
            created_at,
            updated_at,
            logs,
        }
    }
}


