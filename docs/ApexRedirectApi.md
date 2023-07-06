# ApexRedirectApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_apex_redirect**](ApexRedirectApi.md#create_apex_redirect) | **POST** /service/{service_id}/version/{version_id}/apex-redirects | Create an apex redirect
[**delete_apex_redirect**](ApexRedirectApi.md#delete_apex_redirect) | **DELETE** /apex-redirects/{apex_redirect_id} | Delete an apex redirect
[**get_apex_redirect**](ApexRedirectApi.md#get_apex_redirect) | **GET** /apex-redirects/{apex_redirect_id} | Get an apex redirect
[**list_apex_redirects**](ApexRedirectApi.md#list_apex_redirects) | **GET** /service/{service_id}/version/{version_id}/apex-redirects | List apex redirects
[**update_apex_redirect**](ApexRedirectApi.md#update_apex_redirect) | **PUT** /apex-redirects/{apex_redirect_id} | Update an apex redirect



## create_apex_redirect

Create an apex redirect for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateApexRedirectParams {
    // parameters
};
create_apex_redirect(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**service_id2** | Option\<**String**> |  |  |
**version** | Option\<**i32**> |  |  |
**created_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**deleted_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**updated_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**status_code** | Option\<**i32**> | HTTP status code used to redirect the client. |  |
**domains** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of apex domains that should redirect to their WWW subdomain. |  |
**feature_revision** | Option\<**i32**> | Revision number of the apex redirect feature implementation. Defaults to the most recent revision. |  |

### Return type

[**crate::models::ApexRedirect**](ApexRedirect.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_apex_redirect

Delete an apex redirect by its ID.

```rust
let cfg = &Configuration::default();
let params = DeleteApexRedirectParams {
    // parameters
};
delete_apex_redirect(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apex_redirect_id** | **String** |  | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_apex_redirect

Get an apex redirect by its ID.

```rust
let cfg = &Configuration::default();
let params = GetApexRedirectParams {
    // parameters
};
get_apex_redirect(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apex_redirect_id** | **String** |  | [required] |

### Return type

[**crate::models::ApexRedirect**](ApexRedirect.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_apex_redirects

List all apex redirects for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListApexRedirectsParams {
    // parameters
};
list_apex_redirects(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::ApexRedirect&gt;**](ApexRedirect.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_apex_redirect

Update an apex redirect by its ID.

```rust
let cfg = &Configuration::default();
let params = UpdateApexRedirectParams {
    // parameters
};
update_apex_redirect(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apex_redirect_id** | **String** |  | [required] |
**service_id** | Option\<**String**> |  |  |
**version** | Option\<**i32**> |  |  |
**created_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**deleted_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**updated_at** | Option\<**String**> | Date and time in ISO 8601 format. |  |
**status_code** | Option\<**i32**> | HTTP status code used to redirect the client. |  |
**domains** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of apex domains that should redirect to their WWW subdomain. |  |
**feature_revision** | Option\<**i32**> | Revision number of the apex redirect feature implementation. Defaults to the most recent revision. |  |

### Return type

[**crate::models::ApexRedirect**](ApexRedirect.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

