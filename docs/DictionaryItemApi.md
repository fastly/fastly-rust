# DictionaryItemApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_dictionary_item**](DictionaryItemApi.md#bulk_update_dictionary_item) | **PATCH** /service/{service_id}/dictionary/{dictionary_id}/items | Update multiple entries in an edge dictionary
[**create_dictionary_item**](DictionaryItemApi.md#create_dictionary_item) | **POST** /service/{service_id}/dictionary/{dictionary_id}/item | Create an entry in an edge dictionary
[**delete_dictionary_item**](DictionaryItemApi.md#delete_dictionary_item) | **DELETE** /service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key} | Delete an item from an edge dictionary
[**get_dictionary_item**](DictionaryItemApi.md#get_dictionary_item) | **GET** /service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key} | Get an item from an edge dictionary
[**list_dictionary_items**](DictionaryItemApi.md#list_dictionary_items) | **GET** /service/{service_id}/dictionary/{dictionary_id}/items | List items in an edge dictionary
[**update_dictionary_item**](DictionaryItemApi.md#update_dictionary_item) | **PATCH** /service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key} | Update an entry in an edge dictionary
[**upsert_dictionary_item**](DictionaryItemApi.md#upsert_dictionary_item) | **PUT** /service/{service_id}/dictionary/{dictionary_id}/item/{dictionary_item_key} | Insert or update an entry in an edge dictionary



## bulk_update_dictionary_item

Update multiple items in the same dictionary. For faster updates to your service, group your changes into large batches. The maximum batch size is 1000 items. [Contact support](https://support.fastly.com/) to discuss raising this limit.

```rust
let cfg = &Configuration::default();
let params = BulkUpdateDictionaryItemParams {
    // parameters
};
bulk_update_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**bulk_update_dictionary_list_request** | Option\<[**BulkUpdateDictionaryListRequest**](BulkUpdateDictionaryListRequest.md)> |  |  |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_dictionary_item

Create DictionaryItem given service, dictionary ID, item key, and item value.

```rust
let cfg = &Configuration::default();
let params = CreateDictionaryItemParams {
    // parameters
};
create_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::DictionaryItemResponse**](DictionaryItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_dictionary_item

Delete DictionaryItem given service, dictionary ID, and item key.

```rust
let cfg = &Configuration::default();
let params = DeleteDictionaryItemParams {
    // parameters
};
delete_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**dictionary_item_key** | **String** | Item key, maximum 256 characters. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_dictionary_item

Retrieve a single DictionaryItem given service, dictionary ID and item key.

```rust
let cfg = &Configuration::default();
let params = GetDictionaryItemParams {
    // parameters
};
get_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**dictionary_item_key** | **String** | Item key, maximum 256 characters. | [required] |

### Return type

[**crate::models::DictionaryItemResponse**](DictionaryItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_dictionary_items

List of DictionaryItems given service and dictionary ID.

```rust
let cfg = &Configuration::default();
let params = ListDictionaryItemsParams {
    // parameters
};
list_dictionary_items(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**page** | Option\<**i32**> | Current page. |  |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | Field on which to sort. |  |[default to created]
**direction** | Option\<**String**> | Direction in which to sort results. |  |[default to ascend]

### Return type

[**Vec&lt;crate::models::DictionaryItemResponse&gt;**](DictionaryItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_dictionary_item

Update DictionaryItem given service, dictionary ID, item key, and item value.

```rust
let cfg = &Configuration::default();
let params = UpdateDictionaryItemParams {
    // parameters
};
update_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**dictionary_item_key** | **String** | Item key, maximum 256 characters. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::DictionaryItemResponse**](DictionaryItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## upsert_dictionary_item

Upsert DictionaryItem given service, dictionary ID, item key, and item value.

```rust
let cfg = &Configuration::default();
let params = UpsertDictionaryItemParams {
    // parameters
};
upsert_dictionary_item(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |
**dictionary_item_key** | **String** | Item key, maximum 256 characters. | [required] |
**item_key** | Option\<**String**> | Item key, maximum 256 characters. |  |
**item_value** | Option\<**String**> | Item value, maximum 8000 characters. |  |

### Return type

[**crate::models::DictionaryItemResponse**](DictionaryItemResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

