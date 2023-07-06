# IamRolesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_role_permissions**](IamRolesApi.md#add_role_permissions) | **POST** /roles/{role_id}/permissions | Add permissions to a role
[**create_a_role**](IamRolesApi.md#create_a_role) | **POST** /roles | Create a role
[**delete_a_role**](IamRolesApi.md#delete_a_role) | **DELETE** /roles/{role_id} | Delete a role
[**get_a_role**](IamRolesApi.md#get_a_role) | **GET** /roles/{role_id} | Get a role
[**list_role_permissions**](IamRolesApi.md#list_role_permissions) | **GET** /roles/{role_id}/permissions | List permissions in a role
[**list_roles**](IamRolesApi.md#list_roles) | **GET** /roles | List roles
[**remove_role_permissions**](IamRolesApi.md#remove_role_permissions) | **DELETE** /roles/{role_id}/permissions | Remove permissions from a role
[**update_a_role**](IamRolesApi.md#update_a_role) | **PATCH** /roles/{role_id} | Update a role



## add_role_permissions

Add permissions to a role.

```rust
let cfg = &Configuration::default();
let params = AddRolePermissionsParams {
    // parameters
};
add_role_permissions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_a_role

Create a role.

```rust
let cfg = &Configuration::default();
let params = CreateARoleParams {
    // parameters
};
create_a_role(cfg, params)
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


## delete_a_role

Delete a role.

```rust
let cfg = &Configuration::default();
let params = DeleteARoleParams {
    // parameters
};
delete_a_role(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_a_role

Get a role.

```rust
let cfg = &Configuration::default();
let params = GetARoleParams {
    // parameters
};
get_a_role(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_role_permissions

List all permissions in a role.

```rust
let cfg = &Configuration::default();
let params = ListRolePermissionsParams {
    // parameters
};
list_role_permissions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_roles

List all roles.

```rust
let cfg = &Configuration::default();
let params = ListRolesParams {
    // parameters
};
list_roles(cfg, params)
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


## remove_role_permissions

Remove permissions from a role.

```rust
let cfg = &Configuration::default();
let params = RemoveRolePermissionsParams {
    // parameters
};
remove_role_permissions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_a_role

Update a role.

```rust
let cfg = &Configuration::default();
let params = UpdateARoleParams {
    // parameters
};
update_a_role(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Alphanumeric string identifying the role. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

