# LoggingS3Api

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_aws_s3**](LoggingS3Api.md#create_log_aws_s3) | **POST** /service/{service_id}/version/{version_id}/logging/s3 | Create an AWS S3 log endpoint
[**delete_log_aws_s3**](LoggingS3Api.md#delete_log_aws_s3) | **DELETE** /service/{service_id}/version/{version_id}/logging/s3/{logging_s3_name} | Delete an AWS S3 log endpoint
[**get_log_aws_s3**](LoggingS3Api.md#get_log_aws_s3) | **GET** /service/{service_id}/version/{version_id}/logging/s3/{logging_s3_name} | Get an AWS S3 log endpoint
[**list_log_aws_s3**](LoggingS3Api.md#list_log_aws_s3) | **GET** /service/{service_id}/version/{version_id}/logging/s3 | List AWS S3 log endpoints
[**update_log_aws_s3**](LoggingS3Api.md#update_log_aws_s3) | **PUT** /service/{service_id}/version/{version_id}/logging/s3/{logging_s3_name} | Update an AWS S3 log endpoint



## create_log_aws_s3

Create a S3 for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogAwsS3Params {
    // parameters
};
create_log_aws_s3(cfg, params)
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
**access_key** | Option\<**String**> | The access key for your S3 account. Not required if `iam_role` is provided. |  |
**acl** | Option\<**String**> | The access control list (ACL) specific request header. See the AWS documentation for [Access Control List (ACL) Specific Request Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/mpUploadInitiate.html#initiate-mpu-acl-specific-request-headers) for more information. |  |
**bucket_name** | Option\<**String**> | The bucket name for S3 account. |  |
**domain** | Option\<**String**> | The domain of the Amazon S3 endpoint. |  |
**iam_role** | Option\<**String**> | The Amazon Resource Name (ARN) for the IAM role granting Fastly access to S3. Not required if `access_key` and `secret_key` are provided. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**redundancy** | Option\<**String**> | The S3 redundancy level. |  |[default to null]
**secret_key** | Option\<**String**> | The secret key for your S3 account. Not required if `iam_role` is provided. |  |
**server_side_encryption_kms_key_id** | Option\<**String**> | Optional server-side KMS Key Id. Must be set if `server_side_encryption` is set to `aws:kms` or `AES256`. |  |[default to null]
**server_side_encryption** | Option\<**String**> | Set this to `AES256` or `aws:kms` to enable S3 Server Side Encryption. |  |[default to null]

### Return type

[**crate::models::LoggingS3Response**](LoggingS3Response.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_aws_s3

Delete the S3 for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogAwsS3Params {
    // parameters
};
delete_log_aws_s3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_s3_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_aws_s3

Get the S3 for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogAwsS3Params {
    // parameters
};
get_log_aws_s3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_s3_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingS3Response**](LoggingS3Response.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_aws_s3

List all of the S3s for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogAwsS3Params {
    // parameters
};
list_log_aws_s3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingS3Response&gt;**](LoggingS3Response.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_aws_s3

Update the S3 for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogAwsS3Params {
    // parameters
};
update_log_aws_s3(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_s3_name** | **String** | The name for the real-time logging configuration. | [required] |
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
**access_key** | Option\<**String**> | The access key for your S3 account. Not required if `iam_role` is provided. |  |
**acl** | Option\<**String**> | The access control list (ACL) specific request header. See the AWS documentation for [Access Control List (ACL) Specific Request Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/mpUploadInitiate.html#initiate-mpu-acl-specific-request-headers) for more information. |  |
**bucket_name** | Option\<**String**> | The bucket name for S3 account. |  |
**domain** | Option\<**String**> | The domain of the Amazon S3 endpoint. |  |
**iam_role** | Option\<**String**> | The Amazon Resource Name (ARN) for the IAM role granting Fastly access to S3. Not required if `access_key` and `secret_key` are provided. |  |
**path** | Option\<**String**> | The path to upload logs to. |  |[default to null]
**public_key** | Option\<**String**> | A PGP public key that Fastly will use to encrypt your log files before writing them to disk. |  |[default to null]
**redundancy** | Option\<**String**> | The S3 redundancy level. |  |[default to null]
**secret_key** | Option\<**String**> | The secret key for your S3 account. Not required if `iam_role` is provided. |  |
**server_side_encryption_kms_key_id** | Option\<**String**> | Optional server-side KMS Key Id. Must be set if `server_side_encryption` is set to `aws:kms` or `AES256`. |  |[default to null]
**server_side_encryption** | Option\<**String**> | Set this to `AES256` or `aws:kms` to enable S3 Server Side Encryption. |  |[default to null]

### Return type

[**crate::models::LoggingS3Response**](LoggingS3Response.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

