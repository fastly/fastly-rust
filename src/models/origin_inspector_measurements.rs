/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// OriginInspectorMeasurements : A measurements object provides a count of the total number of `responses` received by Fastly from your origin servers in the reported time period, for the relevant POP and backend name as specified in the [entry](#entry-data-model). It also includes the number of responses for specific HTTP response status codes and for status code ranges. This dataset is sparse: only the keys with non-zero values will be included in the record. Where a specific status code does not have a field in this model (e.g., `429 Too Many Requests`), any responses with that code will be counted as part of the range count (`4xx` in this case) but will not be separately identified in the data. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OriginInspectorMeasurements {
    /// Number of responses from origin.
    #[serde(rename = "responses", skip_serializing_if = "Option::is_none")]
    pub responses: Option<i32>,
    /// Number of header bytes from origin.
    #[serde(rename = "resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_header_bytes: Option<i32>,
    /// Number of body bytes from origin.
    #[serde(rename = "resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_body_bytes: Option<i32>,
    /// Number of 1xx \"Informational\" status codes delivered from origin.
    #[serde(rename = "status_1xx", skip_serializing_if = "Option::is_none")]
    pub status_1xx: Option<i32>,
    /// Number of 2xx \"Success\" status codes delivered from origin.
    #[serde(rename = "status_2xx", skip_serializing_if = "Option::is_none")]
    pub status_2xx: Option<i32>,
    /// Number of 3xx \"Redirection\" codes delivered from origin.
    #[serde(rename = "status_3xx", skip_serializing_if = "Option::is_none")]
    pub status_3xx: Option<i32>,
    /// Number of 4xx \"Client Error\" codes delivered from origin.
    #[serde(rename = "status_4xx", skip_serializing_if = "Option::is_none")]
    pub status_4xx: Option<i32>,
    /// Number of 5xx \"Server Error\" codes delivered from origin.
    #[serde(rename = "status_5xx", skip_serializing_if = "Option::is_none")]
    pub status_5xx: Option<i32>,
    /// Number of responses received with status code 200 (Success) from origin.
    #[serde(rename = "status_200", skip_serializing_if = "Option::is_none")]
    pub status_200: Option<i32>,
    /// Number of responses received with status code 204 (No Content) from origin.
    #[serde(rename = "status_204", skip_serializing_if = "Option::is_none")]
    pub status_204: Option<i32>,
    /// Number of responses received with status code 206 (Partial Content) from origin.
    #[serde(rename = "status_206", skip_serializing_if = "Option::is_none")]
    pub status_206: Option<i32>,
    /// Number of responses received with status code 301 (Moved Permanently) from origin.
    #[serde(rename = "status_301", skip_serializing_if = "Option::is_none")]
    pub status_301: Option<i32>,
    /// Number of responses received with status code 302 (Found) from origin.
    #[serde(rename = "status_302", skip_serializing_if = "Option::is_none")]
    pub status_302: Option<i32>,
    /// Number of responses received with status code 304 (Not Modified) from origin.
    #[serde(rename = "status_304", skip_serializing_if = "Option::is_none")]
    pub status_304: Option<i32>,
    /// Number of responses received with status code 400 (Bad Request) from origin.
    #[serde(rename = "status_400", skip_serializing_if = "Option::is_none")]
    pub status_400: Option<i32>,
    /// Number of responses received with status code 401 (Unauthorized) from origin.
    #[serde(rename = "status_401", skip_serializing_if = "Option::is_none")]
    pub status_401: Option<i32>,
    /// Number of responses received with status code 403 (Forbidden) from origin.
    #[serde(rename = "status_403", skip_serializing_if = "Option::is_none")]
    pub status_403: Option<i32>,
    /// Number of responses received with status code 404 (Not Found) from origin.
    #[serde(rename = "status_404", skip_serializing_if = "Option::is_none")]
    pub status_404: Option<i32>,
    /// Number of responses received with status code 416 (Range Not Satisfiable) from origin.
    #[serde(rename = "status_416", skip_serializing_if = "Option::is_none")]
    pub status_416: Option<i32>,
    /// Number of responses received with status code 429 (Too Many Requests) from origin.
    #[serde(rename = "status_429", skip_serializing_if = "Option::is_none")]
    pub status_429: Option<i32>,
    /// Number of responses received with status code 500 (Internal Server Error) from origin.
    #[serde(rename = "status_500", skip_serializing_if = "Option::is_none")]
    pub status_500: Option<i32>,
    /// Number of responses received with status code 501 (Not Implemented) from origin.
    #[serde(rename = "status_501", skip_serializing_if = "Option::is_none")]
    pub status_501: Option<i32>,
    /// Number of responses received with status code 502 (Bad Gateway) from origin.
    #[serde(rename = "status_502", skip_serializing_if = "Option::is_none")]
    pub status_502: Option<i32>,
    /// Number of responses received with status code 503 (Service Unavailable) from origin.
    #[serde(rename = "status_503", skip_serializing_if = "Option::is_none")]
    pub status_503: Option<i32>,
    /// Number of responses received with status code 504 (Gateway Timeout) from origin.
    #[serde(rename = "status_504", skip_serializing_if = "Option::is_none")]
    pub status_504: Option<i32>,
    /// Number of responses received with status code 505 (HTTP Version Not Supported) from origin.
    #[serde(rename = "status_505", skip_serializing_if = "Option::is_none")]
    pub status_505: Option<i32>,
    /// Number of responses from origin with latency between 0 and 1 millisecond.
    #[serde(rename = "latency_0_to_1ms", skip_serializing_if = "Option::is_none")]
    pub latency_0_to_1ms: Option<i32>,
    /// Number of responses from origin with latency between 1 and 5 milliseconds.
    #[serde(rename = "latency_1_to_5ms", skip_serializing_if = "Option::is_none")]
    pub latency_1_to_5ms: Option<i32>,
    /// Number of responses from origin with latency between 5 and 10 milliseconds.
    #[serde(rename = "latency_5_to_10ms", skip_serializing_if = "Option::is_none")]
    pub latency_5_to_10ms: Option<i32>,
    /// Number of responses from origin with latency between 10 and 50 milliseconds.
    #[serde(rename = "latency_10_to_50ms", skip_serializing_if = "Option::is_none")]
    pub latency_10_to_50ms: Option<i32>,
    /// Number of responses from origin with latency between 50 and 100 milliseconds.
    #[serde(rename = "latency_50_to_100ms", skip_serializing_if = "Option::is_none")]
    pub latency_50_to_100ms: Option<i32>,
    /// Number of responses from origin with latency between 100 and 250 milliseconds.
    #[serde(rename = "latency_100_to_250ms", skip_serializing_if = "Option::is_none")]
    pub latency_100_to_250ms: Option<i32>,
    /// Number of responses from origin with latency between 250 and 500 milliseconds.
    #[serde(rename = "latency_250_to_500ms", skip_serializing_if = "Option::is_none")]
    pub latency_250_to_500ms: Option<i32>,
    /// Number of responses from origin with latency between 500 and 1,000 milliseconds.
    #[serde(rename = "latency_500_to_1000ms", skip_serializing_if = "Option::is_none")]
    pub latency_500_to_1000ms: Option<i32>,
    /// Number of responses from origin with latency between 1,000 and 5,000 milliseconds.
    #[serde(rename = "latency_1000_to_5000ms", skip_serializing_if = "Option::is_none")]
    pub latency_1000_to_5000ms: Option<i32>,
    /// Number of responses from origin with latency between 5,000 and 10,000 milliseconds.
    #[serde(rename = "latency_5000_to_10000ms", skip_serializing_if = "Option::is_none")]
    pub latency_5000_to_10000ms: Option<i32>,
    /// Number of responses from origin with latency between 10,000 and 60,000 milliseconds.
    #[serde(rename = "latency_10000_to_60000ms", skip_serializing_if = "Option::is_none")]
    pub latency_10000_to_60000ms: Option<i32>,
    /// Number of responses from origin with latency of 60,000 milliseconds and above.
    #[serde(rename = "latency_60000ms", skip_serializing_if = "Option::is_none")]
    pub latency_60000ms: Option<i32>,
    /// Number of responses received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_responses", skip_serializing_if = "Option::is_none")]
    pub waf_responses: Option<i32>,
    /// Number of header bytes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub waf_resp_header_bytes: Option<i32>,
    /// Number of body bytes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub waf_resp_body_bytes: Option<i32>,
    /// Number of 1xx \"Informational\" status codes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_1xx", skip_serializing_if = "Option::is_none")]
    pub waf_status_1xx: Option<i32>,
    /// Number of 2xx \"Success\" status codes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_2xx", skip_serializing_if = "Option::is_none")]
    pub waf_status_2xx: Option<i32>,
    /// Number of 3xx \"Redirection\" codes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_3xx", skip_serializing_if = "Option::is_none")]
    pub waf_status_3xx: Option<i32>,
    /// Number of 4xx \"Client Error\" codes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_4xx", skip_serializing_if = "Option::is_none")]
    pub waf_status_4xx: Option<i32>,
    /// Number of 5xx \"Server Error\" codes received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_5xx", skip_serializing_if = "Option::is_none")]
    pub waf_status_5xx: Option<i32>,
    /// Number of responses received with status code 200 (Success) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_200", skip_serializing_if = "Option::is_none")]
    pub waf_status_200: Option<i32>,
    /// Number of responses received with status code 204 (No Content) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_204", skip_serializing_if = "Option::is_none")]
    pub waf_status_204: Option<i32>,
    /// Number of responses received with status code 206 (Partial Content) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_206", skip_serializing_if = "Option::is_none")]
    pub waf_status_206: Option<i32>,
    /// Number of responses received with status code 301 (Moved Permanently) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_301", skip_serializing_if = "Option::is_none")]
    pub waf_status_301: Option<i32>,
    /// Number of responses received with status code 302 (Found) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_302", skip_serializing_if = "Option::is_none")]
    pub waf_status_302: Option<i32>,
    /// Number of responses received with status code 304 (Not Modified) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_304", skip_serializing_if = "Option::is_none")]
    pub waf_status_304: Option<i32>,
    /// Number of responses received with status code 400 (Bad Request) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_400", skip_serializing_if = "Option::is_none")]
    pub waf_status_400: Option<i32>,
    /// Number of responses received with status code 401 (Unauthorized) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_401", skip_serializing_if = "Option::is_none")]
    pub waf_status_401: Option<i32>,
    /// Number of responses received with status code 403 (Forbidden) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_403", skip_serializing_if = "Option::is_none")]
    pub waf_status_403: Option<i32>,
    /// Number of responses received with status code 404 (Not Found) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_404", skip_serializing_if = "Option::is_none")]
    pub waf_status_404: Option<i32>,
    /// Number of responses received with status code 416 (Range Not Satisfiable) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_416", skip_serializing_if = "Option::is_none")]
    pub waf_status_416: Option<i32>,
    /// Number of responses received with status code 429 (Too Many Requests) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_429", skip_serializing_if = "Option::is_none")]
    pub waf_status_429: Option<i32>,
    /// Number of responses received with status code 500 (Internal Server Error) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_500", skip_serializing_if = "Option::is_none")]
    pub waf_status_500: Option<i32>,
    /// Number of responses received with status code 501 (Not Implemented) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_501", skip_serializing_if = "Option::is_none")]
    pub waf_status_501: Option<i32>,
    /// Number of responses received with status code 502 (Bad Gateway) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_502", skip_serializing_if = "Option::is_none")]
    pub waf_status_502: Option<i32>,
    /// Number of responses received with status code 503 (Service Unavailable) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_503", skip_serializing_if = "Option::is_none")]
    pub waf_status_503: Option<i32>,
    /// Number of responses received with status code 504 (Gateway Timeout) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_504", skip_serializing_if = "Option::is_none")]
    pub waf_status_504: Option<i32>,
    /// Number of responses received with status code 505 (HTTP Version Not Supported) received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_status_505", skip_serializing_if = "Option::is_none")]
    pub waf_status_505: Option<i32>,
    /// Number of responses with latency between 0 and 1 millisecond received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_0_to_1ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_0_to_1ms: Option<i32>,
    /// Number of responses with latency between 1 and 5 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_1_to_5ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_1_to_5ms: Option<i32>,
    /// Number of responses with latency between 5 and 10 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_5_to_10ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_5_to_10ms: Option<i32>,
    /// Number of responses with latency between 10 and 50 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_10_to_50ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_10_to_50ms: Option<i32>,
    /// Number of responses with latency between 50 and 100 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_50_to_100ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_50_to_100ms: Option<i32>,
    /// Number of responses with latency between 100 and 250 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_100_to_250ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_100_to_250ms: Option<i32>,
    /// Number of responses with latency between 250 and 500 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_250_to_500ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_250_to_500ms: Option<i32>,
    /// Number of responses with latency between 500 and 1,000 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_500_to_1000ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_500_to_1000ms: Option<i32>,
    /// Number of responses with latency between 1,000 and 5,000 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_1000_to_5000ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_1000_to_5000ms: Option<i32>,
    /// Number of responses with latency between 5,000 and 10,000 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_5000_to_10000ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_5000_to_10000ms: Option<i32>,
    /// Number of responses with latency between 10,000 and 60,000 milliseconds received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_10000_to_60000ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_10000_to_60000ms: Option<i32>,
    /// Number of responses with latency of 60,000 milliseconds and above received for origin requests made by the Fastly WAF.
    #[serde(rename = "waf_latency_60000ms", skip_serializing_if = "Option::is_none")]
    pub waf_latency_60000ms: Option<i32>,
    /// Number of responses for origin received by Compute@Edge.
    #[serde(rename = "compute_responses", skip_serializing_if = "Option::is_none")]
    pub compute_responses: Option<i32>,
    /// Number of header bytes for origin received by Compute@Edge.
    #[serde(rename = "compute_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_resp_header_bytes: Option<i32>,
    /// Number of body bytes for origin received by Compute@Edge.
    #[serde(rename = "compute_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_resp_body_bytes: Option<i32>,
    /// Number of 1xx \"Informational\" status codes for origin received by Compute@Edge.
    #[serde(rename = "compute_status_1xx", skip_serializing_if = "Option::is_none")]
    pub compute_status_1xx: Option<i32>,
    /// Number of 2xx \"Success\" status codes for origin received by Compute@Edge.
    #[serde(rename = "compute_status_2xx", skip_serializing_if = "Option::is_none")]
    pub compute_status_2xx: Option<i32>,
    /// Number of 3xx \"Redirection\" codes for origin received by Compute@Edge.
    #[serde(rename = "compute_status_3xx", skip_serializing_if = "Option::is_none")]
    pub compute_status_3xx: Option<i32>,
    /// Number of 4xx \"Client Error\" codes for origin received by Compute@Edge.
    #[serde(rename = "compute_status_4xx", skip_serializing_if = "Option::is_none")]
    pub compute_status_4xx: Option<i32>,
    /// Number of 5xx \"Server Error\" codes for origin received by Compute@Edge.
    #[serde(rename = "compute_status_5xx", skip_serializing_if = "Option::is_none")]
    pub compute_status_5xx: Option<i32>,
    /// Number of responses received with status code 200 (Success) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_200", skip_serializing_if = "Option::is_none")]
    pub compute_status_200: Option<i32>,
    /// Number of responses received with status code 204 (No Content) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_204", skip_serializing_if = "Option::is_none")]
    pub compute_status_204: Option<i32>,
    /// Number of responses received with status code 206 (Partial Content) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_206", skip_serializing_if = "Option::is_none")]
    pub compute_status_206: Option<i32>,
    /// Number of responses received with status code 301 (Moved Permanently) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_301", skip_serializing_if = "Option::is_none")]
    pub compute_status_301: Option<i32>,
    /// Number of responses received with status code 302 (Found) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_302", skip_serializing_if = "Option::is_none")]
    pub compute_status_302: Option<i32>,
    /// Number of responses received with status code 304 (Not Modified) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_304", skip_serializing_if = "Option::is_none")]
    pub compute_status_304: Option<i32>,
    /// Number of responses received with status code 400 (Bad Request) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_400", skip_serializing_if = "Option::is_none")]
    pub compute_status_400: Option<i32>,
    /// Number of responses received with status code 401 (Unauthorized) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_401", skip_serializing_if = "Option::is_none")]
    pub compute_status_401: Option<i32>,
    /// Number of responses received with status code 403 (Forbidden) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_403", skip_serializing_if = "Option::is_none")]
    pub compute_status_403: Option<i32>,
    /// Number of responses received with status code 404 (Not Found) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_404", skip_serializing_if = "Option::is_none")]
    pub compute_status_404: Option<i32>,
    /// Number of responses received with status code 416 (Range Not Satisfiable) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_416", skip_serializing_if = "Option::is_none")]
    pub compute_status_416: Option<i32>,
    /// Number of responses received with status code 429 (Too Many Requests) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_429", skip_serializing_if = "Option::is_none")]
    pub compute_status_429: Option<i32>,
    /// Number of responses received with status code 500 (Internal Server Error) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_500", skip_serializing_if = "Option::is_none")]
    pub compute_status_500: Option<i32>,
    /// Number of responses received with status code 501 (Not Implemented) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_501", skip_serializing_if = "Option::is_none")]
    pub compute_status_501: Option<i32>,
    /// Number of responses received with status code 502 (Bad Gateway) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_502", skip_serializing_if = "Option::is_none")]
    pub compute_status_502: Option<i32>,
    /// Number of responses received with status code 503 (Service Unavailable) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_503", skip_serializing_if = "Option::is_none")]
    pub compute_status_503: Option<i32>,
    /// Number of responses received with status code 504 (Gateway Timeout) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_504", skip_serializing_if = "Option::is_none")]
    pub compute_status_504: Option<i32>,
    /// Number of responses received with status code 505 (HTTP Version Not Supported) for origin received by Compute@Edge.
    #[serde(rename = "compute_status_505", skip_serializing_if = "Option::is_none")]
    pub compute_status_505: Option<i32>,
    /// Number of responses with latency between 0 and 1 millisecond for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_0_to_1ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_0_to_1ms: Option<i32>,
    /// Number of responses with latency between 1 and 5 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_1_to_5ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_1_to_5ms: Option<i32>,
    /// Number of responses with latency between 5 and 10 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_5_to_10ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_5_to_10ms: Option<i32>,
    /// Number of responses with latency between 10 and 50 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_10_to_50ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_10_to_50ms: Option<i32>,
    /// Number of responses with latency between 50 and 100 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_50_to_100ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_50_to_100ms: Option<i32>,
    /// Number of responses with latency between 100 and 250 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_100_to_250ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_100_to_250ms: Option<i32>,
    /// Number of responses with latency between 250 and 500 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_250_to_500ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_250_to_500ms: Option<i32>,
    /// Number of responses with latency between 500 and 1,000 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_500_to_1000ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_500_to_1000ms: Option<i32>,
    /// Number of responses with latency between 1,000 and 5,000 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_1000_to_5000ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_1000_to_5000ms: Option<i32>,
    /// Number of responses with latency between 5,000 and 10,000 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_5000_to_10000ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_5000_to_10000ms: Option<i32>,
    /// Number of responses with latency between 10,000 and 60,000 milliseconds for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_10000_to_60000ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_10000_to_60000ms: Option<i32>,
    /// Number of responses with latency of 60,000 milliseconds and above for origin received by Compute@Edge.
    #[serde(rename = "compute_latency_60000ms", skip_serializing_if = "Option::is_none")]
    pub compute_latency_60000ms: Option<i32>,
    /// Number of responses received for origin requests made by all sources.
    #[serde(rename = "all_responses", skip_serializing_if = "Option::is_none")]
    pub all_responses: Option<i32>,
    /// Number of header bytes received for origin requests made by all sources.
    #[serde(rename = "all_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub all_resp_header_bytes: Option<i32>,
    /// Number of body bytes received for origin requests made by all sources.
    #[serde(rename = "all_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub all_resp_body_bytes: Option<i32>,
    /// Number of 1xx \"Informational\" category status codes delivered received for origin requests made by all sources.
    #[serde(rename = "all_status_1xx", skip_serializing_if = "Option::is_none")]
    pub all_status_1xx: Option<i32>,
    /// Number of 2xx \"Success\" status codes received for origin requests made by all sources.
    #[serde(rename = "all_status_2xx", skip_serializing_if = "Option::is_none")]
    pub all_status_2xx: Option<i32>,
    /// Number of 3xx \"Redirection\" codes received for origin requests made by all sources.
    #[serde(rename = "all_status_3xx", skip_serializing_if = "Option::is_none")]
    pub all_status_3xx: Option<i32>,
    /// Number of 4xx \"Client Error\" codes received for origin requests made by all sources.
    #[serde(rename = "all_status_4xx", skip_serializing_if = "Option::is_none")]
    pub all_status_4xx: Option<i32>,
    /// Number of 5xx \"Server Error\" codes received for origin requests made by all sources.
    #[serde(rename = "all_status_5xx", skip_serializing_if = "Option::is_none")]
    pub all_status_5xx: Option<i32>,
    /// Number of responses received with status code 200 (Success) received for origin requests made by all sources.
    #[serde(rename = "all_status_200", skip_serializing_if = "Option::is_none")]
    pub all_status_200: Option<i32>,
    /// Number of responses received with status code 204 (No Content) received for origin requests made by all sources.
    #[serde(rename = "all_status_204", skip_serializing_if = "Option::is_none")]
    pub all_status_204: Option<i32>,
    /// Number of responses received with status code 206 (Partial Content) received for origin requests made by all sources.
    #[serde(rename = "all_status_206", skip_serializing_if = "Option::is_none")]
    pub all_status_206: Option<i32>,
    /// Number of responses received with status code 301 (Moved Permanently) received for origin requests made by all sources.
    #[serde(rename = "all_status_301", skip_serializing_if = "Option::is_none")]
    pub all_status_301: Option<i32>,
    /// Number of responses received with status code 302 (Found) received for origin requests made by all sources.
    #[serde(rename = "all_status_302", skip_serializing_if = "Option::is_none")]
    pub all_status_302: Option<i32>,
    /// Number of responses received with status code 304 (Not Modified) received for origin requests made by all sources.
    #[serde(rename = "all_status_304", skip_serializing_if = "Option::is_none")]
    pub all_status_304: Option<i32>,
    /// Number of responses received with status code 400 (Bad Request) received for origin requests made by all sources.
    #[serde(rename = "all_status_400", skip_serializing_if = "Option::is_none")]
    pub all_status_400: Option<i32>,
    /// Number of responses received with status code 401 (Unauthorized) received for origin requests made by all sources.
    #[serde(rename = "all_status_401", skip_serializing_if = "Option::is_none")]
    pub all_status_401: Option<i32>,
    /// Number of responses received with status code 403 (Forbidden) received for origin requests made by all sources.
    #[serde(rename = "all_status_403", skip_serializing_if = "Option::is_none")]
    pub all_status_403: Option<i32>,
    /// Number of responses received with status code 404 (Not Found) received for origin requests made by all sources.
    #[serde(rename = "all_status_404", skip_serializing_if = "Option::is_none")]
    pub all_status_404: Option<i32>,
    /// Number of responses received with status code 416 (Range Not Satisfiable) received for origin requests made by all sources.
    #[serde(rename = "all_status_416", skip_serializing_if = "Option::is_none")]
    pub all_status_416: Option<i32>,
    /// Number of responses received with status code 429 (Too Many Requests) received for origin requests made by all sources.
    #[serde(rename = "all_status_429", skip_serializing_if = "Option::is_none")]
    pub all_status_429: Option<i32>,
    /// Number of responses received with status code 500 (Internal Server Error) received for origin requests made by all sources.
    #[serde(rename = "all_status_500", skip_serializing_if = "Option::is_none")]
    pub all_status_500: Option<i32>,
    /// Number of responses received with status code 501 (Not Implemented) received for origin requests made by all sources.
    #[serde(rename = "all_status_501", skip_serializing_if = "Option::is_none")]
    pub all_status_501: Option<i32>,
    /// Number of responses received with status code 502 (Bad Gateway) received for origin requests made by all sources.
    #[serde(rename = "all_status_502", skip_serializing_if = "Option::is_none")]
    pub all_status_502: Option<i32>,
    /// Number of responses received with status code 503 (Service Unavailable) received for origin requests made by all sources.
    #[serde(rename = "all_status_503", skip_serializing_if = "Option::is_none")]
    pub all_status_503: Option<i32>,
    /// Number of responses received with status code 504 (Gateway Timeout) received for origin requests made by all sources.
    #[serde(rename = "all_status_504", skip_serializing_if = "Option::is_none")]
    pub all_status_504: Option<i32>,
    /// Number of responses received with status code 505 (HTTP Version Not Supported) received for origin requests made by all sources.
    #[serde(rename = "all_status_505", skip_serializing_if = "Option::is_none")]
    pub all_status_505: Option<i32>,
    /// Number of responses with latency between 0 and 1 millisecond received for origin requests made by all sources.
    #[serde(rename = "all_latency_0_to_1ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_0_to_1ms: Option<i32>,
    /// Number of responses with latency between 1 and 5 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_1_to_5ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_1_to_5ms: Option<i32>,
    /// Number of responses with latency between 5 and 10 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_5_to_10ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_5_to_10ms: Option<i32>,
    /// Number of responses with latency between 10 and 50 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_10_to_50ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_10_to_50ms: Option<i32>,
    /// Number of responses with latency between 50 and 100 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_50_to_100ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_50_to_100ms: Option<i32>,
    /// Number of responses with latency between 100 and 250 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_100_to_250ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_100_to_250ms: Option<i32>,
    /// Number of responses with latency between 250 and 500 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_250_to_500ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_250_to_500ms: Option<i32>,
    /// Number of responses with latency between 500 and 1,000 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_500_to_1000ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_500_to_1000ms: Option<i32>,
    /// Number of responses with latency between 1,000 and 5,000 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_1000_to_5000ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_1000_to_5000ms: Option<i32>,
    /// Number of responses with latency between 5,000 and 10,000 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_5000_to_10000ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_5000_to_10000ms: Option<i32>,
    /// Number of responses with latency between 10,000 and 60,000 milliseconds received for origin requests made by all sources.
    #[serde(rename = "all_latency_10000_to_60000ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_10000_to_60000ms: Option<i32>,
    /// Number of responses with latency of 60,000 milliseconds and above received for origin requests made by all sources.
    #[serde(rename = "all_latency_60000ms", skip_serializing_if = "Option::is_none")]
    pub all_latency_60000ms: Option<i32>,
}

impl OriginInspectorMeasurements {
    /// A measurements object provides a count of the total number of `responses` received by Fastly from your origin servers in the reported time period, for the relevant POP and backend name as specified in the [entry](#entry-data-model). It also includes the number of responses for specific HTTP response status codes and for status code ranges. This dataset is sparse: only the keys with non-zero values will be included in the record. Where a specific status code does not have a field in this model (e.g., `429 Too Many Requests`), any responses with that code will be counted as part of the range count (`4xx` in this case) but will not be separately identified in the data. 
    pub fn new() -> OriginInspectorMeasurements {
        OriginInspectorMeasurements {
            responses: None,
            resp_header_bytes: None,
            resp_body_bytes: None,
            status_1xx: None,
            status_2xx: None,
            status_3xx: None,
            status_4xx: None,
            status_5xx: None,
            status_200: None,
            status_204: None,
            status_206: None,
            status_301: None,
            status_302: None,
            status_304: None,
            status_400: None,
            status_401: None,
            status_403: None,
            status_404: None,
            status_416: None,
            status_429: None,
            status_500: None,
            status_501: None,
            status_502: None,
            status_503: None,
            status_504: None,
            status_505: None,
            latency_0_to_1ms: None,
            latency_1_to_5ms: None,
            latency_5_to_10ms: None,
            latency_10_to_50ms: None,
            latency_50_to_100ms: None,
            latency_100_to_250ms: None,
            latency_250_to_500ms: None,
            latency_500_to_1000ms: None,
            latency_1000_to_5000ms: None,
            latency_5000_to_10000ms: None,
            latency_10000_to_60000ms: None,
            latency_60000ms: None,
            waf_responses: None,
            waf_resp_header_bytes: None,
            waf_resp_body_bytes: None,
            waf_status_1xx: None,
            waf_status_2xx: None,
            waf_status_3xx: None,
            waf_status_4xx: None,
            waf_status_5xx: None,
            waf_status_200: None,
            waf_status_204: None,
            waf_status_206: None,
            waf_status_301: None,
            waf_status_302: None,
            waf_status_304: None,
            waf_status_400: None,
            waf_status_401: None,
            waf_status_403: None,
            waf_status_404: None,
            waf_status_416: None,
            waf_status_429: None,
            waf_status_500: None,
            waf_status_501: None,
            waf_status_502: None,
            waf_status_503: None,
            waf_status_504: None,
            waf_status_505: None,
            waf_latency_0_to_1ms: None,
            waf_latency_1_to_5ms: None,
            waf_latency_5_to_10ms: None,
            waf_latency_10_to_50ms: None,
            waf_latency_50_to_100ms: None,
            waf_latency_100_to_250ms: None,
            waf_latency_250_to_500ms: None,
            waf_latency_500_to_1000ms: None,
            waf_latency_1000_to_5000ms: None,
            waf_latency_5000_to_10000ms: None,
            waf_latency_10000_to_60000ms: None,
            waf_latency_60000ms: None,
            compute_responses: None,
            compute_resp_header_bytes: None,
            compute_resp_body_bytes: None,
            compute_status_1xx: None,
            compute_status_2xx: None,
            compute_status_3xx: None,
            compute_status_4xx: None,
            compute_status_5xx: None,
            compute_status_200: None,
            compute_status_204: None,
            compute_status_206: None,
            compute_status_301: None,
            compute_status_302: None,
            compute_status_304: None,
            compute_status_400: None,
            compute_status_401: None,
            compute_status_403: None,
            compute_status_404: None,
            compute_status_416: None,
            compute_status_429: None,
            compute_status_500: None,
            compute_status_501: None,
            compute_status_502: None,
            compute_status_503: None,
            compute_status_504: None,
            compute_status_505: None,
            compute_latency_0_to_1ms: None,
            compute_latency_1_to_5ms: None,
            compute_latency_5_to_10ms: None,
            compute_latency_10_to_50ms: None,
            compute_latency_50_to_100ms: None,
            compute_latency_100_to_250ms: None,
            compute_latency_250_to_500ms: None,
            compute_latency_500_to_1000ms: None,
            compute_latency_1000_to_5000ms: None,
            compute_latency_5000_to_10000ms: None,
            compute_latency_10000_to_60000ms: None,
            compute_latency_60000ms: None,
            all_responses: None,
            all_resp_header_bytes: None,
            all_resp_body_bytes: None,
            all_status_1xx: None,
            all_status_2xx: None,
            all_status_3xx: None,
            all_status_4xx: None,
            all_status_5xx: None,
            all_status_200: None,
            all_status_204: None,
            all_status_206: None,
            all_status_301: None,
            all_status_302: None,
            all_status_304: None,
            all_status_400: None,
            all_status_401: None,
            all_status_403: None,
            all_status_404: None,
            all_status_416: None,
            all_status_429: None,
            all_status_500: None,
            all_status_501: None,
            all_status_502: None,
            all_status_503: None,
            all_status_504: None,
            all_status_505: None,
            all_latency_0_to_1ms: None,
            all_latency_1_to_5ms: None,
            all_latency_5_to_10ms: None,
            all_latency_10_to_50ms: None,
            all_latency_50_to_100ms: None,
            all_latency_100_to_250ms: None,
            all_latency_250_to_500ms: None,
            all_latency_500_to_1000ms: None,
            all_latency_1000_to_5000ms: None,
            all_latency_5000_to_10000ms: None,
            all_latency_10000_to_60000ms: None,
            all_latency_60000ms: None,
        }
    }
}


