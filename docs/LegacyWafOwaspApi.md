# LegacyWafOwaspApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_owasp_settings**](LegacyWafOwaspApi.md#create_owasp_settings) | **POST** /service/{service_id}/wafs/{firewall_id}/owasp | Create an OWASP settings object
[**get_owasp_settings**](LegacyWafOwaspApi.md#get_owasp_settings) | **GET** /service/{service_id}/wafs/{firewall_id}/owasp | Get the OWASP settings object
[**update_owasp_settings**](LegacyWafOwaspApi.md#update_owasp_settings) | **PATCH** /service/{service_id}/wafs/{firewall_id}/owasp | Update the OWASP settings object



## create_owasp_settings

Create an OWASP settings object for a particular service and firewall.

```rust
let cfg = &Configuration::default();
let params = CreateOwaspSettingsParams {
    // parameters
};
create_owasp_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_owasp_settings

Get the OWASP settings object for a particular service and firewall.

```rust
let cfg = &Configuration::default();
let params = GetOwaspSettingsParams {
    // parameters
};
get_owasp_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_owasp_settings

Update the OWASP settings object for a particular service and firewall.

```rust
let cfg = &Configuration::default();
let params = UpdateOwaspSettingsParams {
    // parameters
};
update_owasp_settings(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**firewall_id** | **String** | Alphanumeric string identifying a Firewall. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

