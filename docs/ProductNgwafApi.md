# ProductNgwafApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_ngwaf**](ProductNgwafApi.md#disable_product_ngwaf) | **DELETE** /enabled-products/v1/ngwaf/services/{service_id} | Disable product
[**enable_product_ngwaf**](ProductNgwafApi.md#enable_product_ngwaf) | **PUT** /enabled-products/v1/ngwaf/services/{service_id} | Enable product
[**get_product_ngwaf**](ProductNgwafApi.md#get_product_ngwaf) | **GET** /enabled-products/v1/ngwaf/services/{service_id} | Get product enablement status
[**get_product_ngwaf_configuration**](ProductNgwafApi.md#get_product_ngwaf_configuration) | **GET** /enabled-products/v1/ngwaf/services/{service_id}/configuration | Get configuration
[**set_product_ngwaf_configuration**](ProductNgwafApi.md#set_product_ngwaf_configuration) | **PATCH** /enabled-products/v1/ngwaf/services/{service_id}/configuration | Update configuration



## disable_product_ngwaf

Disable the Next-Gen WAF product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductNgwafParams {
    // parameters
};
disable_product_ngwaf(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_product_ngwaf

Enable the Next-Gen WAF product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductNgwafParams {
    // parameters
};
enable_product_ngwaf(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**ngwaf_request_enable** | Option\<[**NgwafRequestEnable**](NgwafRequestEnable.md)> |  |  |

### Return type

[**crate::models::NgwafResponseEnable**](NgwafResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_ngwaf

Get the enablement status of the Next-Gen WAF product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductNgwafParams {
    // parameters
};
get_product_ngwaf(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::NgwafResponseEnable**](NgwafResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_ngwaf_configuration

Get configuration of the Next-Gen WAF product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductNgwafConfigurationParams {
    // parameters
};
get_product_ngwaf_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::NgwafResponseConfigure**](NgwafResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_product_ngwaf_configuration

Update configuration of the Next-Gen WAF product on a service.

```rust
let cfg = &Configuration::default();
let params = SetProductNgwafConfigurationParams {
    // parameters
};
set_product_ngwaf_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**ngwaf_request_update_configuration** | Option\<[**NgwafRequestUpdateConfiguration**](NgwafRequestUpdateConfiguration.md)> |  |  |

### Return type

[**crate::models::NgwafResponseConfigure**](NgwafResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

