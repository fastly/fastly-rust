/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// InitialVersion : Optional initial version payload to seed the new routing config with paths and rules in a single request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InitialVersion {
    /// Whether to activate the initial version on creation.
    #[serde(rename = "activate", skip_serializing_if = "Option::is_none")]
    pub activate: Option<bool>,
    /// A freeform comment for the initial version.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The paths to create on the initial version.
    #[serde(rename = "paths", skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<crate::models::InitialVersionPath>>,
}

impl InitialVersion {
    /// Optional initial version payload to seed the new routing config with paths and rules in a single request.
    pub fn new() -> InitialVersion {
        InitialVersion {
            activate: None,
            comment: None,
            paths: None,
        }
    }
}


