/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolAdditional {
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
    /// The hostname to [override the Host header](https://docs.fastly.com/en/guides/specifying-an-override-host). Defaults to `null` meaning no override of the Host header will occur. This setting can also be added to a Server definition. If the field is set on a Server definition it will override the Pool setting.
    #[serde(rename = "override_host", skip_serializing_if = "Option::is_none")]
    pub override_host: Option<String>,
}

impl PoolAdditional {
    pub fn new() -> PoolAdditional {
        PoolAdditional {
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
        }
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

