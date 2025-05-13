# ObservabilityAggregationsForLogsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**log_aggregations_get**](ObservabilityAggregationsForLogsApi.md#log_aggregations_get) | **GET** /observability/aggregations | Retrieve aggregated log results



## log_aggregations_get

Retrieves aggregated log results.

```rust
let cfg = &Configuration::default();
let params = LogAggregationsGetParams {
    // parameters
};
log_aggregations_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **String** |  | [required] |
**service_id** | **String** |  | [required] |
**start** | **String** |  | [required] |
**end** | **String** |  | [required] |
**series** | **String** |  | [required] |
**limit** | Option\<**f32**> |  |  |
**filter** | Option\<**String**> |  |  |
**dimensions** | Option\<**String**> |  |  |
**sort** | Option\<**String**> |  |  |

### Return type

[**crate::models::LogAggregationsGetResponse**](LogAggregationsGetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

