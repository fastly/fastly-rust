/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafRuleRevisionAttributes {
    /// Message metadata for the rule.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Corresponding ModSecurity rule ID.
    #[serde(rename = "modsec_rule_id", skip_serializing_if = "Option::is_none")]
    pub modsec_rule_id: Option<i32>,
    /// Paranoia level for the rule.
    #[serde(rename = "paranoia_level", skip_serializing_if = "Option::is_none")]
    pub paranoia_level: Option<i32>,
    /// Revision number.
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// Severity metadata for the rule.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    /// The ModSecurity rule logic.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The state, indicating if the revision is the most recent version of the rule.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The VCL representation of the rule logic.
    #[serde(rename = "vcl", skip_serializing_if = "Option::is_none")]
    pub vcl: Option<String>,
}

impl WafRuleRevisionAttributes {
    pub fn new() -> WafRuleRevisionAttributes {
        WafRuleRevisionAttributes {
            message: None,
            modsec_rule_id: None,
            paranoia_level: None,
            revision: None,
            severity: None,
            source: None,
            state: None,
            vcl: None,
        }
    }
}

/// The state, indicating if the revision is the most recent version of the rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "outdated")]
    Outdated,
}

impl Default for State {
    fn default() -> State {
        Self::Latest
    }
}

