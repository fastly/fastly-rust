# LoggingNewrelicotlpApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**create_log_newrelicotlp**](LoggingNewrelicotlpApi.md#create_log_newrelicotlp) | **POST** /service/{service_id}/version/{version_id}/logging/newrelicotlp | Create a New Relic OTLP endpoint
[**delete_log_newrelicotlp**](LoggingNewrelicotlpApi.md#delete_log_newrelicotlp) | **DELETE** /service/{service_id}/version/{version_id}/logging/newrelicotlp/{logging_newrelicotlp_name} | Delete a New Relic OTLP endpoint
[**get_log_newrelicotlp**](LoggingNewrelicotlpApi.md#get_log_newrelicotlp) | **GET** /service/{service_id}/version/{version_id}/logging/newrelicotlp/{logging_newrelicotlp_name} | Get a New Relic OTLP endpoint
[**list_log_newrelicotlp**](LoggingNewrelicotlpApi.md#list_log_newrelicotlp) | **GET** /service/{service_id}/version/{version_id}/logging/newrelicotlp | List New Relic OTLP endpoints
[**update_log_newrelicotlp**](LoggingNewrelicotlpApi.md#update_log_newrelicotlp) | **PUT** /service/{service_id}/version/{version_id}/logging/newrelicotlp/{logging_newrelicotlp_name} | Update a New Relic log endpoint



## create_log_newrelicotlp

Create a New Relic OTLP logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateLogNewrelicotlpParams {
    // parameters
};
create_log_newrelicotlp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to {"timestamp":"%{begin:%Y-%m-%dT%H:%M:%S}t","time_elapsed":"%{time.elapsed.usec}V","is_tls":"%{if(req.is_ssl, \"true\", \"false\")}V","client_ip":"%{req.http.Fastly-Client-IP}V","geo_city":"%{client.geo.city}V","geo_country_code":"%{client.geo.country_code}V","request":"%{req.request}V","host":"%{req.http.Fastly-Orig-Host}V","url":"%{json.escape(req.url)}V","request_referer":"%{json.escape(req.http.Referer)}V","request_user_agent":"%{json.escape(req.http.User-Agent)}V","request_accept_language":"%{json.escape(req.http.Accept-Language)}V","request_accept_charset":"%{json.escape(req.http.Accept-Charset)}V","cache_status":"%{regsub(fastly_info.state, \"^(HIT-(SYNTH)|(HITPASS|HIT|MISS|PASS|ERROR|PIPE)).*\", \"\\2\\3\") }V"}]
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**token** | Option\<**String**> | The Insert API key from the Account page of your New Relic account. Required. |  |
**region** | Option\<**String**> | The region to which to stream logs. |  |[default to Region_US]
**url** | Option\<**String**> | (Optional) URL of the New Relic Trace Observer, if you are using New Relic Infinite Tracing. |  |[default to null]

### Return type

[**crate::models::LoggingNewrelicotlpResponse**](LoggingNewrelicotlpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_log_newrelicotlp

Delete the New Relic OTLP logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteLogNewrelicotlpParams {
    // parameters
};
delete_log_newrelicotlp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_newrelicotlp_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_log_newrelicotlp

Get the details of a New Relic OTLP logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetLogNewrelicotlpParams {
    // parameters
};
get_log_newrelicotlp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_newrelicotlp_name** | **String** | The name for the real-time logging configuration. | [required] |

### Return type

[**crate::models::LoggingNewrelicotlpResponse**](LoggingNewrelicotlpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_log_newrelicotlp

List all of the New Relic OTLP logging objects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListLogNewrelicotlpParams {
    // parameters
};
list_log_newrelicotlp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::LoggingNewrelicotlpResponse&gt;**](LoggingNewrelicotlpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_log_newrelicotlp

Update a New Relic OTLP logging object for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateLogNewrelicotlpParams {
    // parameters
};
update_log_newrelicotlp(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**logging_newrelicotlp_name** | **String** | The name for the real-time logging configuration. | [required] |
**name** | Option\<**String**> | The name for the real-time logging configuration. |  |
**placement** | Option\<**String**> | Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`.  |  |
**response_condition** | Option\<**String**> | The name of an existing condition in the configured endpoint, or leave blank to always execute. |  |
**format** | Option\<**String**> | A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats). |  |[default to {"timestamp":"%{begin:%Y-%m-%dT%H:%M:%S}t","time_elapsed":"%{time.elapsed.usec}V","is_tls":"%{if(req.is_ssl, \"true\", \"false\")}V","client_ip":"%{req.http.Fastly-Client-IP}V","geo_city":"%{client.geo.city}V","geo_country_code":"%{client.geo.country_code}V","request":"%{req.request}V","host":"%{req.http.Fastly-Orig-Host}V","url":"%{json.escape(req.url)}V","request_referer":"%{json.escape(req.http.Referer)}V","request_user_agent":"%{json.escape(req.http.User-Agent)}V","request_accept_language":"%{json.escape(req.http.Accept-Language)}V","request_accept_charset":"%{json.escape(req.http.Accept-Charset)}V","cache_status":"%{regsub(fastly_info.state, \"^(HIT-(SYNTH)|(HITPASS|HIT|MISS|PASS|ERROR|PIPE)).*\", \"\\2\\3\") }V"}]
**format_version** | Option\<**i32**> | The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`.  |  |[default to FormatVersion_v2]
**token** | Option\<**String**> | The Insert API key from the Account page of your New Relic account. Required. |  |
**region** | Option\<**String**> | The region to which to stream logs. |  |[default to Region_US]
**url** | Option\<**String**> | (Optional) URL of the New Relic Trace Observer, if you are using New Relic Infinite Tracing. |  |[default to null]

### Return type

[**crate::models::LoggingNewrelicotlpResponse**](LoggingNewrelicotlpResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

