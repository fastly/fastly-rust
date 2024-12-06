# Usagemetric

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**month** | Option<**String**> | The year and month of the usage element. | 
**usage_type** | Option<**String**> | The usage type identifier for the usage. This is a single, billable metric for the product. | 
**name** | Option<**String**> | Full name of the product usage type as it might appear on a customer's invoice. | 
**region** | Option<**String**> | The geographical area applicable for regionally based products. | 
**unit** | Option<**String**> | The unit for the usage as shown on an invoice. If there is no explicit unit, this field will be \"unit\" (e.g., a request with `product_id` of 'cdn_usage' and `usage_type` of 'North America Requests' has no unit, and will return \"unit\"). | 
**quantity** | Option<**f32**> | The quantity of the usage for the product. | 
**raw_quantity** | Option<**f32**> | The raw units measured for the product. | 
**product_id** | Option<**String**> | The product identifier associated with the usage type. This corresponds to a Fastly product offering. | 
**last_updated_at** | Option<**String**> | The date when the usage metric was last updated. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


