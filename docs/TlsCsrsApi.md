# TlsCsrsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_csr**](TlsCsrsApi.md#create_csr) | **POST** /tls/certificate_signing_requests | Create CSR



## create_csr

Creates a certificate signing request (CSR).

```rust
let cfg = &Configuration::default();
let params = CreateCsrParams {
    // parameters
};
create_csr(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tls_csr** | Option\<[**TlsCsr**](TlsCsr.md)> |  |  |

### Return type

[**crate::models::TlsCsrResponse**](TlsCsrResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

