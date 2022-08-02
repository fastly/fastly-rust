# TlsBulkCertificatesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_bulk_tls_cert**](TlsBulkCertificatesApi.md#delete_bulk_tls_cert) | **DELETE** /tls/bulk/certificates/{certificate_id} | Delete a certificate
[**get_tls_bulk_cert**](TlsBulkCertificatesApi.md#get_tls_bulk_cert) | **GET** /tls/bulk/certificates/{certificate_id} | Get a certificate
[**list_tls_bulk_certs**](TlsBulkCertificatesApi.md#list_tls_bulk_certs) | **GET** /tls/bulk/certificates | List certificates
[**update_bulk_tls_cert**](TlsBulkCertificatesApi.md#update_bulk_tls_cert) | **PATCH** /tls/bulk/certificates/{certificate_id} | Update a certificate
[**upload_tls_bulk_cert**](TlsBulkCertificatesApi.md#upload_tls_bulk_cert) | **POST** /tls/bulk/certificates | Upload a certificate



## delete_bulk_tls_cert

Destroy a certificate. This disables TLS for all domains listed as SAN entries.

```rust
let cfg = &Configuration::default();
let params = DeleteBulkTlsCertParams {
    // parameters
};
delete_bulk_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Alphanumeric string identifying a TLS bulk certificate. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_tls_bulk_cert

Retrieve a single certificate.

```rust
let cfg = &Configuration::default();
let params = GetTlsBulkCertParams {
    // parameters
};
get_tls_bulk_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Alphanumeric string identifying a TLS bulk certificate. | [required] |

### Return type

[**crate::models::TlsBulkCertificateResponse**](TlsBulkCertificateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_tls_bulk_certs

List all certificates.

```rust
let cfg = &Configuration::default();
let params = ListTlsBulkCertsParams {
    // parameters
};
list_tls_bulk_certs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_tls_domain_id** | Option\<**String**> | Filter certificates by their matching, fully-qualified domain name. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | The order in which to list the results by creation date. |  |[default to created_at]

### Return type

[**crate::models::TlsBulkCertificatesResponse**](TlsBulkCertificatesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_bulk_tls_cert

Replace a certificate with a newly reissued certificate. By using this endpoint, the original certificate will cease to be used for future TLS handshakes. Thus, only SAN entries that appear in the replacement certificate will become TLS enabled. Any SAN entries that are missing in the replacement certificate will become disabled.

```rust
let cfg = &Configuration::default();
let params = UpdateBulkTlsCertParams {
    // parameters
};
update_bulk_tls_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Alphanumeric string identifying a TLS bulk certificate. | [required] |
**tls_bulk_certificate** | Option\<[**TlsBulkCertificate**](TlsBulkCertificate.md)> |  |  |

### Return type

[**crate::models::TlsBulkCertificateResponse**](TlsBulkCertificateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## upload_tls_bulk_cert

Upload a new certificate. TLS domains are automatically enabled upon certificate creation. If a domain is already enabled on a previously uploaded certificate, that domain will be updated to use the new certificate for all future TLS handshake requests.

```rust
let cfg = &Configuration::default();
let params = UploadTlsBulkCertParams {
    // parameters
};
upload_tls_bulk_cert(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_bulk_certificate** | Option\<[**TlsBulkCertificate**](TlsBulkCertificate.md)> |  |  |

### Return type

[**crate::models::TlsBulkCertificateResponse**](TlsBulkCertificateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

