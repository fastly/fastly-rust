# HealthcheckApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_healthcheck**](HealthcheckApi.md#create_healthcheck) | **POST** /service/{service_id}/version/{version_id}/healthcheck | Create a health check
[**delete_healthcheck**](HealthcheckApi.md#delete_healthcheck) | **DELETE** /service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name} | Delete a health check
[**get_healthcheck**](HealthcheckApi.md#get_healthcheck) | **GET** /service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name} | Get a health check
[**list_healthchecks**](HealthcheckApi.md#list_healthchecks) | **GET** /service/{service_id}/version/{version_id}/healthcheck | List health checks
[**update_healthcheck**](HealthcheckApi.md#update_healthcheck) | **PUT** /service/{service_id}/version/{version_id}/healthcheck/{healthcheck_name} | Update a health check



## create_healthcheck

Create a health check for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateHealthcheckParams {
    // parameters
};
create_healthcheck(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**check_interval** | Option\<**i32**> | How often to run the health check in milliseconds. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**expected_response** | Option\<**i32**> | The status code expected from the host. |  |
**headers** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of custom headers that will be added to the health check probes. |  |
**host** | Option\<**String**> | Which host to check. |  |
**http_version** | Option\<**String**> | Whether to use version 1.0 or 1.1 HTTP. |  |
**initial** | Option\<**i32**> | When loading a config, the initial number of probes to be seen as OK. |  |
**method** | Option\<**String**> | Which HTTP method to use. |  |
**name** | Option\<**String**> | The name of the health check. |  |
**path** | Option\<**String**> | The path to check. |  |
**threshold** | Option\<**i32**> | How many health checks must succeed to be considered healthy. |  |
**timeout** | Option\<**i32**> | Timeout in milliseconds. |  |
**window** | Option\<**i32**> | The number of most recent health check queries to keep for this health check. |  |

### Return type

[**crate::models::HealthcheckResponse**](HealthcheckResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_healthcheck

Delete the health check for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = DeleteHealthcheckParams {
    // parameters
};
delete_healthcheck(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**healthcheck_name** | **String** | The name of the health check. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_healthcheck

Get the health check for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetHealthcheckParams {
    // parameters
};
get_healthcheck(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**healthcheck_name** | **String** | The name of the health check. | [required] |

### Return type

[**crate::models::HealthcheckResponse**](HealthcheckResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_healthchecks

List all of the health checks for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListHealthchecksParams {
    // parameters
};
list_healthchecks(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::HealthcheckResponse&gt;**](HealthcheckResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_healthcheck

Update the health check for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateHealthcheckParams {
    // parameters
};
update_healthcheck(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**healthcheck_name** | **String** | The name of the health check. | [required] |
**check_interval** | Option\<**i32**> | How often to run the health check in milliseconds. |  |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**expected_response** | Option\<**i32**> | The status code expected from the host. |  |
**headers** | Option\<[**Vec&lt;String&gt;**](String.md)> | Array of custom headers that will be added to the health check probes. |  |
**host** | Option\<**String**> | Which host to check. |  |
**http_version** | Option\<**String**> | Whether to use version 1.0 or 1.1 HTTP. |  |
**initial** | Option\<**i32**> | When loading a config, the initial number of probes to be seen as OK. |  |
**method** | Option\<**String**> | Which HTTP method to use. |  |
**name** | Option\<**String**> | The name of the health check. |  |
**path** | Option\<**String**> | The path to check. |  |
**threshold** | Option\<**i32**> | How many health checks must succeed to be considered healthy. |  |
**timeout** | Option\<**i32**> | Timeout in milliseconds. |  |
**window** | Option\<**i32**> | The number of most recent health check queries to keep for this health check. |  |

### Return type

[**crate::models::HealthcheckResponse**](HealthcheckResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

