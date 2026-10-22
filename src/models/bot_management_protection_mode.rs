/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// BotManagementProtectionMode : The protection mode of the workspace. `off` disables all enforcement. `log` detects and logs bots without blocking or challenging requests. `block` enforces the actions set on categories and bots.

use std::fmt;

/// The protection mode of the workspace. `off` disables all enforcement. `log` detects and logs bots without blocking or challenging requests. `block` enforces the actions set on categories and bots.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BotManagementProtectionMode {
    #[serde(rename = "false")]
    _false,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "block")]
    Block,

}

impl fmt::Display for BotManagementProtectionMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_false => write!(f, "{}", "false"),
            Self::Log => write!(f, "{}", "log"),
            Self::Block => write!(f, "{}", "block"),
        }
    }
}

impl Default for BotManagementProtectionMode {
    fn default() -> BotManagementProtectionMode {
        Self::_false
    }
}




