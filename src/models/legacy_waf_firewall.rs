/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyWafFirewall {
    /// Date and time that VCL was last pushed to cache nodes.
    #[serde(rename = "last_push", skip_serializing_if = "Option::is_none")]
    pub last_push: Option<String>,
    /// Name of the corresponding condition object.
    #[serde(rename = "prefetch_condition", skip_serializing_if = "Option::is_none")]
    pub prefetch_condition: Option<String>,
    /// Name of the corresponding response object.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<crate::models::ReadOnlyVersion>>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<crate::models::ReadOnlyServiceId>>,
    /// The status of the firewall.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The number of rule statuses set to log.
    #[serde(rename = "rule_statuses_log_count", skip_serializing_if = "Option::is_none")]
    pub rule_statuses_log_count: Option<i32>,
    /// The number of rule statuses set to block.
    #[serde(rename = "rule_statuses_block_count", skip_serializing_if = "Option::is_none")]
    pub rule_statuses_block_count: Option<i32>,
    /// The number of rule statuses set to disabled.
    #[serde(rename = "rule_statuses_disabled_count", skip_serializing_if = "Option::is_none")]
    pub rule_statuses_disabled_count: Option<i32>,
}

impl LegacyWafFirewall {
    pub fn new() -> LegacyWafFirewall {
        LegacyWafFirewall {
            last_push: None,
            prefetch_condition: None,
            response: None,
            version: None,
            service_id: None,
            disabled: None,
            rule_statuses_log_count: None,
            rule_statuses_block_count: None,
            rule_statuses_disabled_count: None,
        }
    }
}


