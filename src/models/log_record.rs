/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogRecord {
    /// The ID of the Fastly customer that owns the service.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<crate::models::LogPropertyServiceId>>,
    /// Timestamp of the request in ISO 8601 format.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The autonomous system (AS) number of the client.
    #[serde(rename = "client_as_number", skip_serializing_if = "Option::is_none")]
    pub client_as_number: Option<i32>,
    /// The client's country subdivision code as found in ISO 3166-2.
    #[serde(rename = "client_region", skip_serializing_if = "Option::is_none")]
    pub client_region: Option<String>,
    /// The two-letter ISO 3166-1 country code for the client.
    #[serde(rename = "client_country_code", skip_serializing_if = "Option::is_none")]
    pub client_country_code: Option<String>,
    /// The name of the operating system installed on the client device.
    #[serde(rename = "client_os_name", skip_serializing_if = "Option::is_none")]
    pub client_os_name: Option<String>,
    /// The type of the client's device.
    #[serde(rename = "client_device_type", skip_serializing_if = "Option::is_none")]
    pub client_device_type: Option<String>,
    /// The name of the browser in use on the client device.
    #[serde(rename = "client_browser_name", skip_serializing_if = "Option::is_none")]
    pub client_browser_name: Option<String>,
    /// The version of the browser in use on client device.
    #[serde(rename = "client_browser_version", skip_serializing_if = "Option::is_none")]
    pub client_browser_version: Option<String>,
    /// The name of the Fastly POP that served this request.
    #[serde(rename = "fastly_pop", skip_serializing_if = "Option::is_none")]
    pub fastly_pop: Option<String>,
    /// The name of the origin host that served this request.
    #[serde(rename = "origin_host", skip_serializing_if = "Option::is_none")]
    pub origin_host: Option<String>,
    /// HTTP protocol version in use for this request. For example, HTTP/1.1.
    #[serde(rename = "request_protocol", skip_serializing_if = "Option::is_none")]
    pub request_protocol: Option<String>,
    /// The name of the request host used for this request.
    #[serde(rename = "request_host", skip_serializing_if = "Option::is_none")]
    pub request_host: Option<String>,
    /// The URL path supplied for this request.
    #[serde(rename = "request_path", skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,
    /// HTTP method sent by the client such as \"GET\" or \"POST\".
    #[serde(rename = "request_method", skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    /// Body bytes sent to the client in the response.
    #[serde(rename = "response_bytes_body", skip_serializing_if = "Option::is_none")]
    pub response_bytes_body: Option<i32>,
    /// Header bytes sent to the client in the response.
    #[serde(rename = "response_bytes_header", skip_serializing_if = "Option::is_none")]
    pub response_bytes_header: Option<i32>,
    /// Total bytes sent to the client in the response.
    #[serde(rename = "response_content_length", skip_serializing_if = "Option::is_none")]
    pub response_content_length: Option<i32>,
    /// The content type of the response sent to the client.
    #[serde(rename = "response_content_type", skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    /// The HTTP reason phrase returned for this request, if any.
    #[serde(rename = "response_reason", skip_serializing_if = "Option::is_none")]
    pub response_reason: Option<String>,
    /// The state of the request with optional suffixes describing special cases.
    #[serde(rename = "response_state", skip_serializing_if = "Option::is_none")]
    pub response_state: Option<String>,
    /// The HTTP response code returned for this request.
    #[serde(rename = "response_status", skip_serializing_if = "Option::is_none")]
    pub response_status: Option<i32>,
    /// The time since the request started in seconds.
    #[serde(rename = "response_time", skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f32>,
    /// Indicates whether the request was a HIT or a MISS.
    #[serde(rename = "response_x_cache", skip_serializing_if = "Option::is_none")]
    pub response_x_cache: Option<String>,
    /// Indicates whether this request was fulfilled from cache.
    #[serde(rename = "is_cache_hit", skip_serializing_if = "Option::is_none")]
    pub is_cache_hit: Option<bool>,
    /// Indicates whether the request was handled by a Fastly edge POP.
    #[serde(rename = "is_edge", skip_serializing_if = "Option::is_none")]
    pub is_edge: Option<bool>,
    /// Indicates whether the request was handled by a Fastly shield POP.
    #[serde(rename = "is_shield", skip_serializing_if = "Option::is_none")]
    pub is_shield: Option<bool>,
}

impl LogRecord {
    pub fn new() -> LogRecord {
        LogRecord {
            customer_id: None,
            service_id: None,
            timestamp: None,
            client_as_number: None,
            client_region: None,
            client_country_code: None,
            client_os_name: None,
            client_device_type: None,
            client_browser_name: None,
            client_browser_version: None,
            fastly_pop: None,
            origin_host: None,
            request_protocol: None,
            request_host: None,
            request_path: None,
            request_method: None,
            response_bytes_body: None,
            response_bytes_header: None,
            response_content_length: None,
            response_content_type: None,
            response_reason: None,
            response_state: None,
            response_status: None,
            response_time: None,
            response_x_cache: None,
            is_cache_hit: None,
            is_edge: None,
            is_shield: None,
        }
    }
}


