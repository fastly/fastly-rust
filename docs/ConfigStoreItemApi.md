# ConfigStoreItemApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_config_store_item**](ConfigStoreItemApi.md#bulk_update_config_store_item) | **PATCH** /resources/stores/config/{config_store_id}/items | Update multiple entries in a config store
[**create_config_store_item**](ConfigStoreItemApi.md#create_config_store_item) | **POST** /resources/stores/config/{config_store_id}/item | Create an entry in a config store
[**delete_config_store_item**](ConfigStoreItemApi.md#delete_config_store_item) | **DELETE** /resources/stores/config/{config_store_id}/item/{config_store_item_key} | Delete an item from a config store
[**get_config_store_item**](ConfigStoreItemApi.md#get_config_store_item) | **GET** /resources/stores/config/{config_store_id}/item/{config_store_item_key} | Get an item from a config store
[**list_config_store_items**](ConfigStoreItemApi.md#list_config_store_items) | **GET** /resources/stores/config/{config_store_id}/items | List items in a config store
[**update_config_store_item**](ConfigStoreItemApi.md#update_config_store_item) | **PATCH** /resources/stores/config/{config_store_id}/item/{config_store_item_key} | Update an entry in a config store
[**upsert_config_store_item**](ConfigStoreItemApi.md#upsert_config_store_item) | **PUT** /resources/stores/config/{config_store_id}/item/{config_store_item_key} | Insert or update an entry in a config store



## bulk_update_config_store_item

Add multiple key-value pairs to an individual config store, specified by ID.

```rust
let cfg = &Configuration::default();
let params = BulkUpdateConfigStoreItemParams {
    // parameters
};
bulk_update_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**bulk_update_config_store_list_request** | Option\<[**BulkUpdateConfigStoreListRequest**](BulkUpdateConfigStoreListRequest.md)> |  |  |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_config_store_item

Add a single key-value pair to an individual config store, specified by ID.

```rust
let cfg = &Configuration::default();
let params = CreateConfigStoreItemParams {
    // parameters
};
create_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::ConfigStoreItemResponse**](ConfigStoreItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_config_store_item

Delete an entry in a config store given a config store ID, and item key.

```rust
let cfg = &Configuration::default();
let params = DeleteConfigStoreItemParams {
    // parameters
};
delete_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**config_store_item_key** | **String** | Item key, maximum 256 characters. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_config_store_item

Retrieve a config store entry given a config store ID and item key.

```rust
let cfg = &Configuration::default();
let params = GetConfigStoreItemParams {
    // parameters
};
get_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**config_store_item_key** | **String** | Item key, maximum 256 characters. | [required] |

### Return type

[**crate::models::ConfigStoreItemResponse**](ConfigStoreItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_config_store_items

List the key-value pairs associated with a given config store ID.

```rust
let cfg = &Configuration::default();
let params = ListConfigStoreItemsParams {
    // parameters
};
list_config_store_items(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |

### Return type

[**Vec&lt;crate::models::ConfigStoreItemResponse&gt;**](ConfigStoreItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_config_store_item

Update an entry in a config store given a config store ID, item key, and item value.

```rust
let cfg = &Configuration::default();
let params = UpdateConfigStoreItemParams {
    // parameters
};
update_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**config_store_item_key** | **String** | Item key, maximum 256 characters. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::ConfigStoreItemResponse**](ConfigStoreItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## upsert_config_store_item

Insert or update an entry in a config store given a config store ID, item key, and item value.

```rust
let cfg = &Configuration::default();
let params = UpsertConfigStoreItemParams {
    // parameters
};
upsert_config_store_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**config_store_item_key** | **String** | Item key, maximum 256 characters. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::ConfigStoreItemResponse**](ConfigStoreItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

