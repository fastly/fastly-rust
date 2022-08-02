/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingHoneycombAllOf {
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Honeycomb can ingest.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The Honeycomb Dataset you want to log to.
    #[serde(rename = "dataset", skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    /// The Write Key from the Account page of your Honeycomb account.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl LoggingHoneycombAllOf {
    pub fn new() -> LoggingHoneycombAllOf {
        LoggingHoneycombAllOf {
            format: None,
            dataset: None,
            token: None,
        }
    }
}


