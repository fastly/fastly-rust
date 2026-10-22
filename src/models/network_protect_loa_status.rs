/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectLoaStatus : Fastly operator status for the Letter of Authorization. A status of `pending` means review is incomplete; `approved` means operators accepted it; and `rejected` means they declined it. An approved LOA alone does not make a network ready for advertisement.

use std::fmt;

/// Fastly operator status for the Letter of Authorization. A status of `pending` means review is incomplete; `approved` means operators accepted it; and `rejected` means they declined it. An approved LOA alone does not make a network ready for advertisement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectLoaStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,

}

impl fmt::Display for NetworkProtectLoaStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "{}", "pending"),
            Self::Approved => write!(f, "{}", "approved"),
            Self::Rejected => write!(f, "{}", "rejected"),
        }
    }
}

impl Default for NetworkProtectLoaStatus {
    fn default() -> NetworkProtectLoaStatus {
        Self::Pending
    }
}




