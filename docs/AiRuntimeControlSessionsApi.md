# AiRuntimeControlSessionsApi

> [!NOTE]
> All URIs are relative to `https://api.fastly.com`

Method | HTTP request | Description
------ | ------------ | -----------
[**arc_get_session_logs**](AiRuntimeControlSessionsApi.md#arc_get_session_logs) | **GET** /ai-runtime-control/v1/sessions | Get session logs



## arc_get_session_logs

Returns session logs, including request and response data and metadata, for debugging and audit purposes.

```rust
let cfg = &Configuration::default();
let params = ArcGetSessionLogsParams {
    // parameters
};
arc_get_session_logs(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option\<**String**> | Filter results to a specific virtual key ID. |  |
**provider** | Option\<**String**> | Filter results by AI provider connection ID. |  |
**model** | Option\<**String**> | Filter results by AI model identifier. |  |
**from** | Option\<**String**> | The start of the time range, in RFC 3339 format. |  |
**to** | Option\<**String**> | The end of the time range, in RFC 3339 format. Defaults to `now`. |  |
**cursor** | Option\<**String**> | Cursor for pagination. Use the `next_cursor` value from a previous response to request the next page of results, or the `prev_cursor` value to request the previous page.  |  |
**limit** | Option\<**i32**> | Limit how many results are returned. |  |[default to 100]
**sort** | Option\<**String**> | Sort field. Prefix with `-` for descending order (e.g., `sort=-created_at`).  |  |[default to created_at]

### Return type

[**crate::models::ArcSessionsResponse**](ArcSessionsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

