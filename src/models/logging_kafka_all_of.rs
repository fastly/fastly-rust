/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingKafkaAllOf {
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
}

impl LoggingKafkaAllOf {
    pub fn new() -> LoggingKafkaAllOf {
        LoggingKafkaAllOf {
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
        }
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

