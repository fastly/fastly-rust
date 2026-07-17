/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// PathChange : Modifications to an existing path between versions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PathChange {
    /// Alphanumeric string identifying the path. Stable across versions of the routing config.
    #[serde(rename = "path_id", skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    /// The current path pattern.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The previous path pattern, if it changed.
    #[serde(rename = "old_path", skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    /// Rules that were added to this path.
    #[serde(rename = "rules_added", skip_serializing_if = "Option::is_none")]
    pub rules_added: Option<Vec<crate::models::RuleResponse>>,
    /// Rules that were modified on this path.
    #[serde(rename = "rules_changed", skip_serializing_if = "Option::is_none")]
    pub rules_changed: Option<Vec<crate::models::RuleChange>>,
    /// Rules that were removed from this path.
    #[serde(rename = "rules_deleted", skip_serializing_if = "Option::is_none")]
    pub rules_deleted: Option<Vec<crate::models::RuleResponse>>,
}

impl PathChange {
    /// Modifications to an existing path between versions.
    pub fn new() -> PathChange {
        PathChange {
            path_id: None,
            path: None,
            old_path: None,
            rules_added: None,
            rules_changed: None,
            rules_deleted: None,
        }
    }
}


