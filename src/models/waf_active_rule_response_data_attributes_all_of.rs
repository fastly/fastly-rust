/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafActiveRuleResponseDataAttributesAllOf {
    /// The latest rule revision number that is available for the associated rule revision.
    #[serde(rename = "latest_revision", skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<i32>,
    /// Indicates if the associated rule revision is up to date or not.
    #[serde(rename = "outdated", skip_serializing_if = "Option::is_none")]
    pub outdated: Option<bool>,
}

impl WafActiveRuleResponseDataAttributesAllOf {
    pub fn new() -> WafActiveRuleResponseDataAttributesAllOf {
        WafActiveRuleResponseDataAttributesAllOf {
            latest_revision: None,
            outdated: None,
        }
    }
}


