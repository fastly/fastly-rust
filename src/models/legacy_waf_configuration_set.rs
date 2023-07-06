/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyWafConfigurationSet {
    /// The active configuration set is the default configuration set when creating a new WAF. When Fastly adds configuration sets, the new versions become the default (active).
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The name of the configuration set.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LegacyWafConfigurationSet {
    pub fn new() -> LegacyWafConfigurationSet {
        LegacyWafConfigurationSet {
            active: None,
            name: None,
        }
    }
}


