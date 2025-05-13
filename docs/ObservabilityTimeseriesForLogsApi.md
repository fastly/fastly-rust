# ObservabilityTimeseriesForLogsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**log_timeseries_get**](ObservabilityTimeseriesForLogsApi.md#log_timeseries_get) | **GET** /observability/timeseries | Retrieve log data as time series



## log_timeseries_get

Retrieves log data as time series.

```rust
let cfg = &Configuration::default();
let params = LogTimeseriesGetParams {
    // parameters
};
log_timeseries_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | **String** |  | [required] |
**service_id** | **String** |  | [required] |
**start** | **String** |  | [required] |
**end** | **String** |  | [required] |
**granularity** | **String** |  | [required] |
**series** | **String** |  | [required] |
**filter** | Option\<**String**> |  |  |

### Return type

[**crate::models::LogTimeseriesGetResponse**](LogTimeseriesGetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

