# ProductDdosProtectionApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_ddos_protection**](ProductDdosProtectionApi.md#disable_product_ddos_protection) | **DELETE** /enabled-products/v1/ddos_protection/services/{service_id} | Disable product
[**enable_product_ddos_protection**](ProductDdosProtectionApi.md#enable_product_ddos_protection) | **PUT** /enabled-products/v1/ddos_protection/services/{service_id} | Enable product
[**get_product_ddos_protection**](ProductDdosProtectionApi.md#get_product_ddos_protection) | **GET** /enabled-products/v1/ddos_protection/services/{service_id} | Get product enablement status
[**get_product_ddos_protection_configuration**](ProductDdosProtectionApi.md#get_product_ddos_protection_configuration) | **GET** /enabled-products/v1/ddos_protection/services/{service_id}/configuration | Get configuration
[**get_services_product_ddos_protection**](ProductDdosProtectionApi.md#get_services_product_ddos_protection) | **GET** /enabled-products/v1/ddos_protection/services | Get services with product enabled
[**set_product_ddos_protection_configuration**](ProductDdosProtectionApi.md#set_product_ddos_protection_configuration) | **PATCH** /enabled-products/v1/ddos_protection/services/{service_id}/configuration | Update configuration



## disable_product_ddos_protection

Disable the DDoS Protection product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductDdosProtectionParams {
    // parameters
};
disable_product_ddos_protection(cfg, params)
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


## enable_product_ddos_protection

Enable the DDoS Protection product on a service in 'log' mode.

```rust
let cfg = &Configuration::default();
let params = EnableProductDdosProtectionParams {
    // parameters
};
enable_product_ddos_protection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::DdosProtectionResponseEnable**](DdosProtectionResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_ddos_protection

Get the enablement status of the DDoS Protection product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductDdosProtectionParams {
    // parameters
};
get_product_ddos_protection(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::DdosProtectionResponseEnable**](DdosProtectionResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_ddos_protection_configuration

Get configuration of the DDoS Protection product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductDdosProtectionConfigurationParams {
    // parameters
};
get_product_ddos_protection_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::DdosProtectionResponseConfigure**](DdosProtectionResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_ddos_protection

Get all the services which have the DDoS Protection product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductDdosProtectionParams {
    // parameters
};
get_services_product_ddos_protection(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DdosProtectionResponseBodyGetAllServices**](DdosProtectionResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_product_ddos_protection_configuration

Update configuration of the DDoS Protection product on a service.

```rust
let cfg = &Configuration::default();
let params = SetProductDdosProtectionConfigurationParams {
    // parameters
};
set_product_ddos_protection_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**ddos_protection_request_update_configuration** | Option\<[**DdosProtectionRequestUpdateConfiguration**](DdosProtectionRequestUpdateConfiguration.md)> |  |  |

### Return type

[**crate::models::DdosProtectionResponseConfigure**](DdosProtectionResponseConfigure.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

