# ServiceResponseAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [readonly]
**publish_key** | Option<**String**> | Unused at this time. | 
**paused** | Option<**bool**> | Whether the service is paused. Services are paused due to a lack of traffic for an extended period of time. Services are resumed either when a draft version is activated or a locked version is cloned and reactivated. | 
**versions** | Option<[**Vec&lt;crate::models::SchemasVersionResponse&gt;**](SchemasVersionResponse.md)> | A list of [versions](/reference/api/services/version/) associated with the service. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


