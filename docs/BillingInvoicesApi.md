# BillingInvoicesApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_invoice_by_invoice_id**](BillingInvoicesApi.md#get_invoice_by_invoice_id) | **GET** /billing/v3/invoices/{invoice_id} | Get invoice by ID.
[**get_month_to_date_invoice**](BillingInvoicesApi.md#get_month_to_date_invoice) | **GET** /billing/v3/invoices/month-to-date | Get month-to-date invoice.
[**list_invoices**](BillingInvoicesApi.md#list_invoices) | **GET** /billing/v3/invoices | List of invoices.



## get_invoice_by_invoice_id

Returns invoice associated with the invoice id.

```rust
let cfg = &Configuration::default();
let params = GetInvoiceByInvoiceIdParams {
    // parameters
};
get_invoice_by_invoice_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **i32** |  | [required] |

### Return type

[**crate::models::EomInvoiceResponse**](EomInvoiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_month_to_date_invoice

Returns month-to-date invoice for the current month.

```rust
let cfg = &Configuration::default();
let params = GetMonthToDateInvoiceParams {
    // parameters
};
get_month_to_date_invoice(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MtdInvoiceResponse**](MtdInvoiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_invoices

Returns the list of invoices, sorted by billing start date (newest to oldest).

```rust
let cfg = &Configuration::default();
let params = ListInvoicesParams {
    // parameters
};
list_invoices(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**billing_start_date** | Option\<**String**> |  |  |
**billing_end_date** | Option\<**String**> |  |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 200. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |

### Return type

[**crate::models::ListEomInvoicesResponse**](ListEomInvoicesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

