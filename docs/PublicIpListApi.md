# PublicIpListApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_fastly_ips**](PublicIpListApi.md#list_fastly_ips) | **GET** /public-ip-list | List Fastly's public IPs



## list_fastly_ips

List the public IP addresses for the Fastly network.

```rust
let cfg = &Configuration::default();
let params = ListFastlyIpsParams {
    // parameters
};
list_fastly_ips(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PublicIpList**](PublicIpList.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

