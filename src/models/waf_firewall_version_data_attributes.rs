/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WafFirewallVersionDataAttributes {
    /// Allowed HTTP versions.
    #[serde(rename = "allowed_http_versions", skip_serializing_if = "Option::is_none")]
    pub allowed_http_versions: Option<String>,
    /// A space-separated list of HTTP method names.
    #[serde(rename = "allowed_methods", skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<String>,
    /// Allowed request content types.
    #[serde(rename = "allowed_request_content_type", skip_serializing_if = "Option::is_none")]
    pub allowed_request_content_type: Option<String>,
    /// Allowed request content type charset.
    #[serde(rename = "allowed_request_content_type_charset", skip_serializing_if = "Option::is_none")]
    pub allowed_request_content_type_charset: Option<String>,
    /// The maximum allowed argument name length.
    #[serde(rename = "arg_name_length", skip_serializing_if = "Option::is_none")]
    pub arg_name_length: Option<i32>,
    /// The maximum allowed length of an argument.
    #[serde(rename = "arg_length", skip_serializing_if = "Option::is_none")]
    pub arg_length: Option<i32>,
    /// The maximum allowed size of all files (in bytes).
    #[serde(rename = "combined_file_sizes", skip_serializing_if = "Option::is_none")]
    pub combined_file_sizes: Option<i32>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Score value to add for critical anomalies.
    #[serde(rename = "critical_anomaly_score", skip_serializing_if = "Option::is_none")]
    pub critical_anomaly_score: Option<i32>,
    /// CRS validate UTF8 encoding.
    #[serde(rename = "crs_validate_utf8_encoding", skip_serializing_if = "Option::is_none")]
    pub crs_validate_utf8_encoding: Option<bool>,
    /// Score value to add for error anomalies.
    #[serde(rename = "error_anomaly_score", skip_serializing_if = "Option::is_none")]
    pub error_anomaly_score: Option<i32>,
    /// A space-separated list of country codes in ISO 3166-1 (two-letter) format.
    #[serde(rename = "high_risk_country_codes", skip_serializing_if = "Option::is_none")]
    pub high_risk_country_codes: Option<String>,
    /// HTTP violation threshold.
    #[serde(rename = "http_violation_score_threshold", skip_serializing_if = "Option::is_none")]
    pub http_violation_score_threshold: Option<i32>,
    /// Inbound anomaly threshold.
    #[serde(rename = "inbound_anomaly_score_threshold", skip_serializing_if = "Option::is_none")]
    pub inbound_anomaly_score_threshold: Option<i32>,
    /// Local file inclusion attack threshold.
    #[serde(rename = "lfi_score_threshold", skip_serializing_if = "Option::is_none")]
    pub lfi_score_threshold: Option<i32>,
    /// Whether a specific firewall version is locked from being modified.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The maximum allowed file size, in bytes.
    #[serde(rename = "max_file_size", skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i32>,
    /// The maximum number of arguments allowed.
    #[serde(rename = "max_num_args", skip_serializing_if = "Option::is_none")]
    pub max_num_args: Option<i32>,
    /// Score value to add for notice anomalies.
    #[serde(rename = "notice_anomaly_score", skip_serializing_if = "Option::is_none")]
    pub notice_anomaly_score: Option<i32>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<Box<i32>>,
    /// The configured paranoia level.
    #[serde(rename = "paranoia_level", skip_serializing_if = "Option::is_none")]
    pub paranoia_level: Option<i32>,
    /// PHP injection threshold.
    #[serde(rename = "php_injection_score_threshold", skip_serializing_if = "Option::is_none")]
    pub php_injection_score_threshold: Option<i32>,
    /// Remote code execution threshold.
    #[serde(rename = "rce_score_threshold", skip_serializing_if = "Option::is_none")]
    pub rce_score_threshold: Option<i32>,
    /// A space-separated list of allowed file extensions.
    #[serde(rename = "restricted_extensions", skip_serializing_if = "Option::is_none")]
    pub restricted_extensions: Option<String>,
    /// A space-separated list of allowed header names.
    #[serde(rename = "restricted_headers", skip_serializing_if = "Option::is_none")]
    pub restricted_headers: Option<String>,
    /// Remote file inclusion attack threshold.
    #[serde(rename = "rfi_score_threshold", skip_serializing_if = "Option::is_none")]
    pub rfi_score_threshold: Option<i32>,
    /// Session fixation attack threshold.
    #[serde(rename = "session_fixation_score_threshold", skip_serializing_if = "Option::is_none")]
    pub session_fixation_score_threshold: Option<i32>,
    /// SQL injection attack threshold.
    #[serde(rename = "sql_injection_score_threshold", skip_serializing_if = "Option::is_none")]
    pub sql_injection_score_threshold: Option<i32>,
    /// The maximum size of argument names and values.
    #[serde(rename = "total_arg_length", skip_serializing_if = "Option::is_none")]
    pub total_arg_length: Option<i32>,
    /// Score value to add for warning anomalies.
    #[serde(rename = "warning_anomaly_score", skip_serializing_if = "Option::is_none")]
    pub warning_anomaly_score: Option<i32>,
    /// XSS attack threshold.
    #[serde(rename = "xss_score_threshold", skip_serializing_if = "Option::is_none")]
    pub xss_score_threshold: Option<i32>,
}

impl WafFirewallVersionDataAttributes {
    pub fn new() -> WafFirewallVersionDataAttributes {
        WafFirewallVersionDataAttributes {
            allowed_http_versions: None,
            allowed_methods: None,
            allowed_request_content_type: None,
            allowed_request_content_type_charset: None,
            arg_name_length: None,
            arg_length: None,
            combined_file_sizes: None,
            comment: None,
            critical_anomaly_score: None,
            crs_validate_utf8_encoding: None,
            error_anomaly_score: None,
            high_risk_country_codes: None,
            http_violation_score_threshold: None,
            inbound_anomaly_score_threshold: None,
            lfi_score_threshold: None,
            locked: None,
            max_file_size: None,
            max_num_args: None,
            notice_anomaly_score: None,
            number: None,
            paranoia_level: None,
            php_injection_score_threshold: None,
            rce_score_threshold: None,
            restricted_extensions: None,
            restricted_headers: None,
            rfi_score_threshold: None,
            session_fixation_score_threshold: None,
            sql_injection_score_threshold: None,
            total_arg_length: None,
            warning_anomaly_score: None,
            xss_score_threshold: None,
        }
    }
}


