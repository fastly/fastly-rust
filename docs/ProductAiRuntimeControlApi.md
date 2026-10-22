# ProductAiRuntimeControlApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_ai_runtime_control**](ProductAiRuntimeControlApi.md#disable_product_ai_runtime_control) | **DELETE** /enabled-products/v1/ai_runtime_control | Disable product
[**enable_product_ai_runtime_control**](ProductAiRuntimeControlApi.md#enable_product_ai_runtime_control) | **PUT** /enabled-products/v1/ai_runtime_control | Enable product
[**get_product_ai_runtime_control**](ProductAiRuntimeControlApi.md#get_product_ai_runtime_control) | **GET** /enabled-products/v1/ai_runtime_control | Get product enablement status



## disable_product_ai_runtime_control

Disable the AI Runtime Control product

```rust
let cfg = &Configuration::default();
let params = DisableProductAiRuntimeControlParams {
    // parameters
};
disable_product_ai_runtime_control(cfg, params)
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


## enable_product_ai_runtime_control

Enable the AI Runtime Control product

```rust
let cfg = &Configuration::default();
let params = EnableProductAiRuntimeControlParams {
    // parameters
};
enable_product_ai_runtime_control(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AiRuntimeControlResponseBodyEnable**](AiRuntimeControlResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_product_ai_runtime_control

Get the enablement status of the AI Runtime Control product

```rust
let cfg = &Configuration::default();
let params = GetProductAiRuntimeControlParams {
    // parameters
};
get_product_ai_runtime_control(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AiRuntimeControlResponseBodyEnable**](AiRuntimeControlResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

