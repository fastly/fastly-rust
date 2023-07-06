# SudoApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**request_sudo_access**](SudoApi.md#request_sudo_access) | **POST** /sudo | Request Sudo access



## request_sudo_access

Re-authenticate to allow the provided user to obtain sudo access.

```rust
let cfg = &Configuration::default();
let params = RequestSudoAccessParams {
    // parameters
};
request_sudo_access(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sudo_request** | Option\<[**SudoRequest**](SudoRequest.md)> |  |  |

### Return type

[**crate::models::SudoResponse**](SudoResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/vnd.api+json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

