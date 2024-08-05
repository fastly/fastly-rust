# BillingUsageMetricsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_service_level_usage**](BillingUsageMetricsApi.md#get_service_level_usage) | **GET** /billing/v2/account_customers/{customer_id}/service-usage-metrics | Retrieve service-level usage metrics for a product.
[**get_service_level_usage_types**](BillingUsageMetricsApi.md#get_service_level_usage_types) | **GET** /billing/v2/account_customers/{customer_id}/service-usage-types | Retrieve product usage types for a customer.



## get_service_level_usage

Returns product usage, broken down by service.

```rust
let cfg = &Configuration::default();
let params = GetServiceLevelUsageParams {
    // parameters
};
get_service_level_usage(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |
**product_id** | **String** | The product identifier for the metrics returned (e.g., `cdn_usage`). This field is not required for CSV requests. | [required] |
**usage_type_name** | **String** | The usage type name for the metrics returned (e.g., `North America Requests`). This field is not required for CSV requests. | [required] |
**time_granularity** | **String** |  | [required] |
**start_date** | Option\<**String**> |  |  |
**end_date** | Option\<**String**> |  |  |
**start_month** | Option\<**String**> |  |  |
**end_month** | Option\<**String**> |  |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 100. |  |[default to 5]
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |

### Return type

[**crate::models::Serviceusagemetrics**](Serviceusagemetrics.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_service_level_usage_types

Returns product usage types reported by the customer's services.

```rust
let cfg = &Configuration::default();
let params = GetServiceLevelUsageTypesParams {
    // parameters
};
get_service_level_usage_types(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**crate::models::Serviceusagetypes**](Serviceusagetypes.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

