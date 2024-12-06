# BillingUsageMetricsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_service_level_usage**](BillingUsageMetricsApi.md#get_service_level_usage) | **GET** /billing/v3/service-usage-metrics | Retrieve service-level usage metrics for a product.
[**get_usage_metrics**](BillingUsageMetricsApi.md#get_usage_metrics) | **GET** /billing/v3/usage-metrics | Get monthly usage metrics



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
**product_id** | **String** | The product identifier for the metrics returned (e.g., `cdn_usage`). This field is not required for CSV requests. | [required] |
**usage_type_name** | **String** | The usage type name for the metrics returned (e.g., `North America Requests`). This field is not required for CSV requests. | [required] |
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


## get_usage_metrics

Returns monthly usage metrics for customer by product.

```rust
let cfg = &Configuration::default();
let params = GetUsageMetricsParams {
    // parameters
};
get_usage_metrics(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_month** | Option\<**String**> |  |  |
**end_month** | Option\<**String**> |  |  |

### Return type

[**crate::models::Usagemetric**](Usagemetric.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

