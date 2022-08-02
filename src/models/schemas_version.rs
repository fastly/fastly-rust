/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SchemasVersion {
    /// Whether this is the active version or not.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Unused at this time.
    #[serde(rename = "deployed", skip_serializing_if = "Option::is_none")]
    pub deployed: Option<bool>,
    /// Whether this version is locked or not. Objects can not be added or edited on locked versions.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The number of this version.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// Unused at this time.
    #[serde(rename = "staging", skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,
    /// Unused at this time.
    #[serde(rename = "testing", skip_serializing_if = "Option::is_none")]
    pub testing: Option<bool>,
}

impl SchemasVersion {
    pub fn new() -> SchemasVersion {
        SchemasVersion {
            active: None,
            comment: None,
            deployed: None,
            locked: None,
            number: None,
            staging: None,
            testing: None,
        }
    }
}


