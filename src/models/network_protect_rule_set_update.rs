/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleSetUpdate : Attributes accepted when updating a rule set.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkProtectRuleSetUpdate {
    #[serde(rename = "default_action", skip_serializing_if = "Option::is_none")]
    pub default_action: Option<crate::models::NetworkProtectRuleSetDefaultAction>,
    /// Rules in evaluation order; the first match wins.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::NetworkProtectRule>>,
}

impl NetworkProtectRuleSetUpdate {
    /// Attributes accepted when updating a rule set.
    pub fn new() -> NetworkProtectRuleSetUpdate {
        NetworkProtectRuleSetUpdate {
            default_action: None,
            rules: None,
        }
    }
}


