/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TlsCommon {
    /// A secure certificate to authenticate a server with. Must be in PEM format.
    #[serde(rename = "tls_ca_cert", skip_serializing_if = "Option::is_none")]
    pub tls_ca_cert: Option<String>,
    /// The client certificate used to make authenticated requests. Must be in PEM format.
    #[serde(rename = "tls_client_cert", skip_serializing_if = "Option::is_none")]
    pub tls_client_cert: Option<String>,
    /// The client private key used to make authenticated requests. Must be in PEM format.
    #[serde(rename = "tls_client_key", skip_serializing_if = "Option::is_none")]
    pub tls_client_key: Option<String>,
    /// The hostname used to verify a server's certificate. It can either be the Common Name (CN) or a Subject Alternative Name (SAN).
    #[serde(rename = "tls_cert_hostname", skip_serializing_if = "Option::is_none")]
    pub tls_cert_hostname: Option<String>,
    /// Whether to use TLS.
    #[serde(rename = "use_tls", skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<UseTls>,
}

impl TlsCommon {
    pub fn new() -> TlsCommon {
        TlsCommon {
            tls_ca_cert: None,
            tls_client_cert: None,
            tls_client_key: None,
            tls_cert_hostname: None,
            use_tls: None,
        }
    }
}

/// Whether to use TLS.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UseTls {
    #[serde(rename = "0")]
    UseTlsNoTls,
    #[serde(rename = "1")]
    UseTlsUseTls,
}

impl Default for UseTls {
    fn default() -> UseTls {
        Self::UseTlsNoTls
    }
}

