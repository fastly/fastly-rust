/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RateLimiter {
    /// A human readable name for the rate limiting rule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited.
    #[serde(rename = "uri_dictionary_name", skip_serializing_if = "Option::is_none")]
    pub uri_dictionary_name: Option<String>,
    /// Array of HTTP methods to apply rate limiting to.
    #[serde(rename = "http_methods", skip_serializing_if = "Option::is_none")]
    pub http_methods: Option<std::collections::HashSet<HttpMethods>>,
    /// Upper limit of requests per second allowed by the rate limiter.
    #[serde(rename = "rps_limit", skip_serializing_if = "Option::is_none")]
    pub rps_limit: Option<i32>,
    /// Number of seconds during which the RPS limit must be exceeded in order to trigger a violation.
    #[serde(rename = "window_size", skip_serializing_if = "Option::is_none")]
    pub window_size: Option<WindowSize>,
    /// Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`.
    #[serde(rename = "client_key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<Vec<String>>,
    /// Length of time in minutes that the rate limiter is in effect after the initial violation is detected.
    #[serde(rename = "penalty_box_duration", skip_serializing_if = "Option::is_none")]
    pub penalty_box_duration: Option<i32>,
    /// The action to take when a rate limiter violation is detected.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Custom response to be sent when the rate limit is exceeded. Required if `action` is `response`.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<::std::collections::HashMap<String, String>>,
    /// Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration.
    #[serde(rename = "response_object_name", skip_serializing_if = "Option::is_none")]
    pub response_object_name: Option<String>,
    /// Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries.
    #[serde(rename = "logger_type", skip_serializing_if = "Option::is_none")]
    pub logger_type: Option<LoggerType>,
    /// Revision number of the rate limiting feature implementation. Defaults to the most recent revision.
    #[serde(rename = "feature_revision", skip_serializing_if = "Option::is_none")]
    pub feature_revision: Option<i32>,
}

impl RateLimiter {
    pub fn new() -> RateLimiter {
        RateLimiter {
            name: None,
            uri_dictionary_name: None,
            http_methods: None,
            rps_limit: None,
            window_size: None,
            client_key: None,
            penalty_box_duration: None,
            action: None,
            response: None,
            response_object_name: None,
            logger_type: None,
            feature_revision: None,
        }
    }
}

/// Array of HTTP methods to apply rate limiting to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethods {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "TRACE")]
    TRACE,
}

impl Default for HttpMethods {
    fn default() -> HttpMethods {
        Self::HEAD
    }
}
/// Number of seconds during which the RPS limit must be exceeded in order to trigger a violation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindowSize {
    #[serde(rename = "1")]
    WindowSizeOneSecond,
    #[serde(rename = "10")]
    WindowSizeTenSeconds,
    #[serde(rename = "60")]
    WindowSizeOneMinute,
}

impl Default for WindowSize {
    fn default() -> WindowSize {
        Self::WindowSizeOneSecond
    }
}
/// The action to take when a rate limiter violation is detected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "response")]
    Response,
    #[serde(rename = "response_object")]
    ResponseObject,
    #[serde(rename = "log_only")]
    LogOnly,
}

impl Default for Action {
    fn default() -> Action {
        Self::Response
    }
}
/// Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LoggerType {
    #[serde(rename = "azureblob")]
    Azureblob,
    #[serde(rename = "bigquery")]
    Bigquery,
    #[serde(rename = "cloudfiles")]
    Cloudfiles,
    #[serde(rename = "datadog")]
    Datadog,
    #[serde(rename = "digitalocean")]
    Digitalocean,
    #[serde(rename = "elasticsearch")]
    Elasticsearch,
    #[serde(rename = "ftp")]
    Ftp,
    #[serde(rename = "gcs")]
    Gcs,
    #[serde(rename = "googleanalytics")]
    Googleanalytics,
    #[serde(rename = "heroku")]
    Heroku,
    #[serde(rename = "honeycomb")]
    Honeycomb,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "kafka")]
    Kafka,
    #[serde(rename = "kinesis")]
    Kinesis,
    #[serde(rename = "logentries")]
    Logentries,
    #[serde(rename = "loggly")]
    Loggly,
    #[serde(rename = "logshuttle")]
    Logshuttle,
    #[serde(rename = "newrelic")]
    Newrelic,
    #[serde(rename = "openstack")]
    Openstack,
    #[serde(rename = "papertrail")]
    Papertrail,
    #[serde(rename = "pubsub")]
    Pubsub,
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "scalyr")]
    Scalyr,
    #[serde(rename = "sftp")]
    Sftp,
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "stackdriver")]
    Stackdriver,
    #[serde(rename = "sumologic")]
    Sumologic,
    #[serde(rename = "syslog")]
    Syslog,
}

impl Default for LoggerType {
    fn default() -> LoggerType {
        Self::Azureblob
    }
}

