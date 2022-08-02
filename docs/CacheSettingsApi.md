# CacheSettingsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cache_settings**](CacheSettingsApi.md#create_cache_settings) | **POST** /service/{service_id}/version/{version_id}/cache_settings | Create a cache settings object
[**delete_cache_settings**](CacheSettingsApi.md#delete_cache_settings) | **DELETE** /service/{service_id}/version/{version_id}/cache_settings/{cache_settings_name} | Delete a cache settings object
[**get_cache_settings**](CacheSettingsApi.md#get_cache_settings) | **GET** /service/{service_id}/version/{version_id}/cache_settings/{cache_settings_name} | Get a cache settings object
[**list_cache_settings**](CacheSettingsApi.md#list_cache_settings) | **GET** /service/{service_id}/version/{version_id}/cache_settings | List cache settings objects
[**update_cache_settings**](CacheSettingsApi.md#update_cache_settings) | **PUT** /service/{service_id}/version/{version_id}/cache_settings/{cache_settings_name} | Update a cache settings object



## create_cache_settings

Create a cache settings object.

```rust
let cfg = &Configuration::default();
let params = CreateCacheSettingsParams {
    // parameters
};
create_cache_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**action** | Option\<**String**> | If set, will cause vcl_fetch to terminate after processing this rule with the return state specified. If not set, other configuration logic in vcl_fetch with a lower priority will run after this rule.  |  |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**name** | Option\<**String**> | Name for the cache settings object. |  |
**stale_ttl** | Option\<**i32**> | Maximum time in seconds to continue to use a stale version of the object if future requests to your backend server fail (also known as 'stale if error'). |  |
**ttl** | Option\<**i32**> | Maximum time to consider the object fresh in the cache (the cache 'time to live'). |  |

### Return type

[**crate::models::CacheSettingResponse**](CacheSettingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_cache_settings

Delete a specific cache settings object.

```rust
let cfg = &Configuration::default();
let params = DeleteCacheSettingsParams {
    // parameters
};
delete_cache_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**cache_settings_name** | **String** | Name for the cache settings object. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_cache_settings

Get a specific cache settings object.

```rust
let cfg = &Configuration::default();
let params = GetCacheSettingsParams {
    // parameters
};
get_cache_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**cache_settings_name** | **String** | Name for the cache settings object. | [required] |

### Return type

[**crate::models::CacheSettingResponse**](CacheSettingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_cache_settings

Get a list of all cache settings for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListCacheSettingsParams {
    // parameters
};
list_cache_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::CacheSettingResponse&gt;**](CacheSettingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_cache_settings

Update a specific cache settings object.

```rust
let cfg = &Configuration::default();
let params = UpdateCacheSettingsParams {
    // parameters
};
update_cache_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**cache_settings_name** | **String** | Name for the cache settings object. | [required] |
**action** | Option\<**String**> | If set, will cause vcl_fetch to terminate after processing this rule with the return state specified. If not set, other configuration logic in vcl_fetch with a lower priority will run after this rule.  |  |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**name** | Option\<**String**> | Name for the cache settings object. |  |
**stale_ttl** | Option\<**i32**> | Maximum time in seconds to continue to use a stale version of the object if future requests to your backend server fail (also known as 'stale if error'). |  |
**ttl** | Option\<**i32**> | Maximum time to consider the object fresh in the cache (the cache 'time to live'). |  |

### Return type

[**crate::models::CacheSettingResponse**](CacheSettingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

