# RateLimiterResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A human readable name for the rate limiting rule. | 
**uri_dictionary_name** | Option<**String**> | The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited. | 
**http_methods** | Option<**Vec<String>**> | Array of HTTP methods to apply rate limiting to. | 
**rps_limit** | Option<**i32**> | Upper limit of requests per second allowed by the rate limiter. | 
**window_size** | Option<**i32**> | Number of seconds during which the RPS limit must be exceeded in order to trigger a violation. | 
**client_key** | Option<**Vec<String>**> | Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`. | 
**penalty_box_duration** | Option<**i32**> | Length of time in minutes that the rate limiter is in effect after the initial violation is detected. | 
**action** | Option<**String**> | The action to take when a rate limiter violation is detected. | 
**response** | Option<**::std::collections::HashMap<String, String>**> | Custom response to be sent when the rate limit is exceeded. Required if `action` is `response`. | 
**response_object_name** | Option<**String**> | Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration. | 
**logger_type** | Option<**String**> | Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries. | 
**feature_revision** | Option<**i32**> | Revision number of the rate limiting feature implementation. Defaults to the most recent revision. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> | Alphanumeric string identifying the rate limiter. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


