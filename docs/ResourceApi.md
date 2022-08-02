# ResourceApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_resource**](ResourceApi.md#create_resource) | **POST** /service/{service_id}/version/{version_id}/resource | Create a resource
[**delete_resource**](ResourceApi.md#delete_resource) | **DELETE** /service/{service_id}/version/{version_id}/resource/{resource_id} | Delete a resource
[**get_resource**](ResourceApi.md#get_resource) | **GET** /service/{service_id}/version/{version_id}/resource/{resource_id} | Display a resource
[**list_resources**](ResourceApi.md#list_resources) | **GET** /service/{service_id}/version/{version_id}/resource | List resources
[**update_resource**](ResourceApi.md#update_resource) | **PUT** /service/{service_id}/version/{version_id}/resource/{resource_id} | Update a resource



## create_resource

Create a resource.

```rust
let cfg = &Configuration::default();
let params = CreateResourceParams {
    // parameters
};
create_resource(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name of the resource. |  |
**resource_id** | Option\<**String**> | The ID of the linked resource. |  |

### Return type

[**crate::models::ResourceResponse**](ResourceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_resource

Delete a resource.

```rust
let cfg = &Configuration::default();
let params = DeleteResourceParams {
    // parameters
};
delete_resource(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**resource_id** | **String** | An alphanumeric string identifying the resource. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_resource

Display a resource by its identifier.

```rust
let cfg = &Configuration::default();
let params = GetResourceParams {
    // parameters
};
get_resource(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**resource_id** | **String** | An alphanumeric string identifying the resource. | [required] |

### Return type

[**crate::models::ResourceResponse**](ResourceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_resources

List resources.

```rust
let cfg = &Configuration::default();
let params = ListResourcesParams {
    // parameters
};
list_resources(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::ResourceResponse&gt;**](ResourceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_resource

Update a resource.

```rust
let cfg = &Configuration::default();
let params = UpdateResourceParams {
    // parameters
};
update_resource(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**resource_id** | **String** | An alphanumeric string identifying the resource. | [required] |
**name** | Option\<**String**> | The name of the resource. |  |

### Return type

[**crate::models::ResourceResponse**](ResourceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

