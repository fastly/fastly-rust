/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingScalyrAllOf {
    /// The region that log data will be sent to.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<Region>,
    /// The token to use for authentication ([https://www.scalyr.com/keys](https://www.scalyr.com/keys)).
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The name of the logfile within Scalyr.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl LoggingScalyrAllOf {
    pub fn new() -> LoggingScalyrAllOf {
        LoggingScalyrAllOf {
            region: None,
            token: None,
            project_id: None,
        }
    }
}

/// The region that log data will be sent to.
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

