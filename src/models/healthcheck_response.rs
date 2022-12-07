/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HealthcheckResponse {
    /// How often to run the health check in milliseconds.
    #[serde(rename = "check_interval", skip_serializing_if = "Option::is_none")]
    pub check_interval: Option<i32>,
    /// A freeform descriptive note.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The status code expected from the host.
    #[serde(rename = "expected_response", skip_serializing_if = "Option::is_none")]
    pub expected_response: Option<i32>,
    /// Array of custom headers that will be added to the health check probes.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// Which host to check.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Whether to use version 1.0 or 1.1 HTTP.
    #[serde(rename = "http_version", skip_serializing_if = "Option::is_none")]
    pub http_version: Option<String>,
    /// When loading a config, the initial number of probes to be seen as OK.
    #[serde(rename = "initial", skip_serializing_if = "Option::is_none")]
    pub initial: Option<i32>,
    /// Which HTTP method to use.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The name of the health check.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The path to check.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// How many health checks must succeed to be considered healthy.
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
    /// Timeout in milliseconds.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// The number of most recent health check queries to keep for this health check.
    #[serde(rename = "window", skip_serializing_if = "Option::is_none")]
    pub window: Option<i32>,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<Box<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<i32>>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "deleted_at", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Date and time in ISO 8601 format.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl HealthcheckResponse {
    pub fn new() -> HealthcheckResponse {
        HealthcheckResponse {
            check_interval: None,
            comment: None,
            expected_response: None,
            headers: None,
            host: None,
            http_version: None,
            initial: None,
            method: None,
            name: None,
            path: None,
            threshold: None,
            timeout: None,
            window: None,
            service_id: None,
            version: None,
            created_at: None,
            deleted_at: None,
            updated_at: None,
        }
    }
}


