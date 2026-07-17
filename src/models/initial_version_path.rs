/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// InitialVersionPath : A path on the initial version, with its rules.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InitialVersionPath {
    /// The URL path pattern, beginning with `/`. Maximum 2048 characters.
    #[serde(rename = "path")]
    pub path: String,
    /// The rules to create on this path.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::RuleCreate>>,
}

impl InitialVersionPath {
    /// A path on the initial version, with its rules.
    pub fn new(path: String) -> InitialVersionPath {
        InitialVersionPath {
            path,
            rules: None,
        }
    }
}


