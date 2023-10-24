# PlatformDdosResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Whether or not we were able to successfully execute the query. | 
**meta** | Option<[**crate::models::HistoricalDdosMeta**](HistoricalDdosMeta.md)> |  | 
**msg** | Option<**String**> | If the query was not successful, this will provide a string that explains why. | 
**data** | Option<[**Vec&lt;crate::models::PlatformDdosEntry&gt;**](PlatformDdosEntry.md)> | A list of timeseries. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


