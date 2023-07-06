# Realtime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | Option<**i32**> | Value to use for subsequent requests. | 
**aggregate_delay** | Option<**i32**> | How long the system will wait before aggregating messages for each second. The most recent data returned will have happened at the moment of the request, minus the aggregation delay. | 
**data** | Option<[**Vec&lt;crate::models::RealtimeEntry&gt;**](RealtimeEntry.md)> | A list of [records](#record-data-model), each representing one second of time. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


