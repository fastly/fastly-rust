/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingKafkaResponse {
    /// The name for the real-time logging configuration.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    #[serde(rename = "placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    #[serde(rename = "format_version", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<FormatVersion>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    #[serde(rename = "response_condition", skip_serializing_if = "Option::is_none")]
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
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
    /// The Kafka topic to send logs to. Required.
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// A comma-separated list of IP addresses or hostnames of Kafka brokers. Required.
    #[serde(rename = "brokers", skip_serializing_if = "Option::is_none")]
    pub brokers: Option<String>,
    /// The codec used for compression of your logs.
    #[serde(rename = "compression_codec", skip_serializing_if = "Option::is_none")]
    pub compression_codec: Option<CompressionCodec>,
    /// The number of acknowledgements a leader must receive before a write is considered successful.
    #[serde(rename = "required_acks", skip_serializing_if = "Option::is_none")]
    pub required_acks: Option<RequiredAcks>,
    /// The maximum number of bytes sent in one request. Defaults `0` (no limit).
    #[serde(rename = "request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub request_max_bytes: Option<i32>,
    /// Enables parsing of key=value tuples from the beginning of a logline, turning them into [record headers](https://cwiki.apache.org/confluence/display/KAFKA/KIP-82+-+Add+Record+Headers).
    #[serde(rename = "parse_log_keyvals", skip_serializing_if = "Option::is_none")]
    pub parse_log_keyvals: Option<bool>,
    /// SASL authentication method.
    #[serde(rename = "auth_method", skip_serializing_if = "Option::is_none")]
    pub auth_method: Option<AuthMethod>,
    /// SASL user.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// SASL password.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<crate::models::LoggingUseTls>,
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
    pub version: Option<Box<i32>>,
}

impl LoggingKafkaResponse {
    pub fn new() -> LoggingKafkaResponse {
        LoggingKafkaResponse {
            name: None,
            placement: None,
            format_version: None,
            response_condition: None,
            format: None,
            tls_ca_cert: None,
            tls_client_cert: None,
            tls_client_key: None,
            tls_hostname: None,
            topic: None,
            brokers: None,
            compression_codec: None,
            required_acks: None,
            request_max_bytes: None,
            parse_log_keyvals: None,
            auth_method: None,
            user: None,
            password: None,
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
    #[serde(rename = "waf_debug")]
    WafDebug,
    #[serde(rename = "null")]
    Null,
}

impl Default for Placement {
    fn default() -> Placement {
        Self::None
    }
}
/// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormatVersion {
    #[serde(rename = "1")]
    FormatVersionV1,
    #[serde(rename = "2")]
    FormatVersionV2,
}

impl Default for FormatVersion {
    fn default() -> FormatVersion {
        Self::FormatVersionV1
    }
}
/// The codec used for compression of your logs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionCodec {
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "null")]
    Null,
}

impl Default for CompressionCodec {
    fn default() -> CompressionCodec {
        Self::Gzip
    }
}
/// The number of acknowledgements a leader must receive before a write is considered successful.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequiredAcks {
    #[serde(rename = "1")]
    RequiredAcksOne,
    #[serde(rename = "0")]
    RequiredAcksNone,
    #[serde(rename = "-1")]
    RequiredAcksAll,
}

impl Default for RequiredAcks {
    fn default() -> RequiredAcks {
        Self::RequiredAcksOne
    }
}
/// SASL authentication method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthMethod {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "scram-sha-256")]
    ScramSha256,
    #[serde(rename = "scram-sha-512")]
    ScramSha512,
}

impl Default for AuthMethod {
    fn default() -> AuthMethod {
        Self::Plain
    }
}

