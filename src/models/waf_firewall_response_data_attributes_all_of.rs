/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafFirewallResponseDataAttributesAllOf {
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    /// The number of active Fastly rules set to block on the active or latest firewall version.
    #[serde(rename = "active_rules_fastly_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_block_count: Option<i32>,
    /// The number of active Fastly rules set to log on the active or latest firewall version.
    #[serde(rename = "active_rules_fastly_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_log_count: Option<i32>,
    /// The number of active Fastly rules set to score on the active or latest firewall version.
    #[serde(rename = "active_rules_fastly_score_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_fastly_score_count: Option<i32>,
    /// The number of active OWASP rules set to block on the active or latest firewall version.
    #[serde(rename = "active_rules_owasp_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_block_count: Option<i32>,
    /// The number of active OWASP rules set to log on the active or latest firewall version.
    #[serde(rename = "active_rules_owasp_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_log_count: Option<i32>,
    /// The number of active OWASP rules set to score on the active or latest firewall version.
    #[serde(rename = "active_rules_owasp_score_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_owasp_score_count: Option<i32>,
    /// The number of active Trustwave rules set to block on the active or latest firewall version.
    #[serde(rename = "active_rules_trustwave_block_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_trustwave_block_count: Option<i32>,
    /// The number of active Trustwave rules set to log on the active or latest firewall version.
    #[serde(rename = "active_rules_trustwave_log_count", skip_serializing_if = "Option::is_none")]
    pub active_rules_trustwave_log_count: Option<i32>,
}

impl WafFirewallResponseDataAttributesAllOf {
    pub fn new() -> WafFirewallResponseDataAttributesAllOf {
        WafFirewallResponseDataAttributesAllOf {
            service_id: None,
            active_rules_fastly_block_count: None,
            active_rules_fastly_log_count: None,
            active_rules_fastly_score_count: None,
            active_rules_owasp_block_count: None,
            active_rules_owasp_log_count: None,
            active_rules_owasp_score_count: None,
            active_rules_trustwave_block_count: None,
            active_rules_trustwave_log_count: None,
        }
    }
}


