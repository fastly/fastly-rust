/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafActiveRuleResponseDataAttributes {
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The latest rule revision number that is available for the associated rule revision.
    #[serde(rename = "latest_revision", skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<i32>,
    /// Indicates if the associated rule revision is up to date or not.
    #[serde(rename = "outdated", skip_serializing_if = "Option::is_none")]
    pub outdated: Option<bool>,
}

impl WafActiveRuleResponseDataAttributes {
    pub fn new() -> WafActiveRuleResponseDataAttributes {
        WafActiveRuleResponseDataAttributes {
            created_at: None,
            deleted_at: None,
            updated_at: None,
            latest_revision: None,
            outdated: None,
        }
    }
}


