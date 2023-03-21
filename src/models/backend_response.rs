/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendResponse {
    /// A hostname, IPv4, or IPv6 address for the backend. This is the preferred way to specify the location of your backend.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Whether or not this backend should be automatically load balanced. If true, all backends with this setting that don't have a `request_condition` will be selected based on their `weight`.
    #[serde(rename = "auto_loadbalance", skip_serializing_if = "Option::is_none")]
    pub auto_loadbalance: Option<bool>,
    /// Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`.
    #[serde(rename = "between_bytes_timeout", skip_serializing_if = "Option::is_none")]
    pub between_bytes_timeout: Option<i32>,
    /// Unused.
    #[serde(rename = "client_cert", skip_serializing_if = "Option::is_none")]
    pub client_cert: Option<String>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Maximum duration in milliseconds to wait for a connection to this backend to be established. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.connect_timeout`.
    #[serde(rename = "connect_timeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    /// Maximum duration in milliseconds to wait for the server response to begin after a TCP connection is established and the request has been sent. If exceeded, the connection is aborted and a synthethic `503` response will be presented instead. May be set at runtime using `bereq.first_byte_timeout`.
    #[serde(rename = "first_byte_timeout", skip_serializing_if = "Option::is_none")]
    pub first_byte_timeout: Option<i32>,
    /// The name of the healthcheck to use with this backend.
    #[serde(rename = "healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<String>,
    /// The hostname of the backend. May be used as an alternative to `address` to set the backend location.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// IPv4 address of the backend. May be used as an alternative to `address` to set the backend location.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    /// IPv6 address of the backend. May be used as an alternative to `address` to set the backend location.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    /// How long in seconds to keep a persistent connection to the backend between requests.
    #[serde(rename = "keepalive_time", skip_serializing_if = "Option::is_none")]
    pub keepalive_time: Option<i32>,
    /// Maximum number of concurrent connections this backend will accept.
    #[serde(rename = "max_conn", skip_serializing_if = "Option::is_none")]
    pub max_conn: Option<i32>,
    /// Maximum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    #[serde(rename = "max_tls_version", skip_serializing_if = "Option::is_none")]
    pub max_tls_version: Option<String>,
    /// Minimum allowed TLS version on SSL connections to this backend. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    #[serde(rename = "min_tls_version", skip_serializing_if = "Option::is_none")]
    pub min_tls_version: Option<String>,
    /// The name of the backend.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If set, will replace the client-supplied HTTP `Host` header on connections to this backend. Applied after VCL has been processed, so this setting will take precedence over changing `bereq.http.Host` in VCL.
    #[serde(rename = "override_host", skip_serializing_if = "Option::is_none")]
    pub override_host: Option<String>,
    /// Port on which the backend server is listening for connections from Fastly. Setting `port` to 80 or 443 will also set `use_ssl` automatically (to false and true respectively), unless explicitly overridden by setting `use_ssl` in the same request.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Name of a Condition, which if satisfied, will select this backend during a request. If set, will override any `auto_loadbalance` setting. By default, the first backend added to a service is selected for all requests.
    #[serde(rename = "request_condition", skip_serializing_if = "Option::is_none")]
    pub request_condition: Option<String>,
    /// Identifier of the POP to use as a [shield](https://docs.fastly.com/en/guides/shielding).
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<String>,
    /// CA certificate attached to origin.
    #[serde(rename = "ssl_ca_cert", skip_serializing_if = "Option::is_none")]
    pub ssl_ca_cert: Option<String>,
    /// Overrides `ssl_hostname`, but only for cert verification. Does not affect SNI at all.
    #[serde(rename = "ssl_cert_hostname", skip_serializing_if = "Option::is_none")]
    pub ssl_cert_hostname: Option<String>,
    /// Be strict on checking SSL certs.
    #[serde(rename = "ssl_check_cert", skip_serializing_if = "Option::is_none")]
    pub ssl_check_cert: Option<bool>,
    /// List of [OpenSSL ciphers](https://www.openssl.org/docs/manmaster/man1/ciphers.html) to support for connections to this origin. If your backend server is not able to negotiate a connection meeting this constraint, a synthetic `503` error response will be generated.
    #[serde(rename = "ssl_ciphers", skip_serializing_if = "Option::is_none")]
    pub ssl_ciphers: Option<String>,
    /// Client certificate attached to origin.
    #[serde(rename = "ssl_client_cert", skip_serializing_if = "Option::is_none")]
    pub ssl_client_cert: Option<String>,
    /// Client key attached to origin.
    #[serde(rename = "ssl_client_key", skip_serializing_if = "Option::is_none")]
    pub ssl_client_key: Option<String>,
    /// Use `ssl_cert_hostname` and `ssl_sni_hostname` to configure certificate validation.
    #[serde(rename = "ssl_hostname", skip_serializing_if = "Option::is_none")]
    pub ssl_hostname: Option<String>,
    /// Overrides `ssl_hostname`, but only for SNI in the handshake. Does not affect cert validation at all.
    #[serde(rename = "ssl_sni_hostname", skip_serializing_if = "Option::is_none")]
    pub ssl_sni_hostname: Option<String>,
    /// Whether or not to require TLS for connections to this backend.
    #[serde(rename = "use_ssl", skip_serializing_if = "Option::is_none")]
    pub use_ssl: Option<bool>,
    /// Weight used to load balance this backend against others. May be any positive integer. If `auto_loadbalance` is true, the chance of this backend being selected is equal to its own weight over the sum of all weights for backends that have `auto_loadbalance` set to true.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
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
    /// Indicates whether the version of the service this backend is attached to accepts edits.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl BackendResponse {
    pub fn new() -> BackendResponse {
        BackendResponse {
            address: None,
            auto_loadbalance: None,
            between_bytes_timeout: None,
            client_cert: None,
            comment: None,
            connect_timeout: None,
            first_byte_timeout: None,
            healthcheck: None,
            hostname: None,
            ipv4: None,
            ipv6: None,
            keepalive_time: None,
            max_conn: None,
            max_tls_version: None,
            min_tls_version: None,
            name: None,
            override_host: None,
            port: None,
            request_condition: None,
            shield: None,
            ssl_ca_cert: None,
            ssl_cert_hostname: None,
            ssl_check_cert: None,
            ssl_ciphers: None,
            ssl_client_cert: None,
            ssl_client_key: None,
            ssl_hostname: None,
            ssl_sni_hostname: None,
            use_ssl: None,
            weight: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
            service_id: None,
            version: None,
            locked: None,
        }
    }
}


