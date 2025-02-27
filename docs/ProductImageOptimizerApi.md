# ProductImageOptimizerApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_image_optimizer**](ProductImageOptimizerApi.md#disable_product_image_optimizer) | **DELETE** /enabled-products/v1/image_optimizer/services/{service_id} | Disable product
[**enable_product_image_optimizer**](ProductImageOptimizerApi.md#enable_product_image_optimizer) | **PUT** /enabled-products/v1/image_optimizer/services/{service_id} | Enable product
[**get_product_image_optimizer**](ProductImageOptimizerApi.md#get_product_image_optimizer) | **GET** /enabled-products/v1/image_optimizer/services/{service_id} | Get product enablement status



## disable_product_image_optimizer

Disable the Image Optimizer product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductImageOptimizerParams {
    // parameters
};
disable_product_image_optimizer(cfg, params)
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


## enable_product_image_optimizer

Enable the Image Optimizer product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductImageOptimizerParams {
    // parameters
};
enable_product_image_optimizer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ImageOptimizerResponseBodyEnable**](ImageOptimizerResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_image_optimizer

Get the enablement status of the Image Optimizer product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductImageOptimizerParams {
    // parameters
};
get_product_image_optimizer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ImageOptimizerResponseBodyEnable**](ImageOptimizerResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

