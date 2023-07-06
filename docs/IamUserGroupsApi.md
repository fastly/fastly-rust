# IamUserGroupsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_group_members**](IamUserGroupsApi.md#add_user_group_members) | **POST** /user-groups/{user_group_id}/members | Add members to a user group
[**add_user_group_roles**](IamUserGroupsApi.md#add_user_group_roles) | **POST** /user-groups/{user_group_id}/roles | Add roles to a user group
[**add_user_group_service_groups**](IamUserGroupsApi.md#add_user_group_service_groups) | **POST** /user-groups/{user_group_id}/service-groups | Add service groups to a user group
[**create_a_user_group**](IamUserGroupsApi.md#create_a_user_group) | **POST** /user-groups | Create a user group
[**delete_a_user_group**](IamUserGroupsApi.md#delete_a_user_group) | **DELETE** /user-groups/{user_group_id} | Delete a user group
[**get_a_user_group**](IamUserGroupsApi.md#get_a_user_group) | **GET** /user-groups/{user_group_id} | Get a user group
[**list_user_group_members**](IamUserGroupsApi.md#list_user_group_members) | **GET** /user-groups/{user_group_id}/members | List members of a user group
[**list_user_group_roles**](IamUserGroupsApi.md#list_user_group_roles) | **GET** /user-groups/{user_group_id}/roles | List roles in a user group
[**list_user_group_service_groups**](IamUserGroupsApi.md#list_user_group_service_groups) | **GET** /user-groups/{user_group_id}/service-groups | List service groups in a user group
[**list_user_groups**](IamUserGroupsApi.md#list_user_groups) | **GET** /user-groups | List user groups
[**remove_user_group_members**](IamUserGroupsApi.md#remove_user_group_members) | **DELETE** /user-groups/{user_group_id}/members | Remove members of a user group
[**remove_user_group_roles**](IamUserGroupsApi.md#remove_user_group_roles) | **DELETE** /user-groups/{user_group_id}/roles | Remove roles from a user group
[**remove_user_group_service_groups**](IamUserGroupsApi.md#remove_user_group_service_groups) | **DELETE** /user-groups/{user_group_id}/service-groups | Remove service groups from a user group
[**update_a_user_group**](IamUserGroupsApi.md#update_a_user_group) | **PATCH** /user-groups/{user_group_id} | Update a user group



## add_user_group_members

Add members to a user group.

```rust
let cfg = &Configuration::default();
let params = AddUserGroupMembersParams {
    // parameters
};
add_user_group_members(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## add_user_group_roles

Add roles to a user group.

```rust
let cfg = &Configuration::default();
let params = AddUserGroupRolesParams {
    // parameters
};
add_user_group_roles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## add_user_group_service_groups

Add service groups to a user group.

```rust
let cfg = &Configuration::default();
let params = AddUserGroupServiceGroupsParams {
    // parameters
};
add_user_group_service_groups(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_a_user_group

Create a user group.

```rust
let cfg = &Configuration::default();
let params = CreateAUserGroupParams {
    // parameters
};
create_a_user_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_a_user_group

Delete a user group.

```rust
let cfg = &Configuration::default();
let params = DeleteAUserGroupParams {
    // parameters
};
delete_a_user_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_a_user_group

Get a user group.

```rust
let cfg = &Configuration::default();
let params = GetAUserGroupParams {
    // parameters
};
get_a_user_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_user_group_members

List members of a user group.

```rust
let cfg = &Configuration::default();
let params = ListUserGroupMembersParams {
    // parameters
};
list_user_group_members(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**page** | Option\<**i32**> | Current page. |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_user_group_roles

List roles in a user group.

```rust
let cfg = &Configuration::default();
let params = ListUserGroupRolesParams {
    // parameters
};
list_user_group_roles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**page** | Option\<**i32**> | Current page. |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_user_group_service_groups

List service groups in a user group.

```rust
let cfg = &Configuration::default();
let params = ListUserGroupServiceGroupsParams {
    // parameters
};
list_user_group_service_groups(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**page** | Option\<**i32**> | Current page. |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_user_groups

List all user groups.

```rust
let cfg = &Configuration::default();
let params = ListUserGroupsParams {
    // parameters
};
list_user_groups(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**page** | Option\<**i32**> | Current page. |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## remove_user_group_members

Remove members of a user group

```rust
let cfg = &Configuration::default();
let params = RemoveUserGroupMembersParams {
    // parameters
};
remove_user_group_members(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## remove_user_group_roles

Remove roles from a user group.

```rust
let cfg = &Configuration::default();
let params = RemoveUserGroupRolesParams {
    // parameters
};
remove_user_group_roles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## remove_user_group_service_groups

Remove service groups from a user group.

```rust
let cfg = &Configuration::default();
let params = RemoveUserGroupServiceGroupsParams {
    // parameters
};
remove_user_group_service_groups(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_a_user_group

Update a user group.

```rust
let cfg = &Configuration::default();
let params = UpdateAUserGroupParams {
    // parameters
};
update_a_user_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **String** | Alphanumeric string identifying the user group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

