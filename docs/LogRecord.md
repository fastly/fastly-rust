# LogRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The ID of the Fastly customer that owns the service. | [readonly]
**service_id** | Option<[**crate::models::LogPropertyServiceId**](LogPropertyServiceId.md)> |  | 
**timestamp** | Option<**String**> | Timestamp of the request in ISO 8601 format. | 
**client_as_number** | Option<**i32**> | The autonomous system (AS) number of the client. | [readonly]
**client_region** | Option<**String**> | The client's country subdivision code as found in ISO 3166-2. | [readonly]
**client_country_code** | Option<**String**> | The two-letter ISO 3166-1 country code for the client. | [readonly]
**client_os_name** | Option<**String**> | The name of the operating system installed on the client device. | [readonly]
**client_device_type** | Option<**String**> | The type of the client's device. | [readonly]
**client_browser_name** | Option<**String**> | The name of the browser in use on the client device. | [readonly]
**client_browser_version** | Option<**String**> | The version of the browser in use on client device. | [readonly]
**fastly_pop** | Option<**String**> | The name of the Fastly POP that served this request. | [readonly]
**origin_host** | Option<**String**> | The name of the origin host that served this request. | [readonly]
**request_protocol** | Option<**String**> | HTTP protocol version in use for this request. For example, HTTP/1.1. | [readonly]
**request_host** | Option<**String**> | The name of the request host used for this request. | [readonly]
**request_path** | Option<**String**> | The URL path supplied for this request. | [readonly]
**request_method** | Option<**String**> | HTTP method sent by the client such as \"GET\" or \"POST\". | [readonly]
**response_bytes_body** | Option<**i32**> | Body bytes sent to the client in the response. | [readonly]
**response_bytes_header** | Option<**i32**> | Header bytes sent to the client in the response. | [readonly]
**response_content_length** | Option<**i32**> | Total bytes sent to the client in the response. | [readonly]
**response_content_type** | Option<**String**> | The content type of the response sent to the client. | [readonly]
**response_reason** | Option<**String**> | The HTTP reason phrase returned for this request, if any. | [readonly]
**response_state** | Option<**String**> | The state of the request with optional suffixes describing special cases. | [readonly]
**response_status** | Option<**i32**> | The HTTP response code returned for this request. | [readonly]
**response_time** | Option<**f32**> | The time since the request started in seconds. | [readonly]
**response_x_cache** | Option<**String**> | Indicates whether the request was a HIT or a MISS. | [readonly]
**is_cache_hit** | Option<**bool**> | Indicates whether this request was fulfilled from cache. | [readonly]
**is_edge** | Option<**bool**> | Indicates whether the request was handled by a Fastly edge POP. | [readonly]
**is_shield** | Option<**bool**> | Indicates whether the request was handled by a Fastly shield POP. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


