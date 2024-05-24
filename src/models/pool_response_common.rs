/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PoolResponseCommon {
    /// Maximum duration in milliseconds that Fastly will wait while receiving no data on a download from a backend. If exceeded, the response received so far will be considered complete and the fetch will end. May be set at runtime using `bereq.between_bytes_timeout`.
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
}

impl PoolResponseCommon {
    pub fn new() -> PoolResponseCommon {
        PoolResponseCommon {
            between_bytes_timeout: None,
            connect_timeout: None,
            first_byte_timeout: None,
            max_conn_default: None,
            tls_check_cert: None,
            id: None,
        }
    }
}


