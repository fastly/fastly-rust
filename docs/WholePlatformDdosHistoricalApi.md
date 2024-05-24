# WholePlatformDdosHistoricalApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_platform_ddos_historical**](WholePlatformDdosHistoricalApi.md#get_platform_ddos_historical) | **GET** /metrics/platform/ddos | Get historical DDoS metrics for the entire Fastly platform



## get_platform_ddos_historical

Fetches historical DDoS metrics for the entire Fastly platform.

```rust
let cfg = &Configuration::default();
let params = GetPlatformDdosHistoricalParams {
    // parameters
};
get_platform_ddos_historical(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option\<**String**> | A valid ISO-8601-formatted date and time, or UNIX timestamp, indicating the inclusive start of the query time range. If not provided, a default is chosen based on the provided `downsample` value. |  |
**end** | Option\<**String**> | A valid ISO-8601-formatted date and time, or UNIX timestamp, indicating the exclusive end of the query time range. If not provided, a default is chosen based on the provided `downsample` value. |  |
**downsample** | Option\<**String**> | Duration of sample windows. |  |[default to hour]

### Return type

[**crate::models::PlatformDdosResponse**](PlatformDdosResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

