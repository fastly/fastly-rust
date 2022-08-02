# TlsPrivateKeysApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tls_key**](TlsPrivateKeysApi.md#create_tls_key) | **POST** /tls/private_keys | Create a TLS private key
[**delete_tls_key**](TlsPrivateKeysApi.md#delete_tls_key) | **DELETE** /tls/private_keys/{tls_private_key_id} | Delete a TLS private key
[**get_tls_key**](TlsPrivateKeysApi.md#get_tls_key) | **GET** /tls/private_keys/{tls_private_key_id} | Get a TLS private key
[**list_tls_keys**](TlsPrivateKeysApi.md#list_tls_keys) | **GET** /tls/private_keys | List TLS private keys



## create_tls_key

Create a TLS private key.

```rust
let cfg = &Configuration::default();
let params = CreateTlsKeyParams {
    // parameters
};
create_tls_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_private_key** | Option\<[**TlsPrivateKey**](TlsPrivateKey.md)> |  |  |

### Return type

[**crate::models::TlsPrivateKeyResponse**](TlsPrivateKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tls_key

Destroy a TLS private key. Only private keys not already matched to any certificates can be deleted.

```rust
let cfg = &Configuration::default();
let params = DeleteTlsKeyParams {
    // parameters
};
delete_tls_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_private_key_id** | **String** | Alphanumeric string identifying a private Key. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tls_key

Show a TLS private key.

```rust
let cfg = &Configuration::default();
let params = GetTlsKeyParams {
    // parameters
};
get_tls_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_private_key_id** | **String** | Alphanumeric string identifying a private Key. | [required] |

### Return type

[**crate::models::TlsPrivateKeyResponse**](TlsPrivateKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_keys

List all TLS private keys.

```rust
let cfg = &Configuration::default();
let params = ListTlsKeysParams {
    // parameters
};
list_tls_keys(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_in_use** | Option\<**String**> | Limit the returned keys to those without any matching TLS certificates. The only valid value is false. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::TlsPrivateKeysResponse**](TlsPrivateKeysResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

