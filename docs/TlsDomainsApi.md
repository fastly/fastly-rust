# TlsDomainsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_tls_domains**](TlsDomainsApi.md#list_tls_domains) | **GET** /tls/domains | List TLS domains



## list_tls_domains

List all TLS domains.

```rust
let cfg = &Configuration::default();
let params = ListTlsDomainsParams {
    // parameters
};
list_tls_domains(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_in_use** | Option\<**String**> | Optional. Limit the returned domains to those currently using Fastly to terminate TLS with SNI (that is, domains considered \"in use\") Permitted values: true, false. |  |
**filter_tls_certificates_id** | Option\<**String**> | Optional. Limit the returned domains to those listed in the given TLS certificate's SAN list. |  |
**filter_tls_subscriptions_id** | Option\<**String**> | Optional. Limit the returned domains to those for a given TLS subscription. |  |
**include** | Option\<**String**> | Include related objects. Optional, comma-separated values. Permitted values: `tls_activations`, `tls_certificates`, `tls_subscriptions`, `tls_subscriptions.tls_authorizations`, and `tls_authorizations.globalsign_email_challenge`.  |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | The order in which to list the results by creation date. |  |[default to created_at]

### Return type

[**crate::models::TlsDomainsResponse**](TlsDomainsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

