# KvStoreItemApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**kv_store_delete_item**](KvStoreItemApi.md#kv_store_delete_item) | **DELETE** /resources/stores/kv/{store_id}/keys/{key} | Delete an item.
[**kv_store_get_item**](KvStoreItemApi.md#kv_store_get_item) | **GET** /resources/stores/kv/{store_id}/keys/{key} | Get an item.
[**kv_store_list_item_keys**](KvStoreItemApi.md#kv_store_list_item_keys) | **GET** /resources/stores/kv/{store_id}/keys | List item keys.
[**kv_store_upsert_item**](KvStoreItemApi.md#kv_store_upsert_item) | **PUT** /resources/stores/kv/{store_id}/keys/{key} | Insert or update an item.



## kv_store_delete_item

Delete an item.

```rust
let cfg = &Configuration::default();
let params = KvStoreDeleteItemParams {
    // parameters
};
kv_store_delete_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key** | **String** |  | [required] |
**if_generation_match** | Option\<**i32**> |  |  |
**force** | Option\<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_get_item

Get an item, including its value, metadata (if any), and generation marker.

```rust
let cfg = &Configuration::default();
let params = KvStoreGetItemParams {
    // parameters
};
kv_store_get_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**String**

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_list_item_keys

Lists the matching item keys (or all item keys, if no prefix is supplied).

```rust
let cfg = &Configuration::default();
let params = KvStoreListItemKeysParams {
    // parameters
};
kv_store_list_item_keys(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 100]
**prefix** | Option\<**String**> |  |  |
**consistency** | Option\<**String**> |  |  |[default to strong]

### Return type

[**crate::models::InlineResponse2006**](InlineResponse2006.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## kv_store_upsert_item

Inserts or updates an item's value and metadata.

```rust
let cfg = &Configuration::default();
let params = KvStoreUpsertItemParams {
    // parameters
};
kv_store_upsert_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**key** | **String** |  | [required] |
**if_generation_match** | Option\<**i32**> |  |  |
**time_to_live_sec** | Option\<**i32**> |  |  |
**metadata** | Option\<**String**> |  |  |
**add** | Option\<**bool**> |  |  |[default to false]
**append** | Option\<**bool**> |  |  |[default to false]
**prepend** | Option\<**bool**> |  |  |[default to false]
**background_fetch** | Option\<**bool**> |  |  |[default to false]
**body** | Option\<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

