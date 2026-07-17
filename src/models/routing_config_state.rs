/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RoutingConfigState : The current state of the routing config's versions.

use std::fmt;

/// The current state of the routing config's versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoutingConfigState {
    #[serde(rename = "draft-only")]
    StateDraftOnly,
    #[serde(rename = "active")]
    StateActive,
    #[serde(rename = "active-with-draft")]
    StateActiveWithDraft,

}

impl fmt::Display for RoutingConfigState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::StateDraftOnly => write!(f, "{}", "draft-only"),
            Self::StateActive => write!(f, "{}", "active"),
            Self::StateActiveWithDraft => write!(f, "{}", "active-with-draft"),
        }
    }
}

impl Default for RoutingConfigState {
    fn default() -> RoutingConfigState {
        Self::StateDraftOnly
    }
}




