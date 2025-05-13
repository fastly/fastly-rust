# AclsInComputeApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**compute_acl_create_acls**](AclsInComputeApi.md#compute_acl_create_acls) | **POST** /resources/acls | Create a new ACL
[**compute_acl_delete_s_acl_id**](AclsInComputeApi.md#compute_acl_delete_s_acl_id) | **DELETE** /resources/acls/{acl_id} | Delete an ACL
[**compute_acl_list_acl_entries**](AclsInComputeApi.md#compute_acl_list_acl_entries) | **GET** /resources/acls/{acl_id}/entries | List an ACL
[**compute_acl_list_acls**](AclsInComputeApi.md#compute_acl_list_acls) | **GET** /resources/acls | List ACLs
[**compute_acl_list_acls_s_acl_id**](AclsInComputeApi.md#compute_acl_list_acls_s_acl_id) | **GET** /resources/acls/{acl_id} | Describe an ACL
[**compute_acl_lookup_acls**](AclsInComputeApi.md#compute_acl_lookup_acls) | **GET** /resources/acls/{acl_id}/entry/{acl_ip} | Lookup an ACL
[**compute_acl_update_acls**](AclsInComputeApi.md#compute_acl_update_acls) | **PATCH** /resources/acls/{acl_id}/entries | Update an ACL



## compute_acl_create_acls

Create a new ACL.

```rust
let cfg = &Configuration::default();
let params = ComputeAclCreateAclsParams {
    // parameters
};
compute_acl_create_acls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_acl_create_acls_request** | Option\<[**ComputeAclCreateAclsRequest**](ComputeAclCreateAclsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeAclCreateAclsResponse**](ComputeAclCreateAclsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_delete_s_acl_id

Delete an ACL.

```rust
let cfg = &Configuration::default();
let params = ComputeAclDeleteSAclIdParams {
    // parameters
};
compute_acl_delete_s_acl_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_list_acl_entries

List an ACL.

```rust
let cfg = &Configuration::default();
let params = ComputeAclListAclEntriesParams {
    // parameters
};
compute_acl_list_acl_entries(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** |  | [required] |
**cursor** | Option\<**String**> |  |  |
**limit** | Option\<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::ComputeAclListEntries**](ComputeAclListEntries.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_list_acls

List all ACLs.

```rust
let cfg = &Configuration::default();
let params = ComputeAclListAclsParams {
    // parameters
};
compute_acl_list_acls(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeAclList**](ComputeAclList.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_list_acls_s_acl_id

Describe an ACL.

```rust
let cfg = &Configuration::default();
let params = ComputeAclListAclsSAclIdParams {
    // parameters
};
compute_acl_list_acls_s_acl_id(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** |  | [required] |

### Return type

[**crate::models::ComputeAclCreateAclsResponse**](ComputeAclCreateAclsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_lookup_acls

Find a matching ACL entry for an IP address.

```rust
let cfg = &Configuration::default();
let params = ComputeAclLookupAclsParams {
    // parameters
};
compute_acl_lookup_acls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** |  | [required] |
**acl_ip** | **String** |  | [required] |

### Return type

[**crate::models::ComputeAclLookup**](ComputeAclLookup.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## compute_acl_update_acls

Update an ACL entry with a new operation or action, this allows you to modify an existing entry or delete it.

```rust
let cfg = &Configuration::default();
let params = ComputeAclUpdateAclsParams {
    // parameters
};
compute_acl_update_acls(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acl_id** | **String** |  | [required] |
**compute_acl_update** | Option\<[**ComputeAclUpdate**](ComputeAclUpdate.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

