# LoggingEndpointErrorsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_log_endpoint_errors**](LoggingEndpointErrorsApi.md#get_log_endpoint_errors) | **GET** /observability/service/{service_id}/logging/errors | Stream Log Endpoint Errors



## get_log_endpoint_errors

Provides a near real-time stream of log errors through a hybrid short-polling model. A client should make an initial request using the `from` parameter to specify a start time. The `to` parameter should be used alongside the `from` parameter since the default bucket is 10 seconds.  For pagination, use the URLs provided in the Link header of the response. These contain updated `from` timestamps for retrieving the next or previous page of logs.  Defaults to `application/x-ndjson` format. Use `Accept: application/json` header to request standard JSON array format instead. 

```rust
let cfg = &Configuration::default();
let params = GetLogEndpointErrorsParams {
    // parameters
};
get_log_endpoint_errors(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**from** | Option\<**i64**> |  |  |
**to** | Option\<**i64**> |  |  |
**filter_endpoint** | Option\<**String**> |  |  |

### Return type

**String**

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

