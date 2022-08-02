/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafActiveRuleDataAttributes {
    /// The ModSecurity rule ID of the associated rule revision.
    #[serde(rename = "modsec_rule_id", skip_serializing_if = "Option::is_none")]
    pub modsec_rule_id: Option<i32>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<crate::models::WafRuleRevisionOrLatest>>,
    /// Describes the behavior for the particular rule revision within this firewall version.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl WafActiveRuleDataAttributes {
    pub fn new() -> WafActiveRuleDataAttributes {
        WafActiveRuleDataAttributes {
            modsec_rule_id: None,
            revision: None,
            status: None,
        }
    }
}

/// Describes the behavior for the particular rule revision within this firewall version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "score")]
    Score,
}

impl Default for Status {
    fn default() -> Status {
        Self::Log
    }
}

