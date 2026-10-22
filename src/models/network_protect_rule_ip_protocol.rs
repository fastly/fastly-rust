/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// NetworkProtectRuleIpProtocol : The IP protocol to match. Omit to match any protocol.

use std::fmt;

/// The IP protocol to match. Omit to match any protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkProtectRuleIpProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "icmpv6")]
    Icmpv6,

}

impl fmt::Display for NetworkProtectRuleIpProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Tcp => write!(f, "{}", "tcp"),
            Self::Udp => write!(f, "{}", "udp"),
            Self::Icmp => write!(f, "{}", "icmp"),
            Self::Icmpv6 => write!(f, "{}", "icmpv6"),
        }
    }
}

impl Default for NetworkProtectRuleIpProtocol {
    fn default() -> NetworkProtectRuleIpProtocol {
        Self::Tcp
    }
}




