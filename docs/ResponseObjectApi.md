# ResponseObjectApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_response_object**](ResponseObjectApi.md#create_response_object) | **POST** /service/{service_id}/version/{version_id}/response_object | Create a Response object
[**delete_response_object**](ResponseObjectApi.md#delete_response_object) | **DELETE** /service/{service_id}/version/{version_id}/response_object/{response_object_name} | Delete a Response Object
[**get_response_object**](ResponseObjectApi.md#get_response_object) | **GET** /service/{service_id}/version/{version_id}/response_object/{response_object_name} | Get a Response object
[**list_response_objects**](ResponseObjectApi.md#list_response_objects) | **GET** /service/{service_id}/version/{version_id}/response_object | List Response objects
[**update_response_object**](ResponseObjectApi.md#update_response_object) | **PUT** /service/{service_id}/version/{version_id}/response_object/{response_object_name} | Update a Response object



## create_response_object

Creates a new Response Object.

```rust
let cfg = &Configuration::default();
let params = CreateResponseObjectParams {
    // parameters
};
create_response_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::ResponseObjectResponse**](ResponseObjectResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_response_object

Deletes the specified Response Object.

```rust
let cfg = &Configuration::default();
let params = DeleteResponseObjectParams {
    // parameters
};
delete_response_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**response_object_name** | **String** | Name for the request settings. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_response_object

Gets the specified Response Object.

```rust
let cfg = &Configuration::default();
let params = GetResponseObjectParams {
    // parameters
};
get_response_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**response_object_name** | **String** | Name for the request settings. | [required] |

### Return type

[**crate::models::ResponseObjectResponse**](ResponseObjectResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_response_objects

Returns all Response Objects for the specified service and version.

```rust
let cfg = &Configuration::default();
let params = ListResponseObjectsParams {
    // parameters
};
list_response_objects(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::ResponseObjectResponse&gt;**](ResponseObjectResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_response_object

Updates the specified Response Object.

```rust
let cfg = &Configuration::default();
let params = UpdateResponseObjectParams {
    // parameters
};
update_response_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**response_object_name** | **String** | Name for the request settings. | [required] |

### Return type

[**crate::models::ResponseObjectResponse**](ResponseObjectResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

