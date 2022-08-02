/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafFirewallVersionResponseDataAttributes {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Whether a specific firewall version is currently deployed.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The number of active Fastly rules set to block.
    #[serde(rename = "active_rules_fastly_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_block_count: Option<i32>,
    /// The number of active Fastly rules set to log.
    #[serde(rename = "active_rules_fastly_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_log_count: Option<i32>,
    /// The number of active Fastly rules set to score.
    #[serde(rename = "active_rules_fastly_score_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_score_count: Option<i32>,
    /// The number of active OWASP rules set to block.
    #[serde(rename = "active_rules_owasp_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_block_count: Option<i32>,
    /// The number of active OWASP rules set to log.
    #[serde(rename = "active_rules_owasp_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_log_count: Option<i32>,
    /// The number of active OWASP rules set to score.
    #[serde(rename = "active_rules_owasp_score_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_score_count: Option<i32>,
    /// The number of active Trustwave rules set to block.
    #[serde(rename = "active_rules_trustwave_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_trustwave_block_count: Option<i32>,
    /// The number of active Trustwave rules set to log.
    #[serde(rename = "active_rules_trustwave_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_trustwave_log_count: Option<i32>,
    /// The status of the last deployment of this firewall version.
    #[serde(rename = "last_deployment_status", skip_serializing_if = "Option::is_none")]
    pub last_deployment_status: Option<LastDeploymentStatus>,
    /// Time-stamp (GMT) indicating when the firewall version was last deployed.
    #[serde(rename = "deployed_at", skip_serializing_if = "Option::is_none")]
    pub deployed_at: Option<String>,
    /// Contains error message if the firewall version fails to deploy.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl WafFirewallVersionResponseDataAttributes {
    pub fn new() -> WafFirewallVersionResponseDataAttributes {
        WafFirewallVersionResponseDataAttributes {
            created_at: None,
            deleted_at: None,
            updated_at: None,
            active: None,
            active_rules_fastly_block_count: None,
            active_rules_fastly_log_count: None,
            active_rules_fastly_score_count: None,
            active_rules_owasp_block_count: None,
            active_rules_owasp_log_count: None,
            active_rules_owasp_score_count: None,
            active_rules_trustwave_block_count: None,
            active_rules_trustwave_log_count: None,
            last_deployment_status: None,
            deployed_at: None,
            error: None,
        }
    }
}

/// The status of the last deployment of this firewall version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LastDeploymentStatus {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "in progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for LastDeploymentStatus {
    fn default() -> LastDeploymentStatus {
        Self::Null
    }
}

