# Results

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**requests** | Option<**i32**> | Number of requests processed. | 
**hits** | Option<**i32**> | Number of cache hits. | 
**hits_time** | Option<**f32**> | Total amount of time spent processing cache hits (in seconds). | 
**miss** | Option<**i32**> | Number of cache misses. | 
**miss_time** | Option<**f32**> | Total amount of time spent processing cache misses (in seconds). | 
**pass** | Option<**i32**> | Number of requests that passed through the CDN without being cached. | 
**pass_time** | Option<**f32**> | Total amount of time spent processing cache passes (in seconds). | 
**errors** | Option<**i32**> | Number of cache errors. | 
**restarts** | Option<**i32**> | Number of restarts performed. | 
**hit_ratio** | Option<**f32**> | Ratio of cache hits to cache misses (between 0 and 1). | 
**bandwidth** | Option<**i32**> | Total bytes delivered (`resp_header_bytes` + `resp_body_bytes` + `bereq_header_bytes` + `bereq_body_bytes` + `compute_resp_header_bytes` + `compute_resp_body_bytes` + `compute_bereq_header_bytes` + `compute_bereq_body_bytes` + `websocket_resp_header_bytes` + `websocket_resp_body_bytes` + `websocket_bereq_header_bytes` + `websocket_bereq_body_bytes` + `fanout_resp_header_bytes` + `fanout_resp_body_bytes` + `fanout_bereq_header_bytes` + `fanout_bereq_body_bytes`). | 
**body_size** | Option<**i32**> | Total body bytes delivered (alias for resp_body_bytes). | 
**header_size** | Option<**i32**> | Total header bytes delivered (alias for resp_header_bytes). | 
**req_body_bytes** | Option<**i32**> | Total body bytes received. | 
**req_header_bytes** | Option<**i32**> | Total header bytes received. | 
**resp_body_bytes** | Option<**i32**> | Total body bytes delivered (edge_resp_body_bytes + shield_resp_body_bytes). | 
**resp_header_bytes** | Option<**i32**> | Total header bytes delivered (edge_resp_header_bytes + shield_resp_header_bytes). | 
**bereq_body_bytes** | Option<**i32**> | Total body bytes sent to origin. | 
**bereq_header_bytes** | Option<**i32**> | Total header bytes sent to origin. | 
**uncacheable** | Option<**i32**> | Number of requests that were designated uncachable. | 
**pipe** | Option<**i32**> | Optional. Pipe operations performed (legacy feature). | 
**synth** | Option<**i32**> | Number of requests that returned a synthetic response (i.e., response objects created with the `synthetic` VCL statement). | 
**tls** | Option<**i32**> | Number of requests that were received over TLS. | 
**tls_v10** | Option<**i32**> | Number of requests received over TLS 1.0. | 
**tls_v11** | Option<**i32**> | Number of requests received over TLS 1.1. | 
**tls_v12** | Option<**i32**> | Number of requests received over TLS 1.2. | 
**tls_v13** | Option<**i32**> | Number of requests received over TLS 1.3. | 
**edge_requests** | Option<**i32**> | Number of requests sent by end users to Fastly. | 
**edge_resp_header_bytes** | Option<**i32**> | Total header bytes delivered from Fastly to the end user. | 
**edge_resp_body_bytes** | Option<**i32**> | Total body bytes delivered from Fastly to the end user. | 
**edge_hit_requests** | Option<**i32**> | Number of requests sent by end users to Fastly that resulted in a hit at the edge. | 
**edge_miss_requests** | Option<**i32**> | Number of requests sent by end users to Fastly that resulted in a miss at the edge. | 
**origin_fetches** | Option<**i32**> | Number of requests sent to origin. | 
**origin_fetch_header_bytes** | Option<**i32**> | Total request header bytes sent to origin. | 
**origin_fetch_body_bytes** | Option<**i32**> | Total request body bytes sent to origin. | 
**origin_fetch_resp_header_bytes** | Option<**i32**> | Total header bytes received from origin. | 
**origin_fetch_resp_body_bytes** | Option<**i32**> | Total body bytes received from origin. | 
**origin_revalidations** | Option<**i32**> | Number of responses received from origin with a `304` status code in response to an `If-Modified-Since` or `If-None-Match` request. Under regular scenarios, a revalidation will imply a cache hit. However, if using Fastly Image Optimizer or segmented caching this may result in a cache miss. | 
**origin_cache_fetches** | Option<**i32**> | The total number of completed requests made to backends (origins) that returned cacheable content. | 
**shield** | Option<**i32**> | Number of requests from edge to the shield POP. | 
**shield_resp_body_bytes** | Option<**i32**> | Total body bytes delivered via a shield. | 
**shield_resp_header_bytes** | Option<**i32**> | Total header bytes delivered via a shield. | 
**shield_fetches** | Option<**i32**> | Number of requests made from one Fastly POP to another, as part of shielding. | 
**shield_fetch_header_bytes** | Option<**i32**> | Total request header bytes sent to a shield. | 
**shield_fetch_body_bytes** | Option<**i32**> | Total request body bytes sent to a shield. | 
**shield_fetch_resp_header_bytes** | Option<**i32**> | Total response header bytes sent from a shield to the edge. | 
**shield_fetch_resp_body_bytes** | Option<**i32**> | Total response body bytes sent from a shield to the edge. | 
**shield_revalidations** | Option<**i32**> | Number of responses received from origin with a `304` status code, in response to an `If-Modified-Since` or `If-None-Match` request to a shield. Under regular scenarios, a revalidation will imply a cache hit. However, if using segmented caching this may result in a cache miss. | 
**shield_cache_fetches** | Option<**i32**> | The total number of completed requests made to shields that returned cacheable content. | 
**ipv6** | Option<**i32**> | Number of requests that were received over IPv6. | 
**otfp** | Option<**i32**> | Number of responses that came from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_resp_body_bytes** | Option<**i32**> | Total body bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_resp_header_bytes** | Option<**i32**> | Total header bytes delivered from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield_resp_body_bytes** | Option<**i32**> | Total body bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_shield_resp_header_bytes** | Option<**i32**> | Total header bytes delivered via a shield for the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_manifests** | Option<**i32**> | Number of responses that were manifest files from the Fastly On-the-Fly Packaging service for video-on-demand. | 
**otfp_deliver_time** | Option<**f32**> | Total amount of time spent delivering a response from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds). | 
**otfp_shield_time** | Option<**f32**> | Total amount of time spent delivering a response via a shield from the Fastly On-the-Fly Packaging service for video-on-demand (in seconds). | 
**video** | Option<**i32**> | Number of responses with the video segment or video manifest MIME type (i.e., application/x-mpegurl, application/vnd.apple.mpegurl, application/f4m, application/dash+xml, application/vnd.ms-sstr+xml, ideo/mp2t, audio/aac, video/f4f, video/x-flv, video/mp4, audio/mp4). | 
**pci** | Option<**i32**> | Number of responses with the PCI flag turned on. | 
**log** | Option<**i32**> | Number of log lines sent. | 
**log_bytes** | Option<**i32**> | Total log bytes sent. | 
**http2** | Option<**i32**> | Number of requests received over HTTP/2. | 
**http3** | Option<**i32**> | Number of requests received over HTTP/3. | 
**waf_logged** | Option<**i32**> | Number of requests that triggered a WAF rule and were logged. | 
**waf_blocked** | Option<**i32**> | Number of requests that triggered a WAF rule and were blocked. | 
**waf_passed** | Option<**i32**> | Number of requests that triggered a WAF rule and were passed. | 
**attack_req_body_bytes** | Option<**i32**> | Total body bytes received from requests that triggered a WAF rule. | 
**attack_req_header_bytes** | Option<**i32**> | Total header bytes received from requests that triggered a WAF rule. | 
**attack_logged_req_body_bytes** | Option<**i32**> | Total body bytes received from requests that triggered a WAF rule that was logged. | 
**attack_logged_req_header_bytes** | Option<**i32**> | Total header bytes received from requests that triggered a WAF rule that was logged. | 
**attack_blocked_req_body_bytes** | Option<**i32**> | Total body bytes received from requests that triggered a WAF rule that was blocked. | 
**attack_blocked_req_header_bytes** | Option<**i32**> | Total header bytes received from requests that triggered a WAF rule that was blocked. | 
**attack_passed_req_body_bytes** | Option<**i32**> | Total body bytes received from requests that triggered a WAF rule that was passed. | 
**attack_passed_req_header_bytes** | Option<**i32**> | Total header bytes received from requests that triggered a WAF rule that was passed. | 
**attack_resp_synth_bytes** | Option<**i32**> | Total bytes delivered for requests that triggered a WAF rule and returned a synthetic response. | 
**imgopto** | Option<**i32**> | Number of responses that came from the Fastly Image Optimizer service. If the service receives 10 requests for an image, this stat will be 10 regardless of how many times the image was transformed. | 
**imgopto_resp_body_bytes** | Option<**i32**> | Total body bytes delivered from the Fastly Image Optimizer service, including shield traffic. | 
**imgopto_resp_header_bytes** | Option<**i32**> | Total header bytes delivered from the Fastly Image Optimizer service, including shield traffic. | 
**imgopto_shield_resp_body_bytes** | Option<**i32**> | Total body bytes delivered via a shield from the Fastly Image Optimizer service. | 
**imgopto_shield_resp_header_bytes** | Option<**i32**> | Total header bytes delivered via a shield from the Fastly Image Optimizer service. | 
**imgvideo** | Option<**i32**> | Number of video responses that came from the Fastly Image Optimizer service. | 
**imgvideo_frames** | Option<**i32**> | Number of video frames that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video. | 
**imgvideo_resp_header_bytes** | Option<**i32**> | Total header bytes of video delivered from the Fastly Image Optimizer service. | 
**imgvideo_resp_body_bytes** | Option<**i32**> | Total body bytes of video delivered from the Fastly Image Optimizer service. | 
**imgvideo_shield_resp_header_bytes** | Option<**i32**> | Total header bytes of video delivered via a shield from the Fastly Image Optimizer service. | 
**imgvideo_shield_resp_body_bytes** | Option<**i32**> | Total body bytes of video delivered via a shield from the Fastly Image Optimizer service. | 
**imgvideo_shield** | Option<**i32**> | Number of video responses delivered via a shield that came from the Fastly Image Optimizer service. | 
**imgvideo_shield_frames** | Option<**i32**> | Number of video frames delivered via a shield that came from the Fastly Image Optimizer service. A video frame is an individual image within a sequence of video. | 
**status_200** | Option<**i32**> | Number of responses sent with status code 200 (Success). | 
**status_204** | Option<**i32**> | Number of responses sent with status code 204 (No Content). | 
**status_206** | Option<**i32**> | Number of responses sent with status code 206 (Partial Content). | 
**status_301** | Option<**i32**> | Number of responses sent with status code 301 (Moved Permanently). | 
**status_302** | Option<**i32**> | Number of responses sent with status code 302 (Found). | 
**status_304** | Option<**i32**> | Number of responses sent with status code 304 (Not Modified). | 
**status_400** | Option<**i32**> | Number of responses sent with status code 400 (Bad Request). | 
**status_401** | Option<**i32**> | Number of responses sent with status code 401 (Unauthorized). | 
**status_403** | Option<**i32**> | Number of responses sent with status code 403 (Forbidden). | 
**status_404** | Option<**i32**> | Number of responses sent with status code 404 (Not Found). | 
**status_406** | Option<**i32**> | Number of responses sent with status code 406 (Not Acceptable). | 
**status_416** | Option<**i32**> | Number of responses sent with status code 416 (Range Not Satisfiable). | 
**status_429** | Option<**i32**> | Number of responses sent with status code 429 (Too Many Requests). | 
**status_500** | Option<**i32**> | Number of responses sent with status code 500 (Internal Server Error). | 
**status_501** | Option<**i32**> | Number of responses sent with status code 501 (Not Implemented). | 
**status_502** | Option<**i32**> | Number of responses sent with status code 502 (Bad Gateway). | 
**status_503** | Option<**i32**> | Number of responses sent with status code 503 (Service Unavailable). | 
**status_504** | Option<**i32**> | Number of responses sent with status code 504 (Gateway Timeout). | 
**status_505** | Option<**i32**> | Number of responses sent with status code 505 (HTTP Version Not Supported). | 
**status_1xx** | Option<**i32**> | Number of \"Informational\" category status codes delivered. | 
**status_2xx** | Option<**i32**> | Number of \"Success\" status codes delivered. | 
**status_3xx** | Option<**i32**> | Number of \"Redirection\" codes delivered. | 
**status_4xx** | Option<**i32**> | Number of \"Client Error\" codes delivered. | 
**status_5xx** | Option<**i32**> | Number of \"Server Error\" codes delivered. | 
**object_size_1k** | Option<**i32**> | Number of objects served that were under 1KB in size. | 
**object_size_10k** | Option<**i32**> | Number of objects served that were between 1KB and 10KB in size. | 
**object_size_100k** | Option<**i32**> | Number of objects served that were between 10KB and 100KB in size. | 
**object_size_1m** | Option<**i32**> | Number of objects served that were between 100KB and 1MB in size. | 
**object_size_10m** | Option<**i32**> | Number of objects served that were between 1MB and 10MB in size. | 
**object_size_100m** | Option<**i32**> | Number of objects served that were between 10MB and 100MB in size. | 
**object_size_1g** | Option<**i32**> | Number of objects served that were between 100MB and 1GB in size. | 
**recv_sub_time** | Option<**f32**> | Time spent inside the `vcl_recv` Varnish subroutine (in seconds). | 
**recv_sub_count** | Option<**i32**> | Number of executions of the `vcl_recv` Varnish subroutine. | 
**hash_sub_time** | Option<**f32**> | Time spent inside the `vcl_hash` Varnish subroutine (in seconds). | 
**hash_sub_count** | Option<**i32**> | Number of executions of the `vcl_hash` Varnish subroutine. | 
**miss_sub_time** | Option<**f32**> | Time spent inside the `vcl_miss` Varnish subroutine (in seconds). | 
**miss_sub_count** | Option<**i32**> | Number of executions of the `vcl_miss` Varnish subroutine. | 
**fetch_sub_time** | Option<**f32**> | Time spent inside the `vcl_fetch` Varnish subroutine (in seconds). | 
**fetch_sub_count** | Option<**i32**> | Number of executions of the `vcl_fetch` Varnish subroutine. | 
**pass_sub_time** | Option<**f32**> | Time spent inside the `vcl_pass` Varnish subroutine (in seconds). | 
**pass_sub_count** | Option<**i32**> | Number of executions of the `vcl_pass` Varnish subroutine. | 
**pipe_sub_time** | Option<**f32**> | Time spent inside the `vcl_pipe` Varnish subroutine (in seconds). | 
**pipe_sub_count** | Option<**i32**> | Number of executions of the `vcl_pipe` Varnish subroutine. | 
**deliver_sub_time** | Option<**f32**> | Time spent inside the `vcl_deliver` Varnish subroutine (in seconds). | 
**deliver_sub_count** | Option<**i32**> | Number of executions of the `vcl_deliver` Varnish subroutine. | 
**error_sub_time** | Option<**f32**> | Time spent inside the `vcl_error` Varnish subroutine (in seconds). | 
**error_sub_count** | Option<**i32**> | Number of executions of the `vcl_error` Varnish subroutine. | 
**hit_sub_time** | Option<**f32**> | Time spent inside the `vcl_hit` Varnish subroutine (in seconds). | 
**hit_sub_count** | Option<**i32**> | Number of executions of the `vcl_hit` Varnish subroutine. | 
**prehash_sub_time** | Option<**f32**> | Time spent inside the `vcl_prehash` Varnish subroutine (in seconds). | 
**prehash_sub_count** | Option<**i32**> | Number of executions of the `vcl_prehash` Varnish subroutine. | 
**predeliver_sub_time** | Option<**f32**> | Time spent inside the `vcl_predeliver` Varnish subroutine (in seconds). | 
**predeliver_sub_count** | Option<**i32**> | Number of executions of the `vcl_predeliver` Varnish subroutine. | 
**tls_handshake_sent_bytes** | Option<**i32**> | Number of bytes transferred during TLS handshake. | 
**hit_resp_body_bytes** | Option<**i32**> | Total body bytes delivered for cache hits. | 
**miss_resp_body_bytes** | Option<**i32**> | Total body bytes delivered for cache misses. | 
**pass_resp_body_bytes** | Option<**i32**> | Total body bytes delivered for cache passes. | 
**segblock_origin_fetches** | Option<**i32**> | Number of `Range` requests to origin for segments of resources when using segmented caching. | 
**segblock_shield_fetches** | Option<**i32**> | Number of `Range` requests to a shield for segments of resources when using segmented caching. | 
**compute_requests** | Option<**i32**> | The total number of requests that were received for your service by Fastly. | 
**compute_request_time_ms** | Option<**f32**> | The total, actual amount of time used to process your requests, including active CPU time (in milliseconds). | 
**compute_request_time_billed_ms** | Option<**f32**> | The total amount of request processing time you will be billed for, measured in 50 millisecond increments. | 
**compute_ram_used** | Option<**i32**> | The amount of RAM used for your service by Fastly (in bytes). | 
**compute_execution_time_ms** | Option<**f32**> | The amount of active CPU time used to process your requests (in milliseconds). | 
**compute_req_header_bytes** | Option<**i32**> | Total header bytes received by Compute@Edge. | 
**compute_req_body_bytes** | Option<**i32**> | Total body bytes received by Compute@Edge. | 
**compute_resp_header_bytes** | Option<**i32**> | Total header bytes sent from Compute@Edge to end user. | 
**compute_resp_body_bytes** | Option<**i32**> | Total body bytes sent from Compute@Edge to end user. | 
**compute_resp_status_1xx** | Option<**i32**> | Number of \"Informational\" category status codes delivered by Compute@Edge. | 
**compute_resp_status_2xx** | Option<**i32**> | Number of \"Success\" category status codes delivered by Compute@Edge. | 
**compute_resp_status_3xx** | Option<**i32**> | Number of \"Redirection\" category status codes delivered by Compute@Edge. | 
**compute_resp_status_4xx** | Option<**i32**> | Number of \"Client Error\" category status codes delivered by Compute@Edge. | 
**compute_resp_status_5xx** | Option<**i32**> | Number of \"Server Error\" category status codes delivered by Compute@Edge. | 
**compute_bereq_header_bytes** | Option<**i32**> | Total header bytes sent to backends (origins) by Compute@Edge. | 
**compute_bereq_body_bytes** | Option<**i32**> | Total body bytes sent to backends (origins) by Compute@Edge. | 
**compute_beresp_header_bytes** | Option<**i32**> | Total header bytes received from backends (origins) by Compute@Edge. | 
**compute_beresp_body_bytes** | Option<**i32**> | Total body bytes received from backends (origins) by Compute@Edge. | 
**compute_bereqs** | Option<**i32**> | Number of backend requests started. | 
**compute_bereq_errors** | Option<**i32**> | Number of backend request errors, including timeouts. | 
**compute_resource_limit_exceeded** | Option<**i32**> | Number of times a guest exceeded its resource limit, includes heap, stack, globals, and code execution timeout. | 
**compute_heap_limit_exceeded** | Option<**i32**> | Number of times a guest exceeded its heap limit. | 
**compute_stack_limit_exceeded** | Option<**i32**> | Number of times a guest exceeded its stack limit. | 
**compute_globals_limit_exceeded** | Option<**i32**> | Number of times a guest exceeded its globals limit. | 
**compute_guest_errors** | Option<**i32**> | Number of times a service experienced a guest code error. | 
**compute_runtime_errors** | Option<**i32**> | Number of times a service experienced a guest runtime error. | 
**edge_hit_resp_body_bytes** | Option<**i32**> | Body bytes delivered for edge hits. | 
**edge_hit_resp_header_bytes** | Option<**i32**> | Header bytes delivered for edge hits. | 
**edge_miss_resp_body_bytes** | Option<**i32**> | Body bytes delivered for edge misses. | 
**edge_miss_resp_header_bytes** | Option<**i32**> | Header bytes delivered for edge misses. | 
**origin_cache_fetch_resp_body_bytes** | Option<**i32**> | Body bytes received from origin for cacheable content. | 
**origin_cache_fetch_resp_header_bytes** | Option<**i32**> | Header bytes received from an origin for cacheable content. | 
**shield_hit_requests** | Option<**i32**> | Number of requests that resulted in a hit at a shield. | 
**shield_miss_requests** | Option<**i32**> | Number of requests that resulted in a miss at a shield. | 
**shield_hit_resp_header_bytes** | Option<**i32**> | Header bytes delivered for shield hits. | 
**shield_hit_resp_body_bytes** | Option<**i32**> | Body bytes delivered for shield hits. | 
**shield_miss_resp_header_bytes** | Option<**i32**> | Header bytes delivered for shield misses. | 
**shield_miss_resp_body_bytes** | Option<**i32**> | Body bytes delivered for shield misses. | 
**websocket_req_header_bytes** | Option<**i32**> | Total header bytes received from end users over passthrough WebSocket connections. | 
**websocket_req_body_bytes** | Option<**i32**> | Total message content bytes received from end users over passthrough WebSocket connections. | 
**websocket_resp_header_bytes** | Option<**i32**> | Total header bytes sent to end users over passthrough WebSocket connections. | 
**websocket_resp_body_bytes** | Option<**i32**> | Total message content bytes sent to end users over passthrough WebSocket connections. | 
**websocket_bereq_header_bytes** | Option<**i32**> | Total header bytes sent to backends over passthrough WebSocket connections. | 
**websocket_bereq_body_bytes** | Option<**i32**> | Total message content bytes sent to backends over passthrough WebSocket connections. | 
**websocket_beresp_header_bytes** | Option<**i32**> | Total header bytes received from backends over passthrough WebSocket connections. | 
**websocket_beresp_body_bytes** | Option<**i32**> | Total message content bytes received from backends over passthrough WebSocket connections. | 
**websocket_conn_time_ms** | Option<**i32**> | Total duration of passthrough WebSocket connections with end users. | 
**fanout_recv_publishes** | Option<**i32**> | Total published messages received from the publish API endpoint. | 
**fanout_send_publishes** | Option<**i32**> | Total published messages sent to end users. | 
**kv_store_class_a_operations** | Option<**i32**> | The total number of class a operations for the KV store. | 
**kv_store_class_b_operations** | Option<**i32**> | The total number of class b operations for the KV store. | 
**object_store_class_a_operations** | Option<**i32**> | Use kv_store_class_a_operations. | 
**object_store_class_b_operations** | Option<**i32**> | Use kv_store_class_b_operations. | 
**fanout_req_header_bytes** | Option<**i32**> | Total header bytes received from end users over Fanout connections. | 
**fanout_req_body_bytes** | Option<**i32**> | Total body or message content bytes received from end users over Fanout connections. | 
**fanout_resp_header_bytes** | Option<**i32**> | Total header bytes sent to end users over Fanout connections. | 
**fanout_resp_body_bytes** | Option<**i32**> | Total body or message content bytes sent to end users over Fanout connections, excluding published message content. | 
**fanout_bereq_header_bytes** | Option<**i32**> | Total header bytes sent to backends over Fanout connections. | 
**fanout_bereq_body_bytes** | Option<**i32**> | Total body or message content bytes sent to backends over Fanout connections. | 
**fanout_beresp_header_bytes** | Option<**i32**> | Total header bytes received from backends over Fanout connections. | 
**fanout_beresp_body_bytes** | Option<**i32**> | Total body or message content bytes received from backends over Fanout connections. | 
**fanout_conn_time_ms** | Option<**i32**> | Total duration of Fanout connections with end users. | 
**ddos_action_limit_streams_connections** | Option<**i32**> | For HTTP/2, the number of connections the limit-streams action was applied to. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_limit_streams_requests** | Option<**i32**> | For HTTP/2, the number of requests made on a connection for which the limit-streams action was taken. The limit-streams action caps the allowed number of concurrent streams in a connection. | 
**ddos_action_tarpit_accept** | Option<**i32**> | The number of times the tarpit-accept action was taken. The tarpit-accept action adds a delay when accepting future connections. | 
**ddos_action_tarpit** | Option<**i32**> | The number of times the tarpit action was taken. The tarpit action delays writing the response to the client. | 
**ddos_action_close** | Option<**i32**> | The number of times the close action was taken. The close action aborts the connection as soon as possible. The close action takes effect either right after accept, right after the client hello, or right after the response was sent. | 
**ddos_action_blackhole** | Option<**i32**> | The number of times the blackhole action was taken. The blackhole action quietly closes a TCP connection without sending a reset. The blackhole action quietly closes a TCP connection without notifying its peer (all TCP state is dropped). | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


