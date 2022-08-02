/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingGenericCommon {
    /// How the message should be formatted.
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    /// A timestamp format
    #[serde(rename = "timestamp_format", skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "gzip_level", skip_serializing_if = "Option::is_none")]
    pub gzip_level: Option<i32>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "compression_codec", skip_serializing_if = "Option::is_none")]
    pub compression_codec: Option<CompressionCodec>,
}

impl LoggingGenericCommon {
    pub fn new() -> LoggingGenericCommon {
        LoggingGenericCommon {
            message_type: None,
            timestamp_format: None,
            period: None,
            gzip_level: None,
            compression_codec: None,
        }
    }
}

/// How the message should be formatted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "loggly")]
    Loggly,
    #[serde(rename = "logplex")]
    Logplex,
    #[serde(rename = "blank")]
    Blank,
}

impl Default for MessageType {
    fn default() -> MessageType {
        Self::Classic
    }
}
/// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionCodec {
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "gzip")]
    Gzip,
}

impl Default for CompressionCodec {
    fn default() -> CompressionCodec {
        Self::Zstd
    }
}

