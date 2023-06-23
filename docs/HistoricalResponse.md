# HistoricalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Whether or not we were able to successfully execute the query. | 
**meta** | Option<[**crate::models::HistoricalMeta**](HistoricalMeta.md)> |  | 
**msg** | Option<**String**> | If the query was not successful, this will provide a string that explains why. | 
**data** | Option<[**::std::collections::HashMap&lt;String, crate::models::Array&gt;**](Array.md)> | Contains the results of the query, organized by *service ID*, into arrays where each element describes one service over a *time span*. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


