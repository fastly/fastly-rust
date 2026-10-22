# BotRequestMetricsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**Vec&lt;crate::models::BotRequestMetricsDataItem&gt;**](BotRequestMetricsDataItem.md)> | An array of time series data items containing metric counts. Items are grouped by `timestamp` by default, or by a combination of `timestamp` and `bot` or `datacenter` or `region` when the `group_by` parameter is set. This dataset is sparse: only keys with non-zero values are included.  | 
**meta** | Option<[**crate::models::BotRequestMetricsMetadata**](BotRequestMetricsMetadata.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


