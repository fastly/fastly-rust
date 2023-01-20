# MutualAuthenticationApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mutual_tls_authentication**](MutualAuthenticationApi.md#create_mutual_tls_authentication) | **POST** /tls/mutual_authentications | Create a Mutual Authentication
[**delete_mutual_tls**](MutualAuthenticationApi.md#delete_mutual_tls) | **DELETE** /tls/mutual_authentications/{mutual_authentication_id} | Delete a Mutual TLS
[**get_mutual_authentication**](MutualAuthenticationApi.md#get_mutual_authentication) | **GET** /tls/mutual_authentications/{mutual_authentication_id} | Get a Mutual Authentication
[**list_mutual_authentications**](MutualAuthenticationApi.md#list_mutual_authentications) | **GET** /tls/mutual_authentications | List Mutual Authentications
[**patch_mutual_authentication**](MutualAuthenticationApi.md#patch_mutual_authentication) | **PATCH** /tls/mutual_authentications/{mutual_authentication_id} | Update a Mutual Authentication



## create_mutual_tls_authentication

Create a mutual authentication using a bundle of certificates to enable client-to-server mutual TLS.

```rust
let cfg = &Configuration::default();
let params = CreateMutualTlsAuthenticationParams {
    // parameters
};
create_mutual_tls_authentication(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mutual_authentication** | Option\<[**MutualAuthentication**](MutualAuthentication.md)> |  |  |

### Return type

[**crate::models::MutualAuthenticationResponse**](MutualAuthenticationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_mutual_tls

Remove a Mutual TLS authentication

```rust
let cfg = &Configuration::default();
let params = DeleteMutualTlsParams {
    // parameters
};
delete_mutual_tls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mutual_authentication_id** | **String** | Alphanumeric string identifying a mutual authentication. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_mutual_authentication

Show a Mutual Authentication.

```rust
let cfg = &Configuration::default();
let params = GetMutualAuthenticationParams {
    // parameters
};
get_mutual_authentication(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mutual_authentication_id** | **String** | Alphanumeric string identifying a mutual authentication. | [required] |
**include** | Option\<**String**> | Comma-separated list of related objects to include (optional). Permitted values: `tls_activations`. Including TLS activations will provide you with the TLS domain names that are related to your Mutual TLS authentication.  |  |

### Return type

[**crate::models::MutualAuthenticationResponse**](MutualAuthenticationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_mutual_authentications

List all mutual authentications.

```rust
let cfg = &Configuration::default();
let params = ListMutualAuthenticationsParams {
    // parameters
};
list_mutual_authentications(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option\<**String**> | Comma-separated list of related objects to include (optional). Permitted values: `tls_activations`. Including TLS activations will provide you with the TLS domain names that are related to your Mutual TLS authentication.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::MutualAuthenticationsResponse**](MutualAuthenticationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## patch_mutual_authentication

Update a Mutual Authentication.

```rust
let cfg = &Configuration::default();
let params = PatchMutualAuthenticationParams {
    // parameters
};
patch_mutual_authentication(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mutual_authentication_id** | **String** | Alphanumeric string identifying a mutual authentication. | [required] |
**mutual_authentication** | Option\<[**MutualAuthentication**](MutualAuthentication.md)> |  |  |

### Return type

[**crate::models::MutualAuthenticationResponse**](MutualAuthenticationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

