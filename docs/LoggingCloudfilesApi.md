# LoggingCloudfilesApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_cloudfiles**](LoggingCloudfilesApi.md#create_log_cloudfiles) | **POST** /service/{service_id}/version/{version_id}/logging/cloudfiles | Create a Cloud Files log endpoint
[**delete_log_cloudfiles**](LoggingCloudfilesApi.md#delete_log_cloudfiles) | **DELETE** /service/{service_id}/version/{version_id}/logging/cloudfiles/{logging_cloudfiles_name} | Delete the Cloud Files log endpoint
[**get_log_cloudfiles**](LoggingCloudfilesApi.md#get_log_cloudfiles) | **GET** /service/{service_id}/version/{version_id}/logging/cloudfiles/{logging_cloudfiles_name} | Get a Cloud Files log endpoint
[**list_log_cloudfiles**](LoggingCloudfilesApi.md#list_log_cloudfiles) | **GET** /service/{service_id}/version/{version_id}/logging/cloudfiles | List Cloud Files log endpoints
[**update_log_cloudfiles**](LoggingCloudfilesApi.md#update_log_cloudfiles) | **PUT** /service/{service_id}/version/{version_id}/logging/cloudfiles/{logging_cloudfiles_name} | Update the Cloud Files log endpoint



## create_log_cloudfiles

Create a Cloud Files log endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogCloudfilesParams {
    // parameters
};
create_log_cloudfiles(cfg, params)
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
**access_key** | Option\<**String**> | Your Cloud Files account access key. |  |
**bucket_name** | Option\<**String**> | The name of your Cloud Files container. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**region** | Option\<**String**> | The region to stream logs to. |  |
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**user** | Option\<**String**> | The username for your Cloud Files account. |  |

### Return type

[**crate::models::LoggingCloudfilesResponse**](LoggingCloudfilesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_cloudfiles

Delete the Cloud Files log endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogCloudfilesParams {
    // parameters
};
delete_log_cloudfiles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_cloudfiles_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_cloudfiles

Get the Cloud Files log endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogCloudfilesParams {
    // parameters
};
get_log_cloudfiles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_cloudfiles_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingCloudfilesResponse**](LoggingCloudfilesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_cloudfiles

List all of the Cloud Files log endpoints for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogCloudfilesParams {
    // parameters
};
list_log_cloudfiles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingCloudfilesResponse&gt;**](LoggingCloudfilesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_cloudfiles

Update the Cloud Files log endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogCloudfilesParams {
    // parameters
};
update_log_cloudfiles(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_cloudfiles_name** | **String** | The name for the real-time logging configuration. | [required] |
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
**access_key** | Option\<**String**> | Your Cloud Files account access key. |  |
**bucket_name** | Option\<**String**> | The name of your Cloud Files container. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**region** | Option\<**String**> | The region to stream logs to. |  |
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**user** | Option\<**String**> | The username for your Cloud Files account. |  |

### Return type

[**crate::models::LoggingCloudfilesResponse**](LoggingCloudfilesResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

