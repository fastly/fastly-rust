# RealtimeEntryAggregated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**requests** | Option<**i64**> | Number of requests processed. | 
**logging** | Option<**i64**> | Number of log lines sent (alias for `log`). | 
**log** | Option<**i64**> | Number of log lines sent. | 
**resp_header_bytes** | Option<**i64**> | Total header bytes delivered (edge_resp_header_bytes + shield_resp_header_bytes). | 
**header_size** | Option<**i64**> | Total header bytes delivered (alias for resp_header_bytes). | 
**resp_body_bytes** | Option<**i64**> | Total body bytes delivered (edge_resp_body_bytes + shield_resp_body_bytes). | 
**body_size** | Option<**i64**> | Total body bytes delivered (alias for resp_body_bytes). | 
**hits** | Option<**i64**> | Number of cache hits. | 
**miss** | Option<**i64**> | Number of cache misses. | 
**pass** | Option<**i64**> | Number of requests that passed through the CDN without being cached. | 
**synth** | Option<**i64**> | Number of requests that returned a synthetic response (i.e., response objects created with the `synthetic` VCL statement). | 
**errors** | Option<**i64**> | Number of cache errors. | 
**hits_time** | Option<**f32**> | Total amount of time spent processing cache hits (in seconds). | 
**miss_time** | Option<**f32**> | Total amount of time spent processing cache misses (in seconds). | 
**miss_histogram** | Option<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](SerdeJsonValue.md)> | A histogram. The value in each bucket is the number of requests to the origin whose responses arrived during the time period represented by the bucket. The key of each bucket represents the upper bound (in response time) of that bucket. The buckets vary in width and cover the time periods 0-10ms (in 1ms increments), 10-250ms (in 10ms increments), 250-1,000ms (in 50ms increments), 1,000-3,000ms (in 100ms increments), 3,000-10,000ms (in 500 ms increments), 10,000-20,000ms (in 1,000ms increments), 20,000-60,000ms (in 5,000ms increments), and 60,000ms through infinity (in a single bucket). | 
**compute_requests** | Option<**i64**> | The total number of requests that were received for your service by Fastly. | 
**compute_execution_time_ms** | Option<**f32**> | The amount of active CPU time used to process your requests (in milliseconds). | 
**compute_ram_used** | Option<**i64**> | The amount of RAM used for your service by Fastly (in bytes). | 
**compute_request_time_ms** | Option<**f32**> | The total, actual amount of time used to process your requests, including active CPU time (in milliseconds). | 
**compute_request_time_billed_ms** | Option<**f32**> | The total amount of request processing time you will be billed for, measured in 50 millisecond increments. | 
**shield** | Option<**i64**> | Number of requests from edge to the shield POP. | 
**ipv6** | Option<**i64**> | Number of requests that were received over IPv6. | 
**imgopto** | Option<**i64**> | Number of responses that came from the Fastly Image Optimizer service. If the service receives 10 requests for an image, this stat will be 10 regardless of how many times the image was transformed. | 
**imgopto_shield** | Option<**i64**> | Number of responses that came from the Fastly Image Optimizer service via a shield. | 
**imgopto_transforms** | Option<**i64**> | Number of transforms performed by the Fastly Image Optimizer service. | 
**otfp** | Option<**i64**> | Number of responses that came from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield** | Option<**i64**> | Number of responses that came from the Fastly On-the-Fly Packaging service for video-on-demand via a shield. | 
**otfp_manifests** | Option<**i64**> | Number of responses that were manifest files from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**video** | Option<**i64**> | Number of responses with the video segment or video manifest MIME type (i.e., application/x-mpegurl, application/vnd.apple.mpegurl, application/f4m, application/dash+xml, application/vnd.ms-sstr+xml, ideo/mp2t, audio/aac, video/f4f, video/x-flv, video/mp4, audio/mp4). | 
**pci** | Option<**i64**> | Number of responses with the PCI flag turned on. | 
**http2** | Option<**i64**> | Number of requests received over HTTP/2. | 
**http3** | Option<**i64**> | Number of requests received over HTTP/3. | 
**restarts** | Option<**i64**> | Number of restarts performed. | 
**req_header_bytes** | Option<**i64**> | Total header bytes received. | 
**req_body_bytes** | Option<**i64**> | Total body bytes received. | 
**bereq_header_bytes** | Option<**i64**> | Total header bytes sent to origin. | 
**bereq_body_bytes** | Option<**i64**> | Total body bytes sent to origin. | 
**waf_blocked** | Option<**i64**> | Number of requests that triggered a WAF rule and were blocked. | 
**waf_logged** | Option<**i64**> | Number of requests that triggered a WAF rule and were logged. | 
**waf_passed** | Option<**i64**> | Number of requests that triggered a WAF rule and were passed. | 
**attack_req_header_bytes** | Option<**i64**> | Total header bytes received from requests that triggered a WAF rule. | 
**attack_req_body_bytes** | Option<**i64**> | Total body bytes received from requests that triggered a WAF rule. | 
**attack_resp_synth_bytes** | Option<**i64**> | Total bytes delivered for requests that triggered a WAF rule and returned a synthetic response. | 
**attack_logged_req_header_bytes** | Option<**i64**> | Total header bytes received from requests that triggered a WAF rule that was logged. | 
**attack_logged_req_body_bytes** | Option<**i64**> | Total body bytes received from requests that triggered a WAF rule that was logged. | 
**attack_blocked_req_header_bytes** | Option<**i64**> | Total header bytes received from requests that triggered a WAF rule that was blocked. | 
**attack_blocked_req_body_bytes** | Option<**i64**> | Total body bytes received from requests that triggered a WAF rule that was blocked. | 
**attack_passed_req_header_bytes** | Option<**i64**> | Total header bytes received from requests that triggered a WAF rule that was passed. | 
**attack_passed_req_body_bytes** | Option<**i64**> | Total body bytes received from requests that triggered a WAF rule that was passed. | 
**shield_resp_header_bytes** | Option<**i64**> | Total header bytes delivered via a shield. | 
**shield_resp_body_bytes** | Option<**i64**> | Total body bytes delivered via a shield. | 
**otfp_resp_header_bytes** | Option<**i64**> | Total header bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_resp_body_bytes** | Option<**i64**> | Total body bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield_resp_header_bytes** | Option<**i64**> | Total header bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield_resp_body_bytes** | Option<**i64**> | Total body bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield_time** | Option<**f32**> | Total amount of time spent delivering a response via a shield from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds). | 
**otfp_deliver_time** | Option<**f32**> | Total amount of time spent delivering a response from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds). | 
**imgopto_resp_header_bytes** | Option<**i64**> | Total header bytes delivered from the Fastly Image Optimizer service, including shield traffic. | 
**imgopto_resp_body_bytes** | Option<**i64**> | Total body bytes delivered from the Fastly Image Optimizer service, including shield traffic. | 
**imgopto_shield_resp_header_bytes** | Option<**i64**> | Total header bytes delivered via a shield from the Fastly Image Optimizer service. | 
**imgopto_shield_resp_body_bytes** | Option<**i64**> | Total body bytes delivered via a shield from the Fastly Image Optimizer service. | 
**status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes delivered. | 
**status_2xx** | Option<**i64**> | Number of \"Success\" status codes delivered. | 
**status_3xx** | Option<**i64**> | Number of \"Redirection\" codes delivered. | 
**status_4xx** | Option<**i64**> | Number of \"Client Error\" codes delivered. | 
**status_5xx** | Option<**i64**> | Number of \"Server Error\" codes delivered. | 
**status_200** | Option<**i64**> | Number of responses sent with status code 200 (Success). | 
**status_204** | Option<**i64**> | Number of responses sent with status code 204 (No Content). | 
**status_206** | Option<**i64**> | Number of responses sent with status code 206 (Partial Content). | 
**status_301** | Option<**i64**> | Number of responses sent with status code 301 (Moved Permanently). | 
**status_302** | Option<**i64**> | Number of responses sent with status code 302 (Found). | 
**status_304** | Option<**i64**> | Number of responses sent with status code 304 (Not Modified). | 
**status_400** | Option<**i64**> | Number of responses sent with status code 400 (Bad Request). | 
**status_401** | Option<**i64**> | Number of responses sent with status code 401 (Unauthorized). | 
**status_403** | Option<**i64**> | Number of responses sent with status code 403 (Forbidden). | 
**status_404** | Option<**i64**> | Number of responses sent with status code 404 (Not Found). | 
**status_406** | Option<**i64**> | Number of responses sent with status code 406 (Not Acceptable). | 
**status_416** | Option<**i64**> | Number of responses sent with status code 416 (Range Not Satisfiable). | 
**status_429** | Option<**i64**> | Number of responses sent with status code 429 (Too Many Requests). | 
**status_500** | Option<**i64**> | Number of responses sent with status code 500 (Internal Server Error). | 
**status_501** | Option<**i64**> | Number of responses sent with status code 501 (Not Implemented). | 
**status_502** | Option<**i64**> | Number of responses sent with status code 502 (Bad Gateway). | 
**status_503** | Option<**i64**> | Number of responses sent with status code 503 (Service Unavailable). | 
**status_504** | Option<**i64**> | Number of responses sent with status code 504 (Gateway Timeout). | 
**status_505** | Option<**i64**> | Number of responses sent with status code 505 (HTTP Version Not Supported). | 
**status_530** | Option<**i64**> | Number of responses sent with status code 530. | 
**uncacheable** | Option<**i64**> | Number of requests that were designated uncachable. | 
**pass_time** | Option<**f32**> | Total amount of time spent processing cache passes (in seconds). | 
**tls** | Option<**i64**> | Number of requests that were received over TLS. | 
**tls_v10** | Option<**i64**> | Number of requests received over TLS 1.0. | 
**tls_v11** | Option<**i64**> | Number of requests received over TLS 1.1. | 
**tls_v12** | Option<**i64**> | Number of requests received over TLS 1.2. | 
**tls_v13** | Option<**i64**> | Number of requests received over TLS 1.3. | 
**object_size_1k** | Option<**i64**> | Number of objects served that were under 1KB in size. | 
**object_size_10k** | Option<**i64**> | Number of objects served that were between 1KB and 10KB in size. | 
**object_size_100k** | Option<**i64**> | Number of objects served that were between 10KB and 100KB in size. | 
**object_size_1m** | Option<**i64**> | Number of objects served that were between 100KB and 1MB in size. | 
**object_size_10m** | Option<**i64**> | Number of objects served that were between 1MB and 10MB in size. | 
**object_size_100m** | Option<**i64**> | Number of objects served that were between 10MB and 100MB in size. | 
**object_size_1g** | Option<**i64**> | Number of objects served that were between 100MB and 1GB in size. | 
**object_size_other** | Option<**i64**> | Number of objects served that were larger than 1GB in size. | 
**recv_sub_time** | Option<**f32**> | Time spent inside the `vcl_recv` Varnish subroutine (in nanoseconds). | 
**recv_sub_count** | Option<**i64**> | Number of executions of the `vcl_recv` Varnish subroutine. | 
**hash_sub_time** | Option<**f32**> | Time spent inside the `vcl_hash` Varnish subroutine (in nanoseconds). | 
**hash_sub_count** | Option<**i64**> | Number of executions of the `vcl_hash` Varnish subroutine. | 
**miss_sub_time** | Option<**f32**> | Time spent inside the `vcl_miss` Varnish subroutine (in nanoseconds). | 
**miss_sub_count** | Option<**i64**> | Number of executions of the `vcl_miss` Varnish subroutine. | 
**fetch_sub_time** | Option<**f32**> | Time spent inside the `vcl_fetch` Varnish subroutine (in nanoseconds). | 
**fetch_sub_count** | Option<**i64**> | Number of executions of the `vcl_fetch` Varnish subroutine. | 
**pass_sub_time** | Option<**f32**> | Time spent inside the `vcl_pass` Varnish subroutine (in nanoseconds). | 
**pass_sub_count** | Option<**i64**> | Number of executions of the `vcl_pass` Varnish subroutine. | 
**pipe_sub_time** | Option<**f32**> | Time spent inside the `vcl_pipe` Varnish subroutine (in nanoseconds). | 
**pipe_sub_count** | Option<**i64**> | Number of executions of the `vcl_pipe` Varnish subroutine. | 
**deliver_sub_time** | Option<**f32**> | Time spent inside the `vcl_deliver` Varnish subroutine (in nanoseconds). | 
**deliver_sub_count** | Option<**i64**> | Number of executions of the `vcl_deliver` Varnish subroutine. | 
**error_sub_time** | Option<**f32**> | Time spent inside the `vcl_error` Varnish subroutine (in nanoseconds). | 
**error_sub_count** | Option<**i64**> | Number of executions of the `vcl_error` Varnish subroutine. | 
**hit_sub_time** | Option<**f32**> | Time spent inside the `vcl_hit` Varnish subroutine (in nanoseconds). | 
**hit_sub_count** | Option<**i64**> | Number of executions of the `vcl_hit` Varnish subroutine. | 
**prehash_sub_time** | Option<**f32**> | Time spent inside the `vcl_prehash` Varnish subroutine (in nanoseconds). | 
**prehash_sub_count** | Option<**i64**> | Number of executions of the `vcl_prehash` Varnish subroutine. | 
**predeliver_sub_time** | Option<**f32**> | Time spent inside the `vcl_predeliver` Varnish subroutine (in nanoseconds). | 
**predeliver_sub_count** | Option<**i64**> | Number of executions of the `vcl_predeliver` Varnish subroutine. | 
**hit_resp_body_bytes** | Option<**i64**> | Total body bytes delivered for cache hits. | 
**miss_resp_body_bytes** | Option<**i64**> | Total body bytes delivered for cache misses. | 
**pass_resp_body_bytes** | Option<**i64**> | Total body bytes delivered for cache passes. | 
**compute_req_header_bytes** | Option<**i64**> | Total header bytes received by the Compute platform. | 
**compute_req_body_bytes** | Option<**i64**> | Total body bytes received by the Compute platform. | 
**compute_resp_header_bytes** | Option<**i64**> | Total header bytes sent from Compute to end user. | 
**compute_resp_body_bytes** | Option<**i64**> | Total body bytes sent from Compute to end user. | 
**imgvideo** | Option<**i64**> | Number of video responses that came from the Fastly Image Optimizer service. | 
**imgvideo_frames** | Option<**i64**> | Number of video frames that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video. | 
**imgvideo_resp_header_bytes** | Option<**i64**> | Total header bytes of video delivered from the Fastly Image Optimizer service. | 
**imgvideo_resp_body_bytes** | Option<**i64**> | Total body bytes of video delivered from the Fastly Image Optimizer service. | 
**imgvideo_shield** | Option<**i64**> | Number of video responses delivered via a shield that came from the Fastly Image Optimizer service. | 
**imgvideo_shield_frames** | Option<**i64**> | Number of video frames delivered via a shield that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video. | 
**imgvideo_shield_resp_header_bytes** | Option<**i64**> | Total header bytes of video delivered via a shield from the Fastly Image Optimizer service. | 
**imgvideo_shield_resp_body_bytes** | Option<**i64**> | Total body bytes of video delivered via a shield from the Fastly Image Optimizer service. | 
**log_bytes** | Option<**i64**> | Total log bytes sent. | 
**edge_requests** | Option<**i64**> | Number of requests sent by end users to Fastly. | 
**edge_resp_header_bytes** | Option<**i64**> | Total header bytes delivered from Fastly to the end user. | 
**edge_resp_body_bytes** | Option<**i64**> | Total body bytes delivered from Fastly to the end user. | 
**origin_revalidations** | Option<**i64**> | Number of responses received from origin with a `304` status code in response to an `If-Modified-Since` or `If-None-Match` request. Under regular scenarios, a revalidation will imply a cache hit. However, if using Fastly Image Optimizer or segmented caching this may result in a cache miss. | 
**origin_fetches** | Option<**i64**> | Number of requests sent to origin. | 
**origin_fetch_header_bytes** | Option<**i64**> | Total request header bytes sent to origin. | 
**origin_fetch_body_bytes** | Option<**i64**> | Total request body bytes sent to origin. | 
**origin_fetch_resp_header_bytes** | Option<**i64**> | Total header bytes received from origin. | 
**origin_fetch_resp_body_bytes** | Option<**i64**> | Total body bytes received from origin. | 
**shield_revalidations** | Option<**i64**> | Number of responses received from origin with a `304` status code, in response to an `If-Modified-Since` or `If-None-Match` request to a shield. Under regular scenarios, a revalidation will imply a cache hit. However, if using segmented caching this may result in a cache miss. | 
**shield_fetches** | Option<**i64**> | Number of requests made from one Fastly POP to another, as part of shielding. | 
**shield_fetch_header_bytes** | Option<**i64**> | Total request header bytes sent to a shield. | 
**shield_fetch_body_bytes** | Option<**i64**> | Total request body bytes sent to a shield. | 
**shield_fetch_resp_header_bytes** | Option<**i64**> | Total response header bytes sent from a shield to the edge. | 
**shield_fetch_resp_body_bytes** | Option<**i64**> | Total response body bytes sent from a shield to the edge. | 
**segblock_origin_fetches** | Option<**i64**> | Number of `Range` requests to origin for segments of resources when using segmented caching. | 
**segblock_shield_fetches** | Option<**i64**> | Number of `Range` requests to a shield for segments of resources when using segmented caching. | 
**compute_resp_status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes delivered by the Compute platform. | 
**compute_resp_status_2xx** | Option<**i64**> | Number of \"Success\" category status codes delivered by the Compute platform. | 
**compute_resp_status_3xx** | Option<**i64**> | Number of \"Redirection\" category status codes delivered by the Compute platform. | 
**compute_resp_status_4xx** | Option<**i64**> | Number of \"Client Error\" category status codes delivered by the Compute platform. | 
**compute_resp_status_5xx** | Option<**i64**> | Number of \"Server Error\" category status codes delivered by the Compute platform. | 
**edge_hit_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a hit at the edge. | 
**edge_miss_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a miss at the edge. | 
**compute_bereq_header_bytes** | Option<**i64**> | Total header bytes sent to backends (origins) by the Compute platform. | 
**compute_bereq_body_bytes** | Option<**i64**> | Total body bytes sent to backends (origins) by the Compute platform. | 
**compute_beresp_header_bytes** | Option<**i64**> | Total header bytes received from backends (origins) by the Compute platform. | 
**compute_beresp_body_bytes** | Option<**i64**> | Total body bytes received from backends (origins) by the Compute platform. | 
**origin_cache_fetches** | Option<**i64**> | The total number of completed requests made to backends (origins) that returned cacheable content. | 
**shield_cache_fetches** | Option<**i64**> | The total number of completed requests made to shields that returned cacheable content. | 
**compute_bereqs** | Option<**i64**> | Number of backend requests started. | 
**compute_bereq_errors** | Option<**i64**> | Number of backend request errors, including timeouts. | 
**compute_service_bereq_error** | Option<**i64**> | Number of backend request errors, including timeouts. | 
**compute_resource_limit_exceeded** | Option<**i64**> | Number of times a guest exceeded its resource limit, includes heap, stack, globals, and code execution timeout. | 
**compute_heap_limit_exceeded** | Option<**i64**> | Number of times a guest exceeded its heap limit. | 
**compute_service_memory_exceeded_error** | Option<**i64**> | Number of times a guest exceeded its heap limit. | 
**compute_stack_limit_exceeded** | Option<**i64**> | Number of times a guest exceeded its stack limit. | 
**compute_globals_limit_exceeded** | Option<**i64**> | Number of times a guest exceeded its globals limit. | 
**compute_guest_errors** | Option<**i64**> | Number of times a service experienced a guest code error. | 
**compute_runtime_errors** | Option<**i64**> | Number of times a service experienced a guest runtime error. | 
**edge_hit_resp_body_bytes** | Option<**i64**> | Body bytes delivered for edge hits. | 
**edge_hit_resp_header_bytes** | Option<**i64**> | Header bytes delivered for edge hits. | 
**edge_miss_resp_body_bytes** | Option<**i64**> | Body bytes delivered for edge misses. | 
**edge_miss_resp_header_bytes** | Option<**i64**> | Header bytes delivered for edge misses. | 
**origin_cache_fetch_resp_body_bytes** | Option<**i64**> | Body bytes received from origin for cacheable content. | 
**origin_cache_fetch_resp_header_bytes** | Option<**i64**> | Header bytes received from an origin for cacheable content. | 
**shield_hit_requests** | Option<**i64**> | Number of requests that resulted in a hit at a shield. | 
**shield_miss_requests** | Option<**i64**> | Number of requests that resulted in a miss at a shield. | 
**shield_hit_resp_header_bytes** | Option<**i64**> | Header bytes delivered for shield hits. | 
**shield_hit_resp_body_bytes** | Option<**i64**> | Body bytes delivered for shield hits. | 
**shield_miss_resp_header_bytes** | Option<**i64**> | Header bytes delivered for shield misses. | 
**shield_miss_resp_body_bytes** | Option<**i64**> | Body bytes delivered for shield misses. | 
**websocket_req_header_bytes** | Option<**i64**> | Total header bytes received from end users over passthrough WebSocket connections. | 
**websocket_req_body_bytes** | Option<**i64**> | Total message content bytes received from end users over passthrough WebSocket connections. | 
**websocket_resp_header_bytes** | Option<**i64**> | Total header bytes sent to end users over passthrough WebSocket connections. | 
**websocket_bereq_header_bytes** | Option<**i64**> | Total header bytes sent to backends over passthrough WebSocket connections. | 
**websocket_bereq_body_bytes** | Option<**i64**> | Total message content bytes sent to backends over passthrough WebSocket connections. | 
**websocket_beresp_header_bytes** | Option<**i64**> | Total header bytes received from backends over passthrough WebSocket connections. | 
**websocket_beresp_body_bytes** | Option<**i64**> | Total message content bytes received from backends over passthrough WebSocket connections. | 
**websocket_conn_time_ms** | Option<**i64**> | Total duration of passthrough WebSocket connections with end users. | 
**websocket_resp_body_bytes** | Option<**i64**> | Total message content bytes sent to end users over passthrough WebSocket connections. | 
**fanout_recv_publishes** | Option<**i64**> | Total published messages received from the publish API endpoint. | 
**fanout_send_publishes** | Option<**i64**> | Total published messages sent to end users. | 
**kv_store_class_a_operations** | Option<**i64**> | The total number of class a operations for the KV store. | 
**kv_store_class_b_operations** | Option<**i64**> | The total number of class b operations for the KV store. | 
**object_store_class_a_operations** | Option<**i64**> | Use kv_store_class_a_operations. | 
**object_store_class_b_operations** | Option<**i64**> | Use kv_store_class_b_operations. | 
**fanout_req_header_bytes** | Option<**i64**> | Total header bytes received from end users over Fanout connections. | 
**fanout_req_body_bytes** | Option<**i64**> | Total body or message content bytes received from end users over Fanout connections. | 
**fanout_resp_header_bytes** | Option<**i64**> | Total header bytes sent to end users over Fanout connections. | 
**fanout_resp_body_bytes** | Option<**i64**> | Total body or message content bytes sent to end users over Fanout connections, excluding published message content. | 
**fanout_bereq_header_bytes** | Option<**i64**> | Total header bytes sent to backends over Fanout connections. | 
**fanout_bereq_body_bytes** | Option<**i64**> | Total body or message content bytes sent to backends over Fanout connections. | 
**fanout_beresp_header_bytes** | Option<**i64**> | Total header bytes received from backends over Fanout connections. | 
**fanout_beresp_body_bytes** | Option<**i64**> | Total body or message content bytes received from backends over Fanout connections. | 
**fanout_conn_time_ms** | Option<**i64**> | Total duration of Fanout connections with end users. | 
**ddos_action_limit_streams_connections** | Option<**i64**> | For HTTP/2, the number of connections the limit-streams action was applied to. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_limit_streams_requests** | Option<**i64**> | For HTTP/2, the number of requests made on a connection for which the limit-streams action was taken. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_tarpit_accept** | Option<**i64**> | The number of times the tarpit-accept action was taken. The tarpit-accept action adds a delay when accepting future connections. | 
**ddos_action_tarpit** | Option<**i64**> | The number of times the tarpit action was taken. The tarpit action delays writing the response to the client. | 
**ddos_action_close** | Option<**i64**> | The number of times the close action was taken. The close action aborts the connection as soon as possible. The close action takes effect either right after accept, right after the client hello, or right after the response was sent. | 
**ddos_action_blackhole** | Option<**i64**> | The number of times the blackhole action was taken. The blackhole action quietly closes a TCP connection without sending a reset. The blackhole action quietly closes a TCP connection without notifying its peer (all TCP state is dropped). | 
**bot_challenge_starts** | Option<**i64**> | The number of challenge-start tokens created. | 
**bot_challenge_complete_tokens_passed** | Option<**i64**> | The number of challenge-complete tokens that passed validation. | 
**bot_challenge_complete_tokens_failed** | Option<**i64**> | The number of challenge-complete tokens that failed validation. | 
**bot_challenge_complete_tokens_checked** | Option<**i64**> | The number of challenge-complete tokens checked. | 
**bot_challenge_complete_tokens_disabled** | Option<**i64**> | The number of challenge-complete tokens not checked because the feature was disabled. | 
**bot_challenges_issued** | Option<**i64**> | The number of challenges issued. For example, the issuance of a CAPTCHA challenge. | 
**bot_challenges_succeeded** | Option<**i64**> | The number of successful challenge solutions processed. For example, a correct CAPTCHA solution. | 
**bot_challenges_failed** | Option<**i64**> | The number of failed challenge solutions processed. For example, an incorrect CAPTCHA solution. | 
**bot_challenge_complete_tokens_issued** | Option<**i64**> | The number of challenge-complete tokens issued. For example, issuing a challenge-complete token after a series of CAPTCHA challenges ending in success. | 
**ddos_action_downgrade** | Option<**i64**> | The number of times the downgrade action was taken. The downgrade action restricts the client to http1. | 
**ddos_action_downgraded_connections** | Option<**i64**> | The number of connections the downgrade action was applied to. The downgrade action restricts the connection to http1. | 
**all_hit_requests** | Option<**i64**> | Number of cache hits for a VCL service. | 
**all_miss_requests** | Option<**i64**> | Number of cache misses for a VCL service. | 
**all_pass_requests** | Option<**i64**> | Number of requests that passed through the CDN without being cached for a VCL service. | 
**all_error_requests** | Option<**i64**> | Number of cache errors for a VCL service. | 
**all_synth_requests** | Option<**i64**> | Number of requests that returned a synthetic response (i.e., response objects created with the `synthetic` VCL statement) for a VCL service. | 
**all_edge_hit_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a hit at the edge for a VCL service. | 
**all_edge_miss_requests** | Option<**i64**> | Number of requests sent by end users to Fastly that resulted in a miss at the edge for a VCL service. | 
**all_status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes delivered for all sources. | 
**all_status_2xx** | Option<**i64**> | Number of \"Success\" status codes delivered for all sources. | 
**all_status_3xx** | Option<**i64**> | Number of \"Redirection\" codes delivered for all sources. | 
**all_status_4xx** | Option<**i64**> | Number of \"Client Error\" codes delivered for all sources. | 
**all_status_5xx** | Option<**i64**> | Number of \"Server Error\" codes delivered for all sources. | 
**origin_offload** | Option<**f32**> | Origin Offload measures the ratio of bytes served to end users that were cached by Fastly, over the bytes served to end users, between 0 and 1. ((`edge_resp_body_bytes` + `edge_resp_header_bytes`) - (`origin_fetch_resp_body_bytes` + `origin_fetch_resp_header_bytes`)) / (`edge_resp_body_bytes` + `edge_resp_header_bytes`). | 
**request_denied_get_head_body** | Option<**i64**> | Number of requests where Fastly responded with 400 due to the request being a GET or HEAD request containing a body. | 
**ddos_protection_requests_detect_count** | Option<**i64**> | Number of requests classified as a DDoS attack against a customer origin or service. | 
**ddos_protection_requests_mitigate_count** | Option<**i64**> | Number of requests classified as a DDoS attack against a customer origin or service that were mitigated by the Fastly platform. | 
**ddos_protection_requests_allow_count** | Option<**i64**> | Number of requests analyzed for DDoS attacks against a customer origin or service, but with no DDoS detected. | 
**object_storage_class_a_operations_count** | Option<**i64**> | A count of the number of Class A Object Storage operations. | 
**object_storage_class_b_operations_count** | Option<**i64**> | A count of the number of Class B Object Storage operations. | 
**aia_requests** | Option<**i64**> | Number of requests received by AI Accelerator. | 
**aia_status_1xx** | Option<**i64**> | Number of \"Informational\" category status codes received from AI provider. | 
**aia_status_2xx** | Option<**i64**> | Number of \"Success\" status codes received from AI provider. | 
**aia_status_3xx** | Option<**i64**> | Number of \"Redirection\" received from AI provider. | 
**aia_status_4xx** | Option<**i64**> | Number of \"Client Error\" received from AI provider. | 
**aia_status_5xx** | Option<**i64**> | Number of \"Server Error\" received from AI provider. | 
**aia_response_usage_tokens** | Option<**i64**> | The usage tokens associated with the response returned from the AI Accelerator cache. | 
**aia_origin_usage_tokens** | Option<**i64**> | The number of usage tokens reported by the request to the origin from AI Accelerator. | 
**aia_estimated_time_saved_ms** | Option<**i64**> | The estimated amount of time saved by responses served from the AI Accelerator semantic cache. | 
**request_collapse_usable_count** | Option<**i64**> | Number of requests that were collapsed and satisfied by a usable cache object. | 
**request_collapse_unusable_count** | Option<**i64**> | Number of requests that were collapsed and unable to be satisfied by the resulting cache object. | 
**compute_cache_operations_count** | Option<**i64**> | Number of cache operations executed by the Compute platform. | 
**api_discovery_requests_count** | Option<**i32**> | Number of requests processed by the API Discovery engine. | 
**compute_resp_status_103** | Option<**i32**> | Number of responses delivered with status code 103 (Early Hints) by the Compute platform. | 
**compute_resp_status_200** | Option<**i32**> | Number of responses delivered with status code 200 (Success) by the Compute platform. | 
**compute_resp_status_204** | Option<**i32**> | Number of responses delivered with status code 204 (No Content) by the Compute platform. | 
**compute_resp_status_206** | Option<**i32**> | Number of responses delivered with status code 206 (Partial Content) by the Compute platform. | 
**compute_resp_status_301** | Option<**i32**> | Number of responses delivered with status code 301 (Moved Permanently) by the Compute platform. | 
**compute_resp_status_302** | Option<**i32**> | Number of responses delivered with status code 302 (Found) by the Compute platform. | 
**compute_resp_status_304** | Option<**i32**> | Number of responses delivered with status code 304 (Not Modified) by the Compute platform. | 
**compute_resp_status_400** | Option<**i32**> | Number of responses delivered with status code 400 (Bad Request) by the Compute platform. | 
**compute_resp_status_401** | Option<**i32**> | Number of responses delivered with status code 401 (Unauthorized) by the Compute platform. | 
**compute_resp_status_403** | Option<**i32**> | Number of responses delivered with status code 403 (Forbidden) by the Compute platform. | 
**compute_resp_status_404** | Option<**i32**> | Number of responses delivered with status code 404 (Not Found) by the Compute platform. | 
**compute_resp_status_416** | Option<**i32**> | Number of responses delivered with status code 416 (Range Not Satisfiable) by the Compute platform. | 
**compute_resp_status_429** | Option<**i32**> | Number of responses delivered with status code 429 (Too Many Requests) by the Compute platform. | 
**compute_resp_status_500** | Option<**i32**> | Number of responses delivered with status code 500 (Internal Server Error) by the Compute platform. | 
**compute_resp_status_501** | Option<**i32**> | Number of responses delivered with status code 501 (Not Implemented) by the Compute platform. | 
**compute_resp_status_502** | Option<**i32**> | Number of responses delivered with status code 502 (Bad Gateway) by the Compute platform. | 
**compute_resp_status_503** | Option<**i32**> | Number of responses delivered with status code 503 (Service Unavailable) by the Compute platform. | 
**compute_resp_status_504** | Option<**i32**> | Number of responses delivered with status code 504 (Gateway Timeout) by the Compute platform. | 
**compute_resp_status_505** | Option<**i32**> | Number of responses delivered with status code 505 (HTTP Version Not Supported) by the Compute platform. | 
**compute_resp_status_530** | Option<**i32**> | Number of responses delivered with status code 530 by the Compute platform. | 
**imgopto_compute_requests** | Option<**i32**> | The number of Image Optimizer requests made from Compute services. | 
**dns_billable_responses_count** | Option<**i32**> | Number of billable DNS responses (e.g., A, CNAME). | 
**dns_nonbillable_responses_count** | Option<**i32**> | Number of non-billable DNS responses (e.g., NODATA, NXDOMAIN). | 
**upgrade** | Option<**i32**> | Number of requests that resulted in a WebSocket upgrade. | 
**ngwaf_bot_analysis_request_count** | Option<**i32**> | Count of Next-Gen WAF Bot Management requests. | 
**imgopto_avif_count** | Option<**i32**> | Count of AVIF images delivered to end user by Image Optimizer. | 
**imgopto_jpeg_count** | Option<**i32**> | Count of JPEG images delivered to end user by Image Optimizer. | 
**imgopto_png_count** | Option<**i32**> | Count of PNG images delivered to end user by Image Optimizer. | 
**imgopto_gif_count** | Option<**i32**> | Count of GIF images delivered to end user by Image Optimizer. | 
**imgopto_webp_count** | Option<**i32**> | Count of WebP images delivered to end user by Image Optimizer. | 
**imgopto_jpegxl_count** | Option<**i32**> | Count of JPEGXL images delivered to end user by Image Optimizer. | 
**imgopto_svg_count** | Option<**i32**> | Count of SVG images delivered to end user by Image Optimizer. | 
**imgopto_mp4_count** | Option<**i32**> | Count of MP4s delivered to end user by Image Optimizer. | 
**compute_service_resource_limits_error** | Option<**i32**> | Aggregate of fatal errors caused by exceeding allocated resource limits, specifically runtime duration, vCPU usage, and heap memory limits. | 
**compute_service_runtime_error** | Option<**i32**> | Fatal errors caused by service logic faults, including stack overflows, unreachable code traps, illegal memory access, or attempts to send multiple responses. | 
**compute_service_chain_error** | Option<**i32**> | Fatal errors caused by the service path exceeding hop or service limits, or where a forwarding loop is detected via CDN-Loop headers. | 
**compute_platform_internal_error** | Option<**i32**> | Fatal errors caused by internal errors in Fastly’s Compute platform. | 
**compute_service_timeout_error** | Option<**i32**> | Fatal errors caused by exceeding the per-request runtime limit. | 
**compute_service_vcpu_exceeded_error** | Option<**i32**> | Fatal errors caused by exceeding the per-request vCPU time limit. | 
**compute_service_limits_error** | Option<**i32**> | Non-fatal errors caused by attempts to exceed defined operational limits, such as simultaneous backend requests or cache transactions. | 
**compute_platform_invalid_request_error** | Option<**i32**> | Fatal errors caused by unprocessable requests to the service, such as requests with malformed CDN-Loop headers or invalid purge credentials. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


