/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingLogentriesAllOf {
    /// The port number.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Use token based authentication ([https://logentries.com/doc/input-token/](https://logentries.com/doc/input-token/)).
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<crate::models::LoggingUseTls>,
    /// The region to which to stream logs.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
}

impl LoggingLogentriesAllOf {
    pub fn new() -> LoggingLogentriesAllOf {
        LoggingLogentriesAllOf {
            port: None,
            token: None,
            use_tls: None,
            region: None,
        }
    }
}

/// The region to which to stream logs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "US")]
    US,
    #[serde(rename = "US-2")]
    US2,
    #[serde(rename = "US-3")]
    US3,
    #[serde(rename = "EU")]
    EU,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "AP")]
    AP,
}

impl Default for Region {
    fn default() -> Region {
        Self::US
    }
}

