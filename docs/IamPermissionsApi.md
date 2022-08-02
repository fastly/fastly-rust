# IamPermissionsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_permissions**](IamPermissionsApi.md#list_permissions) | **GET** /permissions | List permissions



## list_permissions

List all permissions.

```rust
let cfg = &Configuration::default();
let params = ListPermissionsParams {
    // parameters
};
list_permissions(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

