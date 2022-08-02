# RateLimiterApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_rate_limiter**](RateLimiterApi.md#delete_rate_limiter) | **DELETE** /rate-limiters/{rate_limiter_id} | Delete a rate limiter
[**get_rate_limiter**](RateLimiterApi.md#get_rate_limiter) | **GET** /rate-limiters/{rate_limiter_id} | Get a rate limiter
[**list_rate_limiters**](RateLimiterApi.md#list_rate_limiters) | **GET** /service/{service_id}/version/{version_id}/rate-limiters | List rate limiters



## delete_rate_limiter

Delete a rate limiter by its ID.

```rust
let cfg = &Configuration::default();
let params = DeleteRateLimiterParams {
    // parameters
};
delete_rate_limiter(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rate_limiter_id** | **String** | Alphanumeric string identifying the rate limiter. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_rate_limiter

Get a rate limiter by its ID.

```rust
let cfg = &Configuration::default();
let params = GetRateLimiterParams {
    // parameters
};
get_rate_limiter(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rate_limiter_id** | **String** | Alphanumeric string identifying the rate limiter. | [required] |

### Return type

[**crate::models::RateLimiterResponse**](RateLimiterResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_rate_limiters

List all rate limiters for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListRateLimitersParams {
    // parameters
};
list_rate_limiters(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::RateLimiterResponse&gt;**](RateLimiterResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

