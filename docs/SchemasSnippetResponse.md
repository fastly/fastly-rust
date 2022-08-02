# SchemasSnippetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name for the snippet. | 
**dynamic** | Option<**i32**> | Sets the snippet version. | 
**_type** | Option<**String**> | The location in generated VCL where the snippet should be placed. | 
**content** | Option<**String**> | The VCL code that specifies exactly what the snippet does. | 
**priority** | Option<**i32**> | Priority determines execution order. Lower numbers execute first. | [default to 100]
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> |  | [readonly]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


