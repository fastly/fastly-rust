# ProductApiDiscoveryApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_api_discovery**](ProductApiDiscoveryApi.md#disable_product_api_discovery) | **DELETE** /enabled-products/v1/api_discovery/services/{service_id} | Disable product
[**enable_product_api_discovery**](ProductApiDiscoveryApi.md#enable_product_api_discovery) | **PUT** /enabled-products/v1/api_discovery/services/{service_id} | Enable product
[**get_product_api_discovery**](ProductApiDiscoveryApi.md#get_product_api_discovery) | **GET** /enabled-products/v1/api_discovery/services/{service_id} | Get product enablement status
[**get_services_product_api_discovery**](ProductApiDiscoveryApi.md#get_services_product_api_discovery) | **GET** /enabled-products/v1/api_discovery/services | Get services with product enabled



## disable_product_api_discovery

Disable the API Discovery product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductApiDiscoveryParams {
    // parameters
};
disable_product_api_discovery(cfg, params)
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


## enable_product_api_discovery

Enable the API Discovery product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductApiDiscoveryParams {
    // parameters
};
enable_product_api_discovery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ApiDiscoveryResponseEnable**](ApiDiscoveryResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_api_discovery

Get the enablement status of the API Discovery product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductApiDiscoveryParams {
    // parameters
};
get_product_api_discovery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ApiDiscoveryResponseEnable**](ApiDiscoveryResponseEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_api_discovery

Get all the services for a customer that has the API Discovery product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductApiDiscoveryParams {
    // parameters
};
get_services_product_api_discovery(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiDiscoveryResponseBodyGetAllServices**](ApiDiscoveryResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

