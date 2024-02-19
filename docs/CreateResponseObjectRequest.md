# CreateResponseObjectRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the response object to create. | 
**status** | Option<**String**> | The status code the response will have. Defaults to 200. | 
**response** | Option<**String**> | The status text the response will have. Defaults to 'OK'. | 
**content** | Option<**String**> | The content the response will deliver. | 
**content_type** | Option<**String**> | The MIME type of your response content. | 
**request_condition** | Option<**String**> | Condition which, if met, will select this configuration during a request. Optional. | 
**cache_condition** | Option<**String**> | Name of the cache condition controlling when this configuration applies. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


