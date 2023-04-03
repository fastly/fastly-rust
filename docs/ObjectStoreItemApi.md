# ObjectStoreItemApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_key_from_store**](ObjectStoreItemApi.md#delete_key_from_store) | **DELETE** /resources/stores/object/{store_id}/keys/{key_name} | Delete object store item.
[**get_keys**](ObjectStoreItemApi.md#get_keys) | **GET** /resources/stores/object/{store_id}/keys | List object store keys.
[**get_value_for_key**](ObjectStoreItemApi.md#get_value_for_key) | **GET** /resources/stores/object/{store_id}/keys/{key_name} | Get the value of an object store item
[**set_value_for_key**](ObjectStoreItemApi.md#set_value_for_key) | **PUT** /resources/stores/object/{store_id}/keys/{key_name} | Insert an item into an object store



## delete_key_from_store

Delete an item from an object store

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
**force** | Option\<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_keys

List the keys of all items within an object store.

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
**prefix** | Option\<**String**> |  |  |

### Return type

[**crate::models::InlineResponse2004**](InlineResponse2004.md)

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

Set a new value for a new or existing key in an object store.

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
**if_generation_match** | Option\<**i32**> |  |  |
**time_to_live_sec** | Option\<**i32**> |  |  |
**metadata** | Option\<**String**> |  |  |
**add** | Option\<**bool**> |  |  |
**append** | Option\<**bool**> |  |  |
**prepend** | Option\<**bool**> |  |  |
**background_fetch** | Option\<**bool**> |  |  |
**body** | Option\<**std::path::PathBuf**> |  |  |

### Return type

[**std::path::PathBuf**](StdpathPathBuf.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

