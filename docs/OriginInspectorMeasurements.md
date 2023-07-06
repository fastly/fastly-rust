# OriginInspectorMeasurements

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**responses** | Option<**i32**> | Number of responses from origin. | 
**resp_header_bytes** | Option<**i32**> | Number of header bytes from origin. | 
**resp_body_bytes** | Option<**i32**> | Number of body bytes from origin. | 
**status_1xx** | Option<**i32**> | Number of 1xx \"Informational\" status codes delivered from origin. | 
**status_2xx** | Option<**i32**> | Number of 2xx \"Success\" status codes delivered from origin. | 
**status_3xx** | Option<**i32**> | Number of 3xx \"Redirection\" codes delivered from origin. | 
**status_4xx** | Option<**i32**> | Number of 4xx \"Client Error\" codes delivered from origin. | 
**status_5xx** | Option<**i32**> | Number of 5xx \"Server Error\" codes delivered from origin. | 
**status_200** | Option<**i32**> | Number of responses received with status code 200 (Success) from origin. | 
**status_204** | Option<**i32**> | Number of responses received with status code 204 (No Content) from origin. | 
**status_206** | Option<**i32**> | Number of responses received with status code 206 (Partial Content) from origin. | 
**status_301** | Option<**i32**> | Number of responses received with status code 301 (Moved Permanently) from origin. | 
**status_302** | Option<**i32**> | Number of responses received with status code 302 (Found) from origin. | 
**status_304** | Option<**i32**> | Number of responses received with status code 304 (Not Modified) from origin. | 
**status_400** | Option<**i32**> | Number of responses received with status code 400 (Bad Request) from origin. | 
**status_401** | Option<**i32**> | Number of responses received with status code 401 (Unauthorized) from origin. | 
**status_403** | Option<**i32**> | Number of responses received with status code 403 (Forbidden) from origin. | 
**status_404** | Option<**i32**> | Number of responses received with status code 404 (Not Found) from origin. | 
**status_416** | Option<**i32**> | Number of responses received with status code 416 (Range Not Satisfiable) from origin. | 
**status_429** | Option<**i32**> | Number of responses received with status code 429 (Too Many Requests) from origin. | 
**status_500** | Option<**i32**> | Number of responses received with status code 500 (Internal Server Error) from origin. | 
**status_501** | Option<**i32**> | Number of responses received with status code 501 (Not Implemented) from origin. | 
**status_502** | Option<**i32**> | Number of responses received with status code 502 (Bad Gateway) from origin. | 
**status_503** | Option<**i32**> | Number of responses received with status code 503 (Service Unavailable) from origin. | 
**status_504** | Option<**i32**> | Number of responses received with status code 504 (Gateway Timeout) from origin. | 
**status_505** | Option<**i32**> | Number of responses received with status code 505 (HTTP Version Not Supported) from origin. | 
**latency_0_to_1ms** | Option<**i32**> | Number of responses from origin with latency between 0 and 1 millisecond. | 
**latency_1_to_5ms** | Option<**i32**> | Number of responses from origin with latency between 1 and 5 milliseconds. | 
**latency_5_to_10ms** | Option<**i32**> | Number of responses from origin with latency between 5 and 10 milliseconds. | 
**latency_10_to_50ms** | Option<**i32**> | Number of responses from origin with latency between 10 and 50 milliseconds. | 
**latency_50_to_100ms** | Option<**i32**> | Number of responses from origin with latency between 50 and 100 milliseconds. | 
**latency_100_to_250ms** | Option<**i32**> | Number of responses from origin with latency between 100 and 250 milliseconds. | 
**latency_250_to_500ms** | Option<**i32**> | Number of responses from origin with latency between 250 and 500 milliseconds. | 
**latency_500_to_1000ms** | Option<**i32**> | Number of responses from origin with latency between 500 and 1,000 milliseconds. | 
**latency_1000_to_5000ms** | Option<**i32**> | Number of responses from origin with latency between 1,000 and 5,000 milliseconds. | 
**latency_5000_to_10000ms** | Option<**i32**> | Number of responses from origin with latency between 5,000 and 10,000 milliseconds. | 
**latency_10000_to_60000ms** | Option<**i32**> | Number of responses from origin with latency between 10,000 and 60,000 milliseconds. | 
**latency_60000ms** | Option<**i32**> | Number of responses from origin with latency of 60,000 milliseconds and above. | 
**waf_responses** | Option<**i32**> | Number of responses received for origin requests made by the Fastly WAF. | 
**waf_resp_header_bytes** | Option<**i32**> | Number of header bytes received for origin requests made by the Fastly WAF. | 
**waf_resp_body_bytes** | Option<**i32**> | Number of body bytes received for origin requests made by the Fastly WAF. | 
**waf_status_1xx** | Option<**i32**> | Number of 1xx \"Informational\" status codes received for origin requests made by the Fastly WAF. | 
**waf_status_2xx** | Option<**i32**> | Number of 2xx \"Success\" status codes received for origin requests made by the Fastly WAF. | 
**waf_status_3xx** | Option<**i32**> | Number of 3xx \"Redirection\" codes received for origin requests made by the Fastly WAF. | 
**waf_status_4xx** | Option<**i32**> | Number of 4xx \"Client Error\" codes received for origin requests made by the Fastly WAF. | 
**waf_status_5xx** | Option<**i32**> | Number of 5xx \"Server Error\" codes received for origin requests made by the Fastly WAF. | 
**waf_status_200** | Option<**i32**> | Number of responses received with status code 200 (Success) received for origin requests made by the Fastly WAF. | 
**waf_status_204** | Option<**i32**> | Number of responses received with status code 204 (No Content) received for origin requests made by the Fastly WAF. | 
**waf_status_206** | Option<**i32**> | Number of responses received with status code 206 (Partial Content) received for origin requests made by the Fastly WAF. | 
**waf_status_301** | Option<**i32**> | Number of responses received with status code 301 (Moved Permanently) received for origin requests made by the Fastly WAF. | 
**waf_status_302** | Option<**i32**> | Number of responses received with status code 302 (Found) received for origin requests made by the Fastly WAF. | 
**waf_status_304** | Option<**i32**> | Number of responses received with status code 304 (Not Modified) received for origin requests made by the Fastly WAF. | 
**waf_status_400** | Option<**i32**> | Number of responses received with status code 400 (Bad Request) received for origin requests made by the Fastly WAF. | 
**waf_status_401** | Option<**i32**> | Number of responses received with status code 401 (Unauthorized) received for origin requests made by the Fastly WAF. | 
**waf_status_403** | Option<**i32**> | Number of responses received with status code 403 (Forbidden) received for origin requests made by the Fastly WAF. | 
**waf_status_404** | Option<**i32**> | Number of responses received with status code 404 (Not Found) received for origin requests made by the Fastly WAF. | 
**waf_status_416** | Option<**i32**> | Number of responses received with status code 416 (Range Not Satisfiable) received for origin requests made by the Fastly WAF. | 
**waf_status_429** | Option<**i32**> | Number of responses received with status code 429 (Too Many Requests) received for origin requests made by the Fastly WAF. | 
**waf_status_500** | Option<**i32**> | Number of responses received with status code 500 (Internal Server Error) received for origin requests made by the Fastly WAF. | 
**waf_status_501** | Option<**i32**> | Number of responses received with status code 501 (Not Implemented) received for origin requests made by the Fastly WAF. | 
**waf_status_502** | Option<**i32**> | Number of responses received with status code 502 (Bad Gateway) received for origin requests made by the Fastly WAF. | 
**waf_status_503** | Option<**i32**> | Number of responses received with status code 503 (Service Unavailable) received for origin requests made by the Fastly WAF. | 
**waf_status_504** | Option<**i32**> | Number of responses received with status code 504 (Gateway Timeout) received for origin requests made by the Fastly WAF. | 
**waf_status_505** | Option<**i32**> | Number of responses received with status code 505 (HTTP Version Not Supported) received for origin requests made by the Fastly WAF. | 
**waf_latency_0_to_1ms** | Option<**i32**> | Number of responses with latency between 0 and 1 millisecond received for origin requests made by the Fastly WAF. | 
**waf_latency_1_to_5ms** | Option<**i32**> | Number of responses with latency between 1 and 5 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_5_to_10ms** | Option<**i32**> | Number of responses with latency between 5 and 10 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_10_to_50ms** | Option<**i32**> | Number of responses with latency between 10 and 50 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_50_to_100ms** | Option<**i32**> | Number of responses with latency between 50 and 100 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_100_to_250ms** | Option<**i32**> | Number of responses with latency between 100 and 250 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_250_to_500ms** | Option<**i32**> | Number of responses with latency between 250 and 500 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_500_to_1000ms** | Option<**i32**> | Number of responses with latency between 500 and 1,000 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_1000_to_5000ms** | Option<**i32**> | Number of responses with latency between 1,000 and 5,000 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_5000_to_10000ms** | Option<**i32**> | Number of responses with latency between 5,000 and 10,000 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_10000_to_60000ms** | Option<**i32**> | Number of responses with latency between 10,000 and 60,000 milliseconds received for origin requests made by the Fastly WAF. | 
**waf_latency_60000ms** | Option<**i32**> | Number of responses with latency of 60,000 milliseconds and above received for origin requests made by the Fastly WAF. | 
**compute_responses** | Option<**i32**> | Number of responses for origin received by Compute@Edge. | 
**compute_resp_header_bytes** | Option<**i32**> | Number of header bytes for origin received by Compute@Edge. | 
**compute_resp_body_bytes** | Option<**i32**> | Number of body bytes for origin received by Compute@Edge. | 
**compute_status_1xx** | Option<**i32**> | Number of 1xx \"Informational\" status codes for origin received by Compute@Edge. | 
**compute_status_2xx** | Option<**i32**> | Number of 2xx \"Success\" status codes for origin received by Compute@Edge. | 
**compute_status_3xx** | Option<**i32**> | Number of 3xx \"Redirection\" codes for origin received by Compute@Edge. | 
**compute_status_4xx** | Option<**i32**> | Number of 4xx \"Client Error\" codes for origin received by Compute@Edge. | 
**compute_status_5xx** | Option<**i32**> | Number of 5xx \"Server Error\" codes for origin received by Compute@Edge. | 
**compute_status_200** | Option<**i32**> | Number of responses received with status code 200 (Success) for origin received by Compute@Edge. | 
**compute_status_204** | Option<**i32**> | Number of responses received with status code 204 (No Content) for origin received by Compute@Edge. | 
**compute_status_206** | Option<**i32**> | Number of responses received with status code 206 (Partial Content) for origin received by Compute@Edge. | 
**compute_status_301** | Option<**i32**> | Number of responses received with status code 301 (Moved Permanently) for origin received by Compute@Edge. | 
**compute_status_302** | Option<**i32**> | Number of responses received with status code 302 (Found) for origin received by Compute@Edge. | 
**compute_status_304** | Option<**i32**> | Number of responses received with status code 304 (Not Modified) for origin received by Compute@Edge. | 
**compute_status_400** | Option<**i32**> | Number of responses received with status code 400 (Bad Request) for origin received by Compute@Edge. | 
**compute_status_401** | Option<**i32**> | Number of responses received with status code 401 (Unauthorized) for origin received by Compute@Edge. | 
**compute_status_403** | Option<**i32**> | Number of responses received with status code 403 (Forbidden) for origin received by Compute@Edge. | 
**compute_status_404** | Option<**i32**> | Number of responses received with status code 404 (Not Found) for origin received by Compute@Edge. | 
**compute_status_416** | Option<**i32**> | Number of responses received with status code 416 (Range Not Satisfiable) for origin received by Compute@Edge. | 
**compute_status_429** | Option<**i32**> | Number of responses received with status code 429 (Too Many Requests) for origin received by Compute@Edge. | 
**compute_status_500** | Option<**i32**> | Number of responses received with status code 500 (Internal Server Error) for origin received by Compute@Edge. | 
**compute_status_501** | Option<**i32**> | Number of responses received with status code 501 (Not Implemented) for origin received by Compute@Edge. | 
**compute_status_502** | Option<**i32**> | Number of responses received with status code 502 (Bad Gateway) for origin received by Compute@Edge. | 
**compute_status_503** | Option<**i32**> | Number of responses received with status code 503 (Service Unavailable) for origin received by Compute@Edge. | 
**compute_status_504** | Option<**i32**> | Number of responses received with status code 504 (Gateway Timeout) for origin received by Compute@Edge. | 
**compute_status_505** | Option<**i32**> | Number of responses received with status code 505 (HTTP Version Not Supported) for origin received by Compute@Edge. | 
**compute_latency_0_to_1ms** | Option<**i32**> | Number of responses with latency between 0 and 1 millisecond for origin received by Compute@Edge. | 
**compute_latency_1_to_5ms** | Option<**i32**> | Number of responses with latency between 1 and 5 milliseconds for origin received by Compute@Edge. | 
**compute_latency_5_to_10ms** | Option<**i32**> | Number of responses with latency between 5 and 10 milliseconds for origin received by Compute@Edge. | 
**compute_latency_10_to_50ms** | Option<**i32**> | Number of responses with latency between 10 and 50 milliseconds for origin received by Compute@Edge. | 
**compute_latency_50_to_100ms** | Option<**i32**> | Number of responses with latency between 50 and 100 milliseconds for origin received by Compute@Edge. | 
**compute_latency_100_to_250ms** | Option<**i32**> | Number of responses with latency between 100 and 250 milliseconds for origin received by Compute@Edge. | 
**compute_latency_250_to_500ms** | Option<**i32**> | Number of responses with latency between 250 and 500 milliseconds for origin received by Compute@Edge. | 
**compute_latency_500_to_1000ms** | Option<**i32**> | Number of responses with latency between 500 and 1,000 milliseconds for origin received by Compute@Edge. | 
**compute_latency_1000_to_5000ms** | Option<**i32**> | Number of responses with latency between 1,000 and 5,000 milliseconds for origin received by Compute@Edge. | 
**compute_latency_5000_to_10000ms** | Option<**i32**> | Number of responses with latency between 5,000 and 10,000 milliseconds for origin received by Compute@Edge. | 
**compute_latency_10000_to_60000ms** | Option<**i32**> | Number of responses with latency between 10,000 and 60,000 milliseconds for origin received by Compute@Edge. | 
**compute_latency_60000ms** | Option<**i32**> | Number of responses with latency of 60,000 milliseconds and above for origin received by Compute@Edge. | 
**all_responses** | Option<**i32**> | Number of responses received for origin requests made by all sources. | 
**all_resp_header_bytes** | Option<**i32**> | Number of header bytes received for origin requests made by all sources. | 
**all_resp_body_bytes** | Option<**i32**> | Number of body bytes received for origin requests made by all sources. | 
**all_status_1xx** | Option<**i32**> | Number of 1xx \"Informational\" category status codes delivered received for origin requests made by all sources. | 
**all_status_2xx** | Option<**i32**> | Number of 2xx \"Success\" status codes received for origin requests made by all sources. | 
**all_status_3xx** | Option<**i32**> | Number of 3xx \"Redirection\" codes received for origin requests made by all sources. | 
**all_status_4xx** | Option<**i32**> | Number of 4xx \"Client Error\" codes received for origin requests made by all sources. | 
**all_status_5xx** | Option<**i32**> | Number of 5xx \"Server Error\" codes received for origin requests made by all sources. | 
**all_status_200** | Option<**i32**> | Number of responses received with status code 200 (Success) received for origin requests made by all sources. | 
**all_status_204** | Option<**i32**> | Number of responses received with status code 204 (No Content) received for origin requests made by all sources. | 
**all_status_206** | Option<**i32**> | Number of responses received with status code 206 (Partial Content) received for origin requests made by all sources. | 
**all_status_301** | Option<**i32**> | Number of responses received with status code 301 (Moved Permanently) received for origin requests made by all sources. | 
**all_status_302** | Option<**i32**> | Number of responses received with status code 302 (Found) received for origin requests made by all sources. | 
**all_status_304** | Option<**i32**> | Number of responses received with status code 304 (Not Modified) received for origin requests made by all sources. | 
**all_status_400** | Option<**i32**> | Number of responses received with status code 400 (Bad Request) received for origin requests made by all sources. | 
**all_status_401** | Option<**i32**> | Number of responses received with status code 401 (Unauthorized) received for origin requests made by all sources. | 
**all_status_403** | Option<**i32**> | Number of responses received with status code 403 (Forbidden) received for origin requests made by all sources. | 
**all_status_404** | Option<**i32**> | Number of responses received with status code 404 (Not Found) received for origin requests made by all sources. | 
**all_status_416** | Option<**i32**> | Number of responses received with status code 416 (Range Not Satisfiable) received for origin requests made by all sources. | 
**all_status_429** | Option<**i32**> | Number of responses received with status code 429 (Too Many Requests) received for origin requests made by all sources. | 
**all_status_500** | Option<**i32**> | Number of responses received with status code 500 (Internal Server Error) received for origin requests made by all sources. | 
**all_status_501** | Option<**i32**> | Number of responses received with status code 501 (Not Implemented) received for origin requests made by all sources. | 
**all_status_502** | Option<**i32**> | Number of responses received with status code 502 (Bad Gateway) received for origin requests made by all sources. | 
**all_status_503** | Option<**i32**> | Number of responses received with status code 503 (Service Unavailable) received for origin requests made by all sources. | 
**all_status_504** | Option<**i32**> | Number of responses received with status code 504 (Gateway Timeout) received for origin requests made by all sources. | 
**all_status_505** | Option<**i32**> | Number of responses received with status code 505 (HTTP Version Not Supported) received for origin requests made by all sources. | 
**all_latency_0_to_1ms** | Option<**i32**> | Number of responses with latency between 0 and 1 millisecond received for origin requests made by all sources. | 
**all_latency_1_to_5ms** | Option<**i32**> | Number of responses with latency between 1 and 5 milliseconds received for origin requests made by all sources. | 
**all_latency_5_to_10ms** | Option<**i32**> | Number of responses with latency between 5 and 10 milliseconds received for origin requests made by all sources. | 
**all_latency_10_to_50ms** | Option<**i32**> | Number of responses with latency between 10 and 50 milliseconds received for origin requests made by all sources. | 
**all_latency_50_to_100ms** | Option<**i32**> | Number of responses with latency between 50 and 100 milliseconds received for origin requests made by all sources. | 
**all_latency_100_to_250ms** | Option<**i32**> | Number of responses with latency between 100 and 250 milliseconds received for origin requests made by all sources. | 
**all_latency_250_to_500ms** | Option<**i32**> | Number of responses with latency between 250 and 500 milliseconds received for origin requests made by all sources. | 
**all_latency_500_to_1000ms** | Option<**i32**> | Number of responses with latency between 500 and 1,000 milliseconds received for origin requests made by all sources. | 
**all_latency_1000_to_5000ms** | Option<**i32**> | Number of responses with latency between 1,000 and 5,000 milliseconds received for origin requests made by all sources. | 
**all_latency_5000_to_10000ms** | Option<**i32**> | Number of responses with latency between 5,000 and 10,000 milliseconds received for origin requests made by all sources. | 
**all_latency_10000_to_60000ms** | Option<**i32**> | Number of responses with latency between 10,000 and 60,000 milliseconds received for origin requests made by all sources. | 
**all_latency_60000ms** | Option<**i32**> | Number of responses with latency of 60,000 milliseconds and above received for origin requests made by all sources. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


