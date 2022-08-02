# VersionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | Whether this is the active version or not. | [default to false]
**comment** | Option<**String**> | A freeform descriptive note. | 
**deployed** | Option<**bool**> | Unused at this time. | 
**locked** | Option<**bool**> | Whether this version is locked or not. Objects can not be added or edited on locked versions. | [default to false]
**number** | Option<**i32**> | The number of this version. | [readonly]
**staging** | Option<**bool**> | Unused at this time. | [default to false]
**testing** | Option<**bool**> | Unused at this time. | [default to false]
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**service_id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


