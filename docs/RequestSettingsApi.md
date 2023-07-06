# RequestSettingsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_request_settings**](RequestSettingsApi.md#create_request_settings) | **POST** /service/{service_id}/version/{version_id}/request_settings | Create a Request Settings object
[**delete_request_settings**](RequestSettingsApi.md#delete_request_settings) | **DELETE** /service/{service_id}/version/{version_id}/request_settings/{request_settings_name} | Delete a Request Settings object
[**get_request_settings**](RequestSettingsApi.md#get_request_settings) | **GET** /service/{service_id}/version/{version_id}/request_settings/{request_settings_name} | Get a Request Settings object
[**list_request_settings**](RequestSettingsApi.md#list_request_settings) | **GET** /service/{service_id}/version/{version_id}/request_settings | List Request Settings objects
[**update_request_settings**](RequestSettingsApi.md#update_request_settings) | **PUT** /service/{service_id}/version/{version_id}/request_settings/{request_settings_name} | Update a Request Settings object



## create_request_settings

Creates a new Request Settings object.

```rust
let cfg = &Configuration::default();
let params = CreateRequestSettingsParams {
    // parameters
};
create_request_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::RequestSettingsResponse**](RequestSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_request_settings

Removes the specified Request Settings object.

```rust
let cfg = &Configuration::default();
let params = DeleteRequestSettingsParams {
    // parameters
};
delete_request_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**request_settings_name** | **String** | Name for the request settings. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_request_settings

Gets the specified Request Settings object.

```rust
let cfg = &Configuration::default();
let params = GetRequestSettingsParams {
    // parameters
};
get_request_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**request_settings_name** | **String** | Name for the request settings. | [required] |

### Return type

[**crate::models::RequestSettingsResponse**](RequestSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_request_settings

Returns a list of all Request Settings objects for the given service and version.

```rust
let cfg = &Configuration::default();
let params = ListRequestSettingsParams {
    // parameters
};
list_request_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::RequestSettingsResponse&gt;**](RequestSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_request_settings

Updates the specified Request Settings object.

```rust
let cfg = &Configuration::default();
let params = UpdateRequestSettingsParams {
    // parameters
};
update_request_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**request_settings_name** | **String** | Name for the request settings. | [required] |
**action** | Option\<**String**> | Allows you to terminate request handling and immediately perform an action. |  |
**bypass_busy_wait** | Option\<**i32**> | Disable collapsed forwarding, so you don't wait for other objects to origin. |  |
**default_host** | Option\<**String**> | Sets the host header. |  |
**force_miss** | Option\<**i32**> | Allows you to force a cache miss for the request. Replaces the item in the cache if the content is cacheable. |  |
**force_ssl** | Option\<**i32**> | Forces the request use SSL (redirects a non-SSL to SSL). |  |
**geo_headers** | Option\<**i32**> | Injects Fastly-Geo-Country, Fastly-Geo-City, and Fastly-Geo-Region into the request headers. |  |
**hash_keys** | Option\<**String**> | Comma separated list of varnish request object fields that should be in the hash key. |  |
**max_stale_age** | Option\<**i32**> | How old an object is allowed to be to serve stale-if-error or stale-while-revalidate. |  |
**name** | Option\<**String**> | Name for the request settings. |  |
**request_condition** | Option\<**String**> | Condition which, if met, will select this configuration during a request. Optional. |  |
**timer_support** | Option\<**i32**> | Injects the X-Timer info into the request for viewing origin fetch durations. |  |
**xff** | Option\<**String**> | Short for X-Forwarded-For. |  |

### Return type

[**crate::models::RequestSettingsResponse**](RequestSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

