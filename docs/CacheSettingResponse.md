# CacheSettingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | If set, will cause vcl_fetch to terminate after processing this rule with the return state specified. If not set, other configuration logic in vcl_fetch with a lower priority will run after this rule.  | 
**cache_condition** | Option<**String**> | Name of the cache condition controlling when this configuration applies. | 
**name** | Option<**String**> | Name for the cache settings object. | 
**stale_ttl** | Option<**i32**> | Maximum time in seconds to continue to use a stale version of the object if future requests to your backend server fail (also known as 'stale if error'). | 
**ttl** | Option<**i32**> | Maximum time to consider the object fresh in the cache (the cache 'time to live'). | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


