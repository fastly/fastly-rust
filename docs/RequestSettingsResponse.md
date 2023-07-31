# RequestSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]
**action** | Option<**String**> | Allows you to terminate request handling and immediately perform an action. | 
**default_host** | Option<**String**> | Sets the host header. | 
**hash_keys** | Option<**String**> | Comma separated list of varnish request object fields that should be in the hash key. | 
**name** | Option<**String**> | Name for the request settings. | 
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**xff** | Option<**String**> | Short for X-Forwarded-For. | 
**bypass_busy_wait** | Option<**String**> | Disable collapsed forwarding, so you don't wait for other objects to origin. | 
**force_miss** | Option<**String**> | Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable. | 
**force_ssl** | Option<**String**> | Forces the request use SSL (redirects a non-SSL to SSL). | 
**geo_headers** | Option<**String**> | Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers. | 
**max_stale_age** | Option<**String**> | How old an object is allowed to be to serve stale-if-error or stale-while-revalidate. | 
**timer_support** | Option<**String**> | Injects the X-Timer info into the request for viewing origin fetch durations. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


