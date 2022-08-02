# LoggingAzureblobApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_azure**](LoggingAzureblobApi.md#create_log_azure) | **POST** /service/{service_id}/version/{version_id}/logging/azureblob | Create an Azure Blob Storage log endpoint
[**delete_log_azure**](LoggingAzureblobApi.md#delete_log_azure) | **DELETE** /service/{service_id}/version/{version_id}/logging/azureblob/{logging_azureblob_name} | Delete the Azure Blob Storage log endpoint
[**get_log_azure**](LoggingAzureblobApi.md#get_log_azure) | **GET** /service/{service_id}/version/{version_id}/logging/azureblob/{logging_azureblob_name} | Get an Azure Blob Storage log endpoint
[**list_log_azure**](LoggingAzureblobApi.md#list_log_azure) | **GET** /service/{service_id}/version/{version_id}/logging/azureblob | List Azure Blob Storage log endpoints
[**update_log_azure**](LoggingAzureblobApi.md#update_log_azure) | **PUT** /service/{service_id}/version/{version_id}/logging/azureblob/{logging_azureblob_name} | Update an Azure Blob Storage log endpoint



## create_log_azure

Create an Azure Blob Storage logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogAzureParams {
    // parameters
};
create_log_azure(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to 2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**message_type** | Option\<**String**> | How the message should be formatted. |  |[default to classic]
**timestamp_format** | Option\<**String**> | A timestamp format |  |
**period** | Option\<**i32**> | How frequently log files are finalized so they can be available for reading (in seconds). |  |[default to 3600]
**gzip_level** | Option\<**i32**> | The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |[default to 0]
**compression_codec** | Option\<**String**> | The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**account_name** | Option\<**String**> | The unique Azure Blob Storage namespace in which your data objects are stored. Required. |  |
**container** | Option\<**String**> | The name of the Azure Blob Storage container in which to store logs. Required. |  |
**sas_token** | Option\<**String**> | The Azure shared access signature providing write access to the blob service objects. Be sure to update your token before it expires or the logging functionality will not work. Required. |  |
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**file_max_bytes** | Option\<**i32**> | The maximum number of bytes for each uploaded file. A value of 0 can be used to indicate there is no limit on the size of uploaded files, otherwise the minimum value is 1048576 bytes (1 MiB.) |  |

### Return type

[**crate::models::LoggingAzureblobResponse**](LoggingAzureblobResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_azure

Delete the Azure Blob Storage logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogAzureParams {
    // parameters
};
delete_log_azure(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_azureblob_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_azure

Get the Azure Blob Storage logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogAzureParams {
    // parameters
};
get_log_azure(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_azureblob_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingAzureblobResponse**](LoggingAzureblobResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_azure

List all of the Azure Blob Storage logging endpoints for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogAzureParams {
    // parameters
};
list_log_azure(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingAzureblobResponse&gt;**](LoggingAzureblobResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_azure

Update the Azure Blob Storage logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogAzureParams {
    // parameters
};
update_log_azure(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_azureblob_name** | **String** | The name for the real-time logging configuration. | [required] |
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
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**account_name** | Option\<**String**> | The unique Azure Blob Storage namespace in which your data objects are stored. Required. |  |
**container** | Option\<**String**> | The name of the Azure Blob Storage container in which to store logs. Required. |  |
**sas_token** | Option\<**String**> | The Azure shared access signature providing write access to the blob service objects. Be sure to update your token before it expires or the logging functionality will not work. Required. |  |
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**file_max_bytes** | Option\<**i32**> | The maximum number of bytes for each uploaded file. A value of 0 can be used to indicate there is no limit on the size of uploaded files, otherwise the minimum value is 1048576 bytes (1 MiB.) |  |

### Return type

[**crate::models::LoggingAzureblobResponse**](LoggingAzureblobResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

