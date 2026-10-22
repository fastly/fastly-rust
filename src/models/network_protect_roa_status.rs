/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRoaStatus : Whether a Route Origin Authorization permits Fastly to announce this prefix. Fastly refreshes this on a schedule; reading a network never triggers a check, so this can be older than the moment you read it. A status of `pending` means no check has completed; `valid` authorizes Fastly at this prefix length; `invalid` means a covering ROA does not authorize Fastly; and `not_found` means no ROA covers the prefix. The controller may omit an unrecognized status, and omission is not an authorization result. A valid ROA alone does not make a network ready for advertisement.

use std::fmt;

/// Whether a Route Origin Authorization permits Fastly to announce this prefix. Fastly refreshes this on a schedule; reading a network never triggers a check, so this can be older than the moment you read it. A status of `pending` means no check has completed; `valid` authorizes Fastly at this prefix length; `invalid` means a covering ROA does not authorize Fastly; and `not_found` means no ROA covers the prefix. The controller may omit an unrecognized status, and omission is not an authorization result. A valid ROA alone does not make a network ready for advertisement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectRoaStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "not_found")]
    NotFound,

}

impl fmt::Display for NetworkProtectRoaStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "{}", "pending"),
            Self::Valid => write!(f, "{}", "valid"),
            Self::Invalid => write!(f, "{}", "invalid"),
            Self::NotFound => write!(f, "{}", "not_found"),
        }
    }
}

impl Default for NetworkProtectRoaStatus {
    fn default() -> NetworkProtectRoaStatus {
        Self::Pending
    }
}




