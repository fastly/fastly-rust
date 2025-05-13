# ProductBrotliCompressionApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_brotli_compression**](ProductBrotliCompressionApi.md#disable_product_brotli_compression) | **DELETE** /enabled-products/v1/brotli_compression/services/{service_id} | Disable product
[**enable_product_brotli_compression**](ProductBrotliCompressionApi.md#enable_product_brotli_compression) | **PUT** /enabled-products/v1/brotli_compression/services/{service_id} | Enable product
[**get_product_brotli_compression**](ProductBrotliCompressionApi.md#get_product_brotli_compression) | **GET** /enabled-products/v1/brotli_compression/services/{service_id} | Get product enablement status
[**get_services_product_brotli_compression**](ProductBrotliCompressionApi.md#get_services_product_brotli_compression) | **GET** /enabled-products/v1/brotli_compression/services | Get services with product enabled



## disable_product_brotli_compression

Disable the Brotli Compression product on a service.

```rust
let cfg = &Configuration::default();
let params = DisableProductBrotliCompressionParams {
    // parameters
};
disable_product_brotli_compression(cfg, params)
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


## enable_product_brotli_compression

Enable the Brotli Compression product on a service.

```rust
let cfg = &Configuration::default();
let params = EnableProductBrotliCompressionParams {
    // parameters
};
enable_product_brotli_compression(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::BrotliCompressionResponseBodyEnable**](BrotliCompressionResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_brotli_compression

Get the enablement status of the Brotli Compression product on a service.

```rust
let cfg = &Configuration::default();
let params = GetProductBrotliCompressionParams {
    // parameters
};
get_product_brotli_compression(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::BrotliCompressionResponseBodyEnable**](BrotliCompressionResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_services_product_brotli_compression

Get all the services which have the Brotli Compression product enabled.

```rust
let cfg = &Configuration::default();
let params = GetServicesProductBrotliCompressionParams {
    // parameters
};
get_services_product_brotli_compression(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BrotliCompressionResponseBodyGetAllServices**](BrotliCompressionResponseBodyGetAllServices.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

