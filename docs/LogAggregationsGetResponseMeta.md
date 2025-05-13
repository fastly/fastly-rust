# LogAggregationsGetResponseMeta

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | Option<**String**> | Specifies the ID of the service for which data should be returned. | 
**start** | Option<**String**> | Start time for the query as supplied in the request. | 
**end** | Option<**String**> | End time for the query as supplied in the request. | 
**limit** | Option<**i32**> | Number of records per page. | [default to 20]
**sort** | Option<**String**> | Comma-separated list of the series names whose values were used to sort the results. | 
**filters** | Option<[**crate::models::LogAggregationsGetResponseMetaFilters**](LogAggregationsGetResponseMetaFilters.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


