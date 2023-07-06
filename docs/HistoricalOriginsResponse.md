# HistoricalOriginsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Whether or not we were able to successfully execute the query. | 
**meta** | Option<[**crate::models::OriginInspectorHistoricalMeta**](OriginInspectorHistoricalMeta.md)> |  | 
**msg** | Option<**String**> | If the query was not successful, this will provide a string that explains why. | 
**data** | Option<[**Vec&lt;crate::models::OriginInspectorEntry&gt;**](OriginInspectorEntry.md)> | A list of timeseries. Each individual timeseries represents a unique combination of dimensions, such as origin host, region or POP. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


