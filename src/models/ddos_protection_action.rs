/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DdosProtectionAction : Action types for a rule.

use std::fmt;

/// Action types for a rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DdosProtectionAction {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "disabled")]
    Disabled,

}

impl fmt::Display for DdosProtectionAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default => write!(f, "{}", "default"),
            Self::Block => write!(f, "{}", "block"),
            Self::Log => write!(f, "{}", "log"),
            Self::Disabled => write!(f, "{}", "disabled"),
        }
    }
}

impl Default for DdosProtectionAction {
    fn default() -> DdosProtectionAction {
        Self::Default
    }
}




