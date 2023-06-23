# HistoricalApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_hist_stats**](HistoricalApi.md#get_hist_stats) | **GET** /stats | Get historical stats
[**get_hist_stats_aggregated**](HistoricalApi.md#get_hist_stats_aggregated) | **GET** /stats/aggregate | Get aggregated historical stats
[**get_hist_stats_field**](HistoricalApi.md#get_hist_stats_field) | **GET** /stats/field/{field} | Get historical stats for a single field
[**get_hist_stats_service**](HistoricalApi.md#get_hist_stats_service) | **GET** /stats/service/{service_id} | Get historical stats for a single service
[**get_hist_stats_service_field**](HistoricalApi.md#get_hist_stats_service_field) | **GET** /stats/service/{service_id}/field/{field} | Get historical stats for a single service/field combination
[**get_regions**](HistoricalApi.md#get_regions) | **GET** /stats/regions | Get region codes
[**get_usage**](HistoricalApi.md#get_usage) | **GET** /stats/usage | Get usage statistics
[**get_usage_month**](HistoricalApi.md#get_usage_month) | **GET** /stats/usage_by_month | Get month-to-date usage statistics
[**get_usage_service**](HistoricalApi.md#get_usage_service) | **GET** /stats/usage_by_service | Get usage statistics per service



## get_hist_stats

Fetches historical stats for each of your Fastly services and groups the results by service ID.

```rust
let cfg = &Configuration::default();
let params = GetHistStatsParams {
    // parameters
};
get_hist_stats(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]
**by** | Option\<**String**> | Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day.  |  |[default to day]
**region** | Option\<**String**> | Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America.  |  |

### Return type

[**crate::models::HistoricalResponse**](HistoricalResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_hist_stats_aggregated

Fetches historical stats information aggregated across all of your Fastly services.

```rust
let cfg = &Configuration::default();
let params = GetHistStatsAggregatedParams {
    // parameters
};
get_hist_stats_aggregated(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]
**by** | Option\<**String**> | Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day.  |  |[default to day]
**region** | Option\<**String**> | Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America.  |  |

### Return type

[**crate::models::HistoricalAggregateResponse**](HistoricalAggregateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_hist_stats_field

Fetches the specified field from the historical stats for each of your services and groups the results by service ID.

```rust
let cfg = &Configuration::default();
let params = GetHistStatsFieldParams {
    // parameters
};
get_hist_stats_field(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** | Name of the stats field. | [required] |
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]
**by** | Option\<**String**> | Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day.  |  |[default to day]
**region** | Option\<**String**> | Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America.  |  |

### Return type

[**crate::models::HistoricalFieldResponse**](HistoricalFieldResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_hist_stats_service

Fetches historical stats for a given service.

```rust
let cfg = &Configuration::default();
let params = GetHistStatsServiceParams {
    // parameters
};
get_hist_stats_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]
**by** | Option\<**String**> | Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day.  |  |[default to day]
**region** | Option\<**String**> | Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America.  |  |

### Return type

[**crate::models::HistoricalAggregateResponse**](HistoricalAggregateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_hist_stats_service_field

Fetches the specified field from the historical stats for a given service.

```rust
let cfg = &Configuration::default();
let params = GetHistStatsServiceFieldParams {
    // parameters
};
get_hist_stats_service_field(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**field** | **String** | Name of the stats field. | [required] |
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]
**by** | Option\<**String**> | Duration of sample windows. One of:   * `hour` - Group data by hour.   * `minute` - Group data by minute.   * `day` - Group data by day.  |  |[default to day]
**region** | Option\<**String**> | Limit query to a specific geographic region. One of:   * `usa` - North America.   * `europe` - Europe.   * `anzac` - Australia and New Zealand.   * `asia` - Asia.   * `asia_india` - India.   * `asia_southkorea` - South Korea.   * `africa_std` - Africa.   * `southamerica_std` - South America.  |  |

### Return type

[**crate::models::HistoricalFieldAggregateResponse**](HistoricalFieldAggregateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_regions

Fetches the list of codes for regions that are covered by the Fastly CDN service.

```rust
let cfg = &Configuration::default();
let params = GetRegionsParams {
    // parameters
};
get_regions(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HistoricalRegionsResponse**](HistoricalRegionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_usage

Returns usage information aggregated across all Fastly services and grouped by region. To aggregate across all Fastly services by time period, see [`/stats/aggregate`](#get-hist-stats-aggregated).

```rust
let cfg = &Configuration::default();
let params = GetUsageParams {
    // parameters
};
get_usage(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]

### Return type

[**crate::models::HistoricalUsageAggregateResponse**](HistoricalUsageAggregateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_usage_month

Returns month-to-date usage details for a given month and year. Usage details are aggregated by service and across all Fastly services, and then grouped by region. This endpoint does not use the `from` or `to` fields for selecting the date for which data is requested. Instead, it uses `month` and `year` integer fields. Both fields are optional and default to the current month and year respectively. When set, an optional `billable_units` field will convert bandwidth to GB and divide requests by 10,000.

```rust
let cfg = &Configuration::default();
let params = GetUsageMonthParams {
    // parameters
};
get_usage_month(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option\<**String**> | 4-digit year. |  |
**month** | Option\<**String**> | 2-digit month. |  |
**billable_units** | Option\<**bool**> | If `true`, return results as billable units. |  |

### Return type

[**crate::models::HistoricalUsageMonthResponse**](HistoricalUsageMonthResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_usage_service

Returns usage information aggregated by service and grouped by service and region. For service stats by time period, see [`/stats`](#get-hist-stats) and [`/stats/field/:field`](#get-hist-stats-field).

```rust
let cfg = &Configuration::default();
let params = GetUsageServiceParams {
    // parameters
};
get_usage_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option\<**String**> | Timestamp that defines the start of the window for which to fetch statistics, including the timestamp itself. Accepts Unix timestamps, or any form of input parsable by the [Chronic Ruby library](https://github.com/mojombo/chronic), such as 'yesterday', or 'two weeks ago'. Default varies based on the value of `by`.  |  |
**to** | Option\<**String**> | Timestamp that defines the end of the window for which to fetch statistics. Accepts the same formats as `from`.  |  |[default to now]

### Return type

[**crate::models::HistoricalUsageServiceResponse**](HistoricalUsageServiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

