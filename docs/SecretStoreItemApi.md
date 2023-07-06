# SecretStoreItemApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_secret**](SecretStoreItemApi.md#create_secret) | **POST** /resources/stores/secret/{store_id}/secrets | Create a new secret in a store.
[**delete_secret**](SecretStoreItemApi.md#delete_secret) | **DELETE** /resources/stores/secret/{store_id}/secrets/{secret_name} | Delete a secret from a store.
[**get_secret**](SecretStoreItemApi.md#get_secret) | **GET** /resources/stores/secret/{store_id}/secrets/{secret_name} | Get secret metadata.
[**get_secrets**](SecretStoreItemApi.md#get_secrets) | **GET** /resources/stores/secret/{store_id}/secrets | List secrets within a store.
[**must_recreate_secret**](SecretStoreItemApi.md#must_recreate_secret) | **PATCH** /resources/stores/secret/{store_id}/secrets | Recreate a secret in a store.
[**recreate_secret**](SecretStoreItemApi.md#recreate_secret) | **PUT** /resources/stores/secret/{store_id}/secrets | Create or recreate a secret in a store.



## create_secret

Create a new secret in a store. Returns an error if a secret already exists with the same name. See `PUT` and `PATCH` methods for ways to recreate an existing secret.  The `secret` field must be Base64-encoded because a secret can contain binary data. In the example below, the unencoded secret is \"Hello, world!\" 

```rust
let cfg = &Configuration::default();
let params = CreateSecretParams {
    // parameters
};
create_secret(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**secret** | Option\<[**Secret**](Secret.md)> |  |  |

### Return type

[**crate::models::SecretResponse**](SecretResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_secret

Delete a secret from a store by name.

```rust
let cfg = &Configuration::default();
let params = DeleteSecretParams {
    // parameters
};
delete_secret(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_secret

Get metadata about a secret by name.

```rust
let cfg = &Configuration::default();
let params = GetSecretParams {
    // parameters
};
get_secret(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |

### Return type

[**crate::models::SecretResponse**](SecretResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_secrets

List all secrets within a store.

```rust
let cfg = &Configuration::default();
let params = GetSecretsParams {
    // parameters
};
get_secrets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**cursor** | Option\<**String**> | Cursor value from a previous response to retrieve the next page. To request the first page, this should be empty. |  |
**limit** | Option\<**String**> | Number of results per page. The maximum is 200. |  |[default to 100]

### Return type

[**crate::models::InlineResponse2006**](InlineResponse2006.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## must_recreate_secret

Recreate a secret based on the secret's name. Returns an error if there is no existing secret with the same name.  The `secret` field must be Base64-encoded because a secret can contain binary data. In the example below, the unencoded secret is \"Hello, world!\" 

```rust
let cfg = &Configuration::default();
let params = MustRecreateSecretParams {
    // parameters
};
must_recreate_secret(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**secret** | Option\<[**Secret**](Secret.md)> |  |  |

### Return type

[**crate::models::SecretResponse**](SecretResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## recreate_secret

Create or recreate a secret based on the secret's name. The response object's `recreated` field will be true if the secret was recreated.  The `secret` field must be Base64-encoded because a secret can contain binary data. In the example below, the unencoded secret is \"Hello, world!\" 

```rust
let cfg = &Configuration::default();
let params = RecreateSecretParams {
    // parameters
};
recreate_secret(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**secret** | Option\<[**Secret**](Secret.md)> |  |  |

### Return type

[**crate::models::SecretResponse**](SecretResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

