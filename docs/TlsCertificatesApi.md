# TlsCertificatesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tls_cert**](TlsCertificatesApi.md#create_tls_cert) | **POST** /tls/certificates | Create a TLS certificate
[**delete_tls_cert**](TlsCertificatesApi.md#delete_tls_cert) | **DELETE** /tls/certificates/{tls_certificate_id} | Delete a TLS certificate
[**get_tls_cert**](TlsCertificatesApi.md#get_tls_cert) | **GET** /tls/certificates/{tls_certificate_id} | Get a TLS certificate
[**list_tls_certs**](TlsCertificatesApi.md#list_tls_certs) | **GET** /tls/certificates | List TLS certificates
[**update_tls_cert**](TlsCertificatesApi.md#update_tls_cert) | **PATCH** /tls/certificates/{tls_certificate_id} | Update a TLS certificate



## create_tls_cert

Create a TLS certificate.

```rust
let cfg = &Configuration::default();
let params = CreateTlsCertParams {
    // parameters
};
create_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_certificate** | Option\<[**TlsCertificate**](TlsCertificate.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_tls_cert

Destroy a TLS certificate. TLS certificates already enabled for a domain cannot be destroyed.

```rust
let cfg = &Configuration::default();
let params = DeleteTlsCertParams {
    // parameters
};
delete_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_certificate_id** | **String** | Alphanumeric string identifying a TLS certificate. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tls_cert

Show a TLS certificate.

```rust
let cfg = &Configuration::default();
let params = GetTlsCertParams {
    // parameters
};
get_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_certificate_id** | **String** | Alphanumeric string identifying a TLS certificate. | [required] |

### Return type

[**crate::models::TlsCertificateResponse**](TlsCertificateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_certs

List all TLS certificates.

```rust
let cfg = &Configuration::default();
let params = ListTlsCertsParams {
    // parameters
};
list_tls_certs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_in_use** | Option\<**String**> | Optional. Limit the returned certificates to those currently using Fastly to terminate TLS (that is, certificates associated with an activation). Permitted values: true, false. |  |
**filter_not_after** | Option\<**String**> | Limit the returned certificates to those that expire prior to the specified date in UTC. Accepts parameters: lte (e.g., filter[not_after][lte]=2020-05-05).  |  |
**filter_tls_domains_id** | Option\<**String**> | Limit the returned certificates to those that include the specific domain. |  |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_activations`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | The order in which to list the results by creation date. |  |[default to created_at]

### Return type

[**crate::models::TlsCertificatesResponse**](TlsCertificatesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_tls_cert

Replace a TLS certificate with a newly reissued TLS certificate, or update a TLS certificate's name. If replacing a TLS certificate, the new TLS certificate must contain all SAN entries as the current TLS certificate. It must either have an exact matching list or contain a superset.

```rust
let cfg = &Configuration::default();
let params = UpdateTlsCertParams {
    // parameters
};
update_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_certificate_id** | **String** | Alphanumeric string identifying a TLS certificate. | [required] |
**tls_certificate** | Option\<[**TlsCertificate**](TlsCertificate.md)> |  |  |

### Return type

[**crate::models::TlsCertificateResponse**](TlsCertificateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

