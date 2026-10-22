# ArcUsageMetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | **String** | The date (or hour) the usage was measured. Formatted as `YYYY-MM-DD` when `granularity` is `daily`, or `YYYY-MM-DD HH:00:00` when `granularity` is `hourly`.  | 
**usage_type** | **String** | The type of usage being measured. | 
**quantity** | **i32** | The quantity of the usage type for this date. | 
**virtual_key_id** | Option<**String**> | The ID of the virtual key this usage is attributed to. | 
**virtual_key_name** | Option<**String**> | The display name of the virtual key this usage is attributed to. | 
**primary_provider** | Option<**String**> | The ID of the provider connection originally requested, before any failover occurred. | 
**provider** | Option<**String**> | The name of the provider connection that ultimately served the request. | 
**provider_connection_id** | Option<**String**> | The ID of the provider connection that ultimately served the request. | 
**model** | Option<**String**> | The AI model identifier used. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


