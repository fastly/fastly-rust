/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingSyslogResponse {
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
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    #[serde(rename = "tls_ca_cert", skip_serializing_if = "Option::is_none")]
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    #[serde(rename = "tls_client_cert", skip_serializing_if = "Option::is_none")]
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    #[serde(rename = "tls_client_key", skip_serializing_if = "Option::is_none")]
    pub tls_client_key: Option<String>,
    /// The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported.
    #[serde(rename = "tls_hostname", skip_serializing_if = "Option::is_none")]
    pub tls_hostname: Option<String>,
    /// A hostname or IPv4 address.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The port number.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
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
    pub use_tls: Option<crate::models::LoggingUseTlsString>,
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
}

impl LoggingSyslogResponse {
    pub fn new() -> LoggingSyslogResponse {
        LoggingSyslogResponse {
            name: None,
            placement: None,
            response_condition: None,
            format: None,
            log_processing_region: None,
            format_version: None,
            tls_ca_cert: None,
            tls_client_cert: None,
            tls_client_key: None,
            tls_hostname: None,
            address: None,
            port: None,
            message_type: None,
            hostname: None,
            ipv4: None,
            token: None,
            use_tls: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            service_id: None,
            version: None,
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

