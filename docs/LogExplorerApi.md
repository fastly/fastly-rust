# LogExplorerApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**get_log_records**](LogExplorerApi.md#get_log_records) | **GET** /observability/log-explorer | Retrieve log records



## get_log_records

Retrieves log records.

```rust
let cfg = &Configuration::default();
let params = GetLogRecordsParams {
    // parameters
};
get_log_records(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**start** | **String** |  | [required] |
**end** | **String** |  | [required] |
**limit** | Option\<**f32**> |  |  |
**next_cursor** | Option\<**String**> |  |  |
**filter** | Option\<**String**> |  |  |

### Return type

[**crate::models::GetLogRecordsResponse**](GetLogRecordsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

