# OriginInspectorHistoricalApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_origin_inspector_historical**](OriginInspectorHistoricalApi.md#get_origin_inspector_historical) | **GET** /metrics/origins/services/{service_id} | Get historical origin data for a service



## get_origin_inspector_historical

Fetches historical origin metrics for a given Fastly service, optionally filtering and grouping the results by origin host, region, or POP. 

```rust
let cfg = &Configuration::default();
let params = GetOriginInspectorHistoricalParams {
    // parameters
};
get_origin_inspector_historical(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**start** | Option\<**String**> | A valid ISO-8601-formatted date and time, or UNIX timestamp, indicating the inclusive start of the query time range. If not provided, a default is chosen based on the provided `downsample` value. |  |
**end** | Option\<**String**> | A valid ISO-8601-formatted date and time, or UNIX timestamp, indicating the exclusive end of the query time range. If not provided, a default is chosen based on the provided `downsample` value. |  |
**downsample** | Option\<**String**> | Duration of sample windows. |  |[default to hour]
**metric** | Option\<**String**> | The metric to retrieve. Up to ten comma-separated metrics are accepted. |  |[default to responses]
**group_by** | Option\<**String**> | Dimensions to return in the query. Multiple dimensions may be separated by commas. For example, `group_by=host` will return one timeseries for every origin host, as a total across all POPs.  |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 200. |  |[default to 100]
**cursor** | Option\<**String**> | Cursor value from a previous response to retrieve the next page. To request the first page, this should be empty. |  |
**region** | Option\<**String**> | Limit query to one or more specific geographic regions. Values should be comma-separated.  |  |
**datacenter** | Option\<**String**> | Limit query to one or more specific POPs. Values should be comma-separated. |  |
**host** | Option\<**String**> | Limit query to one or more specific origin hosts. Values should be comma-separated. |  |

### Return type

[**crate::models::HistoricalOriginsResponse**](HistoricalOriginsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

