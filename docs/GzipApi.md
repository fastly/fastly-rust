# GzipApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gzip_config**](GzipApi.md#create_gzip_config) | **POST** /service/{service_id}/version/{version_id}/gzip | Create a gzip configuration
[**delete_gzip_config**](GzipApi.md#delete_gzip_config) | **DELETE** /service/{service_id}/version/{version_id}/gzip/{gzip_name} | Delete a gzip configuration
[**get_gzip_configs**](GzipApi.md#get_gzip_configs) | **GET** /service/{service_id}/version/{version_id}/gzip/{gzip_name} | Get a gzip configuration
[**list_gzip_configs**](GzipApi.md#list_gzip_configs) | **GET** /service/{service_id}/version/{version_id}/gzip | List gzip configurations
[**update_gzip_config**](GzipApi.md#update_gzip_config) | **PUT** /service/{service_id}/version/{version_id}/gzip/{gzip_name} | Update a gzip configuration



## create_gzip_config

Create a named gzip configuration on a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateGzipConfigParams {
    // parameters
};
create_gzip_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**content_types** | Option\<**String**> | Space-separated list of content types to compress. If you omit this field a default list will be used. |  |
**extensions** | Option\<**String**> | Space-separated list of file extensions to compress. If you omit this field a default list will be used. |  |
**name** | Option\<**String**> | Name of the gzip configuration. |  |

### Return type

[**crate::models::GzipResponse**](GzipResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_gzip_config

Delete a named gzip configuration on a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteGzipConfigParams {
    // parameters
};
delete_gzip_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**gzip_name** | **String** | Name of the gzip configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_gzip_configs

Get the gzip configuration for a particular service, version, and name.

```rust
let cfg = &Configuration::default();
let params = GetGzipConfigsParams {
    // parameters
};
get_gzip_configs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**gzip_name** | **String** | Name of the gzip configuration. | [required] |

### Return type

[**crate::models::GzipResponse**](GzipResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_gzip_configs

List all gzip configurations for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListGzipConfigsParams {
    // parameters
};
list_gzip_configs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::GzipResponse&gt;**](GzipResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_gzip_config

Update a named gzip configuration on a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateGzipConfigParams {
    // parameters
};
update_gzip_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**gzip_name** | **String** | Name of the gzip configuration. | [required] |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**content_types** | Option\<**String**> | Space-separated list of content types to compress. If you omit this field a default list will be used. |  |
**extensions** | Option\<**String**> | Space-separated list of file extensions to compress. If you omit this field a default list will be used. |  |
**name** | Option\<**String**> | Name of the gzip configuration. |  |

### Return type

[**crate::models::GzipResponse**](GzipResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

