/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LegacyWafRuleset {
    /// Date and time WAF ruleset VCL was last deployed.
    #[serde(rename = "last_push", skip_serializing_if = "Option::is_none")]
    pub last_push: Option<String>,
    /// The WAF ruleset VCL currently deployed.
    #[serde(rename = "vcl", skip_serializing_if = "Option::is_none")]
    pub vcl: Option<String>,
}

impl LegacyWafRuleset {
    pub fn new() -> LegacyWafRuleset {
        LegacyWafRuleset {
            last_push: None,
            vcl: None,
        }
    }
}


