/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ComputeAclLookup : An example of an ACL Lookup response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComputeAclLookup {
    /// A valid IPv4 or IPv6 address.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The length of address in the IP addressing space.
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// One of \"ALLOW\" or \"BLOCK\".
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl ComputeAclLookup {
    /// An example of an ACL Lookup response.
    pub fn new() -> ComputeAclLookup {
        ComputeAclLookup {
            prefix: None,
            length: None,
            action: None,
        }
    }
}

