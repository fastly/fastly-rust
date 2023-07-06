# LoggingKafkaApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_kafka**](LoggingKafkaApi.md#create_log_kafka) | **POST** /service/{service_id}/version/{version_id}/logging/kafka | Create a Kafka log endpoint
[**delete_log_kafka**](LoggingKafkaApi.md#delete_log_kafka) | **DELETE** /service/{service_id}/version/{version_id}/logging/kafka/{logging_kafka_name} | Delete the Kafka log endpoint
[**get_log_kafka**](LoggingKafkaApi.md#get_log_kafka) | **GET** /service/{service_id}/version/{version_id}/logging/kafka/{logging_kafka_name} | Get a Kafka log endpoint
[**list_log_kafka**](LoggingKafkaApi.md#list_log_kafka) | **GET** /service/{service_id}/version/{version_id}/logging/kafka | List Kafka log endpoints
[**update_log_kafka**](LoggingKafkaApi.md#update_log_kafka) | **PUT** /service/{service_id}/version/{version_id}/logging/kafka/{logging_kafka_name} | Update the Kafka log endpoint



## create_log_kafka

Create a Kafka logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogKafkaParams {
    // parameters
};
create_log_kafka(cfg, params)
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
**topic** | Option\<**String**> | The Kafka topic to send logs to. Required. |  |
**brokers** | Option\<**String**> | A comma-separated list of IP addresses or hostnames of Kafka brokers. Required. |  |
**compression_codec** | Option\<**String**> | The codec used for compression of your logs. |  |
**required_acks** | Option\<**i32**> | The number of acknowledgements a leader must receive before a write is considered successful. |  |[default to 1]
**request_max_bytes** | Option\<**i32**> | The maximum number of bytes sent in one request. Defaults `0` (no limit). |  |[default to 0]
**parse_log_keyvals** | Option\<**bool**> | Enables parsing of key=value tuples from the beginning of a logline, turning them into [record headers](https://cwiki.apache.org/confluence/display/KAFKA/KIP-82+-+Add+Record+Headers). |  |
**auth_method** | Option\<**String**> | SASL authentication method. |  |
**user** | Option\<**String**> | SASL user. |  |
**password** | Option\<**String**> | SASL password. |  |
**use_tls** | Option\<[**crate::models::LoggingUseTls**](logging_use_tls.md)> |  |  |

### Return type

[**crate::models::LoggingKafkaResponse**](LoggingKafkaResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_kafka

Delete the Kafka logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogKafkaParams {
    // parameters
};
delete_log_kafka(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kafka_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_kafka

Get the Kafka logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogKafkaParams {
    // parameters
};
get_log_kafka(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kafka_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingKafkaResponse**](LoggingKafkaResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_kafka

List all of the Kafka logging endpoints for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogKafkaParams {
    // parameters
};
list_log_kafka(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingKafkaResponse&gt;**](LoggingKafkaResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_kafka

Update the Kafka logging endpoint for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogKafkaParams {
    // parameters
};
update_log_kafka(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_kafka_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingKafkaResponse**](LoggingKafkaResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

