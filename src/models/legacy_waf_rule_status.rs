/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyWafRuleStatus {
    /// Describes the behavior for the particular rule within this firewall. Permitted values: `log`, `block`, and `disabled`. 
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The ModSecurity rule ID.
    #[serde(rename = "modsec_rule_id", skip_serializing_if = "Option::is_none")]
    pub modsec_rule_id: Option<String>,
    /// The Rule ID.
    #[serde(rename = "unique_rule_id", skip_serializing_if = "Option::is_none")]
    pub unique_rule_id: Option<String>,
}

impl LegacyWafRuleStatus {
    pub fn new() -> LegacyWafRuleStatus {
        LegacyWafRuleStatus {
            status: None,
            modsec_rule_id: None,
            unique_rule_id: None,
        }
    }
}


