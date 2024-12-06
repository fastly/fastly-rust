/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogInsightsValues {
    /// The cache hit ratio for the URL specified in the dimension.
    #[serde(rename = "cache_hit_ratio", skip_serializing_if = "Option::is_none")]
    pub cache_hit_ratio: Option<f32>,
    /// The client's country subdivision code as defined by ISO 3166-2.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The cache hit ratio for the region.
    #[serde(rename = "region_chr", skip_serializing_if = "Option::is_none")]
    pub region_chr: Option<f32>,
    /// The error rate for the region.
    #[serde(rename = "region_error_rate", skip_serializing_if = "Option::is_none")]
    pub region_error_rate: Option<f32>,
    /// The HTTP request path.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The URL accounts for this percentage of the status code in this dimension.
    #[serde(rename = "rate_per_status", skip_serializing_if = "Option::is_none")]
    pub rate_per_status: Option<f32>,
    /// The rate at which the reason in this dimension occurs among responses to this URL with a 503 status code.
    #[serde(rename = "rate_per_url", skip_serializing_if = "Option::is_none")]
    pub rate_per_url: Option<f32>,
    /// The rate at which 503 status codes are returned for this URL.
    #[serde(rename = "503_rate_per_url", skip_serializing_if = "Option::is_none")]
    pub var_503_rate_per_url: Option<f32>,
    /// The version of the client's browser.
    #[serde(rename = "browser_version", skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    /// The percentage of requests matching the value in the current dimension.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// The average bandwidth in bytes for responses to requests to the URL in the current dimension.
    #[serde(rename = "average_bandwidth_bytes", skip_serializing_if = "Option::is_none")]
    pub average_bandwidth_bytes: Option<f32>,
    /// The total bandwidth percentage for all responses to requests to the URL in the current dimension.
    #[serde(rename = "bandwidth_percentage", skip_serializing_if = "Option::is_none")]
    pub bandwidth_percentage: Option<f32>,
    /// The average time in seconds to respond to requests to the URL in the current dimension.
    #[serde(rename = "average_response_time", skip_serializing_if = "Option::is_none")]
    pub average_response_time: Option<f32>,
    /// The P95 time in seconds to respond to requests to the URL in the current dimension.
    #[serde(rename = "p95_response_time", skip_serializing_if = "Option::is_none")]
    pub p95_response_time: Option<f32>,
    /// The total percentage of time to respond to all requests to the URL in the current dimension.
    #[serde(rename = "response_time_percentage", skip_serializing_if = "Option::is_none")]
    pub response_time_percentage: Option<f32>,
    /// The miss rate for requests to the URL in the current dimension.
    #[serde(rename = "miss_rate", skip_serializing_if = "Option::is_none")]
    pub miss_rate: Option<f32>,
    /// The percentage of all requests made to the URL in the current dimension.
    #[serde(rename = "request_percentage", skip_serializing_if = "Option::is_none")]
    pub request_percentage: Option<f32>,
}

impl LogInsightsValues {
    pub fn new() -> LogInsightsValues {
        LogInsightsValues {
            cache_hit_ratio: None,
            region: None,
            region_chr: None,
            region_error_rate: None,
            url: None,
            rate_per_status: None,
            rate_per_url: None,
            var_503_rate_per_url: None,
            browser_version: None,
            rate: None,
            average_bandwidth_bytes: None,
            bandwidth_percentage: None,
            average_response_time: None,
            p95_response_time: None,
            response_time_percentage: None,
            miss_rate: None,
            request_percentage: None,
        }
    }
}


