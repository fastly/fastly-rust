# InsightsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_log_insights**](InsightsApi.md#get_log_insights) | **GET** /observability/log-insights | Retrieve log insights



## get_log_insights

Retrieves statistics from sampled log records.

```rust
let cfg = &Configuration::default();
let params = GetLogInsightsParams {
    // parameters
};
get_log_insights(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visualization** | **String** |  | [required] |
**service_id** | **String** |  | [required] |
**start** | **String** |  | [required] |
**end** | **String** |  | [required] |
**pops** | Option\<**String**> |  |  |
**domain** | Option\<**String**> |  |  |
**domain_exact_match** | Option\<**bool**> |  |  |
**limit** | Option\<**f32**> |  |  |

### Return type

[**crate::models::GetLogInsightsResponse**](GetLogInsightsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

