# RateLimiterApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rate_limiter**](RateLimiterApi.md#create_rate_limiter) | **POST** /service/{service_id}/version/{version_id}/rate-limiters | Create a rate limiter
[**delete_rate_limiter**](RateLimiterApi.md#delete_rate_limiter) | **DELETE** /rate-limiters/{rate_limiter_id} | Delete a rate limiter
[**get_rate_limiter**](RateLimiterApi.md#get_rate_limiter) | **GET** /rate-limiters/{rate_limiter_id} | Get a rate limiter
[**list_rate_limiters**](RateLimiterApi.md#list_rate_limiters) | **GET** /service/{service_id}/version/{version_id}/rate-limiters | List rate limiters
[**update_rate_limiter**](RateLimiterApi.md#update_rate_limiter) | **PUT** /rate-limiters/{rate_limiter_id} | Update a rate limiter



## create_rate_limiter

Create a rate limiter for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateRateLimiterParams {
    // parameters
};
create_rate_limiter(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | A human readable name for the rate limiting rule. |  |
**uri_dictionary_name** | Option\<**String**> | The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited. |  |
**http_methods** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of HTTP methods to apply rate limiting to. |  |
**rps_limit** | Option\<**i32**> | Upper limit of requests per second allowed by the rate limiter. |  |
**window_size** | Option\<**i32**> | Number of seconds during which the RPS limit must be exceeded in order to trigger a violation. |  |
**client_key** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`. |  |
**penalty_box_duration** | Option\<**i32**> | Length of time in minutes that the rate limiter is in effect after the initial violation is detected. |  |
**action** | Option\<**String**> | The action to take when a rate limiter violation is detected. |  |
**response_object_name** | Option\<**String**> | Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration. |  |
**logger_type** | Option\<**String**> | Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries. |  |
**feature_revision** | Option\<**i32**> | Revision number of the rate limiting feature implementation. Defaults to the most recent revision. |  |

### Return type

[**crate::models::RateLimiterResponse**](RateLimiterResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


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


## update_rate_limiter

Update a rate limiter by its ID.

```rust
let cfg = &Configuration::default();
let params = UpdateRateLimiterParams {
    // parameters
};
update_rate_limiter(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rate_limiter_id** | **String** | Alphanumeric string identifying the rate limiter. | [required] |
**name** | Option\<**String**> | A human readable name for the rate limiting rule. |  |
**uri_dictionary_name** | Option\<**String**> | The name of an Edge Dictionary containing URIs as keys. If not defined or `null`, all origin URIs will be rate limited. |  |
**http_methods** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of HTTP methods to apply rate limiting to. |  |
**rps_limit** | Option\<**i32**> | Upper limit of requests per second allowed by the rate limiter. |  |
**window_size** | Option\<**i32**> | Number of seconds during which the RPS limit must be exceeded in order to trigger a violation. |  |
**client_key** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of VCL variables used to generate a counter key to identify a client. Example variables include `req.http.Fastly-Client-IP`, `req.http.User-Agent`, or a custom header like `req.http.API-Key`. |  |
**penalty_box_duration** | Option\<**i32**> | Length of time in minutes that the rate limiter is in effect after the initial violation is detected. |  |
**action** | Option\<**String**> | The action to take when a rate limiter violation is detected. |  |
**response_object_name** | Option\<**String**> | Name of existing response object. Required if `action` is `response_object`. Note that the rate limiter response is only updated to reflect the response object content when saving the rate limiter configuration. |  |
**logger_type** | Option\<**String**> | Name of the type of logging endpoint to be used when action is `log_only`. The logging endpoint type is used to determine the appropriate log format to use when emitting log entries. |  |
**feature_revision** | Option\<**i32**> | Revision number of the rate limiting feature implementation. Defaults to the most recent revision. |  |

### Return type

[**crate::models::RateLimiterResponse**](RateLimiterResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

