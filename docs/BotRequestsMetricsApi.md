# BotRequestsMetricsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_bot_request_metrics_service_historical**](BotRequestsMetricsApi.md#get_bot_request_metrics_service_historical) | **GET** /metrics/bots/entities/{service_id}/{granularity} | Get per-service historical time series metrics for bot requests broken down by bot-type.



## get_bot_request_metrics_service_historical

Return historical time series metrics for bot requests broken down by bot-type.

```rust
let cfg = &Configuration::default();
let params = GetBotRequestMetricsServiceHistoricalParams {
    // parameters
};
get_bot_request_metrics_service_historical(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granularity** | **String** | Duration of sample windows. | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**from** | Option\<**String**> | A valid RFC-8339-formatted date and time indicating the inclusive start of the query time range. If not provided, a default is chosen based on the provided `granularity` value. |  |
**to** | Option\<**String**> | A valid RFC-8339-formatted date and time indicating the exclusive end of the query time range. If not provided, a default is chosen based on the provided `granularity` value. |  |
**group_by** | Option\<**String**> | Field to group by in the query. For example, `group_by=bot_id` will return entries grouped by `timestamp` and `bot_id`.  |  |
**datacenter** | Option\<**String**> | Limit query to one or more specific POPs. Values should be comma-separated. |  |
**bot** | Option\<**String**> | Filter results based by Bot ID. |  |
**region** | Option\<**String**> | Limit query to one or more specific geographic regions. Values should be comma-separated.  |  |
**cursor** | Option\<**String**> | Cursor value from the `next_cursor` field of a previous response, used to retrieve the next page. To request the first page, this should be empty. |  |
**metric** | Option\<**String**> | One or more metrics to retrieve. Multiple values should be comma-separated. |  |
**metric_set** | Option\<**String**> | The metric set to retrieve. |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 10000. |  |[default to 1000]

### Return type

[**crate::models::BotRequestMetricsResponse**](BotRequestMetricsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

