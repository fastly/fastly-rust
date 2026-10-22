# AiRuntimeControlUsageMetricsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**arc_export_usage_metrics**](AiRuntimeControlUsageMetricsApi.md#arc_export_usage_metrics) | **GET** /ai-runtime-control/v1/usage-metrics/export | Export usage metrics
[**arc_get_usage_metrics**](AiRuntimeControlUsageMetricsApi.md#arc_get_usage_metrics) | **GET** /ai-runtime-control/v1/usage-metrics | Get usage metrics



## arc_export_usage_metrics

Returns usage metrics for AI Runtime Control requests. Accepts the same filter parameters as [get usage metrics](#arc-get-usage-metrics), except `sort` and `granularity`, which aren't supported here.  Set the `Accept` header to `text/csv` to receive a CSV file download instead of a JSON response. In the CSV output, the column named `time` corresponds to the `date` field in the JSON response. 

```rust
let cfg = &Configuration::default();
let params = ArcExportUsageMetricsParams {
    // parameters
};
arc_export_usage_metrics(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option\<**String**> | Filter results to a specific virtual key ID. |  |
**provider** | Option\<**String**> | Filter results by AI provider connection ID. |  |
**model** | Option\<**String**> | Filter results by AI model identifier. |  |
**from** | Option\<**String**> | The start of the time range, in RFC 3339 format. |  |
**to** | Option\<**String**> | The end of the time range, in RFC 3339 format. Defaults to now. |  |

### Return type

[**crate::models::ArcUsageMetricsResponse**](ArcUsageMetricsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## arc_get_usage_metrics

Returns usage metrics for AI Runtime Control requests, including request counts, session counts, input token counts, and output token counts.

```rust
let cfg = &Configuration::default();
let params = ArcGetUsageMetricsParams {
    // parameters
};
arc_get_usage_metrics(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option\<**String**> | Filter results to a specific virtual key ID. |  |
**provider** | Option\<**String**> | Filter results by AI provider connection ID. |  |
**model** | Option\<**String**> | Filter results by AI model identifier. |  |
**from** | Option\<**String**> | The start of the time range, in RFC 3339 format. |  |
**to** | Option\<**String**> | The end of the time range, in RFC 3339 format. Defaults to now. |  |
**granularity** | Option\<**String**> | The time granularity to aggregate usage by. |  |[default to daily]
**sort** | Option\<**String**> | Sort field. Prefix with `-` for descending order (e.g., `sort=-date`).  |  |[default to date]

### Return type

[**crate::models::ArcUsageMetricsResponse**](ArcUsageMetricsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

