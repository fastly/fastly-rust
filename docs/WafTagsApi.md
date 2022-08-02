# WafTagsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_waf_tags**](WafTagsApi.md#list_waf_tags) | **GET** /waf/tags | List tags



## list_waf_tags

List all tags.

```rust
let cfg = &Configuration::default();
let params = ListWafTagsParams {
    // parameters
};
list_waf_tags(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_name** | Option\<**String**> | Limit the returned tags to a specific name. |  |
**page_number** | Option\<**i32**> | Current page. |  |
**page_size** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**include** | Option\<**String**> | Include relationships. Optional. |  |[default to waf_rules]

### Return type

[**crate::models::WafTagsResponse**](WafTagsResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

