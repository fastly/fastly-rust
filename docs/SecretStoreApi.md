# SecretStoreApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_key**](SecretStoreApi.md#client_key) | **POST** /resources/stores/secret/client-key | Create new client key
[**create_secret_store**](SecretStoreApi.md#create_secret_store) | **POST** /resources/stores/secret | Create new secret store
[**delete_secret_store**](SecretStoreApi.md#delete_secret_store) | **DELETE** /resources/stores/secret/{store_id} | Delete secret store
[**get_secret_store**](SecretStoreApi.md#get_secret_store) | **GET** /resources/stores/secret/{store_id} | Create secret store by ID
[**get_secret_stores**](SecretStoreApi.md#get_secret_stores) | **GET** /resources/stores/secret | Get all secret stores
[**signing_key**](SecretStoreApi.md#signing_key) | **GET** /resources/stores/secret/signing-key | Get public key



## client_key

Create a new client key for encrypting secrets locally before uploading.

```rust
let cfg = &Configuration::default();
let params = ClientKeyParams {
    // parameters
};
client_key(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClientKey**](ClientKey.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_secret_store

Create a new secret store.

```rust
let cfg = &Configuration::default();
let params = CreateSecretStoreParams {
    // parameters
};
create_secret_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_store** | Option\<[**SecretStore**](SecretStore.md)> |  |  |

### Return type

[**crate::models::SecretStoreResponse**](SecretStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_secret_store

Delete a secret store and all of its contents.

```rust
let cfg = &Configuration::default();
let params = DeleteSecretStoreParams {
    // parameters
};
delete_secret_store(cfg, params)
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


## get_secret_store

Get a secret store by ID.

```rust
let cfg = &Configuration::default();
let params = GetSecretStoreParams {
    // parameters
};
get_secret_store(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |

### Return type

[**crate::models::SecretStoreResponse**](SecretStoreResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_secret_stores

Get all secret stores.

```rust
let cfg = &Configuration::default();
let params = GetSecretStoresParams {
    // parameters
};
get_secret_stores(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option\<**String**> | Cursor value from a previous response to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 200. |  |[default to 100]

### Return type

[**crate::models::InlineResponse2005**](InlineResponse2005.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## signing_key

Get the public key used for signing client keys.

```rust
let cfg = &Configuration::default();
let params = SigningKeyParams {
    // parameters
};
signing_key(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SigningKey**](SigningKey.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

