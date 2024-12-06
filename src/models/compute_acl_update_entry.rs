/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ComputeAclUpdateEntry : Defines the structure of an ACL update request entry.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComputeAclUpdateEntry {
    /// One of \"create\" or \"update\", indicating that the rest of this entry is to be added to/updated in the ACL.
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// An IP prefix defined in Classless Inter-Domain Routing (CIDR) format, i.e. a valid IP address (v4 or v6) followed by a forward slash (/) and a prefix length (0-32 or 0-128, depending on address family).
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The action taken on the IP address, either \"block\" or \"allow\".
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl ComputeAclUpdateEntry {
    /// Defines the structure of an ACL update request entry.
    pub fn new() -> ComputeAclUpdateEntry {
        ComputeAclUpdateEntry {
            op: None,
            prefix: None,
            action: None,
        }
    }
}


