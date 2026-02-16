# DomainInspectorMeasurements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**edge_requests** | Option<**i64**> | Number of requests sent by end users to Fastly. | 
**edge_resp_header_bytes** | Option<**i64**> | Total header bytes delivered from Fastly to the end user. | 
**edge_resp_body_bytes** | Option<**i64**> | Total body bytes delivered from Fastly to the end user. | 
**status_1xx** | Option<**i64**> | Number of 1xx \"Informational\" category status codes delivered. | 
**status_2xx** | Option<**i64**> | Number of 2xx \"Success\" status codes delivered. | 
**status_3xx** | Option<**i64**> | Number of 3xx \"Redirection\" codes delivered. | 
**status_4xx** | Option<**i64**> | Number of 4xx \"Client Error\" codes delivered. | 
**status_5xx** | Option<**i64**> | Number of 5xx \"Server Error\" codes delivered. | 
**status_200** | Option<**i64**> | Number of responses delivered with status code 200 (Success). | 
**status_204** | Option<**i64**> | Number of responses delivered with status code 204 (No Content). | 
**status_206** | Option<**i64**> | Number of responses delivered with status code 206 (Partial Content). | 
**status_301** | Option<**i64**> | Number of responses delivered with status code 301 (Moved Permanently). | 
**status_302** | Option<**i64**> | Number of responses delivered with status code 302 (Found). | 
**status_304** | Option<**i64**> | Number of responses delivered with status code 304 (Not Modified). | 
**status_400** | Option<**i64**> | Number of responses delivered with status code 400 (Bad Request). | 
**status_401** | Option<**i64**> | Number of responses delivered with status code 401 (Unauthorized). | 
**status_403** | Option<**i64**> | Number of responses delivered with status code 403 (Forbidden). | 
**status_404** | Option<**i64**> | Number of responses delivered with status code 404 (Not Found). | 
**status_416** | Option<**i64**> | Number of responses delivered with status code 416 (Range Not Satisfiable). | 
**status_429** | Option<**i64**> | Number of responses delivered with status code 429 (Too Many Requests). | 
**status_500** | Option<**i64**> | Number of responses delivered with status code 500 (Internal Server Error). | 
**status_501** | Option<**i64**> | Number of responses delivered with status code 501 (Not Implemented). | 
**status_502** | Option<**i64**> | Number of responses delivered with status code 502 (Bad Gateway). | 
**status_503** | Option<**i64**> | Number of responses delivered with status code 503 (Service Unavailable). | 
**status_504** | Option<**i64**> | Number of responses delivered with status code 504 (Gateway Timeout). | 
**status_505** | Option<**i64**> | Number of responses delivered with status code 505 (HTTP Version Not Supported). | 
**status_530** | Option<**i64**> | Number of responses delivered with status code 530. | 
**requests** | Option<**i64**> | Number of requests processed. | 
**resp_header_bytes** | Option<**i64**> | Total header bytes delivered. | 
**resp_body_bytes** | Option<**i64**> | Total body bytes delivered. | 
**bereq_header_bytes** | Option<**i64**> | Total header bytes sent to origin. | 
**bereq_body_bytes** | Option<**i64**> | Total body bytes sent to origin. | 
**edge_hit_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a hit at the edge. | 
**edge_miss_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a miss at the edge. | 
**origin_fetches** | Option<**i64**> | Number of requests sent to origin. | 
**origin_fetch_resp_header_bytes** | Option<**i64**> | Total header bytes received from origin. | 
**origin_fetch_resp_body_bytes** | Option<**i64**> | Total body bytes received from origin. | 
**bandwidth** | Option<**i64**> | Total bytes delivered (`resp_header_bytes` + `resp_body_bytes` + `bereq_header_bytes` + `bereq_body_bytes`). | 
**edge_hit_ratio** | Option<**f32**> | Ratio of cache hits to cache misses at the edge, between 0 and 1 (`edge_hit_requests` / (`edge_hit_requests` + `edge_miss_requests`)). | 
**origin_offload** | Option<**f32**> | Origin Offload measures the ratio of bytes served to end users that were cached by Fastly, over the bytes served to end users, between 0 and 1. ((`edge_resp_body_bytes` + `edge_resp_header_bytes`) - (`origin_fetch_resp_body_bytes` + `origin_fetch_resp_header_bytes`)) / (`edge_resp_body_bytes` + `edge_resp_header_bytes`). Previously, Origin Offload used a different formula. [Learn more](https://www.fastly.com/documentation/reference/changes/2024/06/add-origin_offload-metric). | 
**origin_status_200** | Option<**i64**> | Number of responses received from origin with status code 200 (Success). | 
**origin_status_204** | Option<**i64**> | Number of responses received from origin with status code 204 (No Content). | 
**origin_status_206** | Option<**i64**> | Number of responses received from origin with status code 206 (Partial Content). | 
**origin_status_301** | Option<**i64**> | Number of responses received from origin with status code 301 (Moved Permanently). | 
**origin_status_302** | Option<**i64**> | Number of responses received from origin with status code 302 (Found). | 
**origin_status_304** | Option<**i64**> | Number of responses received from origin with status code 304 (Not Modified). | 
**origin_status_400** | Option<**i64**> | Number of responses received from origin with status code 400 (Bad Request). | 
**origin_status_401** | Option<**i64**> | Number of responses received from origin with status code 401 (Unauthorized). | 
**origin_status_403** | Option<**i64**> | Number of responses received from origin with status code 403 (Forbidden). | 
**origin_status_404** | Option<**i64**> | Number of responses received from origin with status code 404 (Not Found). | 
**origin_status_416** | Option<**i64**> | Number of responses received from origin with status code 416 (Range Not Satisfiable). | 
**origin_status_429** | Option<**i64**> | Number of responses received from origin with status code 429 (Too Many Requests). | 
**origin_status_500** | Option<**i64**> | Number of responses received from origin with status code 500 (Internal Server Error). | 
**origin_status_501** | Option<**i64**> | Number of responses received from origin with status code 501 (Not Implemented). | 
**origin_status_502** | Option<**i64**> | Number of responses received from origin with status code 502 (Bad Gateway). | 
**origin_status_503** | Option<**i64**> | Number of responses received from origin with status code 503 (Service Unavailable). | 
**origin_status_504** | Option<**i64**> | Number of responses received from origin with status code 504 (Gateway Timeout). | 
**origin_status_505** | Option<**i64**> | Number of responses received from origin with status code 505 (HTTP Version Not Supported). | 
**origin_status_530** | Option<**i64**> | Number of responses received from origin with status code 530. | 
**origin_status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes received from origin. | 
**origin_status_2xx** | Option<**i64**> | Number of \"Success\" status codes received from origin. | 
**origin_status_3xx** | Option<**i64**> | Number of \"Redirection\" codes received from origin. | 
**origin_status_4xx** | Option<**i64**> | Number of \"Client Error\" codes received from origin. | 
**origin_status_5xx** | Option<**i64**> | Number of \"Server Error\" codes received from origin. | 
**compute_bereq_body_bytes** | Option<**i64**> | Total body bytes sent to backends (origins) by the Compute platform. | 
**compute_bereq_errors** | Option<**i64**> | Number of backend request errors, including timeouts, by the Compute platform. | 
**compute_bereq_header_bytes** | Option<**i64**> | Total header bytes sent to backends (origins) by the Compute platform. | 
**compute_bereqs** | Option<**i64**> | Number of backend requests started by the Compute platform. | 
**compute_beresp_body_bytes** | Option<**i64**> | Total body bytes received from backends (origins) by the Compute platform. | 
**compute_beresp_header_bytes** | Option<**i64**> | Total header bytes received from backends (origins) by the Compute platform. | 
**compute_execution_time_ms** | Option<**i64**> | The amount of active CPU time used to process your requests (in milliseconds). | 
**compute_origin_status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes received from origin by the Compute platform. | 
**compute_origin_status_200** | Option<**i64**> | Number of responses received from origin with status code 200 (Success) by the Compute platform. | 
**compute_origin_status_204** | Option<**i64**> | Number of responses received from origin with status code 204 (No Content) by the Compute platform. | 
**compute_origin_status_206** | Option<**i64**> | Number of responses received from origin with status code 206 (Partial Content) by the Compute platform. | 
**compute_origin_status_2xx** | Option<**i64**> | Number of \"Success\" status codes received from origin by the Compute platform. | 
**compute_origin_status_301** | Option<**i64**> | Number of responses received from origin with status code 301 (Moved Permanently) by the Compute platform. | 
**compute_origin_status_302** | Option<**i64**> | Number of responses received from origin with status code 302 (Found) by the Compute platform. | 
**compute_origin_status_304** | Option<**i64**> | Number of responses received from origin with status code 304 (Not Modified) by the Compute platform. | 
**compute_origin_status_3xx** | Option<**i64**> | Number of \"Redirection\" codes received from origin by the Compute platform. | 
**compute_origin_status_400** | Option<**i64**> | Number of responses received from origin with status code 400 (Bad Request) by the Compute platform. | 
**compute_origin_status_401** | Option<**i64**> | Number of responses received from origin with status code 401 (Unauthorized) by the Compute platform. | 
**compute_origin_status_403** | Option<**i64**> | Number of responses received from origin with status code 403 (Forbidden) by the Compute platform. | 
**compute_origin_status_404** | Option<**i64**> | Number of responses received from origin with status code 404 (Not Found) by the Compute platform. | 
**compute_origin_status_416** | Option<**i64**> | Number of responses received from origin with status code 416 (Range Not Satisfiable) by the Compute platform. | 
**compute_origin_status_429** | Option<**i64**> | Number of responses received from origin with status code 429 (Too Many Requests) by the Compute platform. | 
**compute_origin_status_4xx** | Option<**i64**> | Number of \"Client Error\" codes received from origin by the Compute platform. | 
**compute_origin_status_500** | Option<**i64**> | Number of responses received from origin with status code 500 (Internal Server Error) by the Compute platform. | 
**compute_origin_status_501** | Option<**i64**> | Number of responses received from origin with status code 501 (Not Implemented) by the Compute platform. | 
**compute_origin_status_502** | Option<**i64**> | Number of responses received from origin with status code 502 (Bad Gateway) by the Compute platform. | 
**compute_origin_status_503** | Option<**i64**> | Number of responses received from origin with status code 503 (Service Unavailable) by the Compute platform. | 
**compute_origin_status_504** | Option<**i64**> | Number of responses received from origin with status code 504 (Gateway Timeout) by the Compute platform. | 
**compute_origin_status_505** | Option<**i64**> | Number of responses received from origin with status code 505 (HTTP Version Not Supported) by the Compute platform. | 
**compute_origin_status_530** | Option<**i64**> | Number of responses received from origin with status code 530 by the Compute platform. | 
**compute_origin_status_5xx** | Option<**i64**> | Number of \"Server Error\" codes received from origin by the Compute platform. | 
**compute_req_body_bytes** | Option<**i64**> | Total body bytes received by the Compute platform. | 
**compute_req_header_bytes** | Option<**i64**> | Total header bytes received by the Compute platform. | 
**compute_request_time_billed_ms** | Option<**i64**> | The total amount of request processing time you will be billed for, measured in 50 millisecond increments. | 
**compute_request_time_ms** | Option<**i64**> | The total amount of time used to process your requests, including active CPU time (in milliseconds). | 
**compute_request** | Option<**i64**> | The total number of requests that were received by the Compute platform. | 
**compute_resp_body_bytes** | Option<**i64**> | Total body bytes sent from Compute to the end user. | 
**compute_resp_header_bytes** | Option<**i64**> | Total header bytes sent from Compute to the end user. | 
**compute_resp_status_103** | Option<**i64**> | Number of responses delivered with status code 103 (Early Hints) by the Compute platform. | 
**compute_resp_status_1xx** | Option<**i64**> | Number of 1xx \"Informational\" category status codes delivered by the Compute platform. | 
**compute_resp_status_200** | Option<**i64**> | Number of responses delivered with status code 200 (Success) by the Compute platform. | 
**compute_resp_status_204** | Option<**i64**> | Number of responses delivered with status code 204 (No Content) by the Compute platform. | 
**compute_resp_status_206** | Option<**i64**> | Number of responses delivered with status code 206 (Partial Content) by the Compute platform. | 
**compute_resp_status_2xx** | Option<**i64**> | Number of 2xx \"Success\" status codes delivered by the Compute platform. | 
**compute_resp_status_301** | Option<**i64**> | Number of responses delivered with status code 301 (Moved Permanently) by the Compute platform. | 
**compute_resp_status_302** | Option<**i64**> | Number of responses delivered with status code 302 (Found) by the Compute platform. | 
**compute_resp_status_304** | Option<**i64**> | Number of responses delivered with status code 304 (Not Modified) by the Compute platform. | 
**compute_resp_status_3xx** | Option<**i64**> | Number of 3xx \"Redirection\" codes delivered by the Compute platform. | 
**compute_resp_status_400** | Option<**i64**> | Number of responses delivered with status code 400 (Bad Request) by the Compute platform. | 
**compute_resp_status_401** | Option<**i64**> | Number of responses delivered with status code 401 (Unauthorized) by the Compute platform. | 
**compute_resp_status_403** | Option<**i64**> | Number of responses delivered with status code 403 (Forbidden) by the Compute platform. | 
**compute_resp_status_404** | Option<**i64**> | Number of responses delivered with status code 404 (Not Found) by the Compute platform. | 
**compute_resp_status_416** | Option<**i64**> | Number of responses delivered with status code 416 (Range Not Satisfiable) by the Compute platform. | 
**compute_resp_status_429** | Option<**i64**> | Number of responses delivered with status code 429 (Too Many Requests) by the Compute platform. | 
**compute_resp_status_4xx** | Option<**i64**> | Number of 4xx \"Client Error\" codes delivered by the Compute platform. | 
**compute_resp_status_500** | Option<**i64**> | Number of responses delivered with status code 500 (Internal Server Error) by the Compute platform. | 
**compute_resp_status_501** | Option<**i64**> | Number of responses delivered with status code 501 (Not Implemented) by the Compute platform. | 
**compute_resp_status_502** | Option<**i64**> | Number of responses delivered with status code 502 (Bad Gateway) by the Compute platform. | 
**compute_resp_status_503** | Option<**i64**> | Number of responses delivered with status code 503 (Service Unavailable) by the Compute platform. | 
**compute_resp_status_504** | Option<**i64**> | Number of responses delivered with status code 504 (Gateway Timeout) by the Compute platform. | 
**compute_resp_status_505** | Option<**i64**> | Number of responses delivered with status code 505 (HTTP Version Not Supported) by the Compute platform. | 
**compute_resp_status_530** | Option<**i64**> | Number of responses delivered with status code 530 by the Compute platform. | 
**compute_resp_status_5xx** | Option<**i64**> | Number of \"Server Error\" category status codes delivered by the Compute platform. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


