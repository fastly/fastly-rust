# ProductAiAcceleratorApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_ai_accelerator**](ProductAiAcceleratorApi.md#disable_product_ai_accelerator) | **DELETE** /enabled-products/v1/ai_accelerator | Disable product
[**enable_ai_accelerator**](ProductAiAcceleratorApi.md#enable_ai_accelerator) | **PUT** /enabled-products/v1/ai_accelerator | Enable product
[**get_ai_accelerator**](ProductAiAcceleratorApi.md#get_ai_accelerator) | **GET** /enabled-products/v1/ai_accelerator | Get product enablement status



## disable_product_ai_accelerator

Disable the AI Accelerator product

```rust
let cfg = &Configuration::default();
let params = DisableProductAiAcceleratorParams {
    // parameters
};
disable_product_ai_accelerator(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## enable_ai_accelerator

Enable the AI Accelerator product

```rust
let cfg = &Configuration::default();
let params = EnableAiAcceleratorParams {
    // parameters
};
enable_ai_accelerator(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AiAcceleratorResponseBodyEnable**](AiAcceleratorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_ai_accelerator

Get the enablement status of the AI Accelerator product

```rust
let cfg = &Configuration::default();
let params = GetAiAcceleratorParams {
    // parameters
};
get_ai_accelerator(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AiAcceleratorResponseBodyEnable**](AiAcceleratorResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

