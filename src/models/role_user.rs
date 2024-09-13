/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RoleUser : The permissions role assigned to the user. Can be `user`, `billing`, `engineer`, or `superuser`.

use std::fmt;

/// The permissions role assigned to the user. Can be `user`, `billing`, `engineer`, or `superuser`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleUser {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "billing")]
    Billing,
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "superuser")]
    Superuser,

}

impl fmt::Display for RoleUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::User => write!(f, "{}", "user"),
            Self::Billing => write!(f, "{}", "billing"),
            Self::Engineer => write!(f, "{}", "engineer"),
            Self::Superuser => write!(f, "{}", "superuser"),
        }
    }
}

impl Default for RoleUser {
    fn default() -> RoleUser {
        Self::User
    }
}




