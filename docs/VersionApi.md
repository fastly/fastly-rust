# VersionApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_service_version**](VersionApi.md#activate_service_version) | **PUT** /service/{service_id}/version/{version_id}/activate | Activate a service version
[**clone_service_version**](VersionApi.md#clone_service_version) | **PUT** /service/{service_id}/version/{version_id}/clone | Clone a service version
[**create_service_version**](VersionApi.md#create_service_version) | **POST** /service/{service_id}/version | Create a service version
[**deactivate_service_version**](VersionApi.md#deactivate_service_version) | **PUT** /service/{service_id}/version/{version_id}/deactivate | Deactivate a service version
[**get_service_version**](VersionApi.md#get_service_version) | **GET** /service/{service_id}/version/{version_id} | Get a version of a service
[**list_service_versions**](VersionApi.md#list_service_versions) | **GET** /service/{service_id}/version | List versions of a service
[**lock_service_version**](VersionApi.md#lock_service_version) | **PUT** /service/{service_id}/version/{version_id}/lock | Lock a service version
[**update_service_version**](VersionApi.md#update_service_version) | **PUT** /service/{service_id}/version/{version_id} | Update a service version
[**validate_service_version**](VersionApi.md#validate_service_version) | **GET** /service/{service_id}/version/{version_id}/validate | Validate a service version



## activate_service_version

Activate the current version.

```rust
let cfg = &Configuration::default();
let params = ActivateServiceVersionParams {
    // parameters
};
activate_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::VersionResponse**](VersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## clone_service_version

Clone the current configuration into a new version.

```rust
let cfg = &Configuration::default();
let params = CloneServiceVersionParams {
    // parameters
};
clone_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::Version**](Version.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_service_version

Create a version for a particular service.

```rust
let cfg = &Configuration::default();
let params = CreateServiceVersionParams {
    // parameters
};
create_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::VersionCreateResponse**](VersionCreateResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## deactivate_service_version

Deactivate the current version.

```rust
let cfg = &Configuration::default();
let params = DeactivateServiceVersionParams {
    // parameters
};
deactivate_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::VersionResponse**](VersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_service_version

Get the version for a particular service.

```rust
let cfg = &Configuration::default();
let params = GetServiceVersionParams {
    // parameters
};
get_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::VersionResponse**](VersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_service_versions

List the versions for a particular service.

```rust
let cfg = &Configuration::default();
let params = ListServiceVersionsParams {
    // parameters
};
list_service_versions(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**Vec&lt;crate::models::VersionResponse&gt;**](VersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## lock_service_version

Locks the specified version.

```rust
let cfg = &Configuration::default();
let params = LockServiceVersionParams {
    // parameters
};
lock_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::Version**](Version.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_service_version

Update a particular version for a particular service.

```rust
let cfg = &Configuration::default();
let params = UpdateServiceVersionParams {
    // parameters
};
update_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**active** | Option\<**bool**> | Whether this is the active version or not. |  |[default to false]
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**deployed** | Option\<**bool**> | Unused at this time. |  |
**locked** | Option\<**bool**> | Whether this version is locked or not. Objects can not be added or edited on locked versions. |  |[default to false]
**number** | Option\<**i32**> | The number of this version. |  |
**staging** | Option\<**bool**> | Unused at this time. |  |[default to false]
**testing** | Option\<**bool**> | Unused at this time. |  |[default to false]

### Return type

[**crate::models::VersionResponse**](VersionResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## validate_service_version

Validate the version for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ValidateServiceVersionParams {
    // parameters
};
validate_service_version(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

