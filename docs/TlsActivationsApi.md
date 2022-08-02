# TlsActivationsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tls_activation**](TlsActivationsApi.md#create_tls_activation) | **POST** /tls/activations | Enable TLS for a domain using a custom certificate
[**delete_tls_activation**](TlsActivationsApi.md#delete_tls_activation) | **DELETE** /tls/activations/{tls_activation_id} | Disable TLS on a domain
[**get_tls_activation**](TlsActivationsApi.md#get_tls_activation) | **GET** /tls/activations/{tls_activation_id} | Get a TLS activation
[**list_tls_activations**](TlsActivationsApi.md#list_tls_activations) | **GET** /tls/activations | List TLS activations
[**update_tls_activation**](TlsActivationsApi.md#update_tls_activation) | **PATCH** /tls/activations/{tls_activation_id} | Update a certificate



## create_tls_activation

Enable TLS for a particular TLS domain and certificate combination. These relationships must be specified to create the TLS activation.

```rust
let cfg = &Configuration::default();
let params = CreateTlsActivationParams {
    // parameters
};
create_tls_activation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_activation** | Option\<[**TlsActivation**](TlsActivation.md)> |  |  |

### Return type

[**crate::models::TlsActivationResponse**](TlsActivationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tls_activation

Disable TLS on the domain associated with this TLS activation.

```rust
let cfg = &Configuration::default();
let params = DeleteTlsActivationParams {
    // parameters
};
delete_tls_activation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_activation_id** | **String** | Alphanumeric string identifying a TLS activation. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tls_activation

Show a TLS activation.

```rust
let cfg = &Configuration::default();
let params = GetTlsActivationParams {
    // parameters
};
get_tls_activation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_activation_id** | **String** | Alphanumeric string identifying a TLS activation. | [required] |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_certificate`, `tls_configuration`, and `tls_domain`.  |  |

### Return type

[**crate::models::TlsActivationResponse**](TlsActivationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_activations

List all TLS activations.

```rust
let cfg = &Configuration::default();
let params = ListTlsActivationsParams {
    // parameters
};
list_tls_activations(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_tls_certificate_id** | Option\<**String**> | Limit the returned activations to a specific certificate. |  |
**filter_tls_configuration_id** | Option\<**String**> | Limit the returned activations to a specific TLS configuration. |  |
**filter_tls_domain_id** | Option\<**String**> | Limit the returned rules to a specific domain name. |  |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_certificate`, `tls_configuration`, and `tls_domain`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]

### Return type

[**crate::models::TlsActivationsResponse**](TlsActivationsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_tls_activation

Update the certificate used to terminate TLS traffic for the domain associated with this TLS activation.

```rust
let cfg = &Configuration::default();
let params = UpdateTlsActivationParams {
    // parameters
};
update_tls_activation(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_activation_id** | **String** | Alphanumeric string identifying a TLS activation. | [required] |
**tls_activation** | Option\<[**TlsActivation**](TlsActivation.md)> |  |  |

### Return type

[**crate::models::TlsActivationResponse**](TlsActivationResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

