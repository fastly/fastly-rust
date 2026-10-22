/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleAction : Action for traffic matching this rule.

use std::fmt;

/// Action for traffic matching this rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "block")]
    Block,

}

impl fmt::Display for NetworkProtectRuleAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "{}", "allow"),
            Self::Block => write!(f, "{}", "block"),
        }
    }
}

impl Default for NetworkProtectRuleAction {
    fn default() -> NetworkProtectRuleAction {
        Self::Allow
    }
}




