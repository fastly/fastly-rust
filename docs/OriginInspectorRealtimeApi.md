# OriginInspectorRealtimeApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_origin_inspector_last120_seconds**](OriginInspectorRealtimeApi.md#get_origin_inspector_last120_seconds) | **GET** /v1/origins/{service_id}/ts/h | Get real-time origin data for the last 120 seconds
[**get_origin_inspector_last_max_entries**](OriginInspectorRealtimeApi.md#get_origin_inspector_last_max_entries) | **GET** /v1/origins/{service_id}/ts/h/limit/{max_entries} | Get a limited number of real-time origin data entries
[**get_origin_inspector_last_second**](OriginInspectorRealtimeApi.md#get_origin_inspector_last_second) | **GET** /v1/origins/{service_id}/ts/{start_timestamp} | Get real-time origin data from specific time.



## get_origin_inspector_last120_seconds

Get data for the 120 seconds preceding the latest timestamp available for a service.

```rust
let cfg = &Configuration::default();
let params = GetOriginInspectorLast120SecondsParams {
    // parameters
};
get_origin_inspector_last120_seconds(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::OriginInspector**](OriginInspector.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_origin_inspector_last_max_entries

Get data for the `max_entries` seconds preceding the latest timestamp available for a service, up to a maximum of 120 entries.

```rust
let cfg = &Configuration::default();
let params = GetOriginInspectorLastMaxEntriesParams {
    // parameters
};
get_origin_inspector_last_max_entries(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**max_entries** | **i32** | Maximum number of results to display. | [required] |

### Return type

[**crate::models::OriginInspector**](OriginInspector.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_origin_inspector_last_second

Get real-time origin data for the specified reporting period. Specify `0` to get a single entry for the last complete second. The `Timestamp` field included in the response provides the time index of the latest entry in the dataset and can be provided as the `start_timestamp` of the next request for a seamless continuation of the dataset from one request to the next. Due to processing latency, the earliest entry in the response dataset may be earlier than `start_timestamp` by the value of `AggregateDelay`. 

```rust
let cfg = &Configuration::default();
let params = GetOriginInspectorLastSecondParams {
    // parameters
};
get_origin_inspector_last_second(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**start_timestamp** | **i32** | Timestamp in seconds (Unix epoch time). | [required] |

### Return type

[**crate::models::OriginInspector**](OriginInspector.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

