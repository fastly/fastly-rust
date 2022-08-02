# ConditionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | A freeform descriptive note. | 
**name** | Option<**String**> | Name of the condition. Required. | 
**priority** | Option<**String**> | A numeric string. Priority determines execution order. Lower numbers execute first. | [default to 100]
**statement** | Option<**String**> | A conditional expression in VCL used to determine if the condition is met. | 
**service_id** | Option<**String**> |  | [readonly]
**version** | Option<**String**> | A numeric string that represents the service version. | 
**_type** | Option<**String**> | Type of the condition. Required. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


