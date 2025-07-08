/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingOpenstackResponse {
    /// The name for the real-time logging configuration.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    #[serde(rename = "response_condition", skip_serializing_if = "Option::is_none")]
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global.
    #[serde(rename = "log_processing_region", skip_serializing_if = "Option::is_none")]
    pub log_processing_region: Option<LogProcessingRegion>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    #[serde(rename = "format_version", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<FormatVersion>,
    /// How the message should be formatted.
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    /// A timestamp format
    #[serde(rename = "timestamp_format", skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "compression_codec", skip_serializing_if = "Option::is_none")]
    pub compression_codec: Option<CompressionCodec>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    #[serde(rename = "gzip_level", skip_serializing_if = "Option::is_none")]
    pub gzip_level: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<String>>,
    /// Your OpenStack account access key.
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// The name of your OpenStack container.
    #[serde(rename = "bucket_name", skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The path to upload logs to.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Your OpenStack auth url.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The username for your OpenStack account.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl LoggingOpenstackResponse {
    pub fn new() -> LoggingOpenstackResponse {
        LoggingOpenstackResponse {
            name: None,
            placement: None,
            response_condition: None,
            format: None,
            log_processing_region: None,
            format_version: None,
            message_type: None,
            timestamp_format: None,
            compression_codec: None,
            period: None,
            gzip_level: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            service_id: None,
            version: None,
            access_key: None,
            bucket_name: None,
            path: None,
            public_key: None,
            url: None,
            user: None,
        }
    }
}

/// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Placement {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "null")]
    Null,
}

impl Default for Placement {
    fn default() -> Placement {
        Self::None
    }
}
/// The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogProcessingRegion {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "us")]
    Us,
}

impl Default for LogProcessingRegion {
    fn default() -> LogProcessingRegion {
        Self::None
    }
}
/// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormatVersion {
    #[serde(rename = "1")]
    V1,
    #[serde(rename = "2")]
    V2,
}

impl Default for FormatVersion {
    fn default() -> FormatVersion {
        Self::V1
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

