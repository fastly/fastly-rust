# ConfigStoreApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_config_store**](ConfigStoreApi.md#create_config_store) | **POST** /resources/stores/config | Create a config store
[**delete_config_store**](ConfigStoreApi.md#delete_config_store) | **DELETE** /resources/stores/config/{config_store_id} | Delete a config store
[**get_config_store**](ConfigStoreApi.md#get_config_store) | **GET** /resources/stores/config/{config_store_id} | Describe a config store
[**get_config_store_info**](ConfigStoreApi.md#get_config_store_info) | **GET** /resources/stores/config/{config_store_id}/info | Get config store metadata
[**list_config_store_services**](ConfigStoreApi.md#list_config_store_services) | **GET** /resources/stores/config/{config_store_id}/services | List linked services
[**list_config_stores**](ConfigStoreApi.md#list_config_stores) | **GET** /resources/stores/config | List config stores
[**update_config_store**](ConfigStoreApi.md#update_config_store) | **PUT** /resources/stores/config/{config_store_id} | Update a config store



## create_config_store

Create a config store.

```rust
let cfg = &Configuration::default();
let params = CreateConfigStoreParams {
    // parameters
};
create_config_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option\<**String**> | The name of the config store. |  |

### Return type

[**crate::models::ConfigStoreResponse**](ConfigStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_config_store

Delete a config store.

```rust
let cfg = &Configuration::default();
let params = DeleteConfigStoreParams {
    // parameters
};
delete_config_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_config_store

Describe a config store by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetConfigStoreParams {
    // parameters
};
get_config_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |

### Return type

[**crate::models::ConfigStoreResponse**](ConfigStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_config_store_info

Retrieve metadata for a single config store.

```rust
let cfg = &Configuration::default();
let params = GetConfigStoreInfoParams {
    // parameters
};
get_config_store_info(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |

### Return type

[**crate::models::ConfigStoreInfoResponse**](ConfigStoreInfoResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_config_store_services

List services linked to a config store

```rust
let cfg = &Configuration::default();
let params = ListConfigStoreServicesParams {
    // parameters
};
list_config_store_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_config_stores

List config stores.

```rust
let cfg = &Configuration::default();
let params = ListConfigStoresParams {
    // parameters
};
list_config_stores(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec&lt;crate::models::ConfigStoreResponse&gt;**](ConfigStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_config_store

Update a config store.

```rust
let cfg = &Configuration::default();
let params = UpdateConfigStoreParams {
    // parameters
};
update_config_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_store_id** | **String** | An alphanumeric string identifying the config store. | [required] |
**name** | Option\<**String**> | The name of the config store. |  |

### Return type

[**crate::models::ConfigStoreResponse**](ConfigStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

