# ObjectStoreApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_store**](ObjectStoreApi.md#create_store) | **POST** /resources/stores/object | Create an object store.
[**delete_key_from_store**](ObjectStoreApi.md#delete_key_from_store) | **DELETE** /resources/stores/object/{store_id}/keys/{key_name} | Delete object store key.
[**delete_store**](ObjectStoreApi.md#delete_store) | **DELETE** /resources/stores/object/{store_id} | Delete an object store.
[**get_keys**](ObjectStoreApi.md#get_keys) | **GET** /resources/stores/object/{store_id}/keys | List object store keys.
[**get_store**](ObjectStoreApi.md#get_store) | **GET** /resources/stores/object/{store_id} | Describe an object store.
[**get_stores**](ObjectStoreApi.md#get_stores) | **GET** /resources/stores/object | List object stores.
[**get_value_for_key**](ObjectStoreApi.md#get_value_for_key) | **GET** /resources/stores/object/{store_id}/keys/{key_name} | Get object store key value.
[**set_value_for_key**](ObjectStoreApi.md#set_value_for_key) | **PUT** /resources/stores/object/{store_id}/keys/{key_name} | Insert object store key-value.



## create_store

Create a new object store.

```rust
let cfg = &Configuration::default();
let params = CreateStoreParams {
    // parameters
};
create_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | Option\<[**Store**](Store.md)> |  |  |

### Return type

[**crate::models::StoreResponse**](StoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_key_from_store

Delete a key from a customer store.

```rust
let cfg = &Configuration::default();
let params = DeleteKeyFromStoreParams {
    // parameters
};
delete_key_from_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_store

An object store must be empty before it can be deleted.  Deleting an object store that still contains keys will result in a 409 Conflict.

```rust
let cfg = &Configuration::default();
let params = DeleteStoreParams {
    // parameters
};
delete_store(cfg, params)
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


## get_keys

List all keys within an object store.

```rust
let cfg = &Configuration::default();
let params = GetKeysParams {
    // parameters
};
get_keys(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::KeyResponse**](KeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_store

Get an object store by ID.

```rust
let cfg = &Configuration::default();
let params = GetStoreParams {
    // parameters
};
get_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

[**crate::models::StoreResponse**](StoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stores

Get all stores for a given customer.

```rust
let cfg = &Configuration::default();
let params = GetStoresParams {
    // parameters
};
get_stores(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::GetStoresResponse**](GetStoresResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_value_for_key

Get the value associated with a key.

```rust
let cfg = &Configuration::default();
let params = GetValueForKeyParams {
    // parameters
};
get_value_for_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key_name** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](StdpathPathBuf.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_value_for_key

Insert a new key-value pair into an object store.

```rust
let cfg = &Configuration::default();
let params = SetValueForKeyParams {
    // parameters
};
set_value_for_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key_name** | **String** |  | [required] |
**body** | Option\<**std::path::PathBuf**> |  |  |

### Return type

[**std::path::PathBuf**](StdpathPathBuf.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

