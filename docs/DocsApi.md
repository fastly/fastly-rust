# DocsApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_docs**](DocsApi.md#get_docs) | **GET** /docs | Get Fastly API docs as structured data
[**get_docs_section**](DocsApi.md#get_docs_section) | **GET** /docs/section/{section} | Get API docs matching a section filter
[**get_docs_subject**](DocsApi.md#get_docs_subject) | **GET** /docs/subject/{subject} | Get API docs for a single subject



## get_docs

Gets all documentation associated with the Fastly API.

```rust
let cfg = &Configuration::default();
let params = GetDocsParams {
    // parameters
};
get_docs(cfg, params)
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec&lt;serde_json::Value&gt;**](SerdeJsonValue.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_docs_section

Gets all documentation associated with a given Categorical Section where `section` is a regular_expression. Passing `invert=true` will force a return of everything that does not match the given regular expression.

```rust
let cfg = &Configuration::default();
let params = GetDocsSectionParams {
    // parameters
};
get_docs_section(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section** | **String** | The section to search for. Supports regular expressions. | [required] |
**invert** | **String** | Get everything that does not match section. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_docs_subject

Gets all documentation relating to a given 'Subject'.

```rust
let cfg = &Configuration::default();
let params = GetDocsSubjectParams {
    // parameters
};
get_docs_subject(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** | The subject to search for. Supports regular expressions. | [required] |

### Return type

 (empty response body)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

