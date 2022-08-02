/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingSyslogAllOf {
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<crate::models::LoggingMessageType>,
    /// The hostname used for the syslog endpoint.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The IPv4 address used for the syslog endpoint.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    /// Whether to prepend each message with a specific token.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<crate::models::LoggingUseTls>,
}

impl LoggingSyslogAllOf {
    pub fn new() -> LoggingSyslogAllOf {
        LoggingSyslogAllOf {
            message_type: None,
            hostname: None,
            ipv4: None,
            token: None,
            use_tls: None,
        }
    }
}


