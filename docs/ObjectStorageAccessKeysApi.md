# ObjectStorageAccessKeysApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_access_key**](ObjectStorageAccessKeysApi.md#create_access_key) | **POST** /resources/object-storage/access-keys | Create an access key
[**delete_access_key**](ObjectStorageAccessKeysApi.md#delete_access_key) | **DELETE** /resources/object-storage/access-keys/{access_key} | Delete an access key
[**get_access_key**](ObjectStorageAccessKeysApi.md#get_access_key) | **GET** /resources/object-storage/access-keys/{access_key} | Get an access key
[**list_access_keys**](ObjectStorageAccessKeysApi.md#list_access_keys) | **GET** /resources/object-storage/access-keys | List access keys



## create_access_key

Create an access key.

```rust
let cfg = &Configuration::default();
let params = CreateAccessKeyParams {
    // parameters
};
create_access_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | Option\<[**AccessKey**](AccessKey.md)> |  |  |

### Return type

[**crate::models::AccessKeyResponse**](AccessKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_access_key

Delete an access key.

```rust
let cfg = &Configuration::default();
let params = DeleteAccessKeyParams {
    // parameters
};
delete_access_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_access_key

Get an access key by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetAccessKeyParams {
    // parameters
};
get_access_key(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** |  | [required] |

### Return type

[**crate::models::AccessKey**](AccessKey.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_access_keys

List access keys.

```rust
let cfg = &Configuration::default();
let params = ListAccessKeysParams {
    // parameters
};
list_access_keys(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccessKeyResponse**](AccessKeyResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

