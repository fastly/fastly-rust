# BotRequestMetricsMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | Option<**String**> | An RFC-8339-formatted date and time string indicating the inclusive start of the query time range. | 
**to** | Option<**String**> | An RFC-8339-formatted date and time indicating the exclusive end of the query time range. | 
**group_by** | Option<**String**> | List of dimensions (comma-separated) being summed over in the results. | 
**next_cursor** | Option<**String**> | Optional string that can be used to request the next page of results, if any. | 
**limit** | Option<**i32**> | The maximum number of results to return. Defaults to 1000. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


