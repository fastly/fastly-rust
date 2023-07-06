# OriginInspector

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<[**crate::models::OriginInspectorSubsequentRequestTimestamp**](OriginInspectorSubsequentRequestTimestamp.md)> |  | 
**aggregate_delay** | Option<**i32**> | Offset of entry timestamps from the current time due to processing time. | 
**data** | Option<[**Vec&lt;crate::models::OriginInspectorRealtimeEntry&gt;**](OriginInspectorRealtimeEntry.md)> | A list of report [entries](#entry-data-model), each representing one second of time. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


