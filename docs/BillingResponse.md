# BillingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_time** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**start_time** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**invoice_id** | Option<**String**> |  | [readonly]
**customer_id** | Option<**String**> |  | [readonly]
**vendor_state** | Option<**String**> | The current state of our third-party billing vendor. One of `up` or `down`. | [readonly]
**status** | Option<[**crate::models::BillingStatus**](BillingStatus.md)> |  | 
**total** | Option<[**crate::models::BillingTotal**](BillingTotal.md)> |  | 
**regions** | Option<[**::std::collections::HashMap&lt;String, ::std::collections::HashMap&lt;String, serde_json::Value&gt;&gt;**](Map.md)> | Breakdown of regional data for products that are region based. | 
**line_items** | Option<[**Vec&lt;crate::models::BillingResponseLineItem&gt;**](BillingResponseLineItem.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


