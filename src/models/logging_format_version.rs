/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// LoggingFormatVersion : The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 

/// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoggingFormatVersion {
    #[serde(rename = "1")]
    LoggingFormatVersionV1,
    #[serde(rename = "2")]
    LoggingFormatVersionV2,

}

impl ToString for LoggingFormatVersion {
    fn to_string(&self) -> String {
        match self {
            Self::LoggingFormatVersionV1 => String::from("1"),
            Self::LoggingFormatVersionV2 => String::from("2"),
        }
    }
}

impl Default for LoggingFormatVersion {
    fn default() -> LoggingFormatVersion {
        Self::LoggingFormatVersionV1
    }
}




