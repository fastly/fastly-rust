/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */

/// RealtimeEntryAggregated : Aggregates [measurements](#measurements-data-model) across all Fastly POPs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RealtimeEntryAggregated {
    /// Number of requests processed.
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    /// Number of log lines sent (alias for `log`).
    #[serde(rename = "logging", skip_serializing_if = "Option::is_none")]
    pub logging: Option<i64>,
    /// Number of log lines sent.
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<i64>,
    /// Total header bytes delivered (edge_resp_header_bytes + shield_resp_header_bytes).
    #[serde(rename = "resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_header_bytes: Option<i64>,
    /// Total header bytes delivered (alias for resp_header_bytes).
    #[serde(rename = "header_size", skip_serializing_if = "Option::is_none")]
    pub header_size: Option<i64>,
    /// Total body bytes delivered (edge_resp_body_bytes + shield_resp_body_bytes).
    #[serde(rename = "resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub resp_body_bytes: Option<i64>,
    /// Total body bytes delivered (alias for resp_body_bytes).
    #[serde(rename = "body_size", skip_serializing_if = "Option::is_none")]
    pub body_size: Option<i64>,
    /// Number of cache hits.
    #[serde(rename = "hits", skip_serializing_if = "Option::is_none")]
    pub hits: Option<i64>,
    /// Number of cache misses.
    #[serde(rename = "miss", skip_serializing_if = "Option::is_none")]
    pub miss: Option<i64>,
    /// Number of requests that passed through the CDN without being cached.
    #[serde(rename = "pass", skip_serializing_if = "Option::is_none")]
    pub pass: Option<i64>,
    /// Number of requests that returned a synthetic response (i.e., response objects created with the `synthetic` VCL statement).
    #[serde(rename = "synth", skip_serializing_if = "Option::is_none")]
    pub synth: Option<i64>,
    /// Number of cache errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<i64>,
    /// Total amount of time spent processing cache hits (in seconds).
    #[serde(rename = "hits_time", skip_serializing_if = "Option::is_none")]
    pub hits_time: Option<f32>,
    /// Total amount of time spent processing cache misses (in seconds).
    #[serde(rename = "miss_time", skip_serializing_if = "Option::is_none")]
    pub miss_time: Option<f32>,
    /// A histogram. The value in each bucket is the number of requests to the origin whose responses arrived during the time period represented by the bucket. The key of each bucket represents the upper bound (in response time) of that bucket. The buckets vary in width and cover the time periods 0-10ms (in 1ms increments), 10-250ms (in 10ms increments), 250-1,000ms (in 50ms increments), 1,000-3,000ms (in 100ms increments), 3,000-10,000ms (in 500 ms increments), 10,000-20,000ms (in 1,000ms increments), 20,000-60,000ms (in 5,000ms increments), and 60,000ms through infinity (in a single bucket).
    #[serde(rename = "miss_histogram", skip_serializing_if = "Option::is_none")]
    pub miss_histogram: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The total number of requests that were received for your service by Fastly.
    #[serde(rename = "compute_requests", skip_serializing_if = "Option::is_none")]
    pub compute_requests: Option<i64>,
    /// The amount of active CPU time used to process your requests (in milliseconds).
    #[serde(rename = "compute_execution_time_ms", skip_serializing_if = "Option::is_none")]
    pub compute_execution_time_ms: Option<f32>,
    /// The amount of RAM used for your service by Fastly (in bytes).
    #[serde(rename = "compute_ram_used", skip_serializing_if = "Option::is_none")]
    pub compute_ram_used: Option<i64>,
    /// The total, actual amount of time used to process your requests, including active CPU time (in milliseconds).
    #[serde(rename = "compute_request_time_ms", skip_serializing_if = "Option::is_none")]
    pub compute_request_time_ms: Option<f32>,
    /// The total amount of request processing time you will be billed for, measured in 50 millisecond increments.
    #[serde(rename = "compute_request_time_billed_ms", skip_serializing_if = "Option::is_none")]
    pub compute_request_time_billed_ms: Option<f32>,
    /// Number of requests from edge to the shield POP.
    #[serde(rename = "shield", skip_serializing_if = "Option::is_none")]
    pub shield: Option<i64>,
    /// Number of requests that were received over IPv6.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<i64>,
    /// Number of responses that came from the Fastly Image Optimizer service. If the service receives 10 requests for an image, this stat will be 10 regardless of how many times the image was transformed.
    #[serde(rename = "imgopto", skip_serializing_if = "Option::is_none")]
    pub imgopto: Option<i64>,
    /// Number of responses that came from the Fastly Image Optimizer service via a shield.
    #[serde(rename = "imgopto_shield", skip_serializing_if = "Option::is_none")]
    pub imgopto_shield: Option<i64>,
    /// Number of transforms performed by the Fastly Image Optimizer service.
    #[serde(rename = "imgopto_transforms", skip_serializing_if = "Option::is_none")]
    pub imgopto_transforms: Option<i64>,
    /// Number of responses that came from the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp", skip_serializing_if = "Option::is_none")]
    pub otfp: Option<i64>,
    /// Number of responses that came from the Fastly On-the-Fly Packaging service for video-on-demand via a shield.
    #[serde(rename = "otfp_shield", skip_serializing_if = "Option::is_none")]
    pub otfp_shield: Option<i64>,
    /// Number of responses that were manifest files from the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp_manifests", skip_serializing_if = "Option::is_none")]
    pub otfp_manifests: Option<i64>,
    /// Number of responses with the video segment or video manifest MIME type (i.e., application/x-mpegurl, application/vnd.apple.mpegurl, application/f4m, application/dash+xml, application/vnd.ms-sstr+xml, ideo/mp2t, audio/aac, video/f4f, video/x-flv, video/mp4, audio/mp4).
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<i64>,
    /// Number of responses with the PCI flag turned on.
    #[serde(rename = "pci", skip_serializing_if = "Option::is_none")]
    pub pci: Option<i64>,
    /// Number of requests received over HTTP/2.
    #[serde(rename = "http2", skip_serializing_if = "Option::is_none")]
    pub http2: Option<i64>,
    /// Number of requests received over HTTP/3.
    #[serde(rename = "http3", skip_serializing_if = "Option::is_none")]
    pub http3: Option<i64>,
    /// Number of restarts performed.
    #[serde(rename = "restarts", skip_serializing_if = "Option::is_none")]
    pub restarts: Option<i64>,
    /// Total header bytes received.
    #[serde(rename = "req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub req_header_bytes: Option<i64>,
    /// Total body bytes received.
    #[serde(rename = "req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub req_body_bytes: Option<i64>,
    /// Total header bytes sent to origin.
    #[serde(rename = "bereq_header_bytes", skip_serializing_if = "Option::is_none")]
    pub bereq_header_bytes: Option<i64>,
    /// Total body bytes sent to origin.
    #[serde(rename = "bereq_body_bytes", skip_serializing_if = "Option::is_none")]
    pub bereq_body_bytes: Option<i64>,
    /// Number of requests that triggered a WAF rule and were blocked.
    #[serde(rename = "waf_blocked", skip_serializing_if = "Option::is_none")]
    pub waf_blocked: Option<i64>,
    /// Number of requests that triggered a WAF rule and were logged.
    #[serde(rename = "waf_logged", skip_serializing_if = "Option::is_none")]
    pub waf_logged: Option<i64>,
    /// Number of requests that triggered a WAF rule and were passed.
    #[serde(rename = "waf_passed", skip_serializing_if = "Option::is_none")]
    pub waf_passed: Option<i64>,
    /// Total header bytes received from requests that triggered a WAF rule.
    #[serde(rename = "attack_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_req_header_bytes: Option<i64>,
    /// Total body bytes received from requests that triggered a WAF rule.
    #[serde(rename = "attack_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_req_body_bytes: Option<i64>,
    /// Total bytes delivered for requests that triggered a WAF rule and returned a synthetic response.
    #[serde(rename = "attack_resp_synth_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_resp_synth_bytes: Option<i64>,
    /// Total header bytes received from requests that triggered a WAF rule that was logged.
    #[serde(rename = "attack_logged_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_logged_req_header_bytes: Option<i64>,
    /// Total body bytes received from requests that triggered a WAF rule that was logged.
    #[serde(rename = "attack_logged_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_logged_req_body_bytes: Option<i64>,
    /// Total header bytes received from requests that triggered a WAF rule that was blocked.
    #[serde(rename = "attack_blocked_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_blocked_req_header_bytes: Option<i64>,
    /// Total body bytes received from requests that triggered a WAF rule that was blocked.
    #[serde(rename = "attack_blocked_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_blocked_req_body_bytes: Option<i64>,
    /// Total header bytes received from requests that triggered a WAF rule that was passed.
    #[serde(rename = "attack_passed_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_passed_req_header_bytes: Option<i64>,
    /// Total body bytes received from requests that triggered a WAF rule that was passed.
    #[serde(rename = "attack_passed_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub attack_passed_req_body_bytes: Option<i64>,
    /// Total header bytes delivered via a shield.
    #[serde(rename = "shield_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered via a shield.
    #[serde(rename = "shield_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_resp_body_bytes: Option<i64>,
    /// Total header bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub otfp_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub otfp_resp_body_bytes: Option<i64>,
    /// Total header bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp_shield_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub otfp_shield_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand.
    #[serde(rename = "otfp_shield_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub otfp_shield_resp_body_bytes: Option<i64>,
    /// Total amount of time spent delivering a response via a shield from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds).
    #[serde(rename = "otfp_shield_time", skip_serializing_if = "Option::is_none")]
    pub otfp_shield_time: Option<f32>,
    /// Total amount of time spent delivering a response from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds).
    #[serde(rename = "otfp_deliver_time", skip_serializing_if = "Option::is_none")]
    pub otfp_deliver_time: Option<f32>,
    /// Total header bytes delivered from the Fastly Image Optimizer service, including shield traffic.
    #[serde(rename = "imgopto_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub imgopto_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered from the Fastly Image Optimizer service, including shield traffic.
    #[serde(rename = "imgopto_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub imgopto_resp_body_bytes: Option<i64>,
    /// Total header bytes delivered via a shield from the Fastly Image Optimizer service.
    #[serde(rename = "imgopto_shield_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub imgopto_shield_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered via a shield from the Fastly Image Optimizer service.
    #[serde(rename = "imgopto_shield_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub imgopto_shield_resp_body_bytes: Option<i64>,
    /// Number of \"Informational\" category status codes delivered.
    #[serde(rename = "status_1xx", skip_serializing_if = "Option::is_none")]
    pub status_1xx: Option<i64>,
    /// Number of \"Success\" status codes delivered.
    #[serde(rename = "status_2xx", skip_serializing_if = "Option::is_none")]
    pub status_2xx: Option<i64>,
    /// Number of \"Redirection\" codes delivered.
    #[serde(rename = "status_3xx", skip_serializing_if = "Option::is_none")]
    pub status_3xx: Option<i64>,
    /// Number of \"Client Error\" codes delivered.
    #[serde(rename = "status_4xx", skip_serializing_if = "Option::is_none")]
    pub status_4xx: Option<i64>,
    /// Number of \"Server Error\" codes delivered.
    #[serde(rename = "status_5xx", skip_serializing_if = "Option::is_none")]
    pub status_5xx: Option<i64>,
    /// Number of responses sent with status code 200 (Success).
    #[serde(rename = "status_200", skip_serializing_if = "Option::is_none")]
    pub status_200: Option<i64>,
    /// Number of responses sent with status code 204 (No Content).
    #[serde(rename = "status_204", skip_serializing_if = "Option::is_none")]
    pub status_204: Option<i64>,
    /// Number of responses sent with status code 206 (Partial Content).
    #[serde(rename = "status_206", skip_serializing_if = "Option::is_none")]
    pub status_206: Option<i64>,
    /// Number of responses sent with status code 301 (Moved Permanently).
    #[serde(rename = "status_301", skip_serializing_if = "Option::is_none")]
    pub status_301: Option<i64>,
    /// Number of responses sent with status code 302 (Found).
    #[serde(rename = "status_302", skip_serializing_if = "Option::is_none")]
    pub status_302: Option<i64>,
    /// Number of responses sent with status code 304 (Not Modified).
    #[serde(rename = "status_304", skip_serializing_if = "Option::is_none")]
    pub status_304: Option<i64>,
    /// Number of responses sent with status code 400 (Bad Request).
    #[serde(rename = "status_400", skip_serializing_if = "Option::is_none")]
    pub status_400: Option<i64>,
    /// Number of responses sent with status code 401 (Unauthorized).
    #[serde(rename = "status_401", skip_serializing_if = "Option::is_none")]
    pub status_401: Option<i64>,
    /// Number of responses sent with status code 403 (Forbidden).
    #[serde(rename = "status_403", skip_serializing_if = "Option::is_none")]
    pub status_403: Option<i64>,
    /// Number of responses sent with status code 404 (Not Found).
    #[serde(rename = "status_404", skip_serializing_if = "Option::is_none")]
    pub status_404: Option<i64>,
    /// Number of responses sent with status code 406 (Not Acceptable).
    #[serde(rename = "status_406", skip_serializing_if = "Option::is_none")]
    pub status_406: Option<i64>,
    /// Number of responses sent with status code 416 (Range Not Satisfiable).
    #[serde(rename = "status_416", skip_serializing_if = "Option::is_none")]
    pub status_416: Option<i64>,
    /// Number of responses sent with status code 429 (Too Many Requests).
    #[serde(rename = "status_429", skip_serializing_if = "Option::is_none")]
    pub status_429: Option<i64>,
    /// Number of responses sent with status code 500 (Internal Server Error).
    #[serde(rename = "status_500", skip_serializing_if = "Option::is_none")]
    pub status_500: Option<i64>,
    /// Number of responses sent with status code 501 (Not Implemented).
    #[serde(rename = "status_501", skip_serializing_if = "Option::is_none")]
    pub status_501: Option<i64>,
    /// Number of responses sent with status code 502 (Bad Gateway).
    #[serde(rename = "status_502", skip_serializing_if = "Option::is_none")]
    pub status_502: Option<i64>,
    /// Number of responses sent with status code 503 (Service Unavailable).
    #[serde(rename = "status_503", skip_serializing_if = "Option::is_none")]
    pub status_503: Option<i64>,
    /// Number of responses sent with status code 504 (Gateway Timeout).
    #[serde(rename = "status_504", skip_serializing_if = "Option::is_none")]
    pub status_504: Option<i64>,
    /// Number of responses sent with status code 505 (HTTP Version Not Supported).
    #[serde(rename = "status_505", skip_serializing_if = "Option::is_none")]
    pub status_505: Option<i64>,
    /// Number of responses sent with status code 530.
    #[serde(rename = "status_530", skip_serializing_if = "Option::is_none")]
    pub status_530: Option<i64>,
    /// Number of requests that were designated uncachable.
    #[serde(rename = "uncacheable", skip_serializing_if = "Option::is_none")]
    pub uncacheable: Option<i64>,
    /// Total amount of time spent processing cache passes (in seconds).
    #[serde(rename = "pass_time", skip_serializing_if = "Option::is_none")]
    pub pass_time: Option<f32>,
    /// Number of requests that were received over TLS.
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<i64>,
    /// Number of requests received over TLS 1.0.
    #[serde(rename = "tls_v10", skip_serializing_if = "Option::is_none")]
    pub tls_v10: Option<i64>,
    /// Number of requests received over TLS 1.1.
    #[serde(rename = "tls_v11", skip_serializing_if = "Option::is_none")]
    pub tls_v11: Option<i64>,
    /// Number of requests received over TLS 1.2.
    #[serde(rename = "tls_v12", skip_serializing_if = "Option::is_none")]
    pub tls_v12: Option<i64>,
    /// Number of requests received over TLS 1.3.
    #[serde(rename = "tls_v13", skip_serializing_if = "Option::is_none")]
    pub tls_v13: Option<i64>,
    /// Number of objects served that were under 1KB in size.
    #[serde(rename = "object_size_1k", skip_serializing_if = "Option::is_none")]
    pub object_size_1k: Option<i64>,
    /// Number of objects served that were between 1KB and 10KB in size.
    #[serde(rename = "object_size_10k", skip_serializing_if = "Option::is_none")]
    pub object_size_10k: Option<i64>,
    /// Number of objects served that were between 10KB and 100KB in size.
    #[serde(rename = "object_size_100k", skip_serializing_if = "Option::is_none")]
    pub object_size_100k: Option<i64>,
    /// Number of objects served that were between 100KB and 1MB in size.
    #[serde(rename = "object_size_1m", skip_serializing_if = "Option::is_none")]
    pub object_size_1m: Option<i64>,
    /// Number of objects served that were between 1MB and 10MB in size.
    #[serde(rename = "object_size_10m", skip_serializing_if = "Option::is_none")]
    pub object_size_10m: Option<i64>,
    /// Number of objects served that were between 10MB and 100MB in size.
    #[serde(rename = "object_size_100m", skip_serializing_if = "Option::is_none")]
    pub object_size_100m: Option<i64>,
    /// Number of objects served that were between 100MB and 1GB in size.
    #[serde(rename = "object_size_1g", skip_serializing_if = "Option::is_none")]
    pub object_size_1g: Option<i64>,
    /// Number of objects served that were larger than 1GB in size.
    #[serde(rename = "object_size_other", skip_serializing_if = "Option::is_none")]
    pub object_size_other: Option<i64>,
    /// Time spent inside the `vcl_recv` Varnish subroutine (in nanoseconds).
    #[serde(rename = "recv_sub_time", skip_serializing_if = "Option::is_none")]
    pub recv_sub_time: Option<f32>,
    /// Number of executions of the `vcl_recv` Varnish subroutine.
    #[serde(rename = "recv_sub_count", skip_serializing_if = "Option::is_none")]
    pub recv_sub_count: Option<i64>,
    /// Time spent inside the `vcl_hash` Varnish subroutine (in nanoseconds).
    #[serde(rename = "hash_sub_time", skip_serializing_if = "Option::is_none")]
    pub hash_sub_time: Option<f32>,
    /// Number of executions of the `vcl_hash` Varnish subroutine.
    #[serde(rename = "hash_sub_count", skip_serializing_if = "Option::is_none")]
    pub hash_sub_count: Option<i64>,
    /// Time spent inside the `vcl_miss` Varnish subroutine (in nanoseconds).
    #[serde(rename = "miss_sub_time", skip_serializing_if = "Option::is_none")]
    pub miss_sub_time: Option<f32>,
    /// Number of executions of the `vcl_miss` Varnish subroutine.
    #[serde(rename = "miss_sub_count", skip_serializing_if = "Option::is_none")]
    pub miss_sub_count: Option<i64>,
    /// Time spent inside the `vcl_fetch` Varnish subroutine (in nanoseconds).
    #[serde(rename = "fetch_sub_time", skip_serializing_if = "Option::is_none")]
    pub fetch_sub_time: Option<f32>,
    /// Number of executions of the `vcl_fetch` Varnish subroutine.
    #[serde(rename = "fetch_sub_count", skip_serializing_if = "Option::is_none")]
    pub fetch_sub_count: Option<i64>,
    /// Time spent inside the `vcl_pass` Varnish subroutine (in nanoseconds).
    #[serde(rename = "pass_sub_time", skip_serializing_if = "Option::is_none")]
    pub pass_sub_time: Option<f32>,
    /// Number of executions of the `vcl_pass` Varnish subroutine.
    #[serde(rename = "pass_sub_count", skip_serializing_if = "Option::is_none")]
    pub pass_sub_count: Option<i64>,
    /// Time spent inside the `vcl_pipe` Varnish subroutine (in nanoseconds).
    #[serde(rename = "pipe_sub_time", skip_serializing_if = "Option::is_none")]
    pub pipe_sub_time: Option<f32>,
    /// Number of executions of the `vcl_pipe` Varnish subroutine.
    #[serde(rename = "pipe_sub_count", skip_serializing_if = "Option::is_none")]
    pub pipe_sub_count: Option<i64>,
    /// Time spent inside the `vcl_deliver` Varnish subroutine (in nanoseconds).
    #[serde(rename = "deliver_sub_time", skip_serializing_if = "Option::is_none")]
    pub deliver_sub_time: Option<f32>,
    /// Number of executions of the `vcl_deliver` Varnish subroutine.
    #[serde(rename = "deliver_sub_count", skip_serializing_if = "Option::is_none")]
    pub deliver_sub_count: Option<i64>,
    /// Time spent inside the `vcl_error` Varnish subroutine (in nanoseconds).
    #[serde(rename = "error_sub_time", skip_serializing_if = "Option::is_none")]
    pub error_sub_time: Option<f32>,
    /// Number of executions of the `vcl_error` Varnish subroutine.
    #[serde(rename = "error_sub_count", skip_serializing_if = "Option::is_none")]
    pub error_sub_count: Option<i64>,
    /// Time spent inside the `vcl_hit` Varnish subroutine (in nanoseconds).
    #[serde(rename = "hit_sub_time", skip_serializing_if = "Option::is_none")]
    pub hit_sub_time: Option<f32>,
    /// Number of executions of the `vcl_hit` Varnish subroutine.
    #[serde(rename = "hit_sub_count", skip_serializing_if = "Option::is_none")]
    pub hit_sub_count: Option<i64>,
    /// Time spent inside the `vcl_prehash` Varnish subroutine (in nanoseconds).
    #[serde(rename = "prehash_sub_time", skip_serializing_if = "Option::is_none")]
    pub prehash_sub_time: Option<f32>,
    /// Number of executions of the `vcl_prehash` Varnish subroutine.
    #[serde(rename = "prehash_sub_count", skip_serializing_if = "Option::is_none")]
    pub prehash_sub_count: Option<i64>,
    /// Time spent inside the `vcl_predeliver` Varnish subroutine (in nanoseconds).
    #[serde(rename = "predeliver_sub_time", skip_serializing_if = "Option::is_none")]
    pub predeliver_sub_time: Option<f32>,
    /// Number of executions of the `vcl_predeliver` Varnish subroutine.
    #[serde(rename = "predeliver_sub_count", skip_serializing_if = "Option::is_none")]
    pub predeliver_sub_count: Option<i64>,
    /// Total body bytes delivered for cache hits.
    #[serde(rename = "hit_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub hit_resp_body_bytes: Option<i64>,
    /// Total body bytes delivered for cache misses.
    #[serde(rename = "miss_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub miss_resp_body_bytes: Option<i64>,
    /// Total body bytes delivered for cache passes.
    #[serde(rename = "pass_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub pass_resp_body_bytes: Option<i64>,
    /// Total header bytes received by the Compute platform.
    #[serde(rename = "compute_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_req_header_bytes: Option<i64>,
    /// Total body bytes received by the Compute platform.
    #[serde(rename = "compute_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_req_body_bytes: Option<i64>,
    /// Total header bytes sent from Compute to end user.
    #[serde(rename = "compute_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_resp_header_bytes: Option<i64>,
    /// Total body bytes sent from Compute to end user.
    #[serde(rename = "compute_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_resp_body_bytes: Option<i64>,
    /// Number of video responses that came from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo", skip_serializing_if = "Option::is_none")]
    pub imgvideo: Option<i64>,
    /// Number of video frames that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video.
    #[serde(rename = "imgvideo_frames", skip_serializing_if = "Option::is_none")]
    pub imgvideo_frames: Option<i64>,
    /// Total header bytes of video delivered from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub imgvideo_resp_header_bytes: Option<i64>,
    /// Total body bytes of video delivered from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub imgvideo_resp_body_bytes: Option<i64>,
    /// Number of video responses delivered via a shield that came from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo_shield", skip_serializing_if = "Option::is_none")]
    pub imgvideo_shield: Option<i64>,
    /// Number of video frames delivered via a shield that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video.
    #[serde(rename = "imgvideo_shield_frames", skip_serializing_if = "Option::is_none")]
    pub imgvideo_shield_frames: Option<i64>,
    /// Total header bytes of video delivered via a shield from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo_shield_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub imgvideo_shield_resp_header_bytes: Option<i64>,
    /// Total body bytes of video delivered via a shield from the Fastly Image Optimizer service.
    #[serde(rename = "imgvideo_shield_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub imgvideo_shield_resp_body_bytes: Option<i64>,
    /// Total log bytes sent.
    #[serde(rename = "log_bytes", skip_serializing_if = "Option::is_none")]
    pub log_bytes: Option<i64>,
    /// Number of requests sent by end users to Fastly.
    #[serde(rename = "edge_requests", skip_serializing_if = "Option::is_none")]
    pub edge_requests: Option<i64>,
    /// Total header bytes delivered from Fastly to the end user.
    #[serde(rename = "edge_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_resp_header_bytes: Option<i64>,
    /// Total body bytes delivered from Fastly to the end user.
    #[serde(rename = "edge_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_resp_body_bytes: Option<i64>,
    /// Number of responses received from origin with a `304` status code in response to an `If-Modified-Since` or `If-None-Match` request. Under regular scenarios, a revalidation will imply a cache hit. However, if using Fastly Image Optimizer or segmented caching this may result in a cache miss.
    #[serde(rename = "origin_revalidations", skip_serializing_if = "Option::is_none")]
    pub origin_revalidations: Option<i64>,
    /// Number of requests sent to origin.
    #[serde(rename = "origin_fetches", skip_serializing_if = "Option::is_none")]
    pub origin_fetches: Option<i64>,
    /// Total request header bytes sent to origin.
    #[serde(rename = "origin_fetch_header_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_header_bytes: Option<i64>,
    /// Total request body bytes sent to origin.
    #[serde(rename = "origin_fetch_body_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_body_bytes: Option<i64>,
    /// Total header bytes received from origin.
    #[serde(rename = "origin_fetch_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_resp_header_bytes: Option<i64>,
    /// Total body bytes received from origin.
    #[serde(rename = "origin_fetch_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_fetch_resp_body_bytes: Option<i64>,
    /// Number of responses received from origin with a `304` status code, in response to an `If-Modified-Since` or `If-None-Match` request to a shield. Under regular scenarios, a revalidation will imply a cache hit. However, if using segmented caching this may result in a cache miss.
    #[serde(rename = "shield_revalidations", skip_serializing_if = "Option::is_none")]
    pub shield_revalidations: Option<i64>,
    /// Number of requests made from one Fastly POP to another, as part of shielding.
    #[serde(rename = "shield_fetches", skip_serializing_if = "Option::is_none")]
    pub shield_fetches: Option<i64>,
    /// Total request header bytes sent to a shield.
    #[serde(rename = "shield_fetch_header_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_fetch_header_bytes: Option<i64>,
    /// Total request body bytes sent to a shield.
    #[serde(rename = "shield_fetch_body_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_fetch_body_bytes: Option<i64>,
    /// Total response header bytes sent from a shield to the edge.
    #[serde(rename = "shield_fetch_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_fetch_resp_header_bytes: Option<i64>,
    /// Total response body bytes sent from a shield to the edge.
    #[serde(rename = "shield_fetch_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_fetch_resp_body_bytes: Option<i64>,
    /// Number of `Range` requests to origin for segments of resources when using segmented caching.
    #[serde(rename = "segblock_origin_fetches", skip_serializing_if = "Option::is_none")]
    pub segblock_origin_fetches: Option<i64>,
    /// Number of `Range` requests to a shield for segments of resources when using segmented caching.
    #[serde(rename = "segblock_shield_fetches", skip_serializing_if = "Option::is_none")]
    pub segblock_shield_fetches: Option<i64>,
    /// Number of \"Informational\" category status codes delivered by the Compute platform.
    #[serde(rename = "compute_resp_status_1xx", skip_serializing_if = "Option::is_none")]
    pub compute_resp_status_1xx: Option<i64>,
    /// Number of \"Success\" category status codes delivered by the Compute platform.
    #[serde(rename = "compute_resp_status_2xx", skip_serializing_if = "Option::is_none")]
    pub compute_resp_status_2xx: Option<i64>,
    /// Number of \"Redirection\" category status codes delivered by the Compute platform.
    #[serde(rename = "compute_resp_status_3xx", skip_serializing_if = "Option::is_none")]
    pub compute_resp_status_3xx: Option<i64>,
    /// Number of \"Client Error\" category status codes delivered by the Compute platform.
    #[serde(rename = "compute_resp_status_4xx", skip_serializing_if = "Option::is_none")]
    pub compute_resp_status_4xx: Option<i64>,
    /// Number of \"Server Error\" category status codes delivered by the Compute platform.
    #[serde(rename = "compute_resp_status_5xx", skip_serializing_if = "Option::is_none")]
    pub compute_resp_status_5xx: Option<i64>,
    /// Number of requests sent by end users to Fastly that resulted in a hit at the edge.
    #[serde(rename = "edge_hit_requests", skip_serializing_if = "Option::is_none")]
    pub edge_hit_requests: Option<i64>,
    /// Number of requests sent by end users to Fastly that resulted in a miss at the edge.
    #[serde(rename = "edge_miss_requests", skip_serializing_if = "Option::is_none")]
    pub edge_miss_requests: Option<i64>,
    /// Total header bytes sent to backends (origins) by the Compute platform.
    #[serde(rename = "compute_bereq_header_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_bereq_header_bytes: Option<i64>,
    /// Total body bytes sent to backends (origins) by the Compute platform.
    #[serde(rename = "compute_bereq_body_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_bereq_body_bytes: Option<i64>,
    /// Total header bytes received from backends (origins) by the Compute platform.
    #[serde(rename = "compute_beresp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_beresp_header_bytes: Option<i64>,
    /// Total body bytes received from backends (origins) by the Compute platform.
    #[serde(rename = "compute_beresp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub compute_beresp_body_bytes: Option<i64>,
    /// The total number of completed requests made to backends (origins) that returned cacheable content.
    #[serde(rename = "origin_cache_fetches", skip_serializing_if = "Option::is_none")]
    pub origin_cache_fetches: Option<i64>,
    /// The total number of completed requests made to shields that returned cacheable content.
    #[serde(rename = "shield_cache_fetches", skip_serializing_if = "Option::is_none")]
    pub shield_cache_fetches: Option<i64>,
    /// Number of backend requests started.
    #[serde(rename = "compute_bereqs", skip_serializing_if = "Option::is_none")]
    pub compute_bereqs: Option<i64>,
    /// Number of backend request errors, including timeouts.
    #[serde(rename = "compute_bereq_errors", skip_serializing_if = "Option::is_none")]
    pub compute_bereq_errors: Option<i64>,
    /// Number of times a guest exceeded its resource limit, includes heap, stack, globals, and code execution timeout.
    #[serde(rename = "compute_resource_limit_exceeded", skip_serializing_if = "Option::is_none")]
    pub compute_resource_limit_exceeded: Option<i64>,
    /// Number of times a guest exceeded its heap limit.
    #[serde(rename = "compute_heap_limit_exceeded", skip_serializing_if = "Option::is_none")]
    pub compute_heap_limit_exceeded: Option<i64>,
    /// Number of times a guest exceeded its stack limit.
    #[serde(rename = "compute_stack_limit_exceeded", skip_serializing_if = "Option::is_none")]
    pub compute_stack_limit_exceeded: Option<i64>,
    /// Number of times a guest exceeded its globals limit.
    #[serde(rename = "compute_globals_limit_exceeded", skip_serializing_if = "Option::is_none")]
    pub compute_globals_limit_exceeded: Option<i64>,
    /// Number of times a service experienced a guest code error.
    #[serde(rename = "compute_guest_errors", skip_serializing_if = "Option::is_none")]
    pub compute_guest_errors: Option<i64>,
    /// Number of times a service experienced a guest runtime error.
    #[serde(rename = "compute_runtime_errors", skip_serializing_if = "Option::is_none")]
    pub compute_runtime_errors: Option<i64>,
    /// Body bytes delivered for edge hits.
    #[serde(rename = "edge_hit_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_hit_resp_body_bytes: Option<i64>,
    /// Header bytes delivered for edge hits.
    #[serde(rename = "edge_hit_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_hit_resp_header_bytes: Option<i64>,
    /// Body bytes delivered for edge misses.
    #[serde(rename = "edge_miss_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_miss_resp_body_bytes: Option<i64>,
    /// Header bytes delivered for edge misses.
    #[serde(rename = "edge_miss_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub edge_miss_resp_header_bytes: Option<i64>,
    /// Body bytes received from origin for cacheable content.
    #[serde(rename = "origin_cache_fetch_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_cache_fetch_resp_body_bytes: Option<i64>,
    /// Header bytes received from an origin for cacheable content.
    #[serde(rename = "origin_cache_fetch_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub origin_cache_fetch_resp_header_bytes: Option<i64>,
    /// Number of requests that resulted in a hit at a shield.
    #[serde(rename = "shield_hit_requests", skip_serializing_if = "Option::is_none")]
    pub shield_hit_requests: Option<i64>,
    /// Number of requests that resulted in a miss at a shield.
    #[serde(rename = "shield_miss_requests", skip_serializing_if = "Option::is_none")]
    pub shield_miss_requests: Option<i64>,
    /// Header bytes delivered for shield hits.
    #[serde(rename = "shield_hit_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_hit_resp_header_bytes: Option<i64>,
    /// Body bytes delivered for shield hits.
    #[serde(rename = "shield_hit_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_hit_resp_body_bytes: Option<i64>,
    /// Header bytes delivered for shield misses.
    #[serde(rename = "shield_miss_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_miss_resp_header_bytes: Option<i64>,
    /// Body bytes delivered for shield misses.
    #[serde(rename = "shield_miss_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub shield_miss_resp_body_bytes: Option<i64>,
    /// Total header bytes received from end users over passthrough WebSocket connections.
    #[serde(rename = "websocket_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_req_header_bytes: Option<i64>,
    /// Total message content bytes received from end users over passthrough WebSocket connections.
    #[serde(rename = "websocket_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_req_body_bytes: Option<i64>,
    /// Total header bytes sent to end users over passthrough WebSocket connections.
    #[serde(rename = "websocket_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_resp_header_bytes: Option<i64>,
    /// Total header bytes sent to backends over passthrough WebSocket connections.
    #[serde(rename = "websocket_bereq_header_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_bereq_header_bytes: Option<i64>,
    /// Total message content bytes sent to backends over passthrough WebSocket connections.
    #[serde(rename = "websocket_bereq_body_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_bereq_body_bytes: Option<i64>,
    /// Total header bytes received from backends over passthrough WebSocket connections.
    #[serde(rename = "websocket_beresp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_beresp_header_bytes: Option<i64>,
    /// Total message content bytes received from backends over passthrough WebSocket connections.
    #[serde(rename = "websocket_beresp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_beresp_body_bytes: Option<i64>,
    /// Total duration of passthrough WebSocket connections with end users.
    #[serde(rename = "websocket_conn_time_ms", skip_serializing_if = "Option::is_none")]
    pub websocket_conn_time_ms: Option<i64>,
    /// Total message content bytes sent to end users over passthrough WebSocket connections.
    #[serde(rename = "websocket_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub websocket_resp_body_bytes: Option<i64>,
    /// Total published messages received from the publish API endpoint.
    #[serde(rename = "fanout_recv_publishes", skip_serializing_if = "Option::is_none")]
    pub fanout_recv_publishes: Option<i64>,
    /// Total published messages sent to end users.
    #[serde(rename = "fanout_send_publishes", skip_serializing_if = "Option::is_none")]
    pub fanout_send_publishes: Option<i64>,
    /// The total number of class a operations for the KV store.
    #[serde(rename = "kv_store_class_a_operations", skip_serializing_if = "Option::is_none")]
    pub kv_store_class_a_operations: Option<i64>,
    /// The total number of class b operations for the KV store.
    #[serde(rename = "kv_store_class_b_operations", skip_serializing_if = "Option::is_none")]
    pub kv_store_class_b_operations: Option<i64>,
    /// Use kv_store_class_a_operations.
    #[serde(rename = "object_store_class_a_operations", skip_serializing_if = "Option::is_none")]
    pub object_store_class_a_operations: Option<i64>,
    /// Use kv_store_class_b_operations.
    #[serde(rename = "object_store_class_b_operations", skip_serializing_if = "Option::is_none")]
    pub object_store_class_b_operations: Option<i64>,
    /// Total header bytes received from end users over Fanout connections.
    #[serde(rename = "fanout_req_header_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_req_header_bytes: Option<i64>,
    /// Total body or message content bytes received from end users over Fanout connections.
    #[serde(rename = "fanout_req_body_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_req_body_bytes: Option<i64>,
    /// Total header bytes sent to end users over Fanout connections.
    #[serde(rename = "fanout_resp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_resp_header_bytes: Option<i64>,
    /// Total body or message content bytes sent to end users over Fanout connections, excluding published message content.
    #[serde(rename = "fanout_resp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_resp_body_bytes: Option<i64>,
    /// Total header bytes sent to backends over Fanout connections.
    #[serde(rename = "fanout_bereq_header_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_bereq_header_bytes: Option<i64>,
    /// Total body or message content bytes sent to backends over Fanout connections.
    #[serde(rename = "fanout_bereq_body_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_bereq_body_bytes: Option<i64>,
    /// Total header bytes received from backends over Fanout connections.
    #[serde(rename = "fanout_beresp_header_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_beresp_header_bytes: Option<i64>,
    /// Total body or message content bytes received from backends over Fanout connections.
    #[serde(rename = "fanout_beresp_body_bytes", skip_serializing_if = "Option::is_none")]
    pub fanout_beresp_body_bytes: Option<i64>,
    /// Total duration of Fanout connections with end users.
    #[serde(rename = "fanout_conn_time_ms", skip_serializing_if = "Option::is_none")]
    pub fanout_conn_time_ms: Option<i64>,
    /// For HTTP/2, the number of connections the limit-streams action was applied to. The limit-streams action caps the allowed number of concurrent streams in a connection.
    #[serde(rename = "ddos_action_limit_streams_connections", skip_serializing_if = "Option::is_none")]
    pub ddos_action_limit_streams_connections: Option<i64>,
    /// For HTTP/2, the number of requests made on a connection for which the limit-streams action was taken. The limit-streams action caps the allowed number of concurrent streams in a connection.
    #[serde(rename = "ddos_action_limit_streams_requests", skip_serializing_if = "Option::is_none")]
    pub ddos_action_limit_streams_requests: Option<i64>,
    /// The number of times the tarpit-accept action was taken. The tarpit-accept action adds a delay when accepting future connections.
    #[serde(rename = "ddos_action_tarpit_accept", skip_serializing_if = "Option::is_none")]
    pub ddos_action_tarpit_accept: Option<i64>,
    /// The number of times the tarpit action was taken. The tarpit action delays writing the response to the client.
    #[serde(rename = "ddos_action_tarpit", skip_serializing_if = "Option::is_none")]
    pub ddos_action_tarpit: Option<i64>,
    /// The number of times the close action was taken. The close action aborts the connection as soon as possible. The close action takes effect either right after accept, right after the client hello, or right after the response was sent.
    #[serde(rename = "ddos_action_close", skip_serializing_if = "Option::is_none")]
    pub ddos_action_close: Option<i64>,
    /// The number of times the blackhole action was taken. The blackhole action quietly closes a TCP connection without sending a reset. The blackhole action quietly closes a TCP connection without notifying its peer (all TCP state is dropped).
    #[serde(rename = "ddos_action_blackhole", skip_serializing_if = "Option::is_none")]
    pub ddos_action_blackhole: Option<i64>,
    /// The number of challenge-start tokens created.
    #[serde(rename = "bot_challenge_starts", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_starts: Option<i64>,
    /// The number of challenge-complete tokens that passed validation.
    #[serde(rename = "bot_challenge_complete_tokens_passed", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_complete_tokens_passed: Option<i64>,
    /// The number of challenge-complete tokens that failed validation.
    #[serde(rename = "bot_challenge_complete_tokens_failed", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_complete_tokens_failed: Option<i64>,
    /// The number of challenge-complete tokens checked.
    #[serde(rename = "bot_challenge_complete_tokens_checked", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_complete_tokens_checked: Option<i64>,
    /// The number of challenge-complete tokens not checked because the feature was disabled.
    #[serde(rename = "bot_challenge_complete_tokens_disabled", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_complete_tokens_disabled: Option<i64>,
    /// The number of challenges issued. For example, the issuance of a CAPTCHA challenge.
    #[serde(rename = "bot_challenges_issued", skip_serializing_if = "Option::is_none")]
    pub bot_challenges_issued: Option<i64>,
    /// The number of successful challenge solutions processed. For example, a correct CAPTCHA solution.
    #[serde(rename = "bot_challenges_succeeded", skip_serializing_if = "Option::is_none")]
    pub bot_challenges_succeeded: Option<i64>,
    /// The number of failed challenge solutions processed. For example, an incorrect CAPTCHA solution.
    #[serde(rename = "bot_challenges_failed", skip_serializing_if = "Option::is_none")]
    pub bot_challenges_failed: Option<i64>,
    /// The number of challenge-complete tokens issued. For example, issuing a challenge-complete token after a series of CAPTCHA challenges ending in success.
    #[serde(rename = "bot_challenge_complete_tokens_issued", skip_serializing_if = "Option::is_none")]
    pub bot_challenge_complete_tokens_issued: Option<i64>,
    /// The number of times the downgrade action was taken. The downgrade action restricts the client to http1.
    #[serde(rename = "ddos_action_downgrade", skip_serializing_if = "Option::is_none")]
    pub ddos_action_downgrade: Option<i64>,
    /// The number of connections the downgrade action was applied to. The downgrade action restricts the connection to http1.
    #[serde(rename = "ddos_action_downgraded_connections", skip_serializing_if = "Option::is_none")]
    pub ddos_action_downgraded_connections: Option<i64>,
    /// Number of cache hits for a VCL service.
    #[serde(rename = "all_hit_requests", skip_serializing_if = "Option::is_none")]
    pub all_hit_requests: Option<i64>,
    /// Number of cache misses for a VCL service.
    #[serde(rename = "all_miss_requests", skip_serializing_if = "Option::is_none")]
    pub all_miss_requests: Option<i64>,
    /// Number of requests that passed through the CDN without being cached for a VCL service.
    #[serde(rename = "all_pass_requests", skip_serializing_if = "Option::is_none")]
    pub all_pass_requests: Option<i64>,
    /// Number of cache errors for a VCL service.
    #[serde(rename = "all_error_requests", skip_serializing_if = "Option::is_none")]
    pub all_error_requests: Option<i64>,
    /// Number of requests that returned a synthetic response (i.e., response objects created with the `synthetic` VCL statement) for a VCL service.
    #[serde(rename = "all_synth_requests", skip_serializing_if = "Option::is_none")]
    pub all_synth_requests: Option<i64>,
    /// Number of requests sent by end users to Fastly that resulted in a hit at the edge for a VCL service.
    #[serde(rename = "all_edge_hit_requests", skip_serializing_if = "Option::is_none")]
    pub all_edge_hit_requests: Option<i64>,
    /// Number of requests sent by end users to Fastly that resulted in a miss at the edge for a VCL service.
    #[serde(rename = "all_edge_miss_requests", skip_serializing_if = "Option::is_none")]
    pub all_edge_miss_requests: Option<i64>,
    /// Number of \"Informational\" category status codes delivered for all sources.
    #[serde(rename = "all_status_1xx", skip_serializing_if = "Option::is_none")]
    pub all_status_1xx: Option<i64>,
    /// Number of \"Success\" status codes delivered for all sources.
    #[serde(rename = "all_status_2xx", skip_serializing_if = "Option::is_none")]
    pub all_status_2xx: Option<i64>,
    /// Number of \"Redirection\" codes delivered for all sources.
    #[serde(rename = "all_status_3xx", skip_serializing_if = "Option::is_none")]
    pub all_status_3xx: Option<i64>,
    /// Number of \"Client Error\" codes delivered for all sources.
    #[serde(rename = "all_status_4xx", skip_serializing_if = "Option::is_none")]
    pub all_status_4xx: Option<i64>,
    /// Number of \"Server Error\" codes delivered for all sources.
    #[serde(rename = "all_status_5xx", skip_serializing_if = "Option::is_none")]
    pub all_status_5xx: Option<i64>,
    /// Origin Offload measures the ratio of bytes served to end users that were cached by Fastly, over the bytes served to end users, between 0 and 1. ((`edge_resp_body_bytes` + `edge_resp_header_bytes`) - (`origin_fetch_resp_body_bytes` + `origin_fetch_resp_header_bytes`)) / (`edge_resp_body_bytes` + `edge_resp_header_bytes`).
    #[serde(rename = "origin_offload", skip_serializing_if = "Option::is_none")]
    pub origin_offload: Option<f32>,
    /// Number of requests where Fastly responded with 400 due to the request being a GET or HEAD request containing a body.
    #[serde(rename = "request_denied_get_head_body", skip_serializing_if = "Option::is_none")]
    pub request_denied_get_head_body: Option<i64>,
    /// Number of requests classified as a DDoS attack against a customer origin or service.
    #[serde(rename = "ddos_protection_requests_detect_count", skip_serializing_if = "Option::is_none")]
    pub ddos_protection_requests_detect_count: Option<i64>,
    /// Number of requests classified as a DDoS attack against a customer origin or service that were mitigated by the Fastly platform.
    #[serde(rename = "ddos_protection_requests_mitigate_count", skip_serializing_if = "Option::is_none")]
    pub ddos_protection_requests_mitigate_count: Option<i64>,
    /// Number of requests analyzed for DDoS attacks against a customer origin or service, but with no DDoS detected.
    #[serde(rename = "ddos_protection_requests_allow_count", skip_serializing_if = "Option::is_none")]
    pub ddos_protection_requests_allow_count: Option<i64>,
    /// A count of the number of Class A Object Storage operations.
    #[serde(rename = "object_storage_class_a_operations_count", skip_serializing_if = "Option::is_none")]
    pub object_storage_class_a_operations_count: Option<i64>,
    /// A count of the number of Class B Object Storage operations.
    #[serde(rename = "object_storage_class_b_operations_count", skip_serializing_if = "Option::is_none")]
    pub object_storage_class_b_operations_count: Option<i64>,
    /// Number of requests received by AI Accelerator.
    #[serde(rename = "aia_requests", skip_serializing_if = "Option::is_none")]
    pub aia_requests: Option<i64>,
    /// Number of \"Informational\" category status codes received from AI provider.
    #[serde(rename = "aia_status_1xx", skip_serializing_if = "Option::is_none")]
    pub aia_status_1xx: Option<i64>,
    /// Number of \"Success\" status codes received from AI provider.
    #[serde(rename = "aia_status_2xx", skip_serializing_if = "Option::is_none")]
    pub aia_status_2xx: Option<i64>,
    /// Number of \"Redirection\" received from AI provider.
    #[serde(rename = "aia_status_3xx", skip_serializing_if = "Option::is_none")]
    pub aia_status_3xx: Option<i64>,
    /// Number of \"Client Error\" received from AI provider.
    #[serde(rename = "aia_status_4xx", skip_serializing_if = "Option::is_none")]
    pub aia_status_4xx: Option<i64>,
    /// Number of \"Server Error\" received from AI provider.
    #[serde(rename = "aia_status_5xx", skip_serializing_if = "Option::is_none")]
    pub aia_status_5xx: Option<i64>,
    /// The usage tokens associated with the response returned from the AI Accelerator cache.
    #[serde(rename = "aia_response_usage_tokens", skip_serializing_if = "Option::is_none")]
    pub aia_response_usage_tokens: Option<i64>,
    /// The number of usage tokens reported by the request to the origin from AI Accelerator.
    #[serde(rename = "aia_origin_usage_tokens", skip_serializing_if = "Option::is_none")]
    pub aia_origin_usage_tokens: Option<i64>,
    /// The estimated amount of time saved by responses served from the AI Accelerator semantic cache.
    #[serde(rename = "aia_estimated_time_saved_ms", skip_serializing_if = "Option::is_none")]
    pub aia_estimated_time_saved_ms: Option<i64>,
    /// Number of requests that were collapsed and satisfied by a usable cache object.
    #[serde(rename = "request_collapse_usable_count", skip_serializing_if = "Option::is_none")]
    pub request_collapse_usable_count: Option<i64>,
    /// Number of requests that were collapsed and unable to be satisfied by the resulting cache object.
    #[serde(rename = "request_collapse_unusable_count", skip_serializing_if = "Option::is_none")]
    pub request_collapse_unusable_count: Option<i64>,
    /// Number of cache operations executed by the Compute platform.
    #[serde(rename = "compute_cache_operations_count", skip_serializing_if = "Option::is_none")]
    pub compute_cache_operations_count: Option<i64>,
}

impl RealtimeEntryAggregated {
    /// Aggregates [measurements](#measurements-data-model) across all Fastly POPs.
    pub fn new() -> RealtimeEntryAggregated {
        RealtimeEntryAggregated {
            requests: None,
            logging: None,
            log: None,
            resp_header_bytes: None,
            header_size: None,
            resp_body_bytes: None,
            body_size: None,
            hits: None,
            miss: None,
            pass: None,
            synth: None,
            errors: None,
            hits_time: None,
            miss_time: None,
            miss_histogram: None,
            compute_requests: None,
            compute_execution_time_ms: None,
            compute_ram_used: None,
            compute_request_time_ms: None,
            compute_request_time_billed_ms: None,
            shield: None,
            ipv6: None,
            imgopto: None,
            imgopto_shield: None,
            imgopto_transforms: None,
            otfp: None,
            otfp_shield: None,
            otfp_manifests: None,
            video: None,
            pci: None,
            http2: None,
            http3: None,
            restarts: None,
            req_header_bytes: None,
            req_body_bytes: None,
            bereq_header_bytes: None,
            bereq_body_bytes: None,
            waf_blocked: None,
            waf_logged: None,
            waf_passed: None,
            attack_req_header_bytes: None,
            attack_req_body_bytes: None,
            attack_resp_synth_bytes: None,
            attack_logged_req_header_bytes: None,
            attack_logged_req_body_bytes: None,
            attack_blocked_req_header_bytes: None,
            attack_blocked_req_body_bytes: None,
            attack_passed_req_header_bytes: None,
            attack_passed_req_body_bytes: None,
            shield_resp_header_bytes: None,
            shield_resp_body_bytes: None,
            otfp_resp_header_bytes: None,
            otfp_resp_body_bytes: None,
            otfp_shield_resp_header_bytes: None,
            otfp_shield_resp_body_bytes: None,
            otfp_shield_time: None,
            otfp_deliver_time: None,
            imgopto_resp_header_bytes: None,
            imgopto_resp_body_bytes: None,
            imgopto_shield_resp_header_bytes: None,
            imgopto_shield_resp_body_bytes: None,
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
            status_406: None,
            status_416: None,
            status_429: None,
            status_500: None,
            status_501: None,
            status_502: None,
            status_503: None,
            status_504: None,
            status_505: None,
            status_530: None,
            uncacheable: None,
            pass_time: None,
            tls: None,
            tls_v10: None,
            tls_v11: None,
            tls_v12: None,
            tls_v13: None,
            object_size_1k: None,
            object_size_10k: None,
            object_size_100k: None,
            object_size_1m: None,
            object_size_10m: None,
            object_size_100m: None,
            object_size_1g: None,
            object_size_other: None,
            recv_sub_time: None,
            recv_sub_count: None,
            hash_sub_time: None,
            hash_sub_count: None,
            miss_sub_time: None,
            miss_sub_count: None,
            fetch_sub_time: None,
            fetch_sub_count: None,
            pass_sub_time: None,
            pass_sub_count: None,
            pipe_sub_time: None,
            pipe_sub_count: None,
            deliver_sub_time: None,
            deliver_sub_count: None,
            error_sub_time: None,
            error_sub_count: None,
            hit_sub_time: None,
            hit_sub_count: None,
            prehash_sub_time: None,
            prehash_sub_count: None,
            predeliver_sub_time: None,
            predeliver_sub_count: None,
            hit_resp_body_bytes: None,
            miss_resp_body_bytes: None,
            pass_resp_body_bytes: None,
            compute_req_header_bytes: None,
            compute_req_body_bytes: None,
            compute_resp_header_bytes: None,
            compute_resp_body_bytes: None,
            imgvideo: None,
            imgvideo_frames: None,
            imgvideo_resp_header_bytes: None,
            imgvideo_resp_body_bytes: None,
            imgvideo_shield: None,
            imgvideo_shield_frames: None,
            imgvideo_shield_resp_header_bytes: None,
            imgvideo_shield_resp_body_bytes: None,
            log_bytes: None,
            edge_requests: None,
            edge_resp_header_bytes: None,
            edge_resp_body_bytes: None,
            origin_revalidations: None,
            origin_fetches: None,
            origin_fetch_header_bytes: None,
            origin_fetch_body_bytes: None,
            origin_fetch_resp_header_bytes: None,
            origin_fetch_resp_body_bytes: None,
            shield_revalidations: None,
            shield_fetches: None,
            shield_fetch_header_bytes: None,
            shield_fetch_body_bytes: None,
            shield_fetch_resp_header_bytes: None,
            shield_fetch_resp_body_bytes: None,
            segblock_origin_fetches: None,
            segblock_shield_fetches: None,
            compute_resp_status_1xx: None,
            compute_resp_status_2xx: None,
            compute_resp_status_3xx: None,
            compute_resp_status_4xx: None,
            compute_resp_status_5xx: None,
            edge_hit_requests: None,
            edge_miss_requests: None,
            compute_bereq_header_bytes: None,
            compute_bereq_body_bytes: None,
            compute_beresp_header_bytes: None,
            compute_beresp_body_bytes: None,
            origin_cache_fetches: None,
            shield_cache_fetches: None,
            compute_bereqs: None,
            compute_bereq_errors: None,
            compute_resource_limit_exceeded: None,
            compute_heap_limit_exceeded: None,
            compute_stack_limit_exceeded: None,
            compute_globals_limit_exceeded: None,
            compute_guest_errors: None,
            compute_runtime_errors: None,
            edge_hit_resp_body_bytes: None,
            edge_hit_resp_header_bytes: None,
            edge_miss_resp_body_bytes: None,
            edge_miss_resp_header_bytes: None,
            origin_cache_fetch_resp_body_bytes: None,
            origin_cache_fetch_resp_header_bytes: None,
            shield_hit_requests: None,
            shield_miss_requests: None,
            shield_hit_resp_header_bytes: None,
            shield_hit_resp_body_bytes: None,
            shield_miss_resp_header_bytes: None,
            shield_miss_resp_body_bytes: None,
            websocket_req_header_bytes: None,
            websocket_req_body_bytes: None,
            websocket_resp_header_bytes: None,
            websocket_bereq_header_bytes: None,
            websocket_bereq_body_bytes: None,
            websocket_beresp_header_bytes: None,
            websocket_beresp_body_bytes: None,
            websocket_conn_time_ms: None,
            websocket_resp_body_bytes: None,
            fanout_recv_publishes: None,
            fanout_send_publishes: None,
            kv_store_class_a_operations: None,
            kv_store_class_b_operations: None,
            object_store_class_a_operations: None,
            object_store_class_b_operations: None,
            fanout_req_header_bytes: None,
            fanout_req_body_bytes: None,
            fanout_resp_header_bytes: None,
            fanout_resp_body_bytes: None,
            fanout_bereq_header_bytes: None,
            fanout_bereq_body_bytes: None,
            fanout_beresp_header_bytes: None,
            fanout_beresp_body_bytes: None,
            fanout_conn_time_ms: None,
            ddos_action_limit_streams_connections: None,
            ddos_action_limit_streams_requests: None,
            ddos_action_tarpit_accept: None,
            ddos_action_tarpit: None,
            ddos_action_close: None,
            ddos_action_blackhole: None,
            bot_challenge_starts: None,
            bot_challenge_complete_tokens_passed: None,
            bot_challenge_complete_tokens_failed: None,
            bot_challenge_complete_tokens_checked: None,
            bot_challenge_complete_tokens_disabled: None,
            bot_challenges_issued: None,
            bot_challenges_succeeded: None,
            bot_challenges_failed: None,
            bot_challenge_complete_tokens_issued: None,
            ddos_action_downgrade: None,
            ddos_action_downgraded_connections: None,
            all_hit_requests: None,
            all_miss_requests: None,
            all_pass_requests: None,
            all_error_requests: None,
            all_synth_requests: None,
            all_edge_hit_requests: None,
            all_edge_miss_requests: None,
            all_status_1xx: None,
            all_status_2xx: None,
            all_status_3xx: None,
            all_status_4xx: None,
            all_status_5xx: None,
            origin_offload: None,
            request_denied_get_head_body: None,
            ddos_protection_requests_detect_count: None,
            ddos_protection_requests_mitigate_count: None,
            ddos_protection_requests_allow_count: None,
            object_storage_class_a_operations_count: None,
            object_storage_class_b_operations_count: None,
            aia_requests: None,
            aia_status_1xx: None,
            aia_status_2xx: None,
            aia_status_3xx: None,
            aia_status_4xx: None,
            aia_status_5xx: None,
            aia_response_usage_tokens: None,
            aia_origin_usage_tokens: None,
            aia_estimated_time_saved_ms: None,
            request_collapse_usable_count: None,
            request_collapse_unusable_count: None,
            compute_cache_operations_count: None,
        }
    }
}


