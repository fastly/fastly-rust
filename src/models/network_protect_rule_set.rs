/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleSet : Shared ordered rules and default action.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleSet {
    #[serde(rename = "default_action")]
    pub default_action: crate::models::NetworkProtectRuleSetDefaultAction,
    /// Rules in evaluation order with read-only timestamps.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::NetworkProtectRuleResponse>,
}

impl NetworkProtectRuleSet {
    /// Shared ordered rules and default action.
    pub fn new(default_action: crate::models::NetworkProtectRuleSetDefaultAction, rules: Vec<crate::models::NetworkProtectRuleResponse>) -> NetworkProtectRuleSet {
        NetworkProtectRuleSet {
            default_action,
            rules,
        }
    }
}


