# RequestSettingsResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bypass_busy_wait** | Option<**String**> | Disable collapsed forwarding, so you don't wait for other objects to origin. | 
**force_miss** | Option<**String**> | Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable. | 
**force_ssl** | Option<**String**> | Forces the request use SSL (redirects a non-SSL to SSL). | 
**geo_headers** | Option<**String**> | Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers. | 
**max_stale_age** | Option<**String**> | How old an object is allowed to be to serve stale-if-error or stale-while-revalidate. | 
**timer_support** | Option<**String**> | Injects the X-Timer info into the request for viewing origin fetch durations. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


