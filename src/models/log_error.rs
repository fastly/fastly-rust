/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogError {
    /// Sequence number for ordering messages.
    #[serde(rename = "sequence_number", skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i32>,
    /// Timestamp of the error in microseconds.
    #[serde(rename = "error_time_us", skip_serializing_if = "Option::is_none")]
    pub error_time_us: Option<i64>,
    /// The stream type, always 'logging_error' for logging endpoint errors.
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    /// User-friendly error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Name of the logging endpoint that generated the error.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Additional error details as a JSON string.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl LogError {
    pub fn new() -> LogError {
        LogError {
            sequence_number: None,
            error_time_us: None,
            stream: None,
            message: None,
            endpoint: None,
            details: None,
        }
    }
}


