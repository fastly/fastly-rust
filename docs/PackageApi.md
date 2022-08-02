# PackageApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_package**](PackageApi.md#get_package) | **GET** /service/{service_id}/version/{version_id}/package | Get details of the service's Compute@Edge package.



## get_package

List detailed information about the Compute@Edge package for the specified service.

```rust
let cfg = &Configuration::default();
let params = GetPackageParams {
    // parameters
};
get_package(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::PackageResponse**](PackageResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

