/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://developer.fastly.com/reference/api/) 
 *
 */

/// DomainInspectorMeasurements : A measurements object provides a count of the total number of `requests` received by Fastly for your domain in the reported time period and for the relevant POP, as specified in the [entry](#entry-data-model). It also includes the number of responses for specific HTTP response status codes and for status code ranges. This dataset is sparse: only the keys with non-zero values will be included in the record. Where a specific status code does not have a field in this model (e.g., `429 Too Many Requests`), any responses with that code will be counted as part of the range count (`4xx` in this case) but will not be separately identified in the data. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainInspectorMeasurements {
    /// Number of requests sent by end users to Fastly.
    #[serde(rename = "edge_requests", skip_serializing_if = "Option::is_none")]
    pub edge_requests: Option<i32>,
    /// Total header bytes delivered from Fastly to the end user.
    #[serde(rename = "edge_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_resp_header_bytes: Option<i32>,
    /// Total body bytes delivered from Fastly to the end user.
    #[serde(rename = "edge_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_resp_body_bytes: Option<i32>,
    /// Number of 1xx \"Informational\" category status codes delivered.
    #[serde(rename = "status_1xx", skip_serializing_if = "Option::is_none")]
    pub status_1xx: Option<i32>,
    /// Number of 2xx \"Success\" status codes delivered.
    #[serde(rename = "status_2xx", skip_serializing_if = "Option::is_none")]
    pub status_2xx: Option<i32>,
    /// Number of 3xx \"Redirection\" codes delivered.
    #[serde(rename = "status_3xx", skip_serializing_if = "Option::is_none")]
    pub status_3xx: Option<i32>,
    /// Number of 4xx \"Client Error\" codes delivered.
    #[serde(rename = "status_4xx", skip_serializing_if = "Option::is_none")]
    pub status_4xx: Option<i32>,
    /// Number of 5xx \"Server Error\" codes delivered.
    #[serde(rename = "status_5xx", skip_serializing_if = "Option::is_none")]
    pub status_5xx: Option<i32>,
    /// Number of responses delivered with status code 200 (Success).
    #[serde(rename = "status_200", skip_serializing_if = "Option::is_none")]
    pub status_200: Option<i32>,
    /// Number of responses delivered with status code 204 (No Content).
    #[serde(rename = "status_204", skip_serializing_if = "Option::is_none")]
    pub status_204: Option<i32>,
    /// Number of responses delivered with status code 206 (Partial Content).
    #[serde(rename = "status_206", skip_serializing_if = "Option::is_none")]
    pub status_206: Option<i32>,
    /// Number of responses delivered with status code 301 (Moved Permanently).
    #[serde(rename = "status_301", skip_serializing_if = "Option::is_none")]
    pub status_301: Option<i32>,
    /// Number of responses delivered with status code 302 (Found).
    #[serde(rename = "status_302", skip_serializing_if = "Option::is_none")]
    pub status_302: Option<i32>,
    /// Number of responses delivered with status code 304 (Not Modified).
    #[serde(rename = "status_304", skip_serializing_if = "Option::is_none")]
    pub status_304: Option<i32>,
    /// Number of responses delivered with status code 400 (Bad Request).
    #[serde(rename = "status_400", skip_serializing_if = "Option::is_none")]
    pub status_400: Option<i32>,
    /// Number of responses delivered with status code 401 (Unauthorized).
    #[serde(rename = "status_401", skip_serializing_if = "Option::is_none")]
    pub status_401: Option<i32>,
    /// Number of responses delivered with status code 403 (Forbidden).
    #[serde(rename = "status_403", skip_serializing_if = "Option::is_none")]
    pub status_403: Option<i32>,
    /// Number of responses delivered with status code 404 (Not Found).
    #[serde(rename = "status_404", skip_serializing_if = "Option::is_none")]
    pub status_404: Option<i32>,
    /// Number of responses delivered with status code 416 (Range Not Satisfiable).
    #[serde(rename = "status_416", skip_serializing_if = "Option::is_none")]
    pub status_416: Option<i32>,
    /// Number of responses delivered with status code 429 (Too Many Requests).
    #[serde(rename = "status_429", skip_serializing_if = "Option::is_none")]
    pub status_429: Option<i32>,
    /// Number of responses delivered with status code 500 (Internal Server Error).
    #[serde(rename = "status_500", skip_serializing_if = "Option::is_none")]
    pub status_500: Option<i32>,
    /// Number of responses delivered with status code 501 (Not Implemented).
    #[serde(rename = "status_501", skip_serializing_if = "Option::is_none")]
    pub status_501: Option<i32>,
    /// Number of responses delivered with status code 502 (Bad Gateway).
    #[serde(rename = "status_502", skip_serializing_if = "Option::is_none")]
    pub status_502: Option<i32>,
    /// Number of responses delivered with status code 503 (Service Unavailable).
    #[serde(rename = "status_503", skip_serializing_if = "Option::is_none")]
    pub status_503: Option<i32>,
    /// Number of responses delivered with status code 504 (Gateway Timeout).
    #[serde(rename = "status_504", skip_serializing_if = "Option::is_none")]
    pub status_504: Option<i32>,
    /// Number of responses delivered with status code 505 (HTTP Version Not Supported).
    #[serde(rename = "status_505", skip_serializing_if = "Option::is_none")]
    pub status_505: Option<i32>,
    /// Number of requests processed.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<i32>,
    /// Total header bytes delivered.
    #[serde(rename = "resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_header_bytes: Option<i32>,
    /// Total body bytes delivered.
    #[serde(rename = "resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_body_bytes: Option<i32>,
    /// Total header bytes sent to origin.
    #[serde(rename = "bereq_header_bytes", skip_serializing_if = "Option::is_none")]
    pub bereq_header_bytes: Option<i32>,
    /// Total body bytes sent to origin.
    #[serde(rename = "bereq_body_bytes", skip_serializing_if = "Option::is_none")]
    pub bereq_body_bytes: Option<i32>,
    /// Number of requests sent by end users to Fastly that resulted in a hit at the edge.
    #[serde(rename = "edge_hit_requests", skip_serializing_if = "Option::is_none")]
    pub edge_hit_requests: Option<i32>,
    /// Number of requests sent by end users to Fastly that resulted in a miss at the edge.
    #[serde(rename = "edge_miss_requests", skip_serializing_if = "Option::is_none")]
    pub edge_miss_requests: Option<i32>,
    /// Number of requests sent to origin.
    #[serde(rename = "origin_fetches", skip_serializing_if = "Option::is_none")]
    pub origin_fetches: Option<i32>,
    /// Total header bytes received from origin.
    #[serde(rename = "origin_fetch_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_resp_header_bytes: Option<i32>,
    /// Total body bytes received from origin.
    #[serde(rename = "origin_fetch_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_resp_body_bytes: Option<i32>,
    /// Total bytes delivered (`resp_header_bytes` + `resp_body_bytes` + `bereq_header_bytes` + `bereq_body_bytes`).
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// Ratio of cache hits to cache misses at the edge, between 0 and 1 (`edge_hit_requests` / (`edge_hit_requests` + `edge_miss_requests`)).
    #[serde(rename = "edge_hit_ratio", skip_serializing_if = "Option::is_none")]
    pub edge_hit_ratio: Option<f32>,
    /// Ratio of response bytes delivered from the edge compared to what is delivered from origin, between 0 and 1. (`edge_resp_body_bytes` + `edge_resp_header_bytes`) / (`origin_fetch_resp_body_bytes` + `origin_fetch_resp_header_bytes` + `edge_resp_body_bytes` + `edge_resp_header_bytes`).
    #[serde(rename = "origin_offload", skip_serializing_if = "Option::is_none")]
    pub origin_offload: Option<f32>,
    /// Number of responses received from origin with status code 200 (Success).
    #[serde(rename = "origin_status_200", skip_serializing_if = "Option::is_none")]
    pub origin_status_200: Option<i32>,
    /// Number of responses received from origin with status code 204 (No Content).
    #[serde(rename = "origin_status_204", skip_serializing_if = "Option::is_none")]
    pub origin_status_204: Option<i32>,
    /// Number of responses received from origin with status code 206 (Partial Content).
    #[serde(rename = "origin_status_206", skip_serializing_if = "Option::is_none")]
    pub origin_status_206: Option<i32>,
    /// Number of responses received from origin with status code 301 (Moved Permanently).
    #[serde(rename = "origin_status_301", skip_serializing_if = "Option::is_none")]
    pub origin_status_301: Option<i32>,
    /// Number of responses received from origin with status code 302 (Found).
    #[serde(rename = "origin_status_302", skip_serializing_if = "Option::is_none")]
    pub origin_status_302: Option<i32>,
    /// Number of responses received from origin with status code 304 (Not Modified).
    #[serde(rename = "origin_status_304", skip_serializing_if = "Option::is_none")]
    pub origin_status_304: Option<i32>,
    /// Number of responses received from origin with status code 400 (Bad Request).
    #[serde(rename = "origin_status_400", skip_serializing_if = "Option::is_none")]
    pub origin_status_400: Option<i32>,
    /// Number of responses received from origin with status code 401 (Unauthorized).
    #[serde(rename = "origin_status_401", skip_serializing_if = "Option::is_none")]
    pub origin_status_401: Option<i32>,
    /// Number of responses received from origin with status code 403 (Forbidden).
    #[serde(rename = "origin_status_403", skip_serializing_if = "Option::is_none")]
    pub origin_status_403: Option<i32>,
    /// Number of responses received from origin with status code 404 (Not Found).
    #[serde(rename = "origin_status_404", skip_serializing_if = "Option::is_none")]
    pub origin_status_404: Option<i32>,
    /// Number of responses received from origin with status code 416 (Range Not Satisfiable).
    #[serde(rename = "origin_status_416", skip_serializing_if = "Option::is_none")]
    pub origin_status_416: Option<i32>,
    /// Number of responses received from origin with status code 429 (Too Many Requests).
    #[serde(rename = "origin_status_429", skip_serializing_if = "Option::is_none")]
    pub origin_status_429: Option<i32>,
    /// Number of responses received from origin with status code 500 (Internal Server Error).
    #[serde(rename = "origin_status_500", skip_serializing_if = "Option::is_none")]
    pub origin_status_500: Option<i32>,
    /// Number of responses received from origin with status code 501 (Not Implemented).
    #[serde(rename = "origin_status_501", skip_serializing_if = "Option::is_none")]
    pub origin_status_501: Option<i32>,
    /// Number of responses received from origin with status code 502 (Bad Gateway).
    #[serde(rename = "origin_status_502", skip_serializing_if = "Option::is_none")]
    pub origin_status_502: Option<i32>,
    /// Number of responses received from origin with status code 503 (Service Unavailable).
    #[serde(rename = "origin_status_503", skip_serializing_if = "Option::is_none")]
    pub origin_status_503: Option<i32>,
    /// Number of responses received from origin with status code 504 (Gateway Timeout).
    #[serde(rename = "origin_status_504", skip_serializing_if = "Option::is_none")]
    pub origin_status_504: Option<i32>,
    /// Number of responses received from origin with status code 505 (HTTP Version Not Supported).
    #[serde(rename = "origin_status_505", skip_serializing_if = "Option::is_none")]
    pub origin_status_505: Option<i32>,
    /// Number of \"Informational\" category status codes received from origin.
    #[serde(rename = "origin_status_1xx", skip_serializing_if = "Option::is_none")]
    pub origin_status_1xx: Option<i32>,
    /// Number of \"Success\" status codes received from origin.
    #[serde(rename = "origin_status_2xx", skip_serializing_if = "Option::is_none")]
    pub origin_status_2xx: Option<i32>,
    /// Number of \"Redirection\" codes received from origin.
    #[serde(rename = "origin_status_3xx", skip_serializing_if = "Option::is_none")]
    pub origin_status_3xx: Option<i32>,
    /// Number of \"Client Error\" codes received from origin.
    #[serde(rename = "origin_status_4xx", skip_serializing_if = "Option::is_none")]
    pub origin_status_4xx: Option<i32>,
    /// Number of \"Server Error\" codes received from origin.
    #[serde(rename = "origin_status_5xx", skip_serializing_if = "Option::is_none")]
    pub origin_status_5xx: Option<i32>,
}

impl DomainInspectorMeasurements {
    /// A measurements object provides a count of the total number of `requests` received by Fastly for your domain in the reported time period and for the relevant POP, as specified in the [entry](#entry-data-model). It also includes the number of responses for specific HTTP response status codes and for status code ranges. This dataset is sparse: only the keys with non-zero values will be included in the record. Where a specific status code does not have a field in this model (e.g., `429 Too Many Requests`), any responses with that code will be counted as part of the range count (`4xx` in this case) but will not be separately identified in the data. 
    pub fn new() -> DomainInspectorMeasurements {
        DomainInspectorMeasurements {
            edge_requests: None,
            edge_resp_header_bytes: None,
            edge_resp_body_bytes: None,
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
            requests: None,
            resp_header_bytes: None,
            resp_body_bytes: None,
            bereq_header_bytes: None,
            bereq_body_bytes: None,
            edge_hit_requests: None,
            edge_miss_requests: None,
            origin_fetches: None,
            origin_fetch_resp_header_bytes: None,
            origin_fetch_resp_body_bytes: None,
            bandwidth: None,
            edge_hit_ratio: None,
            origin_offload: None,
            origin_status_200: None,
            origin_status_204: None,
            origin_status_206: None,
            origin_status_301: None,
            origin_status_302: None,
            origin_status_304: None,
            origin_status_400: None,
            origin_status_401: None,
            origin_status_403: None,
            origin_status_404: None,
            origin_status_416: None,
            origin_status_429: None,
            origin_status_500: None,
            origin_status_501: None,
            origin_status_502: None,
            origin_status_503: None,
            origin_status_504: None,
            origin_status_505: None,
            origin_status_1xx: None,
            origin_status_2xx: None,
            origin_status_3xx: None,
            origin_status_4xx: None,
            origin_status_5xx: None,
        }
    }
}


