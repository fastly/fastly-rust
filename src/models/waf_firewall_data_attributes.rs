/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafFirewallDataAttributes {
    /// The status of the firewall.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Name of the corresponding condition object.
    #[serde(rename = "prefetch_condition", skip_serializing_if = "Option::is_none")]
    pub prefetch_condition: Option<String>,
    /// Name of the corresponding response object.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(rename = "service_version_number", skip_serializing_if = "Option::is_none")]
    pub service_version_number: Option<Box<i32>>,
}

impl WafFirewallDataAttributes {
    pub fn new() -> WafFirewallDataAttributes {
        WafFirewallDataAttributes {
            disabled: None,
            prefetch_condition: None,
            response: None,
            service_version_number: None,
        }
    }
}


