# ContentApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_check**](ContentApi.md#content_check) | **GET** /content/edge_check | Check status of content in each POP's cache



## content_check

Retrieve headers and MD5 hash of the content for a particular URL from each Fastly edge server. This API is limited to 200 requests per hour.

```rust
let cfg = &Configuration::default();
let params = ContentCheckParams {
    // parameters
};
content_check(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | Option\<**String**> | Full URL (host and path) to check on all nodes. if protocol is omitted, http will be assumed. |  |

### Return type

[**Vec&lt;crate::models::Content&gt;**](Content.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

