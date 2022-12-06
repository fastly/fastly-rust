# Http3Api

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_http3**](Http3Api.md#create_http3) | **POST** /service/{service_id}/version/{version_id}/http3 | Enable support for HTTP/3
[**delete_http3**](Http3Api.md#delete_http3) | **DELETE** /service/{service_id}/version/{version_id}/http3 | Disable support for HTTP/3
[**get_http3**](Http3Api.md#get_http3) | **GET** /service/{service_id}/version/{version_id}/http3 | Get HTTP/3 status



## create_http3

Enable HTTP/3 (QUIC) support for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateHttp3Params {
    // parameters
};
create_http3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**service_id2** | Option\<**String**> |  |  |
**version** | Option\<**i32**> |  |  |
**created_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**deleted_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**updated_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**feature_revision** | Option\<**i32**> | Revision number of the HTTP/3 feature implementation. Defaults to the most recent revision. |  |

### Return type

[**crate::models::Http3**](Http3.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_http3

Disable HTTP/3 (QUIC) support for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteHttp3Params {
    // parameters
};
delete_http3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_http3

Get the status of HTTP/3 (QUIC) support for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetHttp3Params {
    // parameters
};
get_http3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::Http3**](Http3.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

