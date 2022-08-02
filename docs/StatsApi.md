# StatsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_stats**](StatsApi.md#get_service_stats) | **GET** /service/{service_id}/stats/summary | Get stats for a service



## get_service_stats

Get the stats from a service for a block of time. This lists all stats by PoP location, starting with AMS. This call requires parameters to select block of time to query. Use either a timestamp range (using start_time and end_time) or a specified month/year combo (using month and year).

```rust
let cfg = &Configuration::default();
let params = GetServiceStatsParams {
    // parameters
};
get_service_stats(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**month** | Option\<**String**> | 2-digit month. |  |
**year** | Option\<**String**> | 4-digit year. |  |
**start_time** | Option\<**i32**> | Epoch timestamp. Limits the results returned. |  |
**end_time** | Option\<**i32**> | Epoch timestamp. Limits the results returned. |  |

### Return type

[**crate::models::Stats**](Stats.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

