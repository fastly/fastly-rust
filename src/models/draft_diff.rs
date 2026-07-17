/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// DraftDiff : The differences between the draft and active versions of a routing config.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DraftDiff {
    /// Paths that exist in the draft but not in the active version.
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<crate::models::PathWithRules>>,
    /// Paths that exist in the active version but not in the draft.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<crate::models::PathWithRules>>,
    /// Paths that exist in both versions but have changed.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<Vec<crate::models::PathChange>>,
}

impl DraftDiff {
    /// The differences between the draft and active versions of a routing config.
    pub fn new() -> DraftDiff {
        DraftDiff {
            added: None,
            deleted: None,
            modified: None,
        }
    }
}


