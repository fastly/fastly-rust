/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingDatadogAdditional {
    /// The region that log data will be sent to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Datadog can ingest. 
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The API key from your Datadog account. Required.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl LoggingDatadogAdditional {
    pub fn new() -> LoggingDatadogAdditional {
        LoggingDatadogAdditional {
            region: None,
            format: None,
            token: None,
        }
    }
}

/// The region that log data will be sent to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "US")]
    US,
    #[serde(rename = "US3")]
    US3,
    #[serde(rename = "US5")]
    US5,
    #[serde(rename = "EU (legacy, same as EU1)")]
    EULegacySameAsEU1,
    #[serde(rename = "EU1")]
    EU1,
    #[serde(rename = "AP1")]
    AP1,
}

impl Default for Region {
    fn default() -> Region {
        Self::US
    }
}

