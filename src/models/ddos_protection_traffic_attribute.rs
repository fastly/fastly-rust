/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DdosProtectionTrafficAttribute : Name of an attribute type used in traffic stats.

use std::fmt;

/// Name of an attribute type used in traffic stats.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DdosProtectionTrafficAttribute {
    #[serde(rename = "source_ip")]
    SourceIp,
    #[serde(rename = "country_code")]
    CountryCode,
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "asn")]
    Asn,
    #[serde(rename = "source_ip_prefix")]
    SourceIpPrefix,
    #[serde(rename = "user_agent")]
    UserAgent,
    #[serde(rename = "method_path")]
    MethodPath,

}

impl fmt::Display for DdosProtectionTrafficAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SourceIp => write!(f, "{}", "source_ip"),
            Self::CountryCode => write!(f, "{}", "country_code"),
            Self::Host => write!(f, "{}", "host"),
            Self::Asn => write!(f, "{}", "asn"),
            Self::SourceIpPrefix => write!(f, "{}", "source_ip_prefix"),
            Self::UserAgent => write!(f, "{}", "user_agent"),
            Self::MethodPath => write!(f, "{}", "method_path"),
        }
    }
}

impl Default for DdosProtectionTrafficAttribute {
    fn default() -> DdosProtectionTrafficAttribute {
        Self::SourceIp
    }
}




