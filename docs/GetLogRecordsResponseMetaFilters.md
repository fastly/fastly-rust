# GetLogRecordsResponseMetaFilters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**service_id** | Option<**String**> | Specifies the ID of the service for which data should be returned. | 
**start** | Option<**String**> | Start time for the query as supplied in the request. | 
**end** | Option<**String**> | End time for the query as supplied in the request. | 
**domain_exact_match** | Option<**bool**> | Value of the `domain_exact_match` filter as supplied in the request. | 
**limit** | Option<**i32**> | Number of records per page. | [default to 20]
**next_cursor** | Option<**String**> | A cursor to specify the next page of results, if any. | 
**filter_fields** | Option<[**Vec&lt;crate::models::FilterFieldItem&gt;**](FilterFieldItem.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


