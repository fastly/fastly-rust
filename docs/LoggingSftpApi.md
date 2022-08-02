# LoggingSftpApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_sftp**](LoggingSftpApi.md#create_log_sftp) | **POST** /service/{service_id}/version/{version_id}/logging/sftp | Create an SFTP log endpoint
[**delete_log_sftp**](LoggingSftpApi.md#delete_log_sftp) | **DELETE** /service/{service_id}/version/{version_id}/logging/sftp/{logging_sftp_name} | Delete an SFTP log endpoint
[**get_log_sftp**](LoggingSftpApi.md#get_log_sftp) | **GET** /service/{service_id}/version/{version_id}/logging/sftp/{logging_sftp_name} | Get an SFTP log endpoint
[**list_log_sftp**](LoggingSftpApi.md#list_log_sftp) | **GET** /service/{service_id}/version/{version_id}/logging/sftp | List SFTP log endpoints
[**update_log_sftp**](LoggingSftpApi.md#update_log_sftp) | **PUT** /service/{service_id}/version/{version_id}/logging/sftp/{logging_sftp_name} | Update an SFTP log endpoint



## create_log_sftp

Create a SFTP for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogSftpParams {
    // parameters
};
create_log_sftp(cfg, params)
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
**message_type** | Option\<**String**> | How the message should be formatted. |  |[default to MessageType_Classic]
**timestamp_format** | Option\<**String**> | A timestamp format |  |
**period** | Option\<**i32**> | How frequently log files are finalized so they can be available for reading (in seconds). |  |[default to 3600]
**gzip_level** | Option\<**i32**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |[default to 0]
**compression_codec** | Option\<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |
**address** | Option\<**String**> | A hostname or IPv4 address. |  |
**port** | Option\<**i32**> | The port number. |  |[default to 22]
**password** | Option\<**String**> | The password for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**secret_key** | Option\<**String**> | The SSH private key for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. |  |[default to null]
**ssh_known_hosts** | Option\<**String**> | A list of host keys for all hosts we can connect to over SFTP. |  |
**user** | Option\<**String**> | The username for the server. |  |

### Return type

[**crate::models::LoggingSftpResponse**](LoggingSftpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_sftp

Delete the SFTP for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogSftpParams {
    // parameters
};
delete_log_sftp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_sftp_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_sftp

Get the SFTP for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogSftpParams {
    // parameters
};
get_log_sftp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_sftp_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingSftpResponse**](LoggingSftpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_sftp

List all of the SFTPs for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogSftpParams {
    // parameters
};
list_log_sftp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingSftpResponse&gt;**](LoggingSftpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_sftp

Update the SFTP for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogSftpParams {
    // parameters
};
update_log_sftp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_sftp_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**message_type** | Option\<**String**> | How the message should be formatted. |  |[default to MessageType_Classic]
**timestamp_format** | Option\<**String**> | A timestamp format |  |
**period** | Option\<**i32**> | How frequently log files are finalized so they can be available for reading (in seconds). |  |[default to 3600]
**gzip_level** | Option\<**i32**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |[default to 0]
**compression_codec** | Option\<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |
**address** | Option\<**String**> | A hostname or IPv4 address. |  |
**port** | Option\<**i32**> | The port number. |  |[default to 22]
**password** | Option\<**String**> | The password for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**secret_key** | Option\<**String**> | The SSH private key for the server. If both `password` and `secret_key` are passed, `secret_key` will be used in preference. |  |[default to null]
**ssh_known_hosts** | Option\<**String**> | A list of host keys for all hosts we can connect to over SFTP. |  |
**user** | Option\<**String**> | The username for the server. |  |

### Return type

[**crate::models::LoggingSftpResponse**](LoggingSftpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

