# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Customer ID associated with the invoice. | 
**invoice_id** | Option<**String**> | Numeric string identifying the invoice. | 
**invoice_posted_on** | Option<**String**> | Date and time invoice was posted on, in ISO 8601 format. | 
**billing_start_date** | Option<**String**> | Date and time (in ISO 8601 format) for initiation point of a billing cycle, signifying the start of charges for a service or subscription. | 
**billing_end_date** | Option<**String**> | Date and time (in ISO 8601 format) for termination point of a billing cycle, signifying the end of charges for a service or subscription. | 
**statement_number** | Option<**String**> | Alphanumeric string identifying the statement number. | 
**currency_code** | Option<**String**> | Three-letter code representing a specific currency used for financial transactions. | 
**monthly_transaction_amount** | Option<**f32**> | Total billable amount for invoiced services charged within a single month. | 
**transaction_line_items** | Option<[**Vec&lt;crate::models::Invoicelineitems&gt;**](Invoicelineitems.md)> |  | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


