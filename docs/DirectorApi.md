# DirectorApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_director**](DirectorApi.md#delete_director) | **DELETE** /service/{service_id}/version/{version_id}/director/{director_name} | Delete a director
[**get_director**](DirectorApi.md#get_director) | **GET** /service/{service_id}/version/{version_id}/director/{director_name} | Get a director
[**list_directors**](DirectorApi.md#list_directors) | **GET** /service/{service_id}/version/{version_id}/director | List directors
[**update_director**](DirectorApi.md#update_director) | **PUT** /service/{service_id}/version/{version_id}/director/{director_name} | Update a director



## delete_director

Delete the director for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteDirectorParams {
    // parameters
};
delete_director(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**director_name** | **String** | Name for the Director. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_director

Get the director for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetDirectorParams {
    // parameters
};
get_director(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**director_name** | **String** | Name for the Director. | [required] |

### Return type

[**crate::models::DirectorResponse**](DirectorResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_directors

List the directors for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListDirectorsParams {
    // parameters
};
list_directors(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::DirectorResponse&gt;**](DirectorResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_director

Update the director for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateDirectorParams {
    // parameters
};
update_director(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**director_name** | **String** | Name for the Director. | [required] |

### Return type

[**crate::models::DirectorResponse**](DirectorResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

