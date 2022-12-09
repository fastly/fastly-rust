# HttpResponseFormat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**i32**> | The HTTP status code. | [default to 200]
**reason** | Option<**String**> | The HTTP status string. Defaults to a string appropriate for `code`. | 
**headers** | Option<**::std::collections::HashMap<String, String>**> | A map of arbitrary HTTP response headers to include in the response. | 
**body** | Option<**String**> | The response body as a string. | 
**body_bin** | Option<**String**> | The response body as a base64-encoded binary blob. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


