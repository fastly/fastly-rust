# HistoricalDomainsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Whether or not we were able to successfully execute the query. | 
**meta** | Option<[**crate::models::HistoricalDomainsMeta**](HistoricalDomainsMeta.md)> |  | 
**msg** | Option<**String**> | If the query was not successful, this will provide a string that explains why. | 
**data** | Option<[**Vec&lt;crate::models::DomainInspectorEntry&gt;**](DomainInspectorEntry.md)> | A list of timeseries. Each individual timeseries represents a unique combination of dimensions, such as domain, region or POP. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


