/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// TypeServiceAuthorization : Resource type

use std::fmt;

/// Resource type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeServiceAuthorization {
    #[serde(rename = "service_authorization")]
    ServiceAuthorization,

}

impl fmt::Display for TypeServiceAuthorization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ServiceAuthorization => write!(f, "{}", "service_authorization"),
        }
    }
}

impl Default for TypeServiceAuthorization {
    fn default() -> TypeServiceAuthorization {
        Self::ServiceAuthorization
    }
}




