# LoggingHttpsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_https**](LoggingHttpsApi.md#create_log_https) | **POST** /service/{service_id}/version/{version_id}/logging/https | Create an HTTPS log endpoint
[**delete_log_https**](LoggingHttpsApi.md#delete_log_https) | **DELETE** /service/{service_id}/version/{version_id}/logging/https/{logging_https_name} | Delete an HTTPS log endpoint
[**get_log_https**](LoggingHttpsApi.md#get_log_https) | **GET** /service/{service_id}/version/{version_id}/logging/https/{logging_https_name} | Get an HTTPS log endpoint
[**list_log_https**](LoggingHttpsApi.md#list_log_https) | **GET** /service/{service_id}/version/{version_id}/logging/https | List HTTPS log endpoints
[**update_log_https**](LoggingHttpsApi.md#update_log_https) | **PUT** /service/{service_id}/version/{version_id}/logging/https/{logging_https_name} | Update an HTTPS log endpoint



## create_log_https

Create an HTTPS object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogHttpsParams {
    // parameters
};
create_log_https(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` (10k). |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (100MB). |  |[default to 0]
**url** | Option\<**String**> | The URL to send logs to. Must use HTTPS. Required. |  |
**content_type** | Option\<**String**> | Content type of the header sent with the request. |  |[default to null]
**header_name** | Option\<**String**> | Name of the custom header sent with the request. |  |[default to null]
**message_type** | Option\<[**crate::models::LoggingMessageType**](logging_message_type.md)> |  |  |
**header_value** | Option\<**String**> | Value of the custom header sent with the request. |  |[default to null]
**method** | Option\<**String**> | HTTP method used for request. |  |[default to POST]
**json_format** | Option\<**String**> | Enforces valid JSON formatting for log entries. |  |

### Return type

[**crate::models::LoggingHttpsResponse**](LoggingHttpsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_https

Delete the HTTPS object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogHttpsParams {
    // parameters
};
delete_log_https(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_https_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_https

Get the HTTPS object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogHttpsParams {
    // parameters
};
get_log_https(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_https_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingHttpsResponse**](LoggingHttpsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_https

List all of the HTTPS objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogHttpsParams {
    // parameters
};
list_log_https(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingHttpsResponse&gt;**](LoggingHttpsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_https

Update the HTTPS object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogHttpsParams {
    // parameters
};
update_log_https(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_https_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` (10k). |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (100MB). |  |[default to 0]
**url** | Option\<**String**> | The URL to send logs to. Must use HTTPS. Required. |  |
**content_type** | Option\<**String**> | Content type of the header sent with the request. |  |[default to null]
**header_name** | Option\<**String**> | Name of the custom header sent with the request. |  |[default to null]
**message_type** | Option\<[**crate::models::LoggingMessageType**](logging_message_type.md)> |  |  |
**header_value** | Option\<**String**> | Value of the custom header sent with the request. |  |[default to null]
**method** | Option\<**String**> | HTTP method used for request. |  |[default to Method_POST]
**json_format** | Option\<**String**> | Enforces valid JSON formatting for log entries. |  |

### Return type

[**crate::models::LoggingHttpsResponse**](LoggingHttpsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

