# ServiceApi

All URIs are relative to *https://api.fastly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service**](ServiceApi.md#create_service) | **POST** /service | Create a service
[**delete_service**](ServiceApi.md#delete_service) | **DELETE** /service/{service_id} | Delete a service
[**get_service**](ServiceApi.md#get_service) | **GET** /service/{service_id} | Get a service
[**get_service_detail**](ServiceApi.md#get_service_detail) | **GET** /service/{service_id}/details | Get service details
[**list_service_domains**](ServiceApi.md#list_service_domains) | **GET** /service/{service_id}/domain | List the domains within a service
[**list_services**](ServiceApi.md#list_services) | **GET** /service | List services
[**search_service**](ServiceApi.md#search_service) | **GET** /service/search | Search for a service by name
[**update_service**](ServiceApi.md#update_service) | **PUT** /service/{service_id} | Update a service



## create_service

Create a service.

```rust
let cfg = &Configuration::default();
let params = CreateServiceParams {
    // parameters
};
create_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | The name of the service. |  |
**customer_id** | Option\<**String**> | Alphanumeric string identifying the customer. |  |
**_type** | Option\<**String**> | The type of this service. |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_service

Delete a service.

```rust
let cfg = &Configuration::default();
let params = DeleteServiceParams {
    // parameters
};
delete_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::InlineResponse200**](InlineResponse200.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_service

Get a specific service by id.

```rust
let cfg = &Configuration::default();
let params = GetServiceParams {
    // parameters
};
get_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_service_detail

List detailed information on a specified service.

```rust
let cfg = &Configuration::default();
let params = GetServiceDetailParams {
    // parameters
};
get_service_detail(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**version** | Option\<**i32**> | Number identifying a version of the service. |  |

### Return type

[**crate::models::ServiceDetail**](ServiceDetail.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_service_domains

List the domains within a service.

```rust
let cfg = &Configuration::default();
let params = ListServiceDomainsParams {
    // parameters
};
list_service_domains(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |

### Return type

[**Vec&lt;crate::models::DomainResponse&gt;**](DomainResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_services

List services.

```rust
let cfg = &Configuration::default();
let params = ListServicesParams {
    // parameters
};
list_services(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option\<**i32**> | Current page. |  |
**per_page** | Option\<**i32**> | Number of records per page. |  |[default to 20]
**sort** | Option\<**String**> | Field on which to sort. |  |[default to created]
**direction** | Option\<**String**> | Direction in which to sort results. |  |[default to ascend]

### Return type

[**Vec&lt;crate::models::ServiceListResponse&gt;**](ServiceListResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## search_service

Get a specific service by name.

```rust
let cfg = &Configuration::default();
let params = SearchServiceParams {
    // parameters
};
search_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the service. | [required] |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_service

Update a service.

```rust
let cfg = &Configuration::default();
let params = UpdateServiceParams {
    // parameters
};
update_service(cfg, params)
```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** | Alphanumeric string identifying the service. | [required] |
**comment** | Option\<**String**> | A freeform descriptive note. |  |
**name** | Option\<**String**> | The name of the service. |  |
**customer_id** | Option\<**String**> | Alphanumeric string identifying the customer. |  |

### Return type

[**crate::models::ServiceResponse**](ServiceResponse.md)

### Authorization

[token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

