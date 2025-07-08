/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingNewrelicotlpAdditional {
    /// A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The Insert API key from the Account page of your New Relic account. Required.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The region to which to stream logs.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// (Optional) URL of the New Relic Trace Observer, if you are using New Relic Infinite Tracing.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl LoggingNewrelicotlpAdditional {
    pub fn new() -> LoggingNewrelicotlpAdditional {
        LoggingNewrelicotlpAdditional {
            format: None,
            token: None,
            region: None,
            url: None,
        }
    }
}

/// The region to which to stream logs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "US")]
    US,
    #[serde(rename = "EU")]
    EU,
}

impl Default for Region {
    fn default() -> Region {
        Self::US
    }
}

