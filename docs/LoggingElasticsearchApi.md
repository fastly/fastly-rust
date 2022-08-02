# LoggingElasticsearchApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_elasticsearch**](LoggingElasticsearchApi.md#create_log_elasticsearch) | **POST** /service/{service_id}/version/{version_id}/logging/elasticsearch | Create an Elasticsearch log endpoint
[**delete_log_elasticsearch**](LoggingElasticsearchApi.md#delete_log_elasticsearch) | **DELETE** /service/{service_id}/version/{version_id}/logging/elasticsearch/{logging_elasticsearch_name} | Delete an Elasticsearch log endpoint
[**get_log_elasticsearch**](LoggingElasticsearchApi.md#get_log_elasticsearch) | **GET** /service/{service_id}/version/{version_id}/logging/elasticsearch/{logging_elasticsearch_name} | Get an Elasticsearch log endpoint
[**list_log_elasticsearch**](LoggingElasticsearchApi.md#list_log_elasticsearch) | **GET** /service/{service_id}/version/{version_id}/logging/elasticsearch | List Elasticsearch log endpoints
[**update_log_elasticsearch**](LoggingElasticsearchApi.md#update_log_elasticsearch) | **PUT** /service/{service_id}/version/{version_id}/logging/elasticsearch/{logging_elasticsearch_name} | Update an Elasticsearch log endpoint



## create_log_elasticsearch

Create a Elasticsearch logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogElasticsearchParams {
    // parameters
};
create_log_elasticsearch(cfg, params)
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
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Elasticsearch can ingest. |  |
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**index** | Option\<**String**> | The name of the Elasticsearch index to send documents (logs) to. The index must follow the Elasticsearch [index format rules](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html). We support [strftime](https://www.man7.org/linux/man-pages/man3/strftime.3.html) interpolated variables inside braces prefixed with a pound symbol. For example, `#{%F}` will interpolate as `YYYY-MM-DD` with today's date. |  |
**url** | Option\<**String**> | The URL to stream logs to. Must use HTTPS. |  |
**pipeline** | Option\<**String**> | The ID of the Elasticsearch ingest pipeline to apply pre-process transformations to before indexing. Learn more about creating a pipeline in the [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/ingest.html). |  |
**user** | Option\<**String**> | Basic Auth username. |  |
**password** | Option\<**String**> | Basic Auth password. |  |

### Return type

[**crate::models::LoggingElasticsearchResponse**](LoggingElasticsearchResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_elasticsearch

Delete the Elasticsearch logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogElasticsearchParams {
    // parameters
};
delete_log_elasticsearch(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_elasticsearch_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_elasticsearch

Get the Elasticsearch logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogElasticsearchParams {
    // parameters
};
get_log_elasticsearch(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_elasticsearch_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingElasticsearchResponse**](LoggingElasticsearchResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_elasticsearch

List all of the Elasticsearch logging endpoints for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogElasticsearchParams {
    // parameters
};
list_log_elasticsearch(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingElasticsearchResponse&gt;**](LoggingElasticsearchResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_elasticsearch

Update the Elasticsearch logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogElasticsearchParams {
    // parameters
};
update_log_elasticsearch(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_elasticsearch_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). Must produce valid JSON that Elasticsearch can ingest. |  |
**tls_ca_cert** | Option\<**String**> | A secure certificate to authenticate a server with. Must be in PEM format. |  |[default to null]
**tls_client_cert** | Option\<**String**> | The client certificate used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_client_key** | Option\<**String**> | The client private key used to make authenticated requests. Must be in PEM format. |  |[default to null]
**tls_hostname** | Option\<**String**> | The hostname to verify the server's certificate. This should be one of the Subject Alternative Name (SAN) fields for the certificate. Common Names (CN) are not supported. |  |[default to null]
**request_max_entries** | Option\<**i32**> | The maximum number of logs sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` for unbounded. |  |[default to 0]
**index** | Option\<**String**> | The name of the Elasticsearch index to send documents (logs) to. The index must follow the Elasticsearch [index format rules](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html). We support [strftime](https://www.man7.org/linux/man-pages/man3/strftime.3.html) interpolated variables inside braces prefixed with a pound symbol. For example, `#{%F}` will interpolate as `YYYY-MM-DD` with today's date. |  |
**url** | Option\<**String**> | The URL to stream logs to. Must use HTTPS. |  |
**pipeline** | Option\<**String**> | The ID of the Elasticsearch ingest pipeline to apply pre-process transformations to before indexing. Learn more about creating a pipeline in the [Elasticsearch docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/ingest.html). |  |
**user** | Option\<**String**> | Basic Auth username. |  |
**password** | Option\<**String**> | Basic Auth password. |  |

### Return type

[**crate::models::LoggingElasticsearchResponse**](LoggingElasticsearchResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

