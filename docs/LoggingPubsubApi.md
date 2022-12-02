# LoggingPubsubApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_gcp_pubsub**](LoggingPubsubApi.md#create_log_gcp_pubsub) | **POST** /service/{service_id}/version/{version_id}/logging/pubsub | Create a GCP Cloud Pub/Sub log endpoint
[**delete_log_gcp_pubsub**](LoggingPubsubApi.md#delete_log_gcp_pubsub) | **DELETE** /service/{service_id}/version/{version_id}/logging/pubsub/{logging_google_pubsub_name} | Delete a GCP Cloud Pub/Sub log endpoint
[**get_log_gcp_pubsub**](LoggingPubsubApi.md#get_log_gcp_pubsub) | **GET** /service/{service_id}/version/{version_id}/logging/pubsub/{logging_google_pubsub_name} | Get a GCP Cloud Pub/Sub log endpoint
[**list_log_gcp_pubsub**](LoggingPubsubApi.md#list_log_gcp_pubsub) | **GET** /service/{service_id}/version/{version_id}/logging/pubsub | List GCP Cloud Pub/Sub log endpoints
[**update_log_gcp_pubsub**](LoggingPubsubApi.md#update_log_gcp_pubsub) | **PUT** /service/{service_id}/version/{version_id}/logging/pubsub/{logging_google_pubsub_name} | Update a GCP Cloud Pub/Sub log endpoint



## create_log_gcp_pubsub

Create a Pub/Sub logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogGcpPubsubParams {
    // parameters
};
create_log_gcp_pubsub(cfg, params)
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
**user** | Option\<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**secret_key** | Option\<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**account_name** | Option\<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. |  |
**topic** | Option\<**String**> | The Google Cloud Pub/Sub topic to which logs will be published. Required. |  |
**project_id** | Option\<**String**> | Your Google Cloud Platform project ID. Required |  |

### Return type

[**crate::models::LoggingGooglePubsubResponse**](LoggingGooglePubsubResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_gcp_pubsub

Delete a Pub/Sub logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogGcpPubsubParams {
    // parameters
};
delete_log_gcp_pubsub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_google_pubsub_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_gcp_pubsub

Get the details for a Pub/Sub logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogGcpPubsubParams {
    // parameters
};
get_log_gcp_pubsub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_google_pubsub_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingGooglePubsubResponse**](LoggingGooglePubsubResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_gcp_pubsub

List all of the Pub/Sub logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogGcpPubsubParams {
    // parameters
};
list_log_gcp_pubsub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingGooglePubsubResponse&gt;**](LoggingGooglePubsubResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_gcp_pubsub

Update a Pub/Sub logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogGcpPubsubParams {
    // parameters
};
update_log_gcp_pubsub(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_google_pubsub_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to %h %l %u %t "%r" %&gt;s %b]
**user** | Option\<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**secret_key** | Option\<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**account_name** | Option\<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. |  |
**topic** | Option\<**String**> | The Google Cloud Pub/Sub topic to which logs will be published. Required. |  |
**project_id** | Option\<**String**> | Your Google Cloud Platform project ID. Required |  |

### Return type

[**crate::models::LoggingGooglePubsubResponse**](LoggingGooglePubsubResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

