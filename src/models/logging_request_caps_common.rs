/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingRequestCapsCommon {
    /// The maximum number of logs sent in one request. Defaults `0` for unbounded.
    #[serde(rename = "request_max_entries", skip_serializing_if = "Option::is_none")]
    pub request_max_entries: Option<i32>,
    /// The maximum number of bytes sent in one request. Defaults `0` for unbounded.
    #[serde(rename = "request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub request_max_bytes: Option<i32>,
}

impl LoggingRequestCapsCommon {
    pub fn new() -> LoggingRequestCapsCommon {
        LoggingRequestCapsCommon {
            request_max_entries: None,
            request_max_bytes: None,
        }
    }
}


