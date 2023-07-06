# IamServiceGroupsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_service_group_services**](IamServiceGroupsApi.md#add_service_group_services) | **POST** /service-groups/{service_group_id}/services | Add services in a service group
[**create_a_service_group**](IamServiceGroupsApi.md#create_a_service_group) | **POST** /service-groups | Create a service group
[**delete_a_service_group**](IamServiceGroupsApi.md#delete_a_service_group) | **DELETE** /service-groups/{service_group_id} | Delete a service group
[**get_a_service_group**](IamServiceGroupsApi.md#get_a_service_group) | **GET** /service-groups/{service_group_id} | Get a service group
[**list_service_group_services**](IamServiceGroupsApi.md#list_service_group_services) | **GET** /service-groups/{service_group_id}/services | List services to a service group
[**list_service_groups**](IamServiceGroupsApi.md#list_service_groups) | **GET** /service-groups | List service groups
[**remove_service_group_services**](IamServiceGroupsApi.md#remove_service_group_services) | **DELETE** /service-groups/{service_group_id}/services | Remove services from a service group
[**update_a_service_group**](IamServiceGroupsApi.md#update_a_service_group) | **PATCH** /service-groups/{service_group_id} | Update a service group



## add_service_group_services

Add services in a service group.

```rust
let cfg = &Configuration::default();
let params = AddServiceGroupServicesParams {
    // parameters
};
add_service_group_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_a_service_group

Create a service group.

```rust
let cfg = &Configuration::default();
let params = CreateAServiceGroupParams {
    // parameters
};
create_a_service_group(cfg, params)
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


## delete_a_service_group

Delete a service group.

```rust
let cfg = &Configuration::default();
let params = DeleteAServiceGroupParams {
    // parameters
};
delete_a_service_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_a_service_group

Get a service group.

```rust
let cfg = &Configuration::default();
let params = GetAServiceGroupParams {
    // parameters
};
get_a_service_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_service_group_services

List services to a service group.

```rust
let cfg = &Configuration::default();
let params = ListServiceGroupServicesParams {
    // parameters
};
list_service_group_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |
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


## list_service_groups

List all service groups.

```rust
let cfg = &Configuration::default();
let params = ListServiceGroupsParams {
    // parameters
};
list_service_groups(cfg, params)
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


## remove_service_group_services

Remove services from a service group.

```rust
let cfg = &Configuration::default();
let params = RemoveServiceGroupServicesParams {
    // parameters
};
remove_service_group_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_a_service_group

Update a service group.

```rust
let cfg = &Configuration::default();
let params = UpdateAServiceGroupParams {
    // parameters
};
update_a_service_group(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_group_id** | **String** | Alphanumeric string identifying the service group. | [required] |
**request_body** | Option\<[**::std::collections::HashMap&lt;String, serde_json::Value&gt;**](serde_json::Value.md)> |  |  |

### Return type

[**serde_json::Value**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

