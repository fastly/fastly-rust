/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// Permission : The permission the user has in relation to the service.

use std::fmt;

/// The permission the user has in relation to the service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "purge_select")]
    PurgeSelect,
    #[serde(rename = "purge_all")]
    PurgeAll,

}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Full => write!(f, "{}", "full"),
            Self::ReadOnly => write!(f, "{}", "read_only"),
            Self::PurgeSelect => write!(f, "{}", "purge_select"),
            Self::PurgeAll => write!(f, "{}", "purge_all"),
        }
    }
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Full
    }
}




