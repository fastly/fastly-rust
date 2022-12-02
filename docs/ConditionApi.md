# ConditionApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_condition**](ConditionApi.md#create_condition) | **POST** /service/{service_id}/version/{version_id}/condition | Create a condition
[**delete_condition**](ConditionApi.md#delete_condition) | **DELETE** /service/{service_id}/version/{version_id}/condition/{condition_name} | Delete a condition
[**get_condition**](ConditionApi.md#get_condition) | **GET** /service/{service_id}/version/{version_id}/condition/{condition_name} | Describe a condition
[**list_conditions**](ConditionApi.md#list_conditions) | **GET** /service/{service_id}/version/{version_id}/condition | List conditions
[**update_condition**](ConditionApi.md#update_condition) | **PUT** /service/{service_id}/version/{version_id}/condition/{condition_name} | Update a condition



## create_condition

Creates a new condition.

```rust
let cfg = &Configuration::default();
let params = CreateConditionParams {
    // parameters
};
create_condition(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | Name of the condition. Required. |  |
**priority** | Option\<**String**> | A numeric string. Priority determines execution order. Lower numbers execute first. |  |[default to 100]
**statement** | Option\<**String**> | A conditional expression in VCL used to determine if the condition is met. |  |
**service_id2** | Option\<**String**> |  |  |
**version** | Option\<**String**> | A numeric string that represents the service version. |  |
**_type** | Option\<**String**> | Type of the condition. Required. |  |

### Return type

[**crate::models::ConditionResponse**](ConditionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_condition

Deletes the specified condition.

```rust
let cfg = &Configuration::default();
let params = DeleteConditionParams {
    // parameters
};
delete_condition(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**condition_name** | **String** | Name of the condition. Required. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_condition

Gets the specified condition.

```rust
let cfg = &Configuration::default();
let params = GetConditionParams {
    // parameters
};
get_condition(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**condition_name** | **String** | Name of the condition. Required. | [required] |

### Return type

[**crate::models::ConditionResponse**](ConditionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_conditions

Gets all conditions for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListConditionsParams {
    // parameters
};
list_conditions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::ConditionResponse&gt;**](ConditionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_condition

Updates the specified condition.

```rust
let cfg = &Configuration::default();
let params = UpdateConditionParams {
    // parameters
};
update_condition(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**condition_name** | **String** | Name of the condition. Required. | [required] |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | Name of the condition. Required. |  |
**priority** | Option\<**String**> | A numeric string. Priority determines execution order. Lower numbers execute first. |  |[default to 100]
**statement** | Option\<**String**> | A conditional expression in VCL used to determine if the condition is met. |  |
**service_id2** | Option\<**String**> |  |  |
**version** | Option\<**String**> | A numeric string that represents the service version. |  |
**_type** | Option\<**String**> | Type of the condition. Required. |  |

### Return type

[**crate::models::ConditionResponse**](ConditionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

