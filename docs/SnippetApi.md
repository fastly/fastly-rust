# SnippetApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snippet**](SnippetApi.md#create_snippet) | **POST** /service/{service_id}/version/{version_id}/snippet | Create a snippet
[**delete_snippet**](SnippetApi.md#delete_snippet) | **DELETE** /service/{service_id}/version/{version_id}/snippet/{snippet_name} | Delete a snippet
[**get_snippet**](SnippetApi.md#get_snippet) | **GET** /service/{service_id}/version/{version_id}/snippet/{snippet_name} | Get a versioned snippet
[**get_snippet_dynamic**](SnippetApi.md#get_snippet_dynamic) | **GET** /service/{service_id}/snippet/{snippet_id} | Get a dynamic snippet
[**list_snippets**](SnippetApi.md#list_snippets) | **GET** /service/{service_id}/version/{version_id}/snippet | List snippets
[**update_snippet**](SnippetApi.md#update_snippet) | **PUT** /service/{service_id}/version/{version_id}/snippet/{snippet_name} | Update a versioned snippet
[**update_snippet_dynamic**](SnippetApi.md#update_snippet_dynamic) | **PUT** /service/{service_id}/snippet/{snippet_id} | Update a dynamic snippet



## create_snippet

Create a snippet for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateSnippetParams {
    // parameters
};
create_snippet(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**name** | Option\<**String**> | The name for the snippet. |  |
**dynamic** | Option\<**String**> | Sets the snippet version. |  |
**_type** | Option\<**String**> | The location in generated VCL where the snippet should be placed. |  |
**content** | Option\<**String**> | The VCL code that specifies exactly what the snippet does. |  |
**priority** | Option\<**String**> | Priority determines execution order. Lower numbers execute first. |  |[default to 100]

### Return type

[**crate::models::SnippetResponse**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_snippet

Delete a specific snippet for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteSnippetParams {
    // parameters
};
delete_snippet(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**snippet_name** | **String** | The name for the snippet. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_snippet

Get a single snippet for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetSnippetParams {
    // parameters
};
get_snippet(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**snippet_name** | **String** | The name for the snippet. | [required] |

### Return type

[**crate::models::SnippetResponse**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_snippet_dynamic

Get a single dynamic snippet for a particular service.

```rust
let cfg = &Configuration::default();
let params = GetSnippetDynamicParams {
    // parameters
};
get_snippet_dynamic(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**snippet_id** | **String** | Alphanumeric string identifying a VCL Snippet. | [required] |

### Return type

[**crate::models::SnippetResponse**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_snippets

List all snippets for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListSnippetsParams {
    // parameters
};
list_snippets(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::SnippetResponse&gt;**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_snippet

Update a specific snippet for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateSnippetParams {
    // parameters
};
update_snippet(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**snippet_name** | **String** | The name for the snippet. | [required] |

### Return type

[**crate::models::SnippetResponse**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_snippet_dynamic

Update a dynamic snippet for a particular service.

```rust
let cfg = &Configuration::default();
let params = UpdateSnippetDynamicParams {
    // parameters
};
update_snippet_dynamic(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**snippet_id** | **String** | Alphanumeric string identifying a VCL Snippet. | [required] |
**name** | Option\<**String**> | The name for the snippet. |  |
**dynamic** | Option\<**String**> | Sets the snippet version. |  |
**_type** | Option\<**String**> | The location in generated VCL where the snippet should be placed. |  |
**content** | Option\<**String**> | The VCL code that specifies exactly what the snippet does. |  |
**priority** | Option\<**String**> | Priority determines execution order. Lower numbers execute first. |  |[default to 100]

### Return type

[**crate::models::SnippetResponse**](SnippetResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

