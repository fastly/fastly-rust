# Mtdinvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | The Customer ID associated with the invoice. | 
**invoice_id** | Option<**String**> | An alphanumeric string identifying the invoice. | 
**billing_start_date** | Option<**String**> | The date and time (in ISO 8601 format) for the initiation point of a billing cycle, signifying the start of charges for a service or subscription. | 
**billing_end_date** | Option<**String**> | The date and time (in ISO 8601 format) for the termination point of a billing cycle, signifying the end of charges for a service or subscription. | 
**monthly_transaction_amount** | Option<**String**> | The total billable amount for invoiced services charged within a single month. | 
**transaction_line_items** | Option<[**Vec&lt;crate::models::Mtdlineitems&gt;**](Mtdlineitems.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


