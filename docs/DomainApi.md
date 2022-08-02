# DomainApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_domain**](DomainApi.md#check_domain) | **GET** /service/{service_id}/version/{version_id}/domain/{domain_name}/check | Validate DNS configuration for a single domain on a service
[**create_domain**](DomainApi.md#create_domain) | **POST** /service/{service_id}/version/{version_id}/domain | Add a domain name to a service
[**delete_domain**](DomainApi.md#delete_domain) | **DELETE** /service/{service_id}/version/{version_id}/domain/{domain_name} | Remove a domain from a service
[**get_domain**](DomainApi.md#get_domain) | **GET** /service/{service_id}/version/{version_id}/domain/{domain_name} | Describe a domain
[**list_domains**](DomainApi.md#list_domains) | **GET** /service/{service_id}/version/{version_id}/domain | List domains
[**update_domain**](DomainApi.md#update_domain) | **PUT** /service/{service_id}/version/{version_id}/domain/{domain_name} | Update a domain



## check_domain

Checks the status of a specific domain's DNS record for a Service Version. Returns an array in the same format as domain/check_all.

```rust
let cfg = &Configuration::default();
let params = CheckDomainParams {
    // parameters
};
check_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**domain_name** | **String** | The name of the domain or domains associated with this service. | [required] |

### Return type

[**Vec&lt;crate::models::DomainCheckItem&gt;**](DomainCheckItem.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_domain

Create a domain for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = CreateDomainParams {
    // parameters
};
create_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | The name of the domain or domains associated with this service. |  |

### Return type

[**crate::models::DomainResponse**](DomainResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_domain

Delete the domain for a particular service and versions.

```rust
let cfg = &Configuration::default();
let params = DeleteDomainParams {
    // parameters
};
delete_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**domain_name** | **String** | The name of the domain or domains associated with this service. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_domain

Get the domain for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = GetDomainParams {
    // parameters
};
get_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**domain_name** | **String** | The name of the domain or domains associated with this service. | [required] |

### Return type

[**crate::models::DomainResponse**](DomainResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_domains

List all the domains for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = ListDomainsParams {
    // parameters
};
list_domains(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |

### Return type

[**Vec&lt;crate::models::DomainResponse&gt;**](DomainResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_domain

Update the domain for a particular service and version.

```rust
let cfg = &Configuration::default();
let params = UpdateDomainParams {
    // parameters
};
update_domain(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version_id** | **i32** | Integer identifying a service version. | [required] |
**domain_name** | **String** | The name of the domain or domains associated with this service. | [required] |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | The name of the domain or domains associated with this service. |  |

### Return type

[**crate::models::DomainResponse**](DomainResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

