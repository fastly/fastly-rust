/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AclEntry {
    /// Whether to negate the match. Useful primarily when creating individual exceptions to larger subnets.
    #[serde(rename = "negated", skip_serializing_if = "Option::is_none")]
    pub negated: Option<Negated>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// An IP address.
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Number of bits for the subnet mask applied to the IP address. For IPv4 addresses, a value of 32 represents the smallest subnet mask (1 address), 24 represents a class C subnet mask (256 addresses), 16 represents a class B subnet mask (65k addresses), and 8 is class A subnet mask (16m addresses). If not provided, no mask is applied.
    #[serde(rename = "subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<i32>,
}

impl AclEntry {
    pub fn new() -> AclEntry {
        AclEntry {
            negated: None,
            comment: None,
            ip: None,
            subnet: None,
        }
    }
}

/// Whether to negate the match. Useful primarily when creating individual exceptions to larger subnets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Negated {
    #[serde(rename = "0")]
    NegatedDisable,
    #[serde(rename = "1")]
    NegatedEnable,
}

impl Default for Negated {
    fn default() -> Negated {
        Self::NegatedDisable
    }
}

