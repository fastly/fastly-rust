/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ComputeAclListEntries : Defines the structure of what the ACL List endpoint returns.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComputeAclListEntries {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ComputeAclListEntriesMeta>>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::ComputeAclListEntriesItem>>,
}

impl ComputeAclListEntries {
    /// Defines the structure of what the ACL List endpoint returns.
    pub fn new() -> ComputeAclListEntries {
        ComputeAclListEntries {
            meta: None,
            entries: None,
        }
    }
}


