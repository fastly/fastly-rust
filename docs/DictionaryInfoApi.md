# DictionaryInfoApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dictionary_info**](DictionaryInfoApi.md#get_dictionary_info) | **GET** /service/{service_id}/version/{version_id}/dictionary/{dictionary_id}/info | Get edge dictionary metadata



## get_dictionary_info

Retrieve metadata for a single dictionary by ID for a version and service.

```rust
let cfg = &Configuration::default();
let params = GetDictionaryInfoParams {
    // parameters
};
get_dictionary_info(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**dictionary_id** | **String** | Alphanumeric string identifying a Dictionary. | [required] |

### Return type

[**crate::models::DictionaryInfoResponse**](DictionaryInfoResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

