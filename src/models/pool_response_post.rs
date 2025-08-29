/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolResponsePost {
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
    /// Name for the Pool.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Selected POP to serve as a shield for the servers. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](https://www.fastly.com/documentation/reference/api/utils/pops/) to get a list of available POPs used for shielding.
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<String>,
    /// Condition which, if met, will select this configuration during a request. Optional.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
    /// List of OpenSSL ciphers (see the [openssl.org manpages](https://www.openssl.org/docs/man1.1.1/man1/ciphers.html) for details). Optional.
    #[serde(rename = "tls_ciphers", skip_serializing_if = "Option::is_none")]
    pub tls_ciphers: Option<String>,
    /// SNI hostname. Optional.
    #[serde(rename = "tls_sni_hostname", skip_serializing_if = "Option::is_none")]
    pub tls_sni_hostname: Option<String>,
    /// Minimum allowed TLS version on connections to this server. Optional.
    #[serde(rename = "min_tls_version", skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<i32>,
    /// Maximum allowed TLS version on connections to this server. Optional.
    #[serde(rename = "max_tls_version", skip_serializing_if = "Option::is_none")]
    pub max_tls_version: Option<i32>,
    /// Name of the healthcheck to use with this pool. Can be empty and could be reused across multiple backend and pools.
    #[serde(rename = "healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<String>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// What type of load balance group to use.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The hostname to [override the Host header](https://www.fastly.com/documentation/guides/full-site-delivery/domains-and-origins/specifying-an-override-host/). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting.
    #[serde(rename = "override_host", skip_serializing_if = "Option::is_none")]
    pub override_host: Option<String>,
    /// Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, for Delivery services, the response received so far will be considered complete and the fetch will end. For Compute services, timeout expiration is treated as a failure of the backend connection, and an error is generated. May be set at runtime using `bereq.between_bytes_timeout`.
    #[serde(rename = "between_bytes_timeout", skip_serializing_if = "Option::is_none")]
    pub between_bytes_timeout: Option<String>,
    /// How long to wait for a timeout in milliseconds.
    #[serde(rename = "connect_timeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,
    /// How long to wait for the first byte in milliseconds.
    #[serde(rename = "first_byte_timeout", skip_serializing_if = "Option::is_none")]
    pub first_byte_timeout: Option<String>,
    /// Maximum number of connections.
    #[serde(rename = "max_conn_default", skip_serializing_if = "Option::is_none")]
    pub max_conn_default: Option<String>,
    /// Be strict on checking TLS certs.
    #[serde(rename = "tls_check_cert", skip_serializing_if = "Option::is_none")]
    pub tls_check_cert: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Box<String>>,
    /// Percentage of capacity (`0-100`) that needs to be operationally available for a pool to be considered up.
    #[serde(rename = "quorum", skip_serializing_if = "Option::is_none")]
    pub quorum: Option<i32>,
}

impl PoolResponsePost {
    pub fn new() -> PoolResponsePost {
        PoolResponsePost {
            tls_ca_cert: None,
            tls_client_cert: None,
            tls_client_key: None,
            tls_cert_hostname: None,
            use_tls: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            service_id: None,
            version: None,
            name: None,
            shield: None,
            request_condition: None,
            tls_ciphers: None,
            tls_sni_hostname: None,
            min_tls_version: None,
            max_tls_version: None,
            healthcheck: None,
            comment: None,
            _type: None,
            override_host: None,
            between_bytes_timeout: None,
            connect_timeout: None,
            first_byte_timeout: None,
            max_conn_default: None,
            tls_check_cert: None,
            id: None,
            quorum: None,
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
/// What type of load balance group to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "hash")]
    Hash,
    #[serde(rename = "client")]
    Client,
}

impl Default for Type {
    fn default() -> Type {
        Self::Random
    }
}

