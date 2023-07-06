# OriginInspectorHistoricalMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start** | Option<**String**> | Start time that was used to perform the query as an ISO-8601-formatted date and time. | 
**end** | Option<**String**> | End time that was used to perform the query as an ISO-8601-formatted date and time. | 
**downsample** | Option<**String**> | Downsample that was used to perform the query. One of `minute`, `hour` or `day`. | 
**metrics** | Option<**String**> | A comma-separated list of the metrics that were requested. | 
**limit** | Option<**f32**> | The maximum number of results shown per page. | 
**next_cursor** | Option<**String**> | A string that can be used to request the next page of results, if any. | 
**sort** | Option<**String**> | A comma-separated list of keys the results are sorted by. | 
**group_by** | Option<**String**> | A comma-separated list of dimensions being summed over in the query. | 
**filters** | Option<[**crate::models::OriginInspectorHistoricalMetaFilters**](OriginInspectorHistoricalMetaFilters.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


