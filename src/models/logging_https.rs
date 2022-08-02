/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingHttps {
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
    /// The maximum number of logs sent in one request. Defaults `0` (10k).
    #[serde(rename = "request_max_entries", skip_serializing_if = "Option::is_none")]
    pub request_max_entries: Option<i32>,
    /// The maximum number of bytes sent in one request. Defaults `0` (100MB).
    #[serde(rename = "request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub request_max_bytes: Option<i32>,
    /// The URL to send logs to. Must use HTTPS. Required.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Content type of the header sent with the request.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Name of the custom header sent with the request.
    #[serde(rename = "header_name", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    #[serde(rename = "message_type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<crate::models::LoggingMessageType>,
    /// Value of the custom header sent with the request.
    #[serde(rename = "header_value", skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
    /// HTTP method used for request.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    /// Enforces valid JSON formatting for log entries.
    #[serde(rename = "json_format", skip_serializing_if = "Option::is_none")]
    pub json_format: Option<JsonFormat>,
}

impl LoggingHttps {
    pub fn new() -> LoggingHttps {
        LoggingHttps {
            name: None,
            placement: None,
            format_version: None,
            response_condition: None,
            format: None,
            tls_ca_cert: None,
            tls_client_cert: None,
            tls_client_key: None,
            tls_hostname: None,
            request_max_entries: None,
            request_max_bytes: None,
            url: None,
            content_type: None,
            header_name: None,
            message_type: None,
            header_value: None,
            method: None,
            json_format: None,
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
/// HTTP method used for request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
}

impl Default for Method {
    fn default() -> Method {
        Self::POST
    }
}
/// Enforces valid JSON formatting for log entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JsonFormat {
    #[serde(rename = "0")]
    Disabled,
    #[serde(rename = "1")]
    JsonArray,
    #[serde(rename = "2")]
    NewlineDelimitedJson,
}

impl Default for JsonFormat {
    fn default() -> JsonFormat {
        Self::Disabled
    }
}

