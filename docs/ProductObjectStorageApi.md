# ProductObjectStorageApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**disable_product_object_storage**](ProductObjectStorageApi.md#disable_product_object_storage) | **DELETE** /enabled-products/v1/object_storage | Disable product
[**enable_object_storage**](ProductObjectStorageApi.md#enable_object_storage) | **PUT** /enabled-products/v1/object_storage | Enable product
[**get_object_storage**](ProductObjectStorageApi.md#get_object_storage) | **GET** /enabled-products/v1/object_storage | Get product enablement status



## disable_product_object_storage

Disable the Object Storage product

```rust
let cfg = &Configuration::default();
let params = DisableProductObjectStorageParams {
    // parameters
};
disable_product_object_storage(cfg, params)
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


## enable_object_storage

Enable the Object Storage product

```rust
let cfg = &Configuration::default();
let params = EnableObjectStorageParams {
    // parameters
};
enable_object_storage(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ObjectStorageResponseBodyEnable**](ObjectStorageResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_object_storage

Get the enablement status of the Object Storage product

```rust
let cfg = &Configuration::default();
let params = GetObjectStorageParams {
    // parameters
};
get_object_storage(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ObjectStorageResponseBodyEnable**](ObjectStorageResponseBodyEnable.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

