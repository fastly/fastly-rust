# LegacyWafConfigurationSetsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_waf_config_sets**](LegacyWafConfigurationSetsApi.md#list_waf_config_sets) | **GET** /wafs/configuration_sets | List configuration sets
[**list_wafs_config_set**](LegacyWafConfigurationSetsApi.md#list_wafs_config_set) | **GET** /wafs/configuration_sets/{configuration_set_id}/relationships/wafs | List WAFs currently using a configuration set
[**use_waf_config_set**](LegacyWafConfigurationSetsApi.md#use_waf_config_set) | **PATCH** /wafs/configuration_sets/{configuration_set_id}/relationships/wafs | Apply a configuration set to a WAF



## list_waf_config_sets

List all Configuration sets.

```rust
let cfg = &Configuration::default();
let params = ListWafConfigSetsParams {
    // parameters
};
list_waf_config_sets(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_wafs_config_set

List the WAF objects currently using the specified configuration set.

```rust
let cfg = &Configuration::default();
let params = ListWafsConfigSetParams {
    // parameters
};
list_wafs_config_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_set_id** | **String** | Alphanumeric string identifying a WAF configuration set. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## use_waf_config_set

Update one or more WAF objects to use the specified configuration set.

```rust
let cfg = &Configuration::default();
let params = UseWafConfigSetParams {
    // parameters
};
use_waf_config_set(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_set_id** | **String** | Alphanumeric string identifying a WAF configuration set. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

