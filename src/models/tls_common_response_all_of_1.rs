/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsCommonResponseAllOf1 {
    /// Whether to use TLS.
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<UseTls>,
}

impl TlsCommonResponseAllOf1 {
    pub fn new() -> TlsCommonResponseAllOf1 {
        TlsCommonResponseAllOf1 {
            use_tls: None,
        }
    }
}

/// Whether to use TLS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UseTls {
    #[serde(rename = "0")]
    NoTls,
    #[serde(rename = "1")]
    UseTls,
}

impl Default for UseTls {
    fn default() -> UseTls {
        Self::NoTls
    }
}

