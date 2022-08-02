# TlsConfigurationsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tls_config**](TlsConfigurationsApi.md#get_tls_config) | **GET** /tls/configurations/{tls_configuration_id} | Get a TLS configuration
[**list_tls_configs**](TlsConfigurationsApi.md#list_tls_configs) | **GET** /tls/configurations | List TLS configurations
[**update_tls_config**](TlsConfigurationsApi.md#update_tls_config) | **PATCH** /tls/configurations/{tls_configuration_id} | Update a TLS configuration



## get_tls_config

Show a TLS configuration.

```rust
let cfg = &Configuration::default();
let params = GetTlsConfigParams {
    // parameters
};
get_tls_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_configuration_id** | **String** | Alphanumeric string identifying a TLS configuration. | [required] |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `dns_records`.  |  |

### Return type

[**crate::models::TlsConfigurationResponse**](TlsConfigurationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_configs

List all TLS configurations.

```rust
let cfg = &Configuration::default();
let params = ListTlsConfigsParams {
    // parameters
};
list_tls_configs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_bulk** | Option\<**String**> | Optionally filters by the bulk attribute. |  |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `dns_records`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::TlsConfigurationsResponse**](TlsConfigurationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_tls_config

Update a TLS configuration.

```rust
let cfg = &Configuration::default();
let params = UpdateTlsConfigParams {
    // parameters
};
update_tls_config(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_configuration_id** | **String** | Alphanumeric string identifying a TLS configuration. | [required] |
**tls_configuration** | Option\<[**TlsConfiguration**](TlsConfiguration.md)> |  |  |

### Return type

[**crate::models::TlsConfigurationResponse**](TlsConfigurationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

