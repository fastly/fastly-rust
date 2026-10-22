/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectPrefixListFamily : The address family; it cannot change after creation.

use std::fmt;

/// The address family; it cannot change after creation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectPrefixListFamily {
    #[serde(rename = "v4")]
    V4,
    #[serde(rename = "v6")]
    V6,

}

impl fmt::Display for NetworkProtectPrefixListFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::V4 => write!(f, "{}", "v4"),
            Self::V6 => write!(f, "{}", "v6"),
        }
    }
}

impl Default for NetworkProtectPrefixListFamily {
    fn default() -> NetworkProtectPrefixListFamily {
        Self::V4
    }
}




