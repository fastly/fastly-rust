# TokensApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_token**](TokensApi.md#get_token) | **GET** /tokens/{token_id} | Get a token
[**get_token_current**](TokensApi.md#get_token_current) | **GET** /tokens/self | Get the current token
[**list_tokens_customer**](TokensApi.md#list_tokens_customer) | **GET** /customer/{customer_id}/tokens | List tokens for a customer
[**list_tokens_user**](TokensApi.md#list_tokens_user) | **GET** /tokens | List tokens for the authenticated user
[**revoke_token**](TokensApi.md#revoke_token) | **DELETE** /tokens/{token_id} | Revoke a token
[**revoke_token_current**](TokensApi.md#revoke_token_current) | **DELETE** /tokens/self | Revoke the current token



## get_token

Get a single token by its id.

```rust
let cfg = &Configuration::default();
let params = GetTokenParams {
    // parameters
};
get_token(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | Alphanumeric string identifying a token. | [required] |

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_token_current

Get a single token based on the access_token used in the request.

```rust
let cfg = &Configuration::default();
let params = GetTokenCurrentParams {
    // parameters
};
get_token_current(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokenResponse**](TokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tokens_customer

List all tokens belonging to a specific customer.

```rust
let cfg = &Configuration::default();
let params = ListTokensCustomerParams {
    // parameters
};
list_tokens_customer(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** | Alphanumeric string identifying the customer. | [required] |

### Return type

[**Vec&lt;crate::models::TokenResponse&gt;**](TokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tokens_user

List all tokens belonging to the authenticated user.

```rust
let cfg = &Configuration::default();
let params = ListTokensUserParams {
    // parameters
};
list_tokens_user(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec&lt;crate::models::TokenResponse&gt;**](TokenResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## revoke_token

Revoke a specific token by its id.

```rust
let cfg = &Configuration::default();
let params = RevokeTokenParams {
    // parameters
};
revoke_token(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | Alphanumeric string identifying a token. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## revoke_token_current

Revoke a token that is used to authenticate the request.

```rust
let cfg = &Configuration::default();
let params = RevokeTokenCurrentParams {
    // parameters
};
revoke_token_current(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

