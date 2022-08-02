# PopApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_pops**](PopApi.md#list_pops) | **GET** /datacenters | List Fastly POPs



## list_pops

Get a list of all Fastly POPs.

```rust
let cfg = &Configuration::default();
let params = ListPopsParams {
    // parameters
};
list_pops(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec&lt;crate::models::Pop&gt;**](Pop.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

