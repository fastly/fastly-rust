# LoggingKinesisApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_kinesis**](LoggingKinesisApi.md#create_log_kinesis) | **POST** /service/{service_id}/version/{version_id}/logging/kinesis | Create  an Amazon Kinesis log endpoint
[**delete_log_kinesis**](LoggingKinesisApi.md#delete_log_kinesis) | **DELETE** /service/{service_id}/version/{version_id}/logging/kinesis/{logging_kinesis_name} | Delete the Amazon Kinesis log endpoint
[**get_log_kinesis**](LoggingKinesisApi.md#get_log_kinesis) | **GET** /service/{service_id}/version/{version_id}/logging/kinesis/{logging_kinesis_name} | Get an Amazon Kinesis log endpoint
[**list_log_kinesis**](LoggingKinesisApi.md#list_log_kinesis) | **GET** /service/{service_id}/version/{version_id}/logging/kinesis | List Amazon Kinesis log endpoints
[**update_log_kinesis**](LoggingKinesisApi.md#update_log_kinesis) | **PUT** /service/{service_id}/version/{version_id}/logging/kinesis/{logging_kinesis_name} | Update the Amazon Kinesis log endpoint



## create_log_kinesis

Create an Amazon Kinesis Data Streams logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogKinesisParams {
    // parameters
};
create_log_kinesis(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<[**crate::models::LoggingPlacement**](logging_placement.md)> |  |  |
**format_version** | Option\<[**crate::models::LoggingFormatVersion**](logging_format_version.md)> |  |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Kinesis can ingest. |  |[default to {"timestamp":"%{begin:%Y-%m-%dT%H:%M:%S}t","time_elapsed":"%{time.elapsed.usec}V","is_tls":"%{if(req.is_ssl, \"true\", \"false\")}V","client_ip":"%{req.http.Fastly-Client-IP}V","geo_city":"%{client.geo.city}V","geo_country_code":"%{client.geo.country_code}V","request":"%{req.request}V","host":"%{req.http.Fastly-Orig-Host}V","url":"%{json.escape(req.url)}V","request_referer":"%{json.escape(req.http.Referer)}V","request_user_agent":"%{json.escape(req.http.User-Agent)}V","request_accept_language":"%{json.escape(req.http.Accept-Language)}V","request_accept_charset":"%{json.escape(req.http.Accept-Charset)}V","cache_status":"%{regsub(fastly_info.state, \"^(HIT-(SYNTH)|(HITPASS|HIT|MISS|PASS|ERROR|PIPE)).*\", \"\\2\\3\") }V"}]
**topic** | Option\<**String**> | The Amazon Kinesis stream to send logs to. Required. |  |
**region** | Option\<[**crate::models::AwsRegion**](aws_region.md)> |  |  |
**secret_key** | Option\<**String**> | The secret key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified. |  |
**access_key** | Option\<**String**> | The access key associated with the target Amazon Kinesis stream. Not required if `iam_role` is specified. |  |
**iam_role** | Option\<**String**> | The ARN for an IAM role granting Fastly access to the target Amazon Kinesis stream. Not required if `access_key` and `secret_key` are provided. |  |

### Return type

[**crate::models::LoggingKinesisResponse**](LoggingKinesisResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_kinesis

Delete an Amazon Kinesis Data Streams logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogKinesisParams {
    // parameters
};
delete_log_kinesis(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kinesis_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_kinesis

Get the details for an Amazon Kinesis Data Streams logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogKinesisParams {
    // parameters
};
get_log_kinesis(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kinesis_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingKinesisResponse**](LoggingKinesisResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_kinesis

List all of the Amazon Kinesis Data Streams logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogKinesisParams {
    // parameters
};
list_log_kinesis(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingKinesisResponse&gt;**](LoggingKinesisResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_kinesis

Update an Amazon Kinesis Data Streams logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogKinesisParams {
    // parameters
};
update_log_kinesis(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kinesis_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingKinesisResponse**](LoggingKinesisResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

