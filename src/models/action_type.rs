/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ActionType : The type of action to take when a rule matches.

use std::fmt;

/// The type of action to take when a rule matches.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "service")]
    ActionTypeService,

}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ActionTypeService => write!(f, "{}", "service"),
        }
    }
}

impl Default for ActionType {
    fn default() -> ActionType {
        Self::ActionTypeService
    }
}




