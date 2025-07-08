# LoggingGrafanacloudlogsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_log_grafanacloudlogs**](LoggingGrafanacloudlogsApi.md#create_log_grafanacloudlogs) | **POST** /service/{service_id}/version/{version_id}/logging/grafanacloudlogs | Create a Grafana Cloud Logs log endpoint
[**delete_log_grafanacloudlogs**](LoggingGrafanacloudlogsApi.md#delete_log_grafanacloudlogs) | **DELETE** /service/{service_id}/version/{version_id}/logging/grafanacloudlogs/{logging_grafanacloudlogs_name} | Delete the Grafana Cloud Logs log endpoint
[**get_log_grafanacloudlogs**](LoggingGrafanacloudlogsApi.md#get_log_grafanacloudlogs) | **GET** /service/{service_id}/version/{version_id}/logging/grafanacloudlogs/{logging_grafanacloudlogs_name} | Get a Grafana Cloud Logs log endpoint
[**list_log_grafanacloudlogs**](LoggingGrafanacloudlogsApi.md#list_log_grafanacloudlogs) | **GET** /service/{service_id}/version/{version_id}/logging/grafanacloudlogs | List Grafana Cloud Logs log endpoints
[**update_log_grafanacloudlogs**](LoggingGrafanacloudlogsApi.md#update_log_grafanacloudlogs) | **PUT** /service/{service_id}/version/{version_id}/logging/grafanacloudlogs/{logging_grafanacloudlogs_name} | Update a Grafana Cloud Logs log endpoint



## create_log_grafanacloudlogs

Create a Grafana Cloud Logs logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogGrafanacloudlogsParams {
    // parameters
};
create_log_grafanacloudlogs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/). |  |
**log_processing_region** | Option\<**String**> | The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global. |  |[default to LogProcessingRegion_None]
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**user** | Option\<**String**> | The Grafana Cloud Logs Dataset you want to log to. |  |
**url** | Option\<**String**> | The URL of the Loki instance in your Grafana stack. |  |
**token** | Option\<**String**> | The Grafana Access Policy token with `logs:write` access scoped to your Loki instance. |  |
**index** | Option\<**String**> | The Stream Labels, a JSON string used to identify the stream. |  |

### Return type

[**crate::models::LoggingGrafanacloudlogsResponse**](LoggingGrafanacloudlogsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_grafanacloudlogs

Delete the Grafana Cloud Logs logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogGrafanacloudlogsParams {
    // parameters
};
delete_log_grafanacloudlogs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_grafanacloudlogs_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_grafanacloudlogs

Get the details of a Grafana Cloud Logs logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogGrafanacloudlogsParams {
    // parameters
};
get_log_grafanacloudlogs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_grafanacloudlogs_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingGrafanacloudlogsResponse**](LoggingGrafanacloudlogsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_grafanacloudlogs

List all of the Grafana Cloud Logs logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogGrafanacloudlogsParams {
    // parameters
};
list_log_grafanacloudlogs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingGrafanacloudlogsResponse&gt;**](LoggingGrafanacloudlogsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_grafanacloudlogs

Update a Grafana Cloud Logs logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogGrafanacloudlogsParams {
    // parameters
};
update_log_grafanacloudlogs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_grafanacloudlogs_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://www.fastly.com/documentation/guides/integrations/streaming-logs/custom-log-formats/). |  |
**log_processing_region** | Option\<**String**> | The geographic region where the logs will be processed before streaming. Valid values are `us`, `eu`, and `none` for global. |  |[default to LogProcessingRegion_None]
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**user** | Option\<**String**> | The Grafana Cloud Logs Dataset you want to log to. |  |
**url** | Option\<**String**> | The URL of the Loki instance in your Grafana stack. |  |
**token** | Option\<**String**> | The Grafana Access Policy token with `logs:write` access scoped to your Loki instance. |  |
**index** | Option\<**String**> | The Stream Labels, a JSON string used to identify the stream. |  |

### Return type

[**crate::models::LoggingGrafanacloudlogsResponse**](LoggingGrafanacloudlogsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

