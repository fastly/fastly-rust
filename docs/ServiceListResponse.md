# ServiceListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**deleted_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**comment** | Option<**String**> | A freeform descriptive note. | 
**name** | Option<**String**> | The name of the service. | 
**customer_id** | Option<**String**> | Alphanumeric string identifying the customer. | 
**_type** | Option<**String**> | The type of this service. | 
**id** | Option<**String**> |  | [readonly]
**version** | Option<**i32**> | Current [version](https://www.fastly.com/documentation/reference/api/services/version/) of the service. | 
**versions** | Option<[**Vec&lt;crate::models::SchemasVersionResponse&gt;**](SchemasVersionResponse.md)> | A list of [versions](https://www.fastly.com/documentation/reference/api/services/version/) associated with the service. | 
**environments** | Option<[**Vec&lt;crate::models::Environment&gt;**](Environment.md)> | A list of environments where the service has been deployed. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


