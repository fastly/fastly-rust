/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRegion : The region served by this policy. Default is the only accepted value.

use std::fmt;

/// The region served by this policy. Default is the only accepted value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectRegion {
    #[serde(rename = "default")]
    Default,

}

impl fmt::Display for NetworkProtectRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default => write!(f, "{}", "default"),
        }
    }
}

impl Default for NetworkProtectRegion {
    fn default() -> NetworkProtectRegion {
        Self::Default
    }
}




