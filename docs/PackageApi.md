# PackageApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_package**](PackageApi.md#get_package) | **GET** /service/{service_id}/version/{version_id}/package | Get details of the service's Compute@Edge package.
[**put_package**](PackageApi.md#put_package) | **PUT** /service/{service_id}/version/{version_id}/package | Upload a Compute@Edge package.



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


## put_package

Upload a Compute@Edge package associated with the specified service version.

```rust
let cfg = &Configuration::default();
let params = PutPackageParams {
    // parameters
};
put_package(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**expect** | Option\<**String**> | We recommend using the Expect header because it may identify issues with the request based upon the headers alone instead of requiring you to wait until the entire binary package upload has completed. |  |
**package** | Option\<**std::path::PathBuf**> | The content of the Wasm binary package. |  |

### Return type

[**crate::models::PackageResponse**](PackageResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

