# AclEntryApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_update_acl_entries**](AclEntryApi.md#bulk_update_acl_entries) | **PATCH** /service/{service_id}/acl/{acl_id}/entries | Update multiple ACL entries
[**create_acl_entry**](AclEntryApi.md#create_acl_entry) | **POST** /service/{service_id}/acl/{acl_id}/entry | Create an ACL entry
[**delete_acl_entry**](AclEntryApi.md#delete_acl_entry) | **DELETE** /service/{service_id}/acl/{acl_id}/entry/{acl_entry_id} | Delete an ACL entry
[**get_acl_entry**](AclEntryApi.md#get_acl_entry) | **GET** /service/{service_id}/acl/{acl_id}/entry/{acl_entry_id} | Describe an ACL entry
[**list_acl_entries**](AclEntryApi.md#list_acl_entries) | **GET** /service/{service_id}/acl/{acl_id}/entries | List ACL entries
[**update_acl_entry**](AclEntryApi.md#update_acl_entry) | **PATCH** /service/{service_id}/acl/{acl_id}/entry/{acl_entry_id} | Update an ACL entry



## bulk_update_acl_entries

Update multiple ACL entries on the same ACL. For faster updates to your service, group your changes into large batches. The maximum batch size is 1000 entries. [Contact support](https://support.fastly.com/) to discuss raising this limit.

```rust
let cfg = &Configuration::default();
let params = BulkUpdateAclEntriesParams {
    // parameters
};
bulk_update_acl_entries(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**bulk_update_acl_entries_request** | Option\<[**BulkUpdateAclEntriesRequest**](BulkUpdateAclEntriesRequest.md)> |  |  |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_acl_entry

Add an ACL entry to an ACL.

```rust
let cfg = &Configuration::default();
let params = CreateAclEntryParams {
    // parameters
};
create_acl_entry(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**acl_entry** | Option\<[**AclEntry**](AclEntry.md)> |  |  |

### Return type

[**crate::models::AclEntryResponse**](AclEntryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_acl_entry

Delete an ACL entry from a specified ACL.

```rust
let cfg = &Configuration::default();
let params = DeleteAclEntryParams {
    // parameters
};
delete_acl_entry(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**acl_entry_id** | **String** | Alphanumeric string identifying an ACL Entry. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_acl_entry

Retrieve a single ACL entry.

```rust
let cfg = &Configuration::default();
let params = GetAclEntryParams {
    // parameters
};
get_acl_entry(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**acl_entry_id** | **String** | Alphanumeric string identifying an ACL Entry. | [required] |

### Return type

[**crate::models::AclEntryResponse**](AclEntryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_acl_entries

List ACL entries for a specified ACL.

```rust
let cfg = &Configuration::default();
let params = ListAclEntriesParams {
    // parameters
};
list_acl_entries(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**page** | Option\<**i32**> | Current page. |  |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | Field on which to sort. |  |[default to created]
**direction** | Option\<**String**> | Direction in which to sort results. |  |[default to ascend]

### Return type

[**Vec&lt;crate::models::AclEntryResponse&gt;**](AclEntryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_acl_entry

Update an ACL entry for a specified ACL.

```rust
let cfg = &Configuration::default();
let params = UpdateAclEntryParams {
    // parameters
};
update_acl_entry(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**acl_id** | **String** | Alphanumeric string identifying a ACL. | [required] |
**acl_entry_id** | **String** | Alphanumeric string identifying an ACL Entry. | [required] |
**acl_entry** | Option\<[**AclEntry**](AclEntry.md)> |  |  |

### Return type

[**crate::models::AclEntryResponse**](AclEntryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

