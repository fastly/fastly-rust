# VclDiffApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vcl_diff_service_versions**](VclDiffApi.md#vcl_diff_service_versions) | **GET** /service/{service_id}/vcl/diff/from/{from_version_id}/to/{to_version_id} | Get a comparison of the VCL changes between two service versions



## vcl_diff_service_versions

Get a comparison of the VCL changes between two service versions.

```rust
let cfg = &Configuration::default();
let params = VclDiffServiceVersionsParams {
    // parameters
};
vcl_diff_service_versions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**from_version_id** | **i32** | The version number of the service to which changes in the generated VCL are being compared. Can either be a positive number from 1 to your maximum version or a negative number from -1 down (-1 is latest version etc). | [required] |
**to_version_id** | **i32** | The version number of the service from which changes in the generated VCL are being compared. Uses same numbering scheme as `from`. | [required] |
**format** | Option\<**String**> | Optional method to format the diff field. |  |[default to text]

### Return type

[**crate::models::VclDiff**](VclDiff.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

