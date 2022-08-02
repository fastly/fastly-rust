# LoggingSplunkApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_splunk**](LoggingSplunkApi.md#create_log_splunk) | **POST** /service/{service_id}/version/{version_id}/logging/splunk | Create a Splunk log endpoint
[**delete_log_splunk**](LoggingSplunkApi.md#delete_log_splunk) | **DELETE** /service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name} | Delete a Splunk log endpoint
[**get_log_splunk**](LoggingSplunkApi.md#get_log_splunk) | **GET** /service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name} | Get a Splunk log endpoint
[**list_log_splunk**](LoggingSplunkApi.md#list_log_splunk) | **GET** /service/{service_id}/version/{version_id}/logging/splunk | List Splunk log endpoints
[**update_log_splunk**](LoggingSplunkApi.md#update_log_splunk) | **PUT** /service/{service_id}/version/{version_id}/logging/splunk/{logging_splunk_name} | Update a Splunk log endpoint



## create_log_splunk

Create a Splunk logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogSplunkParams {
    // parameters
};
create_log_splunk(cfg, params)
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
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**url** | Option\<**String**> | The URL to post logs to. |  |
**token** | Option\<**String**> | A Splunk token for use in posting logs over HTTP to your collector. |  |
**use_tls** | Option\<[**crate::models::LoggingUseTls**](logging_use_tls.md)> |  |  |

### Return type

[**crate::models::LoggingSplunkResponse**](LoggingSplunkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_splunk

Delete the Splunk logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogSplunkParams {
    // parameters
};
delete_log_splunk(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_splunk_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_splunk

Get the details for a Splunk logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogSplunkParams {
    // parameters
};
get_log_splunk(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_splunk_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingSplunkResponse**](LoggingSplunkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_splunk

List all of the Splunk logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogSplunkParams {
    // parameters
};
list_log_splunk(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingSplunkResponse&gt;**](LoggingSplunkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_splunk

Update the Splunk logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogSplunkParams {
    // parameters
};
update_log_splunk(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_splunk_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**url** | Option\<**String**> | The URL to post logs to. |  |
**token** | Option\<**String**> | A Splunk token for use in posting logs over HTTP to your collector. |  |
**use_tls** | Option\<[**crate::models::LoggingUseTls**](logging_use_tls.md)> |  |  |

### Return type

[**crate::models::LoggingSplunkResponse**](LoggingSplunkResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

