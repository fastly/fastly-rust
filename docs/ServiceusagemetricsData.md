# ServiceusagemetricsData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> |  | [readonly]
**start_time** | Option<**String**> | Date and time (in ISO 8601 format) for initiation point of a billing cycle, signifying the start of charges for a service or subscription. | 
**end_time** | Option<**String**> | Date and time (in ISO 8601 format) for termination point of a billing cycle, signifying the end of charges for a service or subscription. | 
**usage_type** | Option<**String**> | The usage type identifier for the usage. This is a single, billable metric for the product. | 
**unit** | Option<**String**> | The unit for the usage as shown on an invoice. If there is no explicit unit, this field will be \"unit\" (e.g., a request with `product_id` of 'cdn_usage' and `usage_type` of 'North America Requests' has no unit, and will return \"unit\"). | 
**details** | Option<[**Vec&lt;crate::models::Serviceusagemetric&gt;**](Serviceusagemetric.md)> |  | 
**meta** | Option<[**crate::models::Metadata**](Metadata.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


