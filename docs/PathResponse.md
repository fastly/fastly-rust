# PathResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> | Alphanumeric string identifying the path. Stable across versions of the routing config. | [readonly]
**path** | Option<**String**> | The URL path pattern, beginning with `/`. Maximum 2048 characters. | 
**links** | Option<**::std::collections::HashMap<String, String>**> | HATEOAS links to related resources. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


