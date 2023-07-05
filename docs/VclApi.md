# VclApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_vcl**](VclApi.md#create_custom_vcl) | **POST** /service/{service_id}/version/{version_id}/vcl | Create a custom VCL file
[**delete_custom_vcl**](VclApi.md#delete_custom_vcl) | **DELETE** /service/{service_id}/version/{version_id}/vcl/{vcl_name} | Delete a custom VCL file
[**get_custom_vcl**](VclApi.md#get_custom_vcl) | **GET** /service/{service_id}/version/{version_id}/vcl/{vcl_name} | Get a custom VCL file
[**get_custom_vcl_boilerplate**](VclApi.md#get_custom_vcl_boilerplate) | **GET** /service/{service_id}/version/{version_id}/boilerplate | Get boilerplate VCL
[**get_custom_vcl_generated**](VclApi.md#get_custom_vcl_generated) | **GET** /service/{service_id}/version/{version_id}/generated_vcl | Get the generated VCL for a service
[**get_custom_vcl_generated_highlighted**](VclApi.md#get_custom_vcl_generated_highlighted) | **GET** /service/{service_id}/version/{version_id}/generated_vcl/content | Get the generated VCL with syntax highlighting
[**get_custom_vcl_highlighted**](VclApi.md#get_custom_vcl_highlighted) | **GET** /service/{service_id}/version/{version_id}/vcl/{vcl_name}/content | Get a custom VCL file with syntax highlighting
[**get_custom_vcl_raw**](VclApi.md#get_custom_vcl_raw) | **GET** /service/{service_id}/version/{version_id}/vcl/{vcl_name}/download | Download a custom VCL file
[**lint_vcl_default**](VclApi.md#lint_vcl_default) | **POST** /vcl_lint | Lint (validate) VCL using a default set of flags.
[**lint_vcl_for_service**](VclApi.md#lint_vcl_for_service) | **POST** /service/{service_id}/lint | Lint (validate) VCL using flags set for the service.
[**list_custom_vcl**](VclApi.md#list_custom_vcl) | **GET** /service/{service_id}/version/{version_id}/vcl | List custom VCL files
[**set_custom_vcl_main**](VclApi.md#set_custom_vcl_main) | **PUT** /service/{service_id}/version/{version_id}/vcl/{vcl_name}/main | Set a custom VCL file as main
[**update_custom_vcl**](VclApi.md#update_custom_vcl) | **PUT** /service/{service_id}/version/{version_id}/vcl/{vcl_name} | Update a custom VCL file



## create_custom_vcl

Upload a VCL for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateCustomVclParams {
    // parameters
};
create_custom_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**content** | Option\<**String**> | The VCL code to be included. |  |
**main** | Option\<**bool**> | Set to `true` when this is the main VCL, otherwise `false`. |  |
**name** | Option\<**String**> | The name of this VCL. |  |

### Return type

[**crate::models::VclResponse**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_custom_vcl

Delete the uploaded VCL for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteCustomVclParams {
    // parameters
};
delete_custom_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl

Get the uploaded VCL for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetCustomVclParams {
    // parameters
};
get_custom_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |
**no_content** | Option\<**String**> | Omit VCL content. |  |[default to 0]

### Return type

[**crate::models::VclResponse**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl_boilerplate

Return boilerplate VCL with the service's TTL from the [settings](/reference/api/vcl-services/settings/).

```rust
let cfg = &Configuration::default();
let params = GetCustomVclBoilerplateParams {
    // parameters
};
get_custom_vcl_boilerplate(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

**String**

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl_generated

Display the generated VCL for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetCustomVclGeneratedParams {
    // parameters
};
get_custom_vcl_generated(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::VclResponse**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl_generated_highlighted

Display the content of generated VCL with HTML syntax highlighting. Include line numbers by sending `lineno=true` as a request parameter.

```rust
let cfg = &Configuration::default();
let params = GetCustomVclGeneratedHighlightedParams {
    // parameters
};
get_custom_vcl_generated_highlighted(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::VclSyntaxHighlightingResponse**](VclSyntaxHighlightingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl_highlighted

Get the uploaded VCL for a particular service and version with HTML syntax highlighting. Include line numbers by sending `lineno=true` as a request parameter.

```rust
let cfg = &Configuration::default();
let params = GetCustomVclHighlightedParams {
    // parameters
};
get_custom_vcl_highlighted(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |

### Return type

[**crate::models::VclSyntaxHighlightingResponse**](VclSyntaxHighlightingResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_custom_vcl_raw

Download the specified VCL.

```rust
let cfg = &Configuration::default();
let params = GetCustomVclRawParams {
    // parameters
};
get_custom_vcl_raw(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |

### Return type

**String**

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## lint_vcl_default

This endpoint validates the submitted VCL against a default set of enabled flags. Consider using the `/service/{service_id}/lint` operation to validate VCL in the context of a specific service.

```rust
let cfg = &Configuration::default();
let params = LintVclDefaultParams {
    // parameters
};
lint_vcl_default(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::ValidatorResult**](ValidatorResult.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## lint_vcl_for_service

Services may have flags set by a Fastly employee or by the purchase of products as addons to the service, which modify the way VCL is interpreted by that service.  This endpoint validates the submitted VCL in the context of the specified service.

```rust
let cfg = &Configuration::default();
let params = LintVclForServiceParams {
    // parameters
};
lint_vcl_for_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::ValidatorResult**](ValidatorResult.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_custom_vcl

List the uploaded VCLs for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListCustomVclParams {
    // parameters
};
list_custom_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::VclResponse&gt;**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## set_custom_vcl_main

Set the specified VCL as the main.

```rust
let cfg = &Configuration::default();
let params = SetCustomVclMainParams {
    // parameters
};
set_custom_vcl_main(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |

### Return type

[**crate::models::VclResponse**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_custom_vcl

Update the uploaded VCL for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateCustomVclParams {
    // parameters
};
update_custom_vcl(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**vcl_name** | **String** | The name of this VCL. | [required] |
**content** | Option\<**String**> | The VCL code to be included. |  |
**main** | Option\<**bool**> | Set to `true` when this is the main VCL, otherwise `false`. |  |
**name** | Option\<**String**> | The name of this VCL. |  |

### Return type

[**crate::models::VclResponse**](VclResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

