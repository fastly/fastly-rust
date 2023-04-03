# DomainOwnershipsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_domain_ownerships**](DomainOwnershipsApi.md#list_domain_ownerships) | **GET** /domain-ownerships | List domain-ownerships



## list_domain_ownerships

List all domain-ownerships.

```rust
let cfg = &Configuration::default();
let params = ListDomainOwnershipsParams {
    // parameters
};
list_domain_ownerships(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2002**](InlineResponse2002.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

