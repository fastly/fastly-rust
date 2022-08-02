/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Server {
    /// Weight (`1-100`) used to load balance this server against others.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Maximum number of connections. If the value is `0`, it inherits the value from pool's `max_conn_default`.
    #[serde(rename = "max_conn", skip_serializing_if = "Option::is_none")]
    pub max_conn: Option<i32>,
    /// Port number. Setting port `443` does not force TLS. Set `use_tls` in pool to force TLS.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// A hostname, IPv4, or IPv6 address for the server. Required.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Allows servers to be enabled and disabled in a pool.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The hostname to override the Host header. Defaults to `null` meaning no override of the Host header if not set. This setting can also be added to a Pool definition. However, the server setting will override the Pool setting.
    #[serde(rename = "override_host", skip_serializing_if = "Option::is_none")]
    pub override_host: Option<String>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            weight: None,
            max_conn: None,
            port: None,
            address: None,
            comment: None,
            disabled: None,
            override_host: None,
        }
    }
}


