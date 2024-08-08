# Billing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_time** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**start_time** | Option<**String**> | Date and time in ISO 8601 format. | [readonly]
**customer_id** | Option<**String**> |  | [readonly]
**vendor_state** | Option<**String**> | The current state of our third-party billing vendor. One of `up` or `down`. | [readonly]
**status** | Option<[**crate::models::BillingStatus**](BillingStatus.md)> |  | 
**total** | Option<[**crate::models::BillingTotal**](BillingTotal.md)> |  | 
**regions** | Option<[**::std::collections::HashMap&lt;String, crate::models::BillingRegions&gt;**](BillingRegions.md)> | Breakdown of regional data for products that are region based. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


