# MetricsPlatformApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_platform_metrics_service_historical**](MetricsPlatformApi.md#get_platform_metrics_service_historical) | **GET** /metrics/platform/services/{service_id}/{granularity} | Get historical time series metrics for a single service



## get_platform_metrics_service_historical

Fetches historical metrics for a single service for a given granularity.

```rust
let cfg = &Configuration::default();
let params = GetPlatformMetricsServiceHistoricalParams {
    // parameters
};
get_platform_metrics_service_historical(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**granularity** | **String** | Duration of sample windows. | [required] |
**from** | Option\<**String**> | A valid RFC-8339-formatted date and time indicating the inclusive start of the query time range. If not provided, a default is chosen based on the provided `granularity` value. |  |
**to** | Option\<**String**> | A valid RFC-8339-formatted date and time indicating the exclusive end of the query time range. If not provided, a default is chosen based on the provided `granularity` value. |  |
**metric** | Option\<**String**> | The metric(s) to retrieve. Multiple values should be comma-separated. |  |
**metric_set** | Option\<**String**> | The metric set(s) to retrieve. Multiple values should be comma-separated. |  |
**group_by** | Option\<**String**> | Field to group_by in the query. For example, `group_by=region` will return entries for grouped by timestamp and region.  |  |
**region** | Option\<**String**> | Limit query to one or more specific geographic regions. Values should be comma-separated.  |  |
**datacenter** | Option\<**String**> | Limit query to one or more specific POPs. Values should be comma-separated. |  |
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 10000. |  |[default to 1000]

### Return type

[**crate::models::PlatformMetricsResponse**](PlatformMetricsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

