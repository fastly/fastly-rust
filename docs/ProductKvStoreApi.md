# ProductKvStoreApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_kv_store**](ProductKvStoreApi.md#disable_product_kv_store) | **DELETE** /enabled-products/v1/kv_store | Disable product
[**enable_kv_store**](ProductKvStoreApi.md#enable_kv_store) | **PUT** /enabled-products/v1/kv_store | Enable product
[**get_kv_store**](ProductKvStoreApi.md#get_kv_store) | **GET** /enabled-products/v1/kv_store | Get product enablement status



## disable_product_kv_store

Disable the KV Store product

```rust
let cfg = &Configuration::default();
let params = DisableProductKvStoreParams {
    // parameters
};
disable_product_kv_store(cfg, params)
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


## enable_kv_store

Enable the KV Store product

```rust
let cfg = &Configuration::default();
let params = EnableKvStoreParams {
    // parameters
};
enable_kv_store(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::KvStoreResponseBodyEnable**](KvStoreResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_kv_store

Get the enablement status of the KV Store product

```rust
let cfg = &Configuration::default();
let params = GetKvStoreParams {
    // parameters
};
get_kv_store(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::KvStoreResponseBodyEnable**](KvStoreResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

