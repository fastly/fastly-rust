# RealtimeApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stats_last120_seconds**](RealtimeApi.md#get_stats_last120_seconds) | **GET** /v1/channel/{service_id}/ts/h | Get real-time data for the last 120 seconds
[**get_stats_last120_seconds_limit_entries**](RealtimeApi.md#get_stats_last120_seconds_limit_entries) | **GET** /v1/channel/{service_id}/ts/h/limit/{max_entries} | Get a limited number of real-time data entries
[**get_stats_last_second**](RealtimeApi.md#get_stats_last_second) | **GET** /v1/channel/{service_id}/ts/{timestamp_in_seconds} | Get real-time data from specified time



## get_stats_last120_seconds

Get data for the 120 seconds preceding the latest timestamp available for a service.

```rust
let cfg = &Configuration::default();
let params = GetStatsLast120SecondsParams {
    // parameters
};
get_stats_last120_seconds(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::Realtime**](Realtime.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stats_last120_seconds_limit_entries

Get data for the 120 seconds preceding the latest timestamp available for a service, up to a maximum of `max_entries` entries.

```rust
let cfg = &Configuration::default();
let params = GetStatsLast120SecondsLimitEntriesParams {
    // parameters
};
get_stats_last120_seconds_limit_entries(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**max_entries** | **i32** | Maximum number of results to show. | [required] |

### Return type

[**crate::models::Realtime**](Realtime.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stats_last_second

Get real-time data for the specified reporting period. Specify `0` to get a single entry for the last complete second. The `Timestamp` field included in the response provides the time index of the latest entry in the dataset and can be provided as the `start_timestamp` of the next request for a seamless continuation of the dataset from one request to the next.

```rust
let cfg = &Configuration::default();
let params = GetStatsLastSecondParams {
    // parameters
};
get_stats_last_second(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**timestamp_in_seconds** | **i32** | Timestamp in seconds (Unix epoch time). | [required] |

### Return type

[**crate::models::Realtime**](Realtime.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

