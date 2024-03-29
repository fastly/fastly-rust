# ResponseObjectResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_condition** | Option<**String**> | Name of the cache condition controlling when this configuration applies. | 
**content** | Option<**String**> | The content to deliver for the response object, can be empty. | 
**content_type** | Option<**String**> | The MIME type of the content, can be empty. | 
**name** | Option<**String**> | Name for the request settings. | 
**status** | Option<**String**> | The HTTP status code. | [default to 200]
**response** | Option<**String**> | The HTTP response. | [default to Ok]
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


