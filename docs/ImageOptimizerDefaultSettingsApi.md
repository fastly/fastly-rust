# ImageOptimizerDefaultSettingsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_default_settings**](ImageOptimizerDefaultSettingsApi.md#get_default_settings) | **GET** /service/{service_id}/version/{version_id}/image_optimizer_default_settings | Get current Image Optimizer Default Settings
[**update_default_settings**](ImageOptimizerDefaultSettingsApi.md#update_default_settings) | **PATCH** /service/{service_id}/version/{version_id}/image_optimizer_default_settings | Update Image Optimizer Default Settings



## get_default_settings

Retrieve the current Image Optimizer default settings. All properties in the response will be populated. 

```rust
let cfg = &Configuration::default();
let params = GetDefaultSettingsParams {
    // parameters
};
get_default_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::DefaultSettingsResponse**](DefaultSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_default_settings

Update one or more default settings. A minimum of one property is required. The endpoint will respond with the new Image Optimizer default settings, with all properties populated. 

```rust
let cfg = &Configuration::default();
let params = UpdateDefaultSettingsParams {
    // parameters
};
update_default_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**default_settings** | Option\<[**DefaultSettings**](DefaultSettings.md)> |  |  |

### Return type

[**crate::models::DefaultSettingsResponse**](DefaultSettingsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

