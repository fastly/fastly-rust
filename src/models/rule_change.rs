/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RuleChange : A modification to an existing rule's action between versions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleChange {
    /// Alphanumeric string identifying the rule. Stable across versions of the routing config.
    #[serde(rename = "rule_id", skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "old_action", skip_serializing_if = "Option::is_none")]
    pub old_action: Option<Box<crate::models::Action>>,
    #[serde(rename = "new_action", skip_serializing_if = "Option::is_none")]
    pub new_action: Option<Box<crate::models::Action>>,
}

impl RuleChange {
    /// A modification to an existing rule's action between versions.
    pub fn new() -> RuleChange {
        RuleChange {
            rule_id: None,
            old_action: None,
            new_action: None,
        }
    }
}


