# KvStoreApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**kv_store_create**](KvStoreApi.md#kv_store_create) | **POST** /resources/stores/kv | Create a KV store.
[**kv_store_delete**](KvStoreApi.md#kv_store_delete) | **DELETE** /resources/stores/kv/{store_id} | Delete a KV store.
[**kv_store_get**](KvStoreApi.md#kv_store_get) | **GET** /resources/stores/kv/{store_id} | Describe a KV store.
[**kv_store_list**](KvStoreApi.md#kv_store_list) | **GET** /resources/stores/kv | List all KV stores.



## kv_store_create

Create a KV store.

```rust
let cfg = &Configuration::default();
let params = KvStoreCreateParams {
    // parameters
};
kv_store_create(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location** | Option\<**String**> |  |  |
**kv_store_request_create** | Option\<[**KvStoreRequestCreate**](KvStoreRequestCreate.md)> |  |  |

### Return type

[**crate::models::KvStoreDetails**](KvStoreDetails.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_delete

A KV store must be empty before it can be deleted. Attempting to delete a KV store that contains items will result in a response with a `409` status code.

```rust
let cfg = &Configuration::default();
let params = KvStoreDeleteParams {
    // parameters
};
kv_store_delete(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_get

Get details of a KV store.

```rust
let cfg = &Configuration::default();
let params = KvStoreGetParams {
    // parameters
};
kv_store_get(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

[**crate::models::KvStoreDetails**](KvStoreDetails.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_list

List all KV stores.

```rust
let cfg = &Configuration::default();
let params = KvStoreListParams {
    // parameters
};
kv_store_list(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 1000]

### Return type

[**crate::models::InlineResponse2003**](InlineResponse2003.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

