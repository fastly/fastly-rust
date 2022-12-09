# HeaderApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_header_object**](HeaderApi.md#create_header_object) | **POST** /service/{service_id}/version/{version_id}/header | Create a Header object
[**delete_header_object**](HeaderApi.md#delete_header_object) | **DELETE** /service/{service_id}/version/{version_id}/header/{header_name} | Delete a Header object
[**get_header_object**](HeaderApi.md#get_header_object) | **GET** /service/{service_id}/version/{version_id}/header/{header_name} | Get a Header object
[**list_header_objects**](HeaderApi.md#list_header_objects) | **GET** /service/{service_id}/version/{version_id}/header | List Header objects
[**update_header_object**](HeaderApi.md#update_header_object) | **PUT** /service/{service_id}/version/{version_id}/header/{header_name} | Update a Header object



## create_header_object

Creates a new Header object.

```rust
let cfg = &Configuration::default();
let params = CreateHeaderObjectParams {
    // parameters
};
create_header_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**action** | Option\<**String**> | Accepts a string value. |  |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**dst** | Option\<**String**> | Header to set. |  |
**ignore_if_set** | Option\<**i32**> | Don't add the header if it is added already. Only applies to 'set' action. |  |
**name** | Option\<**String**> | A handle to refer to this Header object. |  |
**priority** | Option\<**i32**> | Priority determines execution order. Lower numbers execute first. |  |[default to 100]
**regex** | Option\<**String**> | Regular expression to use. Only applies to `regex` and `regex_repeat` actions. |  |
**request_condition** | Option\<**String**> | Condition which, if met, will select this configuration during a request. Optional. |  |
**response_condition** | Option\<**String**> | Optional name of a response condition to apply. |  |
**src** | Option\<**String**> | Variable to be used as a source for the header content. Does not apply to `delete` action. |  |
**substitution** | Option\<**String**> | Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions. |  |
**_type** | Option\<**String**> | Accepts a string value. |  |

### Return type

[**crate::models::HeaderResponse**](HeaderResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_header_object

Deletes a Header object by name.

```rust
let cfg = &Configuration::default();
let params = DeleteHeaderObjectParams {
    // parameters
};
delete_header_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**header_name** | **String** | A handle to refer to this Header object. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_header_object

Retrieves a Header object by name.

```rust
let cfg = &Configuration::default();
let params = GetHeaderObjectParams {
    // parameters
};
get_header_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**header_name** | **String** | A handle to refer to this Header object. | [required] |

### Return type

[**crate::models::HeaderResponse**](HeaderResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_header_objects

Retrieves all Header objects for a particular Version of a Service.

```rust
let cfg = &Configuration::default();
let params = ListHeaderObjectsParams {
    // parameters
};
list_header_objects(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::HeaderResponse&gt;**](HeaderResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_header_object

Modifies an existing Header object by name.

```rust
let cfg = &Configuration::default();
let params = UpdateHeaderObjectParams {
    // parameters
};
update_header_object(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**header_name** | **String** | A handle to refer to this Header object. | [required] |
**action** | Option\<**String**> | Accepts a string value. |  |
**cache_condition** | Option\<**String**> | Name of the cache condition controlling when this configuration applies. |  |
**dst** | Option\<**String**> | Header to set. |  |
**ignore_if_set** | Option\<**i32**> | Don't add the header if it is added already. Only applies to 'set' action. |  |
**name** | Option\<**String**> | A handle to refer to this Header object. |  |
**priority** | Option\<**i32**> | Priority determines execution order. Lower numbers execute first. |  |[default to 100]
**regex** | Option\<**String**> | Regular expression to use. Only applies to `regex` and `regex_repeat` actions. |  |
**request_condition** | Option\<**String**> | Condition which, if met, will select this configuration during a request. Optional. |  |
**response_condition** | Option\<**String**> | Optional name of a response condition to apply. |  |
**src** | Option\<**String**> | Variable to be used as a source for the header content. Does not apply to `delete` action. |  |
**substitution** | Option\<**String**> | Value to substitute in place of regular expression. Only applies to `regex` and `regex_repeat` actions. |  |
**_type** | Option\<**String**> | Accepts a string value. |  |

### Return type

[**crate::models::HeaderResponse**](HeaderResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

