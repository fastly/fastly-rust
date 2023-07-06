# EventAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin** | Option<**bool**> | Indicates if event was performed by Fastly. | 
**created_at** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**customer_id** | Option<**String**> |  | [readonly]
**description** | Option<**String**> | Description of the event. | 
**event_type** | Option<**String**> | Type of event. Can be used with `filter[event_type]` | 
**ip** | Option<**String**> | IP addresses that the event was requested from. | 
**metadata** | Option<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](SerdeJsonValue.md)> | Hash of key value pairs of additional information. | 
**service_id** | Option<**String**> |  | [readonly]
**user_id** | Option<**String**> |  | [readonly]
**token_id** | Option<**String**> |  | [readonly]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


