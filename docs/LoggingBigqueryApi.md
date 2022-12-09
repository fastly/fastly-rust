# LoggingBigqueryApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_bigquery**](LoggingBigqueryApi.md#create_log_bigquery) | **POST** /service/{service_id}/version/{version_id}/logging/bigquery | Create a BigQuery log endpoint
[**delete_log_bigquery**](LoggingBigqueryApi.md#delete_log_bigquery) | **DELETE** /service/{service_id}/version/{version_id}/logging/bigquery/{logging_bigquery_name} | Delete a BigQuery log endpoint
[**get_log_bigquery**](LoggingBigqueryApi.md#get_log_bigquery) | **GET** /service/{service_id}/version/{version_id}/logging/bigquery/{logging_bigquery_name} | Get a BigQuery log endpoint
[**list_log_bigquery**](LoggingBigqueryApi.md#list_log_bigquery) | **GET** /service/{service_id}/version/{version_id}/logging/bigquery | List BigQuery log endpoints
[**update_log_bigquery**](LoggingBigqueryApi.md#update_log_bigquery) | **PUT** /service/{service_id}/version/{version_id}/logging/bigquery/{logging_bigquery_name} | Update a BigQuery log endpoint



## create_log_bigquery

Create a BigQuery logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogBigqueryParams {
    // parameters
};
create_log_bigquery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name of the BigQuery logging object. Used as a primary key for API access. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table. |  |
**user** | Option\<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**secret_key** | Option\<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**account_name** | Option\<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. |  |
**dataset** | Option\<**String**> | Your BigQuery dataset. |  |
**table** | Option\<**String**> | Your BigQuery table. |  |
**template_suffix** | Option\<**String**> | BigQuery table name suffix template. Optional. |  |
**project_id** | Option\<**String**> | Your Google Cloud Platform project ID. Required |  |

### Return type

[**crate::models::LoggingBigqueryResponse**](LoggingBigqueryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_bigquery

Delete a BigQuery logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogBigqueryParams {
    // parameters
};
delete_log_bigquery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_bigquery_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_bigquery

Get the details for a BigQuery logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogBigqueryParams {
    // parameters
};
get_log_bigquery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_bigquery_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingBigqueryResponse**](LoggingBigqueryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_bigquery

List all of the BigQuery logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogBigqueryParams {
    // parameters
};
list_log_bigquery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingBigqueryResponse&gt;**](LoggingBigqueryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_bigquery

Update a BigQuery logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogBigqueryParams {
    // parameters
};
update_log_bigquery(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_bigquery_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name of the BigQuery logging object. Used as a primary key for API access. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce JSON that matches the schema of your BigQuery table. |  |
**user** | Option\<**String**> | Your Google Cloud Platform service account email address. The `client_email` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**secret_key** | Option\<**String**> | Your Google Cloud Platform account secret key. The `private_key` field in your service account authentication JSON. Not required if `account_name` is specified. |  |
**account_name** | Option\<**String**> | The name of the Google Cloud Platform service account associated with the target log collection service. Not required if `user` and `secret_key` are provided. |  |
**dataset** | Option\<**String**> | Your BigQuery dataset. |  |
**table** | Option\<**String**> | Your BigQuery table. |  |
**template_suffix** | Option\<**String**> | BigQuery table name suffix template. Optional. |  |
**project_id** | Option\<**String**> | Your Google Cloud Platform project ID. Required |  |

### Return type

[**crate::models::LoggingBigqueryResponse**](LoggingBigqueryResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

