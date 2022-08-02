# RequestSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | Allows you to terminate request handling and immediately perform an action. | 
**bypass_busy_wait** | Option<**i32**> | Disable collapsed forwarding, so you don't wait for other objects to origin. | 
**default_host** | Option<**String**> | Sets the host header. | 
**force_miss** | Option<**i32**> | Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable. | 
**force_ssl** | Option<**i32**> | Forces the request use SSL (redirects a non-SSL to SSL). | 
**geo_headers** | Option<**i32**> | Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers. | 
**hash_keys** | Option<**String**> | Comma separated list of varnish request object fields that should be in the hash key. | 
**max_stale_age** | Option<**i32**> | How old an object is allowed to be to serve stale-if-error or stale-while-revalidate. | 
**name** | Option<**String**> | Name for the request settings. | 
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**timer_support** | Option<**i32**> | Injects the X-Timer info into the request for viewing origin fetch durations. | 
**xff** | Option<**String**> | Short for X-Forwarded-For. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


