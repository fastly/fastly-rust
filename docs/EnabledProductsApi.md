# EnabledProductsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product**](EnabledProductsApi.md#disable_product) | **DELETE** /enabled-products/v1/{product_id}/services/{service_id} | Disable a product
[**enable_product**](EnabledProductsApi.md#enable_product) | **PUT** /enabled-products/v1/{product_id}/services/{service_id} | Enable a product
[**get_enabled_product**](EnabledProductsApi.md#get_enabled_product) | **GET** /enabled-products/v1/{product_id}/services/{service_id} | Get enabled product
[**get_product_configuration**](EnabledProductsApi.md#get_product_configuration) | **GET** /enabled-products/v1/{product_id}/services/{service_id}/configuration | Get configuration for a product
[**set_product_configuration**](EnabledProductsApi.md#set_product_configuration) | **PATCH** /enabled-products/v1/{product_id}/services/{service_id}/configuration | Update configuration for a product



## disable_product

Disable a product on a service. Supported product IDs: `brotli_compression`,`domain_inspector`,`fanout`,`image_optimizer`,`origin_inspector`, `websockets`, `bot_management`, `ngwaf`, `ddos_protection`, and `log_explorer_insights`.

```rust
let cfg = &Configuration::default();
let params = DisableProductParams {
    // parameters
};
disable_product(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_product

Enable a product on a service. Supported product IDs: `brotli_compression`,`domain_inspector`,`fanout`,`image_optimizer`,`origin_inspector`, `websockets`, `bot_management`, `ngwaf`, `ddos_protection`, and `log_explorer_insights`.

```rust
let cfg = &Configuration::default();
let params = EnableProductParams {
    // parameters
};
enable_product(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**set_workspace_id** | Option\<[**SetWorkspaceId**](SetWorkspaceId.md)> |  |  |

### Return type

[**crate::models::EnabledProductResponse**](EnabledProductResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_enabled_product

Get enabled product on a service. Supported product IDs: `brotli_compression`,`domain_inspector`,`fanout`,`image_optimizer`,`origin_inspector`, `websockets`, `bot_management`, `ngwaf`, `ddos_protection`, and `log_explorer_insights`.

```rust
let cfg = &Configuration::default();
let params = GetEnabledProductParams {
    // parameters
};
get_enabled_product(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::EnabledProductResponse**](EnabledProductResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_configuration

Get configuration for an enabled product on a service. Supported product IDs: `ngwaf` and `ddos_protection`.

```rust
let cfg = &Configuration::default();
let params = GetProductConfigurationParams {
    // parameters
};
get_product_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ConfiguredProductResponse**](ConfiguredProductResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_product_configuration

Update configuration for an enabled product on a service. Supported product IDs: `ngwaf` and `ddos_protection`.

```rust
let cfg = &Configuration::default();
let params = SetProductConfigurationParams {
    // parameters
};
set_product_configuration(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**set_configuration** | Option\<[**SetConfiguration**](SetConfiguration.md)> |  |  |

### Return type

[**crate::models::ConfiguredProductResponse**](ConfiguredProductResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

