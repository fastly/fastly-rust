# ResponseObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_condition** | Option<**String**> | Name of the cache condition controlling when this configuration applies. | 
**content** | Option<**String**> | The content to deliver for the response object, can be empty. | 
**content_type** | Option<**String**> | The MIME type of the content, can be empty. | 
**name** | Option<**String**> | Name for the request settings. | 
**status** | Option<**i32**> | The HTTP status code. | [default to 200]
**response** | Option<**String**> | The HTTP response. | [default to Ok]
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


