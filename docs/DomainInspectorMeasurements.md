# DomainInspectorMeasurements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**edge_requests** | Option<**i32**> | Number of requests sent by end users to Fastly. | 
**edge_resp_header_bytes** | Option<**i32**> | Total header bytes delivered from Fastly to the end user. | 
**edge_resp_body_bytes** | Option<**i32**> | Total body bytes delivered from Fastly to the end user. | 
**status_1xx** | Option<**i32**> | Number of 1xx \"Informational\" category status codes delivered. | 
**status_2xx** | Option<**i32**> | Number of 2xx \"Success\" status codes delivered. | 
**status_3xx** | Option<**i32**> | Number of 3xx \"Redirection\" codes delivered. | 
**status_4xx** | Option<**i32**> | Number of 4xx \"Client Error\" codes delivered. | 
**status_5xx** | Option<**i32**> | Number of 5xx \"Server Error\" codes delivered. | 
**status_200** | Option<**i32**> | Number of responses delivered with status code 200 (Success). | 
**status_204** | Option<**i32**> | Number of responses delivered with status code 204 (No Content). | 
**status_206** | Option<**i32**> | Number of responses delivered with status code 206 (Partial Content). | 
**status_301** | Option<**i32**> | Number of responses delivered with status code 301 (Moved Permanently). | 
**status_302** | Option<**i32**> | Number of responses delivered with status code 302 (Found). | 
**status_304** | Option<**i32**> | Number of responses delivered with status code 304 (Not Modified). | 
**status_400** | Option<**i32**> | Number of responses delivered with status code 400 (Bad Request). | 
**status_401** | Option<**i32**> | Number of responses delivered with status code 401 (Unauthorized). | 
**status_403** | Option<**i32**> | Number of responses delivered with status code 403 (Forbidden). | 
**status_404** | Option<**i32**> | Number of responses delivered with status code 404 (Not Found). | 
**status_416** | Option<**i32**> | Number of responses delivered with status code 416 (Range Not Satisfiable). | 
**status_429** | Option<**i32**> | Number of responses delivered with status code 429 (Too Many Requests). | 
**status_500** | Option<**i32**> | Number of responses delivered with status code 500 (Internal Server Error). | 
**status_501** | Option<**i32**> | Number of responses delivered with status code 501 (Not Implemented). | 
**status_502** | Option<**i32**> | Number of responses delivered with status code 502 (Bad Gateway). | 
**status_503** | Option<**i32**> | Number of responses delivered with status code 503 (Service Unavailable). | 
**status_504** | Option<**i32**> | Number of responses delivered with status code 504 (Gateway Timeout). | 
**status_505** | Option<**i32**> | Number of responses delivered with status code 505 (HTTP Version Not Supported). | 
**requests** | Option<**i32**> | Number of requests processed. | 
**resp_header_bytes** | Option<**i32**> | Total header bytes delivered. | 
**resp_body_bytes** | Option<**i32**> | Total body bytes delivered. | 
**bereq_header_bytes** | Option<**i32**> | Total header bytes sent to origin. | 
**bereq_body_bytes** | Option<**i32**> | Total body bytes sent to origin. | 
**edge_hit_requests** | Option<**i32**> | Number of requests sent by end users to Fastly that resulted in a hit at the edge. | 
**edge_miss_requests** | Option<**i32**> | Number of requests sent by end users to Fastly that resulted in a miss at the edge. | 
**origin_fetches** | Option<**i32**> | Number of requests sent to origin. | 
**origin_fetch_resp_header_bytes** | Option<**i32**> | Total header bytes received from origin. | 
**origin_fetch_resp_body_bytes** | Option<**i32**> | Total body bytes received from origin. | 
**bandwidth** | Option<**i32**> | Total bytes delivered (`resp_header_bytes` + `resp_body_bytes` + `bereq_header_bytes` + `bereq_body_bytes`). | 
**edge_hit_ratio** | Option<**f32**> | Ratio of cache hits to cache misses at the edge, between 0 and 1 (`edge_hit_requests` / (`edge_hit_requests` + `edge_miss_requests`)). | 
**origin_offload** | Option<**f32**> | Ratio of response bytes delivered from the edge compared to what is delivered from origin, between 0 and 1. (`edge_resp_body_bytes` + `edge_resp_header_bytes`) / (`origin_fetch_resp_body_bytes` + `origin_fetch_resp_header_bytes` + `edge_resp_body_bytes` + `edge_resp_header_bytes`). | 
**origin_status_200** | Option<**i32**> | Number of responses received from origin with status code 200 (Success). | 
**origin_status_204** | Option<**i32**> | Number of responses received from origin with status code 204 (No Content). | 
**origin_status_206** | Option<**i32**> | Number of responses received from origin with status code 206 (Partial Content). | 
**origin_status_301** | Option<**i32**> | Number of responses received from origin with status code 301 (Moved Permanently). | 
**origin_status_302** | Option<**i32**> | Number of responses received from origin with status code 302 (Found). | 
**origin_status_304** | Option<**i32**> | Number of responses received from origin with status code 304 (Not Modified). | 
**origin_status_400** | Option<**i32**> | Number of responses received from origin with status code 400 (Bad Request). | 
**origin_status_401** | Option<**i32**> | Number of responses received from origin with status code 401 (Unauthorized). | 
**origin_status_403** | Option<**i32**> | Number of responses received from origin with status code 403 (Forbidden). | 
**origin_status_404** | Option<**i32**> | Number of responses received from origin with status code 404 (Not Found). | 
**origin_status_416** | Option<**i32**> | Number of responses received from origin with status code 416 (Range Not Satisfiable). | 
**origin_status_429** | Option<**i32**> | Number of responses received from origin with status code 429 (Too Many Requests). | 
**origin_status_500** | Option<**i32**> | Number of responses received from origin with status code 500 (Internal Server Error). | 
**origin_status_501** | Option<**i32**> | Number of responses received from origin with status code 501 (Not Implemented). | 
**origin_status_502** | Option<**i32**> | Number of responses received from origin with status code 502 (Bad Gateway). | 
**origin_status_503** | Option<**i32**> | Number of responses received from origin with status code 503 (Service Unavailable). | 
**origin_status_504** | Option<**i32**> | Number of responses received from origin with status code 504 (Gateway Timeout). | 
**origin_status_505** | Option<**i32**> | Number of responses received from origin with status code 505 (HTTP Version Not Supported). | 
**origin_status_1xx** | Option<**i32**> | Number of \"Informational\" category status codes received from origin. | 
**origin_status_2xx** | Option<**i32**> | Number of \"Success\" status codes received from origin. | 
**origin_status_3xx** | Option<**i32**> | Number of \"Redirection\" codes received from origin. | 
**origin_status_4xx** | Option<**i32**> | Number of \"Client Error\" codes received from origin. | 
**origin_status_5xx** | Option<**i32**> | Number of \"Server Error\" codes received from origin. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


