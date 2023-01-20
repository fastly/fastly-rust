# SettingsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_settings**](SettingsApi.md#get_service_settings) | **GET** /service/{service_id}/version/{version_id}/settings | Get service settings
[**update_service_settings**](SettingsApi.md#update_service_settings) | **PUT** /service/{service_id}/version/{version_id}/settings | Update service settings



## get_service_settings

Get the settings for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetServiceSettingsParams {
    // parameters
};
get_service_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::SettingsResponse**](SettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_service_settings

Update the settings for a particular service and version. NOTE: If you override TTLs with custom VCL, any general.default_ttl value will not be honored and the expected behavior may change. 

```rust
let cfg = &Configuration::default();
let params = UpdateServiceSettingsParams {
    // parameters
};
update_service_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**general_default_host** | Option\<**String**> | The default host name for the version. |  |
**general_default_ttl** | Option\<**i32**> | The default time-to-live (TTL) for the version. |  |
**general_stale_if_error** | Option\<**bool**> | Enables serving a stale object if there is an error. |  |[default to false]
**general_stale_if_error_ttl** | Option\<**i32**> | The default time-to-live (TTL) for serving the stale object for the version. |  |[default to 43200]

### Return type

[**crate::models::SettingsResponse**](SettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

