/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// ComputeAclList : An example of an ACL list response.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComputeAclList {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::ComputeAclCreateAclsResponse>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ComputeAclListMeta>>,
}

impl ComputeAclList {
    /// An example of an ACL list response.
    pub fn new() -> ComputeAclList {
        ComputeAclList {
            data: None,
            meta: None,
        }
    }
}


