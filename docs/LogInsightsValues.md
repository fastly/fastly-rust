# LogInsightsValues

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_hit_ratio** | Option<**f32**> | The cache hit ratio for the URL specified in the dimension. | 
**region** | Option<**String**> | The client's country subdivision code as defined by ISO 3166-2. | 
**region_chr** | Option<**f32**> | The cache hit ratio for the region. | 
**region_error_rate** | Option<**f32**> | The error rate for the region. | 
**url** | Option<**String**> | The HTTP request path. | 
**rate_per_status** | Option<**f32**> | The URL accounts for this percentage of the status code in this dimension. | 
**rate_per_url** | Option<**f32**> | The rate at which the reason in this dimension occurs among responses to this URL with a 503 status code. | 
**var_503_rate_per_url** | Option<**f32**> | The rate at which 503 status codes are returned for this URL. | 
**browser_version** | Option<**String**> | The version of the client's browser. | 
**rate** | Option<**f32**> | The percentage of requests matching the value in the current dimension. | 
**average_bandwidth_bytes** | Option<**f32**> | The average bandwidth in bytes for responses to requests to the URL in the current dimension. | 
**bandwidth_percentage** | Option<**f32**> | The total bandwidth percentage for all responses to requests to the URL in the current dimension. | 
**average_response_time** | Option<**f32**> | The average time in seconds to respond to requests to the URL in the current dimension. | 
**p95_response_time** | Option<**f32**> | The P95 time in seconds to respond to requests to the URL in the current dimension. | 
**response_time_percentage** | Option<**f32**> | The total percentage of time to respond to all requests to the URL in the current dimension. | 
**miss_rate** | Option<**f32**> | The miss rate for requests to the URL in the current dimension. | 
**request_percentage** | Option<**f32**> | The percentage of all requests made to the URL in the current dimension. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


