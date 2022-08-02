# LoggingSyslogApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_syslog**](LoggingSyslogApi.md#create_log_syslog) | **POST** /service/{service_id}/version/{version_id}/logging/syslog | Create a syslog log endpoint
[**delete_log_syslog**](LoggingSyslogApi.md#delete_log_syslog) | **DELETE** /service/{service_id}/version/{version_id}/logging/syslog/{logging_syslog_name} | Delete a syslog log endpoint
[**get_log_syslog**](LoggingSyslogApi.md#get_log_syslog) | **GET** /service/{service_id}/version/{version_id}/logging/syslog/{logging_syslog_name} | Get a syslog log endpoint
[**list_log_syslog**](LoggingSyslogApi.md#list_log_syslog) | **GET** /service/{service_id}/version/{version_id}/logging/syslog | List Syslog log endpoints
[**update_log_syslog**](LoggingSyslogApi.md#update_log_syslog) | **PUT** /service/{service_id}/version/{version_id}/logging/syslog/{logging_syslog_name} | Update a syslog log endpoint



## create_log_syslog

Create a Syslog for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogSyslogParams {
    // parameters
};
create_log_syslog(cfg, params)
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
**address** | Option\<**String**> | A hostname or IPv4 address. |  |
**port** | Option\<**i32**> | The port number. |  |[default to 514]
**message_type** | Option\<[**crate::models::LoggingMessageType**](logging_message_type.md)> |  |  |
**hostname** | Option\<**String**> | The hostname used for the syslog endpoint. |  |
**ipv4** | Option\<**String**> | The IPv4 address used for the syslog endpoint. |  |
**token** | Option\<**String**> | Whether to prepend each message with a specific token. |  |[default to null]
**use_tls** | Option\<[**crate::models::LoggingUseTls**](logging_use_tls.md)> |  |  |

### Return type

[**crate::models::LoggingSyslogResponse**](LoggingSyslogResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_syslog

Delete the Syslog for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogSyslogParams {
    // parameters
};
delete_log_syslog(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_syslog_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_syslog

Get the Syslog for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogSyslogParams {
    // parameters
};
get_log_syslog(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_syslog_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingSyslogResponse**](LoggingSyslogResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_syslog

List all of the Syslogs for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogSyslogParams {
    // parameters
};
list_log_syslog(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingSyslogResponse&gt;**](LoggingSyslogResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_syslog

Update the Syslog for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogSyslogParams {
    // parameters
};
update_log_syslog(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_syslog_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**address** | Option\<**String**> | A hostname or IPv4 address. |  |
**port** | Option\<**i32**> | The port number. |  |[default to 514]
**message_type** | Option\<[**crate::models::LoggingMessageType**](logging_message_type.md)> |  |  |
**hostname** | Option\<**String**> | The hostname used for the syslog endpoint. |  |
**ipv4** | Option\<**String**> | The IPv4 address used for the syslog endpoint. |  |
**token** | Option\<**String**> | Whether to prepend each message with a specific token. |  |[default to null]
**use_tls** | Option\<[**crate::models::LoggingUseTls**](logging_use_tls.md)> |  |  |

### Return type

[**crate::models::LoggingSyslogResponse**](LoggingSyslogResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

